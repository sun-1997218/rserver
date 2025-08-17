pub mod listener;
pub mod config;
use crate::listener::{Listener,TcpListenerWithOptions};
use crate::config::RserverConfig;
use tokio::net::TcpStream;

use std::net::SocketAddr;
use std::task::{Context,Poll};
use std::time::Duration;
use std::future::Future;

use tokio::task::JoinSet;


//hyper

use hyper::{Request,Response};
use hyper::body::Incoming;
use hyper_util::service::TowerToHyperService;
use tower::Service;

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct Rserver{
    config: RserverConfig,
}



impl Rserver{ 
    pub fn new(config:RserverConfig)->Self{
        Self{
            config,
        }
    }
    pub fn config(&self)-> &RserverConfig{
        &self.config
    }
}

impl Rserver{
    //run函数是Rserver的入口函数，它负责启动服务器并处理连接
    pub async fn run(&self) -> Result<(), BoxError> {
               let addr = format!("{}:{}",self.config.host,self.config.port)
            .parse::<SocketAddr>()?;

        let mut listener = TcpListenerWithOptions::new(addr,self.config.tcp_nodelay).await?;
        tracing::info!("Server listening on {}", addr);


        let mut join_set: JoinSet<Result<(), BoxError>> = JoinSet::new();
        // 简单的连接接受循环
        loop {
            // 清理已完成的连接
            while join_set.try_join_next().is_some() {
                tracing::info!("join_set_next {} closed", addr);
            }
            match listener.accept().await {
                Ok((stream, addr)) => {
                    tracing::info!("New connection from {}", addr);
                    //使用join_set来管理连接处理器
                    join_set.spawn(async move {
                        Self::handle_connection(stream, addr).await
                    });
                },
                Err(e) => {
                    tracing::error!("Accept error: {}", e);
                }
            }
        }
    }

    async fn handle_connection(stream: TcpStream, addr: SocketAddr) -> Result<(), BoxError> {
        tracing::info!("Handling connection from {}", addr);

        // 按照 sui-http 的方式：创建一个简单的服务，直接处理请求
        let service = SimpleHttpService{};

              // 创建 Hyper 连接构建器
        let builder = hyper_util::server::conn::auto::Builder::new(
            hyper_util::rt::TokioExecutor::new()
        );
        
        // 将 TcpStream 转换为 Hyper 的 IO
        let io = hyper_util::rt::TokioIo::new(stream);
        
        // 使用 Hyper 处理 HTTP 连接
        let hyper_service = TowerToHyperService::new(service);
        if let Err(e) = builder.serve_connection_with_upgrades(io, hyper_service).await {
            tracing::error!("❌ HTTP connection error: {}", e);
            return Err(e);
        }
        
        tracing::info!("🔌 Connection closed for {}", addr);
        Ok(())
          
         
    }
 
}


//里面没有成员，所以不需要实现new函数
#[derive(Clone)]
pub struct SimpleHttpService{}

impl Service<Request<Incoming>> for SimpleHttpService {
    type Response = Response<String>;
    type Error = BoxError;  // 使用我们定义的 BoxError
    type Future = std::pin::Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        Box::pin(async move {
            // 从请求扩展中获取连接信息
            // 从请求扩展中获取连接信息，避免临时值生命周期问题
            let client_addr = req
                .extensions()
                .get::<SocketAddr>()
                .cloned()
                .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 0)));

            // 模拟一些处理时间
            tokio::time::sleep(Duration::from_millis(100)).await;
            // 根据路径返回不同响应
            let (status, body) = match req.uri().path() {
                "/" => (200, "Welcome to Rserver! ��".to_string()),
                "/hello" => (200, format!("Hello from Rserver! Your IP: {}", client_addr)),
                "/status" => (200, "🟢 Server is running".to_string()),
                "/api/version" => (200, "Rserver v1.0.0".to_string()),
                _ => (404, "❌ Not Found".to_string()),
            };
            
            let response = Response::builder()
                .status(status)
                .header("Content-Type", "text/plain; charset=utf-8")
                .header("Server", "Rserver/1.0")
                .body(body.to_string())
                .unwrap();
            
            Ok(response)
        })
    }
}





 
    

