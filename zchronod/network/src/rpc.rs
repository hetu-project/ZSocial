use tonic::{transport::Server, Request, Response, Status, IntoRequest};
use log::info;
use log::kv::ToKey;
use api::CONTEXT;
use proto::zchronod::zchronod_server::{Zchronod, ZchronodServer};
use proto::zchronod::{ZchronodRequest, ZchronodResp};

pub(crate) struct RpcServer {
    port: String,
}

impl RpcServer {
    pub(crate) fn new(listen: &str) -> Self {
        RpcServer {
            port: listen.to_string(),
        }
    }

    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.port.parse()?;
        Server::builder()
            .add_service(ZchronodServer::new(ZchronodService {}))
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[derive(Default)]
pub struct ZchronodService {}

#[tonic::async_trait]
impl Zchronod for ZchronodService {
    async fn send(&self, request: Request<ZchronodRequest>) -> Result<Response<ZchronodResp>, Status> {
        info!("recv request from {:?}",request.get_ref().msg.as_ref().unwrap().id);
        if let Some(mut ctx) = unsafe { CONTEXT.take() } {
            ctx.get_network().send(request.get_ref().msg.clone().unwrap());
        }

        Ok(Response::new(ZchronodResp {
            resp: None,
        }))
    }
}
