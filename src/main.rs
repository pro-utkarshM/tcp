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