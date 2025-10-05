use std::io;
use std::collections::HashMap;
use std::net::Ipv4Addr;
extern crate tun_tap;
use etherparse;

mod tcp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TcpConnection {
    src_addr: Ipv4Addr,
    src_port: u16,
    dst_addr: Ipv4Addr,
    dst_port: u16,
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<TcpConnection, tcp::TcpState> = Default::default();
    
    // Create a TUN interface named "tun0"
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let read_bytes = &buf[..nbytes];

        if nbytes < 4 {
            eprintln!("Packet too short: {} bytes", nbytes);
            continue;
        }

        // Link-level header: flags + protocol
        let flags = u16::from_be_bytes([read_bytes[0], read_bytes[1]]);
        let proto = u16::from_be_bytes([read_bytes[2], read_bytes[3]]);

        if proto != 0x0800 {
            eprintln!("Non IPv4 packet received, proto={:#06x}", proto);
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&read_bytes[4..nbytes]) {
            Ok(ip_header) => {
                let proto = ip_header.protocol();
                if proto != etherparse::IpNumber::TCP {
                    eprintln!("Non TCP packet received, proto={:?}", proto);
                    continue;
                }

                let ip_hdr_size = ip_header.slice().len();
                match etherparse::TcpHeaderSlice::from_slice(&read_bytes[4 + ip_hdr_size..nbytes]) {
                    Ok(tcp_header) => {
                        let tcp_hdr_size = tcp_header.slice().len();
                        let payload = &read_bytes[4 + ip_hdr_size + tcp_hdr_size..nbytes];

                        // Track connection and handle packet
                        let conn = TcpConnection {
                            src_addr: ip_header.source_addr(),
                            src_port: tcp_header.source_port(),
                            dst_addr: ip_header.destination_addr(),
                            dst_port: tcp_header.destination_port(),
                        };

                        connections.entry(conn)
                            .or_default()
                            .on_packet(&ip_header, &tcp_header, payload);

                        // Print packet info
                        eprintln!(
                            "[TCP] {}:{} -> {}:{}, seq={}, ack={}, payload={} bytes, flags={:#06x}",
                            ip_header.source_addr(),
                            tcp_header.source_port(),
                            ip_header.destination_addr(),
                            tcp_header.destination_port(),
                            tcp_header.sequence_number(),
                            tcp_header.acknowledgment_number(),
                            payload.len(),
                            flags
                        );
                    }
                    Err(e) => {
                        eprintln!("Failed to parse TCP header: {}", e);
                        continue;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse IPv4 header: {}", e);
            }
        }
    }
    Ok(())
}
