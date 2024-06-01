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

        let mut buf: [u8; 3] = [0; 3];
        stream.read(&mut buf)?;
        println!("{buf:?}");
        if buf[0] != PKT_VERSION {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("version mismatch: expected {}, received {}", PKT_VERSION, buf[0])));
        } else if buf[1] == PKT_FUCKOFF {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "received a 'fuck off' packet."));
        }

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
        self.stream.write(&vec![])?;
        Ok(())
    }
}