#![no_std]
#![no_main]

use core::str::FromStr;

use defmt::*;
use rand_core::RngCore;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_net::{tcp::TcpSocket, Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_stm32::{
    bind_interrupts,
    eth::{self, generic_smi::GenericSMI, Ethernet, PacketQueue},
    gpio::{AnyPin, Input, Pin, Pull},
    peripherals::{self, ETH},
    rng::{self, Rng},
    time::Hertz,
    Config,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

// MQTT related imports
use heapless::String;
use typenum::consts::*;

use hyped_core::{
    format,
    format_string::show,
    log_types::LogLevel,
    mqtt::{HypedMqttClient, MqttMessage},
    mqtt_topics::MqttTopics,
};
use serde::{Deserialize, Serialize};

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
    RNG => rng::InterruptHandler<peripherals::RNG>;
});
