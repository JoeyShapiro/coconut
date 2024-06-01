// data rx / tx

use std::io::Write;

pub struct Connection {
    stream: std::net::TcpStream,
    version: u8,
    id: u8,
}

pub struct Packet {
    pub id: u8,
    data: Vec<f32>,
}

impl Connection {
    pub fn new(version: u8, id: u8, name: String, addr: String) -> Result<Self, std::io::Error> {
        let mut stream = std::net::TcpStream::connect(addr)?;

        // send the version and id
        let mut data: Vec<u8> = vec![];
        data.push(version);
        data.push(id);
        data.extend(name.as_bytes());
        stream.write(&data)?;

        Ok(Self {  version, id, stream })
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