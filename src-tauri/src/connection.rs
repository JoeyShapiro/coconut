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

impl Connection {
    pub fn new(id: u8, name: String, addr: String) -> Result<Self, std::io::Error> {
        let mut stream = std::net::TcpStream::connect(addr)?;

        // send the version and id
        let mut data: Vec<u8> = vec![];
        data.push(PKT_VERSION);
        data.push(id);
        data.extend(name.as_bytes());
        stream.write(&data)?;
        // TODO sample rate

        let mut buf: [u8; 3] = [0; 3];
        stream.read(&mut buf)?;
        println!("{buf:?}");
        if buf[0] != PKT_VERSION {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("version mismatch: expected {}, received {}", PKT_VERSION, buf[0])));
        } else if buf[1] == PKT_FUCKOFF {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "received a 'fuck off' packet."));
        }

        let d: f32 = 42.000001;
        let i: u32 = d.to_bits(); // they already did they type punning :(
        println!("{} -> {}", d, i);
        // bit shifing
        let b1: u8 = 10;
        let b2: u8 = 12;
        let b: u32 = ((b1 as u32) << 24) + ((b2 as u32) << 16);
        let b3: u8 = (b >> 24) as u8;
        println!("b3 {b3}");
        let b4: u8 = (b >> 16) as u8;
        println!("b4 {b4}");

        let i1: u8 = (i >> 0) as u8;
        let i2: u8 = (i >> 8) as u8;
        let i3: u8 = (i >> 16) as u8;
        let i4: u8 = (i >> 24) as u8;

        let ii: u32 = ((i4 as u32) << 24) + ((i3 as u32) << 16) + ((i2 as u32) << 8) + ((i1 as u32) << 0);
        println!("ii {ii}");

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

    pub fn tx_data(&mut self, data: Vec<f32>) -> Result<(), std::io::Error> {
        // println!("tx: sending data {}", data.len());
        // send the data
        let mut buf: [u8; 1024] = [0; 1024];
        let mut i = 0;
        buf[i] = self.version;
        i+=1;
        buf[i] = self.id;
        i+=1;
        while i < 1024 {
            // i have to do this. even if a vec
            // buf[i] = data[i-2];
            i+=1;
        }
        if i-2 != data.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "data does not fit in packet")); 
        }

        self.stream.write(&buf)?;
        Ok(())
    }
}