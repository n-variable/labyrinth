extern crate multiaddr;

use multiaddr::{Multiaddr, multiaddr};

struct Peer {
    id: String
    multiaddr: String,
    transport: String,

}

impl Peer {
    // Ping this peer
    fn ping(&self) {

    }
}