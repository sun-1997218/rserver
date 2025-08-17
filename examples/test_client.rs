use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 连接到服务器
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    
    println!("🔌 连接到服务器: {}", addr);
    
    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            println!("✅ 连接成功！");
            
            // 发送HTTP请求
            let request = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nConnection: close\r\n\r\n";
            stream.write_all(request.as_bytes()).await?;
            stream.flush().await?;
            
            println!("📤 发送请求: {}", request.trim());
            
            // 读取响应
            let mut buffer = Vec::new();
            let mut temp_buffer = [0; 1024];
            
            loop {
                match stream.read(&mut temp_buffer).await {
                    Ok(0) => break, // 连接关闭
                    Ok(n) => {
                        buffer.extend_from_slice(&temp_buffer[..n]);
                    }
                    Err(e) => {
                        eprintln!("❌ 读取响应出错: {}", e);
                        break;
                    }
                }
            }
            
            // 显示响应
            let response = String::from_utf8_lossy(&buffer);
            println!("📥 收到响应:");
            println!("{}", response);
            
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ 连接失败: {}", e);
            eprintln!("💡 请确保服务器正在运行 (cargo run --example basic_server)");
            std::process::exit(1);
        }
    }
}
