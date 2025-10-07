use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};

pub enum TcpState {
    // TODO: add fields here to track TCP state (sequence numbers, window size, etc.)
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

impl Default for TcpState {
    fn default() -> Self {
        TcpState::Listen
    }
}

impl TcpState {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: &Ipv4HeaderSlice<'a>,
        tcp_header: &TcpHeaderSlice<'a>,
        payload: &'a [u8],
    ) {
        let buf = &mut [0u8; 1500];
        match *self {
            TcpState::Closed => {
                // In CLOSED state, ignore all packets
                return;
            }
            TcpState::Listen => {
                if !tcp_header.syn() {
                    // Ignore non-SYN packets in LISTEN state
                    return;
                }
                //need to start establishing a connection here
                let mut syn_ack=etherparse::TcpHeader::new(tcp_header.destination_port(), tcp_header.source_port(), unimplemented!(), unimplemented!());
                syn_ack.syn=true;
                syn_ack.ack=true;

                let mut ip=etherparse::Ipv4Header::new(
                    syn_ack.slice().len(),
                    64, // TTL
                    etherparse::IpNumber::TCP,
                    ip_header.destination_addr(),
                    ip_header.source_addr(),
                );
            }
        // log TCP packet details
        eprintln!(
            "TCP packet: {}:{} -> {}:{}, payload={} bytes",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            payload.len()
        );

        // TODO: implement TCP state tracking here
        }
    }
}
