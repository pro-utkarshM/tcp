use std::io;
extern crate tun_tap;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let read_bytes = &buf[..nbytes];

        eprintln!("Read {} bytes: {:x?}", nbytes, read_bytes);
    }
    Ok(())
}