use proto::zchronod::Event;

pub static mut CONTEXT: Option<Node> = None;

pub struct Node {
    network: Box<dyn NetworkInterface>,
    //consensus: Box<dyn ConsensusInterface>,
}

impl Node {
    pub fn new(network: Box<dyn NetworkInterface>) -> Self {
        Node {
            network,
        }
    }
    pub fn set_network(&mut self, network: Box<dyn NetworkInterface>) {
        self.network = network;
    }

    // pub fn set_consensus(&mut self, consensus: Box<dyn ConsensusInterface>) {
    //     self.consensus = consensus;
    // }
    pub fn get_network(&self) -> &Box<dyn NetworkInterface> {
        &self.network
    }
    pub fn run(&mut self) {
        self.network.run();
    }
}

pub trait NetworkInterface {
    fn run(&mut self);
    fn send(&self, msg: Event);
}

pub trait ConsensusInterface {}
