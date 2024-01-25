use futures::future::ok;
use log::{error, info};
use prost::bytes::Buf;
use prost::Message;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use gossipd::gossipd::{Gossipd, GossipdOptions};
use proto::zchronod::zchronod_server::Zchronod;
use proto::zchronod::Event;
use bytes::Bytes;


pub(crate) struct GossipServer {
    send: Sender<Event>,
    gossip: Gossipd<Event>,
}

impl GossipServer {
    pub(crate) fn new(peers: &Vec<String>, listen_address: &str) -> Self {
        let mut gossip_options = GossipdOptions::default();
        gossip_options.listen_addr = listen_address.to_string();
        for peer in peers {
            info!("add peer {}", peer);
            gossip_options.add_peer(peer.to_string());// format as /ip4/192.168.0.1/tcp/80
        }
        let mut gossip: Gossipd<Event> = Gossipd::new(gossip_options);
        GossipServer { send: gossip.create_sender(), gossip }
    }

    pub fn register_receive(&mut self, f: fn()) {
        self.gossip.with_handler(|peer_id, message| {
            match Event::decode(Bytes::from(message.data)) {
                Ok(event) => {
                    info!("recv msg which event is {:?}", event.id);
                }
                Err(err) => {
                    error!("failed to decode");
                }
            }
        });
    }

    pub async fn send(&self, msg: Event) -> Result<(), String> {
        match self.send.send(msg).await {
            Ok(()) => {
                Ok(())
            }
            Err(_) => {
                error!("failed to send msg");
                Err("faild to send".to_string())
            }
        }
    }


    pub(crate) fn start(&mut self) {
        self.gossip.start();
    }
}