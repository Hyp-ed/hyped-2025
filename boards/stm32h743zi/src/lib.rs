#![no_std]
use embassy_net::{Ipv4Address, Ipv4Cidr};

pub mod log;
pub mod tasks;

pub static BROKER_IP: Ipv4Address = Ipv4Address::new(169, 254, 195, 61);
pub static BROKER_CIDR:Ipv4Cidr = Ipv4Cidr::new(BROKER_IP, 24);

pub static GATEWAY_IP: Ipv4Address = Ipv4Address::new(169, 254, 195, 141);
pub static GATEWAY_ADDRESS:(Ipv4Address, u16) = (GATEWAY_IP, 1883);