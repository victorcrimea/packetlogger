use clap::Parser;
use pcap::{Capture, Device};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional interface to monitor on, defaults to any
    interface: Option<String>,

    /// Port to monitor
    #[arg(short, long, default_value_t = 8000)]
    port: u16,

    /// List available devices and exit
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
            println!("{}", dev.name);
        }
        return;
    }

    let device = devices.into_iter().find(|d| d.name == interface).unwrap();

    println!(
        "monitoring incoming traffic on device {}, port {}",
        device.name, cli.port
    );

    let mut capture = Capture::from_device(device).unwrap();
    capture = capture.immediate_mode(true);
    let mut capture = capture.open().unwrap();

    let bpf_program = format!("dst port {}", cli.port);
    capture.filter(bpf_program.as_str(), false).unwrap();

    let mut total_packets_rx = 0u64;
    let mut total_bytes_rx = 0u64;
    while let Ok(packet) = capture.next_packet() {
        total_packets_rx += 1;
        total_bytes_rx += packet.header.caplen as u64;
        println!(
            "packet_num: {}, total_bytes: {}",
            total_packets_rx, total_bytes_rx
        );
    }
}
