use std::io;

fn main() -> io::Result<()> {
    // Create tun tap interface for spoofing kernel's NIC to steal TCP packets
    // into user space.
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    // Packet buffer
    let mut buf = [0u8; 1504]; // MTU + 4 for the header
    loop {
        // parsing packet buffer
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // no ipv4
            continue;
        }
            
        // Parsing the ip packet
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dest = p.destination_addr();
                let proto = p.protocol();
                if proto != 0x06 {
                    //not tcp
                    continue;
                }

                // Parsing the TCP packet
                match etherparse::TcpHeaderSlice::from_slice(&buf[4+p.slice().len()..]) {
                    Ok(p) => {
                        println!(
                            "{} -> {} | {} bytes | port: {}", 
                            src, 
                            dest, 
                            p.slice().len(), 
                            p.destination_port(),
                        );
                    }, 
                    Err(e) => {
                        println!("ignoring weird tcp packet {:?}", e);
                    }
                }

                println!("{} -> {} | {} bytes | protocol: {}", src, dest, p.payload_len(), proto);
            },
            Err(e) => {
                println!("ignoring weird packet {:?}", e);
            }
        }
    }
    Ok(())
}
