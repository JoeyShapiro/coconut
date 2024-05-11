// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate soundio;

use std::f64::consts::PI;

struct State {
	// input_stream: soundio::InStream<'static>,
	output_stream: soundio::OutStream<'static>,
}

impl State {
	fn new() -> Result<Self, String> {
		let mut ctx = soundio::Context::new();
		ctx.set_app_name("Coconut Player");
		ctx.connect()?;
		// We have to flush events so we can scan devices.
		ctx.flush_events();

		let output_dev = ctx
			.default_output_device()
			.map_err(|_| "Error getting default output device".to_string())?;

		println!(
			"Output device: {} {}",
			output_dev.name(),
			if output_dev.is_raw() { "raw" } else { "cooked" }
		);

		let mut sine = SineWavePlayer {
			phase: 0.0,
			amplitude: 0.3,
			frequency: 200.0,
		};

		let output_stream = output_dev.open_outstream(
			48000,
			soundio::Format::Float32LE,
			soundio::ChannelLayout::get_default(2),
			0.5,
			move |x| sine.write_callback(x),
			None::<fn()>,
			None::<fn(soundio::Error)>,
		)?;

		Ok(Self {
			// input_stream: soundio::InStream::new(),
			output_stream,
		})
	}

	fn start(&mut self) -> Result<(), String> {
		self.output_stream.start()?;
		Ok(())
	}
}

fn main() {
	// match State::new().un {
	// 	Ok(_) => ,
	// 	Err(x) => println!("Error initializing audio: {}", x),
	// };
	let mut state = State::new().unwrap();
	// do run in new thread
	std::thread::spawn(move || {
		match run_player() {
			Ok(_) => (),
			Err(x) => println!("Error running audio: {}", x),
		}
	});

	tauri::Builder::default()
	.run(tauri::generate_context!())
	.expect("error while running tauri application");
}

struct SineWavePlayer {
    phase: f64, // Phase is updated each time the write callback is called.
    frequency: f64,
    amplitude: f64, // TODO: For some reason amplitude close to 1 (maybe > 0.99?) and high frequency (e.g. 18 kHz) gives weird low frequency aliasing or something.
}

impl SineWavePlayer {
    fn write_callback(&mut self, stream: &mut soundio::OutStreamWriter) {
        let mut frames_left = stream.frame_count_max();

        loop {
            if let Err(e) = stream.begin_write(frames_left) {
                println!("Error writing to stream: {}", e);
                return;
            }
            let phase_step = self.frequency / stream.sample_rate() as f64 * 2.0 * PI;

            for c in 0..stream.channel_count() {
                for f in 0..stream.frame_count() {
                    stream.set_sample(c, f, (self.phase.sin() * self.amplitude) as f32);
                    self.phase += phase_step;
                }
            }

            frames_left -= stream.frame_count();
            if frames_left <= 0 {
                break;
            }

            stream.end_write();
        }
    }
}

fn run_player() -> Result<(), String> {
	let mut ctx = soundio::Context::new();
	ctx.set_app_name("Coconut Player");
	ctx.connect()?;
	// We have to flush events so we can scan devices.
	ctx.flush_events();

	let output_dev = ctx
		.default_output_device()
		.map_err(|_| "Error getting default output device".to_string())?;

	println!(
		"Output device: {} {}",
		output_dev.name(),
		if output_dev.is_raw() { "raw" } else { "cooked" }
	);

	let mut sine = SineWavePlayer {
		phase: 0.0,
		amplitude: 0.3,
		frequency: 200.0,
	};

	let mut output_stream = output_dev.open_outstream(
		48000,
		soundio::Format::Float32LE,
		soundio::ChannelLayout::get_default(2),
		0.5,
		move |x| sine.write_callback(x),
		None::<fn()>,
		None::<fn(soundio::Error)>,
	)?;

	output_stream.start()?;

	loop {
		std::thread::sleep(std::time::Duration::from_secs(1));
	}
}
