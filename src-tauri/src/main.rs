// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cell::RefCell, thread::sleep};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use serde::Serialize;

struct Settings {
    amplifier: f32,
    users: Vec<User>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct User {
    id: u8,
    name: String,
    pos: Pos,
    is_current: bool, // TODO must be camal case or something 
}

#[derive(Serialize, Clone)]
struct Pos {
    x: f32,
    y: f32,
}

#[derive(Default)]
struct AppState(std::sync::Arc<std::sync::Mutex<Option<Settings>>>);

fn main() {
    let host = cpal::default_host();

    // Find devices.
    let input_device = host.default_input_device()
    	.expect("failed to find input device");

    let output_device = host.default_output_device()
    	.expect("failed to find output device");

    println!("Using input device: \"{}\"", input_device.name().unwrap());
    println!("Using output device: \"{}\"", output_device.name().unwrap());

    // We'll try and use the same configuration between streams to keep it simple.
    // TODO might be changing valume of eveeryint else
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

    // TODO like in 270, each person gets a time split. this is know known and used. then they are amplified accordingly
    // TODO need to change the amplification

    // get the current path
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let mut state = AppState(std::sync::Arc::new(std::sync::Mutex::new(Some(Settings { amplifier: 1.0, users: fetch_users() }))));

    // oh its between threads
    // create a mutex
    // cloning a reference?
    let r = std::sync::Arc::clone(&state.0);
    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        // get the amplifier from the settings or default to 1.0
        let amp = if let Some(settings) = & *r.lock().unwrap() {
            settings.amplifier
        } else {
            1.0
        };
        // fill the output buffer with samples from the ring buffer
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => {
                    s*amp
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
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_amplifier, set_amplifier, get_users])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");

	drop(input_stream);
    drop(output_stream);
}

// dont really need this, but being consistant
#[tauri::command]
fn get_amplifier() -> f32 {
    1.0
}

#[tauri::command]
fn set_amplifier(state: tauri::State<'_, AppState>, value: f32) {
    if let Some(settings) = &mut *state.0.lock().unwrap() {
        settings.amplifier = value;
        println!("Amplifier: {}", value);
    } else {
        println!("Settings is None, can't set amplifier");
    }
}

#[tauri::command]
fn get_users(state: tauri::State<'_, AppState>) -> Vec<User> {
    state.0.lock().unwrap().as_ref().unwrap().users.clone()
}

fn fetch_users() -> Vec<User> {
    vec![
        User { id: 1, name: "John".to_string(), pos: Pos { x: 100.0, y: 100.0 }, is_current: false },
        User { id: 2, name: "Jane".to_string(), pos: Pos { x: 200.0, y: 200.0 }, is_current: false },
        User { id: 3, name: "Joey".to_string(), pos: Pos { x: 200.0, y: 250.0 }, is_current: true },
    ]
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
