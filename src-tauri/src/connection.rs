// data rx / tx

use std::io::{Read, Write};

pub struct Connection {
    stream: std::net::TcpStream,
    version: u8,
    id: u8,
}

pub struct Packet {
    pub id: u8,
    data: Vec<f32>,
}

const PKT_VERSION: u8 = 1;
const PKT_FUCKOFF: u8 = 0;
const PKT_GREETINGS: u8 = 1;
const PKT_SAMPLE: u8 = 2;

impl Connection {
    pub fn new(id: u8, name: String, addr: String) -> Result<Self, std::io::Error> {
        let mut stream = std::net::TcpStream::connect(addr)?;

        // send the version and id
        let mut data: Vec<u8> = vec![];
        data.push(PKT_VERSION);
        data.push(PKT_GREETINGS);
        data.push(id);
        data.extend(name.as_bytes());
        stream.write(&data)?;
        // TODO sample rate

        let mut buf: [u8; 3] = [0; 3];
        stream.read(&mut buf)?;
        if buf[0] != PKT_VERSION {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("version mismatch: expected {}, received {}", PKT_VERSION, buf[0])));
        } else if buf[1] == PKT_FUCKOFF {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "received a 'fuck off' packet."));
        }

        // let d: f32 = 42.100001; // if the 1 is not here, it does not have precision
        // let i: u32 = d.to_bits(); // they already did they type punning :(
        // println!("{} -> {}", d, i);
        // let b = sample_to_bytes(d);
        // println!("to bytes {:?}", b);
        // println!("{} -> {:.32?}", d, sample_from_bytes(b));

        Ok(Self {  version: buf[0], id, stream })
    }

    pub fn rx_data(&mut self) -> Vec<Packet> {
        // receive the data
        let mut data: Vec<Packet> = vec![];
        for _ in 0..512 {
            data.push(Packet { id: 1, data: vec![0.0] });
        }
        for _ in 0..512 {
            data.push(Packet { id: 2, data: vec![0.0] });
        }
        data
    }

    pub fn tx_data(&mut self, data: &mut Vec<f32>) -> Result<(), std::io::Error> {
        if data.len() != 512 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "data does not fit in packet")); 
        }

        // println!("tx: sending data {}", data.len());
        // send the data
        let mut buf: [u8; 2051] = [0; 2051];
        let mut i = 0;
        buf[i] = self.version;
        i+=1;
        buf[i] = PKT_SAMPLE;
        i+=1;
        buf[i] = self.id;
        i+=1;
        while let Some(d) = data.pop() {
            // TODO either do this or use j for data
            // i have to do this. even if a vec
            // buf[i] = data[i-2];
            let b = sample_to_bytes(d);
            buf[i] = b[0];
            buf[i+1] = b[1];
            buf[i+2] = b[2];
            buf[i+3] = b[3];
            i+=4;
        }

        self.stream.write(&buf)?;
        let mut bufo: [u8; 3] = [0; 3];
        self.stream.read(&mut bufo)?;



        Ok(())
    }
}

// did this without ai help, but there is prolly a function for this
fn sample_to_bytes(sample: f32) -> [u8; 4] {
    let i = sample.to_bits();

    // bit shifting
    let i1: u8 = (i >> 0) as u8;
    let i2: u8 = (i >> 8) as u8;
    let i3: u8 = (i >> 16) as u8;
    let i4: u8 = (i >> 24) as u8;

    [i1, i2, i3, i4]
}

fn sample_from_bytes(data: [u8; 4]) -> f32 {
    let ii: u32 = ((data[3] as u32) << 24) + ((data[2] as u32) << 16) + ((data[1] as u32) << 8) + ((data[0] as u32) << 0);
    f32::from_bits(ii)
}
