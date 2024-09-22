# 🚀 Greatway: Rust-Powered API Gateway

Greatway is a high-performance, feature-rich API Gateway built in Rust. Designed for speed, security, and ease of use, it provides a robust solution for managing and protecting your microservices architecture.

## ✨ Features

Greatway comes with batteries included, offering:

- 🛑 **Rate Limiting**: Protect your services from abuse and ensure fair usage.
- 🔐 **Authentication**: Secure your APIs with built-in authentication mechanisms.
- 📊 **Request Logging**: Gain insights with comprehensive, configurable logging.

## 🤔 Why Greatway?

- ⚡ **Performance**: Built in Rust, Greatway offers exceptional speed and low resource usage.
- 🛡️ **Security**: Designed with a security-first approach to safeguard your APIs.
- 🧩 **Simplicity**: Easy to set up and configure, reducing operational overhead.
- 📈 **Scalability**: Efficiently handles high-traffic scenarios.

## 🚀 Quick Start

1. Add Greatway to your `Cargo.toml`:
   ```toml
   [dependencies]
   greatway = "0.1.0"
   ```

2. Integrate Greatway into your Actix-Web application:

   ```rust
   use actix_web::{web, App, HttpServer};
   use greatway::Greatway;

   #[actix_web::main]
   async fn main() -> std::io::Result<()> {
       HttpServer::new(|| {
           App::new()
               .wrap(Greatway::new())
               .service(web::resource("/api").to(|| async { "API Gateway Operational" }))
       })
       .bind("127.0.0.1:8080")?
       .run()
       .await
   }
   ```

## ⚙️ Configuration

Greatway is designed to be highly configurable. Here's a basic example:

```rust
let gateway = Greatway::new()
    .with_rate_limit(100, Duration::from_secs(60))
    .with_jwt_auth("your-secret-key")
    .with_logging(LogLevel::Info);
```

For more advanced configurations, please refer to our [documentation](https://docs.greatway.rs).

## 📊 Performance

Benchmarks show that Greatway can handle:

- 🚄 Up to 100,000 requests per second on standard hardware
- ⏱️ Sub-millisecond latency for routing requests
- 🪶 Minimal CPU and memory footprint

## 🤝 Contributing

We welcome contributions to Greatway! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## 📜 License

Greatway is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## 🆘 Support

For questions, issues, or feature requests, please open an issue on our [GitHub repository](https://github.com/greatway/greatway).

---

🌟 Greatway: Empowering your APIs with speed, security, and simplicity.
