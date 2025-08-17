use rserver::{Rserver,config::RserverConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化 tracing 日志
    tracing_subscriber::fmt::init();
    
    // 创建自定义配置
    let config = RserverConfig::default()
        .host("0.0.0.0".to_string())  // 监听所有网络接口
        .port(3000)                    // 使用端口3000
        .tcp_nodelay(false)            // 禁用TCP_NODELAY
        .tcp_keepalive(Some(Duration::from_secs(60)))  // 设置keepalive为60秒
        .max_connections(500);         // 最大连接数500
    
    println!("🚀 启动自定义配置HTTP服务器...");
    println!("📍 监听地址: {}:{}", config.host, config.port);
    println!("⚡ TCP_NODELAY: {}", config.tcp_nodelay);
    println!("💓 TCP_KEEPALIVE: {:?}", config.tcp_keepalive);
    println!("🔗 最大连接数: {}", config.max_connections);
    
    // 创建并运行服务器
    let server = Rserver::new(config);
    
    // 运行服务器
    if let Err(e) = server.run().await {
        eprintln!("❌ 服务器运行出错: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}
