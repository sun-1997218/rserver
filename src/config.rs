use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RserverConfig {
    pub host: String,
    pub port: u16,
    pub tcp_nodelay: bool,
    pub tcp_keepalive: Option<Duration>,
    pub max_connections: usize,
    pub accept_http1: bool,
    pub enable_connect_protocol: bool,
}

impl Default for RserverConfig{
    fn default() -> Self{
        Self{
           host:"127.0.0.1".to_string(),
           port:8080,
           tcp_nodelay:true,
           tcp_keepalive:None,
           max_connections:1000,
           accept_http1:true,
           enable_connect_protocol:true,
        }
    }
}

impl RserverConfig{
    pub fn host(mut self ,host:String) -> Self{
        self.host = host;
        self
    }
    pub fn port(mut self ,port:u16) -> Self{
        self.port = port;
        self
    }
    pub fn tcp_nodelay(mut self ,tcp_nodelay:bool) -> Self{ 
        self.tcp_nodelay = tcp_nodelay;
        self
    }
    pub fn tcp_keepalive(mut self ,tcp_keepalive:Option<Duration>) -> Self{
        self.tcp_keepalive = tcp_keepalive;
        self
    }
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }
    
    pub fn accept_http1(mut self, accept: bool) -> Self {
        self.accept_http1 = accept;
        self
    }
    
    pub fn enable_connect_protocol(mut self, enable: bool) -> Self {
        self.enable_connect_protocol = enable;
        self
    }
}
