use std::io;

fn main() -> io::Result<()> {

    // Create tun tap interface for spoofing kernel's NIC to steal TCP packets
    // into user space.
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    // Packet buffer
    let mut buf = vec![0u8; 1504]; // MTU + 4 for the header
    let nbytes = nic.recv(&mut buf[..])?;
    println!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);

    Ok(())
}
