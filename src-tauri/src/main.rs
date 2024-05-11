// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
	#[cfg(any(
        not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        )),
        not(feature = "jack")
    ))]
    let host = cpal::default_host();

    // Find devices.
    let input_device = host.default_input_device()
    	.expect("failed to find input device");

    let output_device = host.default_output_device()
    	.expect("failed to find output device");

    println!("Using input device: \"{}\"", input_device.name().unwrap());
    println!("Using output device: \"{}\"", output_device.name().unwrap());

    // We'll try and use the same configuration between streams to keep it simple.
    let config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();

    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (150.0 / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // The buffer to share samples
    let ring = ringbuf::HeapRb::<f32>::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        // push the samples into the ring buffer
        for &sample in data {
            if producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    // create a file
    let data = "samples\n";
    let mut f = std::fs::File::create("samples.csv").expect("Unable to create file");
    std::io::Write::write_all(&mut f, data.as_bytes()).expect("Unable to write data");

    // get the current path
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let mut amplifier = 5.0;
    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        // fill the output buffer with samples from the ring buffer
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => {
                    // write the sample to the file
                    std::io::Write::write_all(&mut f, s.to_string().as_bytes()).expect("Unable to write data");
                    std::io::Write::write_all(&mut f, "\n".as_bytes()).expect("Unable to write data");
                    s*amplifier
                },
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };

    // Build streams.
    println!("Attempting to build both streams with f32 samples and `{config:?}`.");
    let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn, None).unwrap();
    let output_stream = output_device.build_output_stream(&config, output_data_fn, err_fn, None).unwrap();
    println!("Successfully built streams.");

    // Play the streams.
    println!("Starting the input and output streams with `{}` milliseconds of latency.", 150.0);
    input_stream.play().unwrap();
    output_stream.play().unwrap();

	tauri::Builder::default()
		.run(tauri::generate_context!())
		.expect("error while running tauri application");

	drop(input_stream);
    drop(output_stream);
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
