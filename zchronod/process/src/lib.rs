use std::fs;
use std::sync::{Arc, RwLock};
use log::info;
use network::Network;
use chronod::Consensus;
use api::{CONTEXT, NetworkInterface};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;


#[derive(Debug, Deserialize, Serialize)]
struct Config {
    peers: Vec<String>,
    rpc: RpcConfig,
    gossip: GossipConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct RpcConfig {
    port: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GossipConfig {
    port: String,
}

fn parse_config_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;  // start with src/../..
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

pub unsafe fn init_chrono_node(config: &str) {
    let conf = parse_config_file(config).unwrap();
    let network = Network::init(&conf.peers, &conf.rpc.port, &conf.gossip.port);
    //  let cons = Consensus::init();

    CONTEXT = Some(api::Node::new(Box::new(network)));

    info!("[{}] zchronod service started",module_path!())
    // network::set().expect("TODO: panic message");
}
