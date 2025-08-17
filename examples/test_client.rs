use tokio::task::JoinSet;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始并发测试...");
    let start_time = Instant::now();
    
    // 创建任务集合
    let mut join_set = JoinSet::new();
    
    // 并发发送 10 个请求
    for i in 1..=10 {
        join_set.spawn(async move {
            let client = reqwest::Client::new();
            let url = format!("http://127.0.0.1:8080/hello");
            
            let start = Instant::now();
            match client.get(&url).send().await {
                Ok(response) => {
                    let duration = start.elapsed();
                    let status = response.status();
                    let body = response.text().await.unwrap_or_else(|_| "无法读取响应体".to_string());
                    
                    println!("✅ 请求 {}: 状态={}, 耗时={:?}, 响应长度={}", 
                            i, status, duration, body.len());
                    Ok((i, status, duration))
                }
                Err(e) => {
                    let duration = start.elapsed();
                    println!("❌ 请求 {}: 失败, 耗时={:?}, 错误={}", i, duration, e);
                    Err((i, e))
                }
            }
        });
    }
    
    // 等待所有请求完成
    let mut success_count = 0;
    let mut fail_count = 0;
    
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok((i, status, duration))) => {
                success_count += 1;
                println!("🎯 请求 {} 成功完成: 状态={}, 耗时={:?}", i, status, duration);
            }
            Ok(Err((i, e))) => {
                fail_count += 1;
                println!("💥 请求 {} 失败: {}", i, e);
            }
            Err(e) => {
                fail_count += 1;
                println!("💥 任务执行失败: {}", e);
            }
        }
    }
    
    let total_time = start_time.elapsed();
    println!("\n📊 测试结果:");
    println!("   总耗时: {:?}", total_time);
    println!("   成功请求: {}", success_count);
    println!("   失败请求: {}", fail_count);
    println!("   平均耗时: {:?}", total_time / 10);
    
    if success_count > 0 {
        println!("🎉 并发测试成功！你的 Rserver 可以同时处理多个连接！");
    } else {
        println!("😞 所有请求都失败了，请检查服务器是否正在运行");
    }
    
    Ok(())
} 