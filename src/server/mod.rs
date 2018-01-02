mod codec;
pub mod proto;
pub mod service;

use std::net::SocketAddr;
use tokio_proto::TcpServer;
use server::proto::StunProto;
use server::service::Stun;

pub struct Server {
    addr: SocketAddr
}

pub trait StunServer {
    fn new(addr: SocketAddr) -> Server;
    fn serve(&self);
}

impl StunServer for Server {

    fn new(addr: SocketAddr) -> Server {
        Server { addr }
    }

    fn serve(&self) {
        // The builder requires a protocol and an address
        let server = TcpServer::new(StunProto, self.addr);

        // We provide a way to *instantiate* the service for each new
        // connection; here, we just immediately return a new instance.
        server.serve(|| Ok(Stun));
    }
}

pub struct MessageRequest {
    header: u32
}

pub struct MessageResponse {
    header: u32
}
