pub struct State {}

impl Default for State {
    fn default() -> Self {
        State {}
    }
}

impl State {
    // Lifetime 'a is basically the lifetime of the buffer itself
    pub fn on_packet<'a>(
        &mut self,
        ip_header: etherparse::Ipv4HeaderSlice<'a>, 
        tcp_header: etherparse::TcpHeaderSlice<'a>, 
        data: &'a [u8]
    ) {
        println!(
            "{}:{} -> {}:{} | {} bytes", 
            ip_header.source_addr(), 
            tcp_header.source_port(),
            ip_header.destination_addr(), 
            tcp_header.destination_port(),
            data.len(),
        );
    }
}
