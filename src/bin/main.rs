extern crate tokio_proto;
extern crate rusty_mirror as stun;

use stun::server::Server;
use stun::server::StunServer;

fn main() {
    let server = Server::new("0.0.0.0:12345".parse().unwrap());
    server.serve();
}
