use std::sync::Arc;

use tokio::{net::TcpListener, runtime::Runtime};

use crate::core::{codec, handler::NetHandler};

pub struct Server<'a> {
    pub addr: &'a str,
    rt_boss: Arc<Runtime>,
    rt_worker: Arc<Runtime>,
}

impl<'a> Server<'a> {
    pub fn new(addr: &'a str) -> Server<'a> {
        let rt_boss = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(1)
            .build()
            .unwrap();
        let rt_worker = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(num_cpus::get() << 1)
            .build()
            .unwrap();
        Server {
            addr,
            rt_boss: Arc::new(rt_boss),
            rt_worker: Arc::new(rt_worker),
        }
    }
    pub async fn start(&self) {
        let listener = TcpListener::bind(self.addr).await.unwrap();
        println!("Server listening on {}", self.addr);
        let rt_worker = self.rt_worker.clone();
        let rt_boss = self.rt_boss.clone();
        rt_boss.spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                NetHandler::on_open();
                rt_worker.spawn(async move {
                    let (mut reader, _) = socket.split();
                    loop {
                        let data = codec::HttpCodec::decode(&mut reader).await;
                        match data {
                            Ok(data) => {
                                NetHandler::on_message(data);
                            }
                            Err(_) => {
                                NetHandler::on_error();
                                break;
                            }
                        }
                    }
                });
            }
        });
    }
}
