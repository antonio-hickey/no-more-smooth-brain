use std::io;

///  State of the Send Sequence Space (RFC 793 | Section 3.2 | Figure 4) 
///```
///            1         2          3          4
///       ----------|----------|----------|----------
///              SND.UNA    SND.NXT    SND.UNA
///                                   +SND.WND
///
/// 1 - old sequence numbers which have been acknowledged
/// 2 - sequence numbers of unacknowledged data
/// 3 - sequence numbers allowed for new data transmission
/// 4 - future sequence numbers which are not yet allowed
///```
struct SendSequenceSpace {
    una: u32, // send unacknowledged
    nxt: u32, // send next
    wnd: u16, // send window
    up: bool,   // send urgent pointer
    wl1: usize, // segment sequence number used for last window update
    wl2: usize, // segment acknowledgment number used for last window update
    iss: u32, // initial send sequence number
}

///  State of the Receive Sequence Space (RFC 793 | Section 3.2 | Figure 5)
///```
///               1          2          3
///           ----------|----------|----------
///                  RCV.NXT    RCV.NXT
///                            +RCV.WND
///
///1 - old sequence numbers which have been acknowledged
///2 - sequence numbers allowed for new reception
///3 - future sequence numbers which are not yet allowed
///```
struct RecieveSequenceSpace {
    nxt: u32, // recieve next
    wnd: u16, // recieve window
    up: bool,   // recieve urgent pointer
    irs: u32, // recieve initial recieve sequence number
}

enum State {
    Closed,
    Listen,
    SynRecieved,
    Established,
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: RecieveSequenceSpace,
}

impl Connection {
    // Lifetime 'a is basically the lifetime of the buffer itself
    pub fn accept<'a>(
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>, 
        tcp_header: etherparse::TcpHeaderSlice<'a>, 
        data: &'a [u8]
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8; 1500];

        if !tcp_header.syn() {
            // only expected SYN packet here
            return Ok(None);
        }

        let iss = 0;
        let cnx = Connection {
            state: State::SynRecieved,
            send: SendSequenceSpace { 
                iss: 0,
                una: iss,
                nxt: iss + 1,
                wnd: 10,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            recv: RecieveSequenceSpace { 
                nxt: tcp_header.sequence_number() + 1, 
                wnd: tcp_header.window_size(), 
                up: false, 
                irs: tcp_header.sequence_number(),
            }
        };

        // Need to start establishing a connection
        let mut syn_ack = etherparse::TcpHeader::new(
            tcp_header.destination_port(),
            tcp_header.source_port(),
            cnx.send.iss,
            cnx.send.wnd,
        );
        syn_ack.acknowledgment_number = cnx.recv.nxt;
        syn_ack.syn = true;
        syn_ack.ack = true;
        let ip_packet = etherparse::Ipv4Header::new(
            syn_ack.header_len(),
            64,
            etherparse::IpTrafficClass::Tcp,
            [
                ip_header.destination()[0],
                ip_header.destination()[1],
                ip_header.destination()[2],
                ip_header.destination()[3],
            ],
            [
                ip_header.source()[0],
                ip_header.source()[1],
                ip_header.source()[2],
                ip_header.source()[3],
            ],
        );

        syn_ack.checksum = syn_ack
            .calc_checksum_ipv4(&ip_packet, &[])
            .expect("failed to compute checksum");

        // write out the headers
        let unwritten = {
            let mut unwritten = &mut buf[..];
            ip_packet.write(&mut unwritten);
            syn_ack.write(&mut unwritten);
            unwritten.len()
        };

        nic.send(&buf[..&buf.len() - unwritten])?;
        Ok(Some(cnx))
    }

    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>, 
        tcp_header: etherparse::TcpHeaderSlice<'a>, 
        data: &'a [u8]
    ) -> io::Result<()> {
        // unimplemented, but don't want to panic here
        Ok(())
    }
}
