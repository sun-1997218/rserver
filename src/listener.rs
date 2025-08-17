use std::net::SocketAddr;
use tokio::net::{TcpListener,TcpStream};
use std::io;


pub trait Listener{
    type Io;
    type Addr;

    async fn accept(&mut self) -> io::Result<(Self::Io, Self::Addr)>;
}

pub struct TcpListenerWithOptions{
    inner:TcpListener,
    tcp_nodelay:bool,
   
}

impl TcpListenerWithOptions{
    //有async 和 ？操作符的函数，返回值只能是 Result<T,E>和Option<T>
    pub async fn new (addr:SocketAddr,tcp_nodelay:bool) -> io::Result<Self>{
        //使用TcpListener::bind绑定地址 tokio的TcpListener
        let inner = TcpListener::bind(addr).await?;
        Ok(Self{
            inner,
            tcp_nodelay,
           
        })
    }
}

impl Listener for TcpListenerWithOptions{
    type Io = TcpStream;
    type Addr = SocketAddr;

    /*TcpStream
    TcpStream 是tokio提供的TCP流，它实现了AsyncRead和AsyncWrite trait，可以用于读写TCP数据。
    TcpStream 提供了一些方法来设置TCP连接的选项，比如设置TCP_NODELAY和TCP_KEEPALIVE。
    TcpStream 还提供了一些方法来获取连接的本地和远程地址。
    TcpStream 还提供了一些方法来获取连接的本地和远程地址。
    */
    async fn accept(&mut self) -> io::Result<(Self::Io, Self::Addr)>{
        // 接受连接
        let (stream,addr) = self.inner.accept().await?;
        if self.tcp_nodelay{
            // 设置TCP_NODELAY
            stream.set_nodelay(true)?;
        }
        // 注意：tokio的TcpStream没有直接的set_keepalive方法
        // 如果需要设置keepalive，需要使用更底层的socket操作
        // 这里暂时跳过keepalive设置
        Ok((stream,addr))
    }
}