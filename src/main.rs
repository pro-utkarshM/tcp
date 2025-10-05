use std::io;
extern crate tun_tap;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let read_bytes = &buf[..nbytes];
        
        //eth_frame from the link level
        let flags = u16::from_be_bytes([read_bytes[0], read_bytes[1]]);
        let proto = u16::from_be_bytes([read_bytes[2], read_bytes[3]]);

        if proto!= 0x0800 {
            eprintln!("Non IPv4 packet received, proto={:#06x}", proto);
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&read_bytes[4..nbytes]) {
            Ok(packet) => {
                let proto = packet.protocol();  
                if proto != etherparse::IpNumber::from(0x06) {
                    eprint!("Non TCP packet received, proto={:?}", proto);
                    continue;
                }
                match etherparse::TcpHeaderSlice::from_slice(&read_bytes[4 + packet.slice().len()..nbytes]) {
                    Ok(tcp_header) => {
                        eprintln!(
                            "TCP packet: {}:{} -> {}:{}, seq={}, ack={}",
                            packet.source_addr(),
                            tcp_header.source_port(),
                            packet.destination_addr(),
                            tcp_header.destination_port(),
                            tcp_header.sequence_number(),
                            tcp_header.acknowledgment_number(),
                        );
                },
                    Err(e) => {
                        eprintln!("Failed to parse TCP header: {}", e);
                        continue;
                    }
                }
                // Print out some information about the packet
                // including the flags from the link layer header
                // (for TUN, flags are usually 0x0000 or 0x0004)
                // For TAP, flags may include 0x0001 (broadcast) or
                // 0x0002 (multicast)
                // See https://www.kernel.org/doc/html/latest/networking/packet_mmap.html
                // for more details about the flags
                eprintln!(
                    "IPv4 packet: {} -> {}, {} bytes of protocol: {:?}, flags: {:#06x}",
                    packet.source_addr(),
                    packet.destination_addr(),
                    packet.payload_len().unwrap_or(0),
                    packet.protocol(),
                    flags
                );
            },
            Err(e) => {
                eprintln!("Failed to parse IPv4 header: {}", e);
            }
        }
    }
    Ok(())
}