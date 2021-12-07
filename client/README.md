# Redcar rust client

A rust client for Redcar

## Example
operation redcar server by rust client

KV to server
```rust
#[tokio::main]
async fn main() {
    let conf = config::Config {
        host: "http://127.0.0.1:8519",
    };
    let mut c = client::Client::new(&conf);
    let key = String::from("bar");
    let val = String::from("foo");
    c.put(key.clone().into_bytes(), val.clone().into_bytes())
        .await;
    let r_val = c.range(key.clone().into_bytes()).await;
    if r_val[0] != val.clone().into_bytes() {
        panic!(
            "recv val is not raw val: {:?} {:?}",
            String::from_utf8(r_val[0].clone()).unwrap(),
            val
        )
    }
}
```
