use rserver::{Rserver,config::RserverConfig};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化 tracing 日志
    tracing_subscriber::fmt::init();
    
    // 创建默认配置
    let config = RserverConfig::default();
    
    println!("🚀 启动基础HTTP服务器...");
    println!("📍 监听地址: {}:{}", config.host, config.port);
    println!("⚡ TCP_NODELAY: {}", config.tcp_nodelay);
    
    // 创建并运行服务器
    let server = Rserver::new(config);
    
    // 运行服务器（这会阻塞直到出错）
    if let Err(e) = server.run().await {
        eprintln!("❌ 服务器运行出错: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}
