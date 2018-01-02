use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use server::{MessageRequest, MessageResponse};

pub struct StunCodec;

impl Decoder for StunCodec {
    type Item = MessageRequest;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<MessageRequest>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(_s) => Ok(Some(MessageRequest { header: 42 })),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for StunCodec {
    type Item = MessageResponse;
    type Error = io::Error;

    fn encode(&mut self, msg: MessageResponse, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.header.to_string().as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}
