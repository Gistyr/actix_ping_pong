# actix_ping_pong
Very basic "ping pong" server built with actix-web     
#### Example Settings
```
impl Default for Settings {
    fn default() -> Self {
        return Self {
            logging: true,
            log_level: "debug".to_string(),
            client_url: "https://my.domain.example.com".to_string(),
            bind_socket: "127.0.0.1:8080".to_string(),
        }
    }
}
```
