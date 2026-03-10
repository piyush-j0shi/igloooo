use std::collections::HashMap;

#[derive(Debug)]
enum PacketType {
    TCP { scr_port: u16, dst_port: u16 },
    UDP { scr_port: u16, dst_port: u16 },
    Unknown,
}

#[derive(Debug)]
struct Packet<'a> {
    src_ip: &'a str,
    dst_ip: &'a str,
    packet_type: PacketType,
    payload: Vec<u8>,
}

impl<'a> Packet<'a> {
    fn summary(&self) {
        println!("source ip : {}", self.src_ip);
        println!("destination ip : {}", self.dst_ip);
        println!("packet type : {:?}", self.packet_type);
        println!("payload : {:#?}", self.payload);
    }

    fn size(&self) {
        println!("size of payload : {}", self.payload.len());
    }

    fn is_suspicious(&self) -> (bool, &str) {
        if self.payload.is_empty() {
            return (true, "palyload empty");
        } else if self.payload.len() > 10000 {
            return (true, "oversized payload");
        } else {
            match std::str::from_utf8(&self.payload) {
                Ok(s) => {
                    if s.contains("attack") || s.contains("malware") || s.contains("malicious") {
                        return (true, "conatins suspicous words");
                    } else {
                        return (false, "correct thing");
                    }
                }
                Err(e) => {
                    println!("something went wrong : {}", e);
                    return (true, "Something went wrong so marking it true");
                }
            }
        }
    }
}

#[derive(Debug)]
struct Connection {
    src_ip: String,
    dst_ip: String,
    src_port: u16,
    dst_port: u16,
    protocol: String,
    packet_count: u64,
    bytes_transferred: u64,
}

impl Connection {
    fn new(packet: &Packet) -> Self {
        let result = match packet.packet_type {
            PacketType::TCP { scr_port, dst_port } => ("TCP".to_string(), (scr_port, dst_port)),
            PacketType::UDP { scr_port, dst_port } => ("UDP".to_string(), (scr_port, dst_port)),
            PacketType::Unknown => ("Unknown".to_string(), (0, 0)),
        };

        Connection {
            src_ip: packet.src_ip.to_string(),
            dst_ip: packet.dst_ip.to_string(),
            src_port: result.1 .0,
            dst_port: result.1 .1,
            protocol: result.0,
            packet_count: 0,
            bytes_transferred: 0,
        }
    }

    fn update(&mut self, packet: &Packet) {
        self.packet_count += 1;
        self.bytes_transferred += packet.payload.len() as u64;
    }
}

#[derive(Debug)]
struct AnalyzeState {
    connections: HashMap<String, Connection>,
    events: Vec<String>,
}

impl AnalyzeState {
    fn add_or_update_connection(&mut self, packet: &Packet) {
        let the_key = format!(
            "{}{}-{:?}",
            packet.src_ip, packet.dst_ip, packet.packet_type
        );

        let suspeciousness = packet.is_suspicious();

        if suspeciousness.0 == false {
            if let Some(got_connection) = self.connections.get_mut(&the_key) {
                got_connection.update(packet);
                println!("updated connection is : {:#?}", self.connections);
            } else {
                let mut conn = Connection::new(packet);
                conn.update(packet);
                self.connections.insert(the_key, conn);
                println!("new connection is : {:#?}", self.connections);
            }
        } else if suspeciousness.0 == true {
            let shit = format!(
                "Is suspecious : {} | Reason : {:?}",
                suspeciousness.0, suspeciousness.1
            );
            self.events.push(shit);
            println!("suiiiiii : {:#?}", self.events);
        } else {
            println!("man we are working something went wrong, we also don't know.");
        }
    }
}

fn main() {
    let mut analyzestate = AnalyzeState {
        connections: HashMap::new(),
        events: vec![],
    };

    let packets = vec![
        Packet {
            src_ip: "127.0.0.1",
            dst_ip: "192.168.0.10",
            packet_type: PacketType::TCP {
                scr_port: 50000,
                dst_port: 80,
            },
            payload: b"hello world".to_vec(),
        },
        Packet {
            src_ip: "192.168.0.5",
            dst_ip: "10.0.0.1",
            packet_type: PacketType::UDP {
                scr_port: 1234,
                dst_port: 53,
            },
            payload: b"normal dns request".to_vec(),
        },
        Packet {
            src_ip: "10.0.0.2",
            dst_ip: "10.0.0.3",
            packet_type: PacketType::TCP {
                scr_port: 4444,
                dst_port: 22,
            },
            payload: b"attack detected!".to_vec(),
        },
    ];

    for packet in packets {
        packet.summary();
        analyzestate.add_or_update_connection(&packet);
    }

    println!("Final Analyzer State: {:#?}", analyzestate);
}
