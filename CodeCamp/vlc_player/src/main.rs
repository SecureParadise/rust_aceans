use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use eframe::egui;
use symphonia::core::audio::AudioBufferRef;
use symphonia::core::audio::Signal;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

struct MediaPlayer {
    audio_stream: Option<cpal::Stream>,
}

impl Default for MediaPlayer {
    fn default() -> Self {
        Self { audio_stream: None }
    }
}

impl eframe::App for MediaPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open File").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.load_audio(&path);
                }
            }
        });
    }
}

impl MediaPlayer {
    fn load_audio(&mut self, path: &std::path::Path) {
        let src = std::fs::File::open(path).unwrap();
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let hint = Hint::new();
        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        let decoder_opts = DecoderOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .unwrap();
        let mut format_reader = probed.format;

        let track = format_reader.default_track().unwrap();
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)
            .unwrap();

        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        let mut packet_store = Vec::new();

        self.audio_stream = Some(
            device
                .build_output_stream(
                    &config.config(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        if packet_store.is_empty() {
                            if let Ok(packet) = format_reader.next_packet() {
                                packet_store.push(packet);
                            }
                        }

                        if let Some(packet) = packet_store.pop() {
                            if let Ok(buffer) = decoder.decode(&packet) {
                                match buffer {
                                    AudioBufferRef::F32(buf) => {
                                        let samples = buf.chan(0);
                                        let len = samples.len().min(data.len());
                                        data[..len].copy_from_slice(&samples[..len]);
                                    }
                                    AudioBufferRef::F64(buf) => {
                                        let samples = buf.chan(0);
                                        for (i, sample) in
                                            samples.iter().take(data.len()).enumerate()
                                        {
                                            data[i] = *sample as f32;
                                        }
                                    }
                                    AudioBufferRef::U8(buf) => {
                                        let samples = buf.chan(0);
                                        for (i, sample) in
                                            samples.iter().take(data.len()).enumerate()
                                        {
                                            data[i] = (*sample as f32 - 128.0) / 128.0;
                                        }
                                    }
                                    AudioBufferRef::U16(buf) => {
                                        let samples = buf.chan(0);
                                        for (i, sample) in
                                            samples.iter().take(data.len()).enumerate()
                                        {
                                            data[i] = (*sample as f32 - 32768.0) / 32768.0;
                                        }
                                    }
                                    AudioBufferRef::U24(buf) => {
                                        let samples = buf.chan(0);
                                        for (i, sample) in
                                            samples.iter().take(data.len()).enumerate()
                                        {
                                            // Correct way to handle u24 samples
                                            let bytes = sample.to_le_bytes();
                                            let sample_i32 = i32::from_le_bytes([
                                                bytes[0], bytes[1], bytes[2], 0,
                                            ]);
                                            data[i] =
                                                (sample_i32 as f32 - 8_388_608.0) / 8_388_608.0;
                                        }
                                    }
                                    AudioBufferRef::U32(buf) => {
                                        let samples = buf.chan(0);
                                        for (i, sample) in
                                            samples.iter().take(data.len()).enumerate()
                                        {
                                            data[i] = (*sample as f32 - 2_147_483_648.0)
                                                / 2_147_483_648.0;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    },
                    |err| eprintln!("Audio error: {}", err),
                    None,
                )
                .unwrap(),
        );

        self.audio_stream.as_ref().unwrap().play().unwrap();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Media Player",
        options,
        Box::new(|_| Ok(Box::new(MediaPlayer::default()))),
    )
}
