extern crate toml;

use argh::FromArgs;
use rand::Rng;
use std::fs;
use serde_derive::Deserialize;
use serde::{Serialize, Deserialize};

#[derive(FromArgs)]
/// Reach new heights.
struct GoUp {
    /// provide can interface name like can0 or vcan0
    #[argh(option, short = 'i')]
    interface: String,

    /// whether or not can id for matching like -m 123,12d,211
    #[argh(option, short = 'c')]
    toml_path: Option<String>,
}

#[derive(Deserialize)]
pub struct CanInfo {
    pub period_ms: u32,
    pub can_id: u32,
    pub data: [u32; 8],
}

pub use socketcan::{CANFrame, CANSocket, CANSocketOpenError};
fn main() {
    println!("fuzzbus Starting!");

    let up: GoUp = argh::from_env();
    if up.toml_path == None {
        random(up.interface);
    }
    else {
        conditional_fuzz(up.interface, up.toml_path.unwrap());
    }
}

fn random(interface: String) {

    let mut rng = rand::thread_rng();
    let mut can_id;
    let mut data: [u8; 8];
    let mut frame;

    let socket = CANSocket::open(&interface).unwrap();

    loop {
        can_id = rng.gen_range(0x100, 0x800);
        data = rng.gen();
        frame = CANFrame::new(can_id, &data, false, false).unwrap();
        socket.write_frame(&frame).unwrap();
    }

}

fn conditional_fuzz(interface: String, config_path: String) {
    
    let toml_data = fs::read_to_string(config_path).unwrap();

    let can_info: CanInfo = toml::from_str(&toml_data).unwrap();

    println!("{:?}",toml_data);
    //println!("canid: {} data: {:?} period: {} ", can_info.can_id, can_info.data, can_info.period_ms);
}