use embassy_net::{Ipv4Address, Ipv4Cidr};


pub const BROKER_IP: Ipv4Address = Ipv4Address::new(169, 254, 195, 61);
pub const BROKER_CIDR:Ipv4Cidr = Ipv4Cidr::new(BROKER_IP, 24);

pub const GATEWAY_IP: Ipv4Address = Ipv4Address::new(169, 254, 195, 141);
pub const GATEWAY_ADDRESS:(Ipv4Address, u16) = (GATEWAY_IP, 1883);