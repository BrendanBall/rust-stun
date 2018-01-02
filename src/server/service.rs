use std::io;
use tokio_service::Service;
use futures::{future, Future};
use server::{MessageRequest, MessageResponse};

pub struct Stun;

impl Service for Stun {
    type Request = MessageRequest;
    type Response = MessageResponse;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immediate.
        let res = MessageResponse {header: req.header};
        Box::new(future::ok(res))
    }
}
