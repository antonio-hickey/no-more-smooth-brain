use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dest: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    // Keeping track of tcp connections & states
    let mut connections: HashMap<Quad, tcp::Connection> = Default::default();

    // Create tun tap interface for spoofing kernel's NIC to steal TCP packets
    // into user space.
    let mut nic = tun_tap::Iface::without_packet_info("tun0", tun_tap::Mode::Tun)?;

    // Packet buffer
    let mut buf = [0u8; 1504]; // MTU + 4 for the header
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
            
        // Parsing the ip packet
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[..nbytes]) {
            Ok(ip_header) => {
                let ip_header_size = ip_header.slice().len();
                let src = ip_header.source_addr();
                let dest = ip_header.destination_addr();
                if ip_header.protocol() != 0x06 {
                    //not tcp
                    continue;
                }

                // Parsing the TCP packet
                match etherparse::TcpHeaderSlice::from_slice(&buf[ip_header.slice().len()..]) {
                    Ok(tcp_header) => {
                        let data_i = ip_header_size + tcp_header.slice().len();
                        match connections.entry(Quad { 
                            src: (src, tcp_header.source_port()),
                            dest: (dest, tcp_header.destination_port()) 
                        }) {
                            Entry::Occupied(mut cnx) => {
                                cnx
                                    .get_mut()
                                    .on_packet(&mut nic, ip_header, tcp_header, &buf[data_i..nbytes])?;
                            },
                            Entry::Vacant(e) => {
                                if let Some(cnx) = tcp::Connection::accept(
                                    &mut nic,
                                    ip_header, 
                                    tcp_header, 
                                    &buf[data_i..nbytes]
                                )? {
                                    e.insert(cnx);
                                }
                            }
                        }
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
