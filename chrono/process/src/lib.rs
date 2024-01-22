use std::sync::{Arc, RwLock};

static mut CONTEXT: Option<Arc<RwLock<Box<Node>>>> = None;

struct Node {
    config: i8,
    network: i8,
    consensus: i8,
    data: i8,
}

impl Node {
    unsafe fn default() {  //todo use Singleton
        CONTEXT = Some(Arc::new(RwLock::new(Box::new(Node {
            config: 0,
            network: 0,
            consensus: 0,
            data: 0,
        }))));
    }
}

pub fn init_chrono_node(config: &str) {
    unsafe {
        Node::default();
    }
}
