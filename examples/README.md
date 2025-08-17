# Rserver 示例

这个目录包含了使用 Rserver 库的各种示例。

## 示例列表

### 1. 基础服务器 (`basic_server.rs`)
最简单的HTTP服务器示例，使用默认配置：
```bash
cargo run --example basic_server
```
- 监听地址：127.0.0.1:8080
- 使用默认配置
- 适合快速测试

### 2. 自定义配置服务器 (`custom_config_server.rs`)
展示如何自定义服务器配置：
```bash
cargo run --example custom_config_server
```
- 监听地址：0.0.0.0:3000
- 自定义TCP选项
- 自定义连接限制

### 3. 测试客户端 (`test_client.rs`)
用于测试HTTP服务器的客户端：
```bash
cargo run --example test_client
```
- 连接到本地服务器
- 发送HTTP请求
- 显示服务器响应

## 使用方法

1. **启动服务器**：
   ```bash
   # 启动基础服务器
   cargo run --example basic_server
   
   # 或启动自定义配置服务器
   cargo run --example custom_config_server
   ```

2. **测试服务器**：
   ```bash
   # 在另一个终端中运行测试客户端
   cargo run --example test_client
   ```

3. **查看日志**：
   服务器会输出详细的连接和请求日志。

## 注意事项

- 确保端口没有被其他程序占用
- 基础服务器默认监听 8080 端口
- 自定义配置服务器监听 3000 端口
- 测试客户端默认连接到 127.0.0.1:8080
