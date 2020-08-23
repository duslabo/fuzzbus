use argh::FromArgs;
use rand::Rng;

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

pub use socketcan::{CANFrame, CANSocket, CANSocketOpenError};
fn main() {
    println!("fuzzbus Starting!");

    let up: GoUp = argh::from_env();

    random(up.interface);

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
