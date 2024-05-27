// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cell::RefCell, io::Write, os::unix::thread, process::exit, thread::sleep};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::producer;
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
    // TODO add amp and stuff, so i dont have to do it every time
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
    let input_ring = ringbuf::HeapRb::<f32>::new(latency_samples * 2);
    let (mut input_producer, mut input_consumer) = input_ring.split();
    let output_ring = ringbuf::HeapRb::<f32>::new(latency_samples * 2);
    let (mut output_producer, mut output_consumer) = output_ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        output_producer.push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        println!("1: input data fn");
        let mut output_fell_behind = false;
        // push the samples into the ring buffer
        for &sample in data {
            if input_producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            eprintln!("input fn: output stream fell behind: try increasing latency: {}", input_producer.len());
            exit(1);
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
        println!("2: output data fn");
        let mut input_fell_behind = false;
        // get the amplifier from the settings or default to 1.0
        let amp = if let Some(settings) = & *r.lock().unwrap() {
            settings.amplifier
        } else {
            1.0
        };
        
        // fill the output buffer with samples from the ring buffer
        for sample in data {
            *sample = match output_consumer.pop() {
                Some(s) => {
                    s * amp
                },
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            eprintln!("output fn: input stream fell behind: try increasing latency {}", output_consumer.len());
        }
    };

    // let tx_data_fn = move |data: &mut [f32]| {
    //     for sample in data {
    //         *sample = match input_consumer.pop() {
    //             Some(s) => s,
    //             None => 0.0,
    //         };
    //     }
    // };
    let mut oscillator = Oscillator {
        waveform: Waveform::Sine,
        sample_rate: config.sample_rate.0 as f32,
        current_sample_index: 0.0,
        frequency_hz: 220.0,
    };

    let connection = std::sync::Arc::new(std::sync::Mutex::new(Some(Connection::new(0, 0).unwrap())));
    let conn_ring = ringbuf::HeapRb::<f32>::new(latency_samples * 2);
    let (mut conn_producer, mut conn_consumer) = conn_ring.split();


    let r_conn_tx = std::sync::Arc::clone(&connection);
    std::thread::spawn(move || {
        println!("Starting the tx thread");

        loop {
            println!("3: tx: waiting for data");

            let mut data: Vec<f32> = vec![];
            for _ in 0..512 {
                let d = match input_consumer.pop() {
                    Some(s) => s,
                    None => 0.0,
                };
                data.push(d);
            }

            {
                let mut binding = r_conn_tx.lock().unwrap();
                let conn = match &mut *binding {
                    Some(c) => c,
                    None => {
                        eprintln!("Connection is None, can't send data");
                        return;
                    }
                };
                // "send" the data
                conn.tx_data(data).unwrap();
            }
        }
    });

    let r_conn_rx = std::sync::Arc::clone(&connection);
    std::thread::spawn(move || {
        println!("Starting the rx thread");
        let time_at_start = std::time::Instant::now();

        loop {
            println!("4: rx: waiting for data");
            
            // let time_since_start = std::time::Instant::now()
            //     .duration_since(time_at_start)
            //     .as_secs_f32();
            // if time_since_start < 1.0 {
            //     oscillator.set_waveform(Waveform::Sine);
            // } else if time_since_start < 2.0 {
            //     oscillator.set_waveform(Waveform::Triangle);
            // } else if time_since_start < 3.0 {
            //     oscillator.set_waveform(Waveform::Square);
            // } else if time_since_start < 4.0 {
            //     oscillator.set_waveform(Waveform::Saw);
            // } else {
            //     oscillator.set_waveform(Waveform::Sine);
            // }

            let mut output_fell_behind = false;
            // push the samples into the ring buffer
            let mut data: Vec<Packet> = vec![];
            {
                let mut binding = r_conn_rx.lock().unwrap();
                let conn = match &mut *binding {
                    Some(c) => c,
                    None => {
                        eprintln!("Connection is None, can't send data");
                        return;
                    }
                };
                data = conn.rx_data();
            }
            while let Some(sample) = data.pop() {
                if output_producer.push(oscillator.tick() * 0.1).is_err() {
                    output_fell_behind = true;
                }
            }

            sleep(std::time::Duration::from_millis(12));
            if output_fell_behind {
                eprintln!("rx: output stream fell behind: try increasing latency {}", output_producer.len());
            }
        }
    });

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

pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub waveform: Waveform,
    pub current_sample_index: f32,
    pub frequency_hz: f32,
}

impl Oscillator {
    fn advance_sample(&mut self) {
        self.current_sample_index = (self.current_sample_index + 1.0) % self.sample_rate;
    }

    fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    fn calculate_sine_output_from_freq(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.current_sample_index * freq * two_pi / self.sample_rate).sin()
    }

    fn is_multiple_of_freq_above_nyquist(&self, multiple: f32) -> bool {
        self.frequency_hz * multiple > self.sample_rate / 2.0
    }

    fn sine_wave(&mut self) -> f32 {
        self.advance_sample();
        self.calculate_sine_output_from_freq(self.frequency_hz)
    }

    fn generative_waveform(&mut self, harmonic_index_increment: i32, gain_exponent: f32) -> f32 {
        self.advance_sample();
        let mut output = 0.0;
        let mut i = 1;
        while !self.is_multiple_of_freq_above_nyquist(i as f32) {
            let gain = 1.0 / (i as f32).powf(gain_exponent);
            output += gain * self.calculate_sine_output_from_freq(self.frequency_hz * i as f32);
            i += harmonic_index_increment;
        }
        output
    }

    fn square_wave(&mut self) -> f32 {
        self.generative_waveform(2, 1.0)
    }

    fn saw_wave(&mut self) -> f32 {
        self.generative_waveform(1, 1.0)
    }

    fn triangle_wave(&mut self) -> f32 {
        self.generative_waveform(2, 2.0)
    }

    fn tick(&mut self) -> f32 {
        match self.waveform {
            Waveform::Sine => self.sine_wave(),
            Waveform::Square => self.square_wave(),
            Waveform::Saw => self.saw_wave(),
            Waveform::Triangle => self.triangle_wave(),
        }
    }
}

// data tx / rx

struct Connection {
    // stream: std::net::TcpStream,
    version: u8,
    id: u8,
}

struct Packet {
    id: u8,
    data: Vec<f32>,
}

impl Connection {
    fn new(version: u8, id: u8) -> Result<Self, std::io::Error> {
        // let stream = std::net::TcpStream::connect("127.0.0.1:42069")?;
        Ok(Self {  version, id })
    }

    fn rx_data(&mut self) -> Vec<Packet> {
        // receive the data
        let mut data = vec![Packet { id: 0, data: vec![] }];
        for i in 0..512 {
            data.push(Packet { id: 0, data: vec![0.0] });
        }
        data
    }

    fn tx_data(&mut self, data: Vec<f32>) -> Result<(), std::io::Error> {
        println!("tx: sending data {}", data.len());
        // send the data
        // self.stream.write(&vec![])?;
        Ok(())
    }
}
