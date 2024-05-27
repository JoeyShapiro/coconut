// data rx / tx

pub struct Connection {
    // stream: std::net::TcpStream,
    version: u8,
    id: u8,
}

pub struct Packet {
    pub id: u8,
    data: Vec<f32>,
}

impl Connection {
    pub fn new(version: u8, id: u8) -> Result<Self, std::io::Error> {
        // let stream = std::net::TcpStream::connect("127.0.0.1:42069")?;
        Ok(Self {  version, id })
    }

    pub fn rx_data(&mut self) -> Vec<Packet> {
        // receive the data
        let mut data: Vec<Packet> = vec![];
        for _ in 0..512 {
            data.push(Packet { id: 1, data: vec![0.0] });
        }
        data
    }

    pub fn tx_data(&mut self, data: Vec<f32>) -> Result<(), std::io::Error> {
        // println!("tx: sending data {}", data.len());
        // send the data
        // self.stream.write(&vec![])?;
        Ok(())
    }
}