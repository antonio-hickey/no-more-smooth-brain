use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dest: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    // Keeping track of tcp states
    let mut connections: HashMap<Quad, tcp::State> = Default::default();

    // Create tun tap interface for spoofing kernel's NIC to steal TCP packets
    // into user space.
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    // Packet buffer
    let mut buf = [0u8; 1504]; // MTU + 4 for the header
    loop {
        // Parsing packet buffer
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // no ipv4
            continue;
        }
            
        // Parsing the ip packet
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip_header) => {
                let ip_header_size = ip_header.slice().len();
                let src = ip_header.source_addr();
                let dest = ip_header.destination_addr();
                if ip_header.protocol() != 0x06 {
                    //not tcp
                    continue;
                }

                // Parsing the TCP packet
                match etherparse::TcpHeaderSlice::from_slice(&buf[4+ip_header.slice().len()..]) {
                    Ok(tcp_header) => {
                        let data_i = 4 + ip_header_size + tcp_header.slice().len();

                        connections
                            .entry(Quad { 
                                src: (src, tcp_header.source_port()),
                                dest: (dest, tcp_header.destination_port()) 
                            })
                            .or_default()
                            .on_packet(ip_header, tcp_header, &buf[data_i..]);
                    }, 
                    Err(e) => {
                        println!("ignoring weird tcp packet {:?}", e);
                    }
                }
            },
            Err(e) => {
                println!("ignoring weird packet {:?}", e);
            }
        }
    }
}
