use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};

pub struct TcpState {
    // TODO: add fields here to track TCP state (sequence numbers, window size, etc.)
}

impl Default for TcpState {
    fn default() -> Self {
        TcpState {}
    }
}

impl TcpState {
    pub fn on_packet<'a>(
        &mut self,
        ip_header: &Ipv4HeaderSlice<'a>,
        tcp_header: &TcpHeaderSlice<'a>,
        payload: &'a [u8],
    ) {
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
