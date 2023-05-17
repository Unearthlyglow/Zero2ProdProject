
[5/17/23 zero2prod]
pg 36:: How do we find a random available port for our tests?
The operating system comes to the rescue, we iwll be using port 0.
Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan 
for an available port which will then be bound to the application. 

Taking a break at page 38. When I get back I will start on page 38 on 3.6.

PBR: If you have to take notes, try out doing it in mark down. 
With 

``` Rust
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}
```