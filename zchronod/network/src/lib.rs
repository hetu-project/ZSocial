use crate::gossip::GossipServer;
use crate::rpc::RpcServer;

mod gossip;
mod rpc;

use api::NetworkInterface;
use proto::zchronod::Event;

pub struct Network {
    gossip_server: GossipServer,
    rpc_server: RpcServer,
}

pub struct Config {
    // Add fields for the Tonic RPC server configuration
    address: String,
    port: u16,
    // Add other configuration fields as needed
}

impl Network {
    pub fn init(peers:&Vec<String>,rpc:&str,gossip:&str) -> Self {
        Network {
            gossip_server: GossipServer::new(peers,gossip),
            rpc_server: RpcServer::new(rpc),
        }
    }
}
impl NetworkInterface for Network {

    fn run(&mut self) {
        // Start gossip server
        self.gossip_server.start();

        // Start RPC server
        self.rpc_server.run();

    }

    fn send(&self, msg: Event) {
        self.gossip_server.send(msg);
    }
}

