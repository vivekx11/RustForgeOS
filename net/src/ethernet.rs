//! Ethernet frame handling

use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddress(pub [u8; 6]);

impl MacAddress {
    pub const BROADCAST: MacAddress = MacAddress([0xff; 6]);
    
    pub fn new(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EtherType {
    Ipv4 = 0x0800,
    Arp = 0x0806,
    Ipv6 = 0x86DD,
}

pub struct EthernetFrame {
    pub dest: MacAddress,
    pub src: MacAddress,
    pub ether_type: EtherType,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 14 {
            return Err("Frame too short");
        }

        let dest = MacAddress::new([data[0], data[1], data[2], data[3], data[4], data[5]]);
        let src = MacAddress::new([data[6], data[7], data[8], data[9], data[10], data[11]]);
        let ether_type_raw = u16::from_be_bytes([data[12], data[13]]);
        
        let ether_type = match ether_type_raw {
            0x0800 => EtherType::Ipv4,
            0x0806 => EtherType::Arp,
            0x86DD => EtherType::Ipv6,
            _ => return Err("Unknown EtherType"),
        };

        let payload = data[14..].to_vec();

        Ok(Self {
            dest,
            src,
            ether_type,
            payload,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut frame = Vec::with_capacity(14 + self.payload.len());
        frame.extend_from_slice(&self.dest.0);
        frame.extend_from_slice(&self.src.0);
        frame.extend_from_slice(&(self.ether_type as u16).to_be_bytes());
        frame.extend_from_slice(&self.payload);
        frame
    }
}
