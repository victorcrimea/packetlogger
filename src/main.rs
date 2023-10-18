use pcap::{Capture, Device};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional interface to monitor on
    interface: Option<String>,

    /// Port to monitor
    #[arg(short, long, default_value_t = 8000)]
    port: u16,

    /// List available devices
    #[arg(short, long)]
    list: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut interface = "any".to_string();

    if let Some(interface_name) = cli.interface.as_deref() {
        interface = interface_name.to_string();
    }

    let devices = Device::list().unwrap();
    if cli.list {
        for dev in devices {
            println!("{:?}", dev);
        }
        return;
    }

    let device = devices.into_iter().find(|d| d.name == interface).unwrap();

    println!("Device name: {}", device.name);

    let capture = Capture::from_device(device).unwrap();
    let capture = capture.promisc(true);
    let capture = capture.immediate_mode(true);
    let mut capture = capture.open().unwrap();
    let program = format!("dst port {}", cli.port);
    capture.filter(program.as_str(), false).unwrap();
    println!("monitoring incoming traffic");
    let mut rx = 0u64;
    let mut total_bytes = 0u64;
    while let Ok(packet) = capture.next_packet() {
        rx += 1;
        total_bytes += packet.header.caplen as u64;
        println!("packet_num: {}, total_bytes: {}", rx, total_bytes);
    }
}
