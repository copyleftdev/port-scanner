extern crate pnet;

use pnet::datalink::{self};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::{TcpFlags, MutableTcpPacket};
use pnet::transport::{transport_channel, TransportChannelType::Layer3, TransportSender};
use std::net::Ipv4Addr;

fn send_syn_packet(sender: &mut TransportSender, src_ip: Ipv4Addr, dst_ip: Ipv4Addr, dst_port: u16) {
    let mut packet = [0u8; 20];
    let mut tcp_packet = MutableTcpPacket::new(&mut packet).unwrap();
    tcp_packet.set_source(12345); // Using a fixed source port for simplicity
    tcp_packet.set_destination(dst_port);
    tcp_packet.set_flags(TcpFlags::SYN);
    sender.send_to(tcp_packet, std::net::IpAddr::V4(dst_ip)).unwrap();
}

fn main() {
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(|iface| iface.is_up() && !iface.ips.is_empty() && iface.is_broadcast()).unwrap();
    let (mut sender, _) = transport_channel(4096, Layer3(IpNextHeaderProtocols::Tcp)).unwrap();
    let _src_ip = interface.ips.iter().find(|ip| ip.is_ipv4()).unwrap().ip().to_string().parse::<Ipv4Addr>().unwrap();
    let dst_ip = "192.168.1.1".parse::<Ipv4Addr>().unwrap();

    for port in 1..1024 {
        send_syn_packet(&mut sender, _src_ip, dst_ip, port);
    }
}
