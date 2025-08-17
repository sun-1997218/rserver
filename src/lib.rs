mod listener;
mod config;
use crate::listener::{Listener,TcpListenerWithOptions};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::net::SocketAddr;
use crate::config::RserverConfig;
use tokio::task::JoinSet;

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
    pub async fn run(&self) ->std::io::Result<()>{
       let addr = format!("{}:{}",self.config.host,self.config.port)
            .parse::<SocketAddr>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput,e))?;

        let mut listener = TcpListenerWithOptions::new(addr,self.config.tcp_nodelay).await?;
        tracing::info!("Server listening on {}", addr);


        let mut join_set = JoinSet::new();
        // 简单的连接接受循环
        loop {
            while  join_set.try_join_next().is_some(){}
            match listener.accept().await {
                Ok((stream, addr)) => {
                    tracing::info!("New connection from {}", addr);
                    //使用join_set来管理连接处理器
                    join_set.spawn(async move{
                          if let Err(e) = Self::handle_connection(stream,addr).await{
                            tracing::error!("Error handling connection from {}: {}", addr, e);
                          }
                        }
                    );
                },
                Err(e) => {
                    tracing::error!("Accept error: {}", e);
                }
            }
        }
    }

    async fn handle_connection( stream:TcpStream,addr:SocketAddr)->std::io::Result<()>{
        tracing::info!("Handling connection from {}", addr);
    
            // 简单的HTTP响应
            let response = format!(
                "HTTP/1.1 200 OK\r\n\
                 Content-Length: {}\r\n\
                 Content-Type: text/plain\r\n\
                 \r\n\
                 Hello from Rserver! Your IP: {}",
                format!("Hello from Rserver! Your IP: {}", addr).len(),
                addr
            );
            
            // 发送响应
            let mut stream = stream;
            stream.write_all(response.as_bytes()).await?;
            stream.flush().await?;
            
            tracing::info!("Response sent to {}", addr);
            Ok(())
    }
 
}



 
    

