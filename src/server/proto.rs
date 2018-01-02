use std::io;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use server::codec::StunCodec;
use server::{MessageRequest, MessageResponse};

pub struct StunProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for StunProto {
    type Request = MessageRequest;
    type Response = MessageResponse;
    type Transport = Framed<T, StunCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(StunCodec))
    }
}
