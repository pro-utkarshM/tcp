use std::io;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let nbytes = nic.recv(&mut buf[..])?;
    let read_bytes = &buf[..nbytes];

    eprintln!("Read {} bytes: {:x?}", nbytes, read_bytes);
    Ok(())
}