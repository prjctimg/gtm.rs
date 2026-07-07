use std::fs::File;
use std::num::{NonZeroU16, NonZeroU32};
use std::time::Duration;

use rodio::Source;
use symphonia::core::codecs::registry::CodecRegistry;
use symphonia::core::codecs::audio::{AudioDecoder, AudioDecoderOptions};
use symphonia::core::codecs::CodecParameters;
use symphonia::core::formats::probe::{Hint, Probe};
use symphonia::core::formats::{FormatOptions, FormatReader, TrackType};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::units::Timestamp;

use crate::backend::{AudioError, AudioResult};

fn build_codec_registry() -> CodecRegistry {
    let mut registry = CodecRegistry::new();
    symphonia::default::register_enabled_codecs(&mut registry);
    registry.register_audio_decoder::<symphonia_adapter_libopus::OpusDecoder>();
    registry
}

fn build_probe() -> Probe {
    let mut probe = Probe::new();
    symphonia::default::register_enabled_formats(&mut probe);
    probe
}

/// A `rodio::Source` backed by symphonia's own decoder pipeline.
pub struct SymphoniaSource {
    reader: Box<dyn FormatReader>,
    track_id: u32,
    codec_params: symphonia::core::codecs::audio::AudioCodecParameters,
    opts: AudioDecoderOptions,
    decoder: Box<dyn AudioDecoder>,
    buffer: Vec<f32>,
    buffer_pos: usize,
    channels: NonZeroU16,
    sample_rate: NonZeroU32,
    eof: bool,
    duration: f64,
}

impl SymphoniaSource {
    pub fn from_file(path: &str, _start_pos: f64) -> AudioResult<Box<dyn Source<Item = f32> + Send>> {
        let file = File::open(path).map_err(|e| AudioError::OpenFailed(e.to_string()))?;
        let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());
        let probe = build_probe();
        let hint = Hint::new();
        let fmt_opts = FormatOptions::default();
        let meta_opts = MetadataOptions::default();
        let reader = probe
            .probe(&hint, mss, fmt_opts, meta_opts)
            .map_err(|e| AudioError::DecodeError(format!("probe failed: {e}")))?;

        let track = reader
            .default_track(TrackType::Audio)
            .ok_or_else(|| AudioError::UnsupportedFormat("no audio track found".into()))?;

        let codec_params = track
            .codec_params
            .as_ref()
            .and_then(|p| match p {
                CodecParameters::Audio(a) => Some(a.clone()),
                _ => None,
            })
            .ok_or_else(|| AudioError::UnsupportedFormat("no audio codec params".into()))?;

        let duration = track
            .duration
            .zip(track.time_base)
            .and_then(|(d, tb)| {
                let ts = Timestamp::new(d.get() as i64);
                tb.calc_time(ts).map(|t| t.as_secs_f64())
            })
            .unwrap_or(0.0);
        let track_id = track.id;
        let opts = AudioDecoderOptions::default();

        Self::new(reader, track_id, codec_params, opts, duration)
            .map(|s| Box::new(s) as Box<dyn Source<Item = f32> + Send>)
    }

    fn new(
        reader: Box<dyn FormatReader>,
        track_id: u32,
        codec_params: symphonia::core::codecs::audio::AudioCodecParameters,
        opts: AudioDecoderOptions,
        duration: f64,
    ) -> AudioResult<Self> {
        let sample_rate =
            NonZeroU32::new(codec_params.sample_rate.unwrap_or(44100)).unwrap_or(NonZeroU32::MIN);
        let channels = codec_params
            .channels
            .as_ref()
            .map(|c| NonZeroU16::new(c.count() as u16).unwrap_or(NonZeroU16::MIN))
            .unwrap_or(NonZeroU16::MIN);
        let registry = build_codec_registry();
        let decoder = registry
            .make_audio_decoder(&codec_params, &opts)
            .map_err(|e| AudioError::DecodeError(format!("decoder init failed: {e}")))?;
        Ok(Self {
            reader,
            track_id,
            codec_params,
            opts,
            decoder,
            buffer: Vec::new(),
            buffer_pos: 0,
            channels,
            sample_rate,
            eof: false,
            duration,
        })
    }
}

impl Iterator for SymphoniaSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.buffer_pos < self.buffer.len() {
                let sample = self.buffer[self.buffer_pos];
                self.buffer_pos += 1;
                return Some(sample);
            }
            if self.eof {
                return None;
            }
            self.buffer.clear();
            self.buffer_pos = 0;
            loop {
                match self.reader.next_packet() {
                    Ok(Some(packet)) => {
                        if packet.track_id != self.track_id {
                            continue;
                        }
                        match self.decoder.decode(&packet) {
                            Ok(buf) => {
                                buf.copy_to_vec_interleaved::<f32>(&mut self.buffer);
                                break;
                            }
                            Err(symphonia::core::errors::Error::DecodeError(d)) => {
                                log::warn!("decode error: {d}");
                                continue;
                            }
                            Err(symphonia::core::errors::Error::IoError(e)) => {
                                log::error!("io error: {e}");
                                self.eof = true;
                                return None;
                            }
                            Err(symphonia::core::errors::Error::ResetRequired) => {
                                self.reader
                                    .tracks()
                                    .iter()
                                    .find(|t| t.id == self.track_id)
                                    .and_then(|t| t.codec_params.as_ref())
                                    .and_then(|p| match p {
                                        CodecParameters::Audio(a) => {
                                            self.codec_params = a.clone();
                                            let registry = build_codec_registry();
                                            match registry.make_audio_decoder(
                                                &a.clone(),
                                                &self.opts,
                                            ) {
                                                Ok(d) => {
                                                    self.decoder = d;
                                                    Some(())
                                                }
                                                Err(_) => None,
                                            }
                                        }
                                        _ => None,
                                    });
                                continue;
                            }
                            Err(e) => {
                                log::error!("unrecoverable error: {e}");
                                self.eof = true;
                                return None;
                            }
                        }
                    }
                    Ok(None) => {
                        self.eof = true;
                        return None;
                    }
                    Err(e) => {
                        log::error!("packet read error: {e}");
                        self.eof = true;
                        return None;
                    }
                }
            }
        }
    }
}

impl Source for SymphoniaSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> NonZeroU16 {
        self.channels
    }

    fn sample_rate(&self) -> NonZeroU32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        if self.duration > 0.0 {
            Some(Duration::from_secs_f64(self.duration))
        } else {
            None
        }
    }
}
