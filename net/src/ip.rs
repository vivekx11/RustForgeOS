//! IPv4 packet handling

use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Address(pub [u8; 4]);

impl Ipv4Address {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self([a, b, c, d])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IpProtocol {
    Icmp = 1,
    Tcp = 6,
    Udp = 17,
}

pub struct Ipv4Packet {
    pub version: u8,
    pub ihl: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: IpProtocol,
    pub checksum: u16,
    pub src: Ipv4Address,
    pub dest: Ipv4Address,
    pub payload: Vec<u8>,
}

impl Ipv4Packet {
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 20 {
            return Err("Packet too short");
        }

        let version = data[0] >> 4;
        let ihl = data[0] & 0x0F;
        let dscp = data[1] >> 2;
        let ecn = data[1] & 0x03;
        let total_length = u16::from_be_bytes([data[2], data[3]]);
        let identification = u16::from_be_bytes([data[4], data[5]]);
        let flags = data[6] >> 5;
        let fragment_offset = u16::from_be_bytes([data[6] & 0x1F, data[7]]);
        let ttl = data[8];
        let protocol_raw = data[9];
        let checksum = u16::from_be_bytes([data[10], data[11]]);
        let src = Ipv4Address([data[12], data[13], data[14], data[15]]);
        let dest = Ipv4Address([data[16], data[17], data[18], data[19]]);

        let protocol = match protocol_raw {
            1 => IpProtocol::Icmp,
            6 => IpProtocol::Tcp,
            17 => IpProtocol::Udp,
            _ => return Err("Unknown protocol"),
        };

        let header_len = (ihl as usize) * 4;
        let payload = data[header_len..].to_vec();

        Ok(Self {
            version,
            ihl,
            dscp,
            ecn,
            total_length,
            identification,
            flags,
            fragment_offset,
            ttl,
            protocol,
            checksum,
            src,
            dest,
            payload,
        })
    }

    pub fn calculate_checksum(data: &[u8]) -> u16 {
        let mut sum: u32 = 0;
        
        for chunk in data.chunks(2) {
            let word = if chunk.len() == 2 {
                u16::from_be_bytes([chunk[0], chunk[1]]) as u32
            } else {
                (chunk[0] as u32) << 8
            };
            sum += word;
        }

        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }
}
