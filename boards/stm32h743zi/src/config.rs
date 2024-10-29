use embassy_net::{Ipv4Address, Ipv4Cidr};

pub const BOARD_STATIC_IP: Ipv4Address = Ipv4Address::new(169, 254, 195, 61);
pub const BOARD_STATIC_ADDRESS: Ipv4Cidr = Ipv4Cidr::new(BOARD_STATIC_IP, 24);

pub const MQTT_BROKER_IP: Ipv4Address = Ipv4Address::new(169, 254, 195, 141);
pub const MQTT_BROKER_ADDRESS: (Ipv4Address, u16) = (MQTT_BROKER_IP, 1883);
