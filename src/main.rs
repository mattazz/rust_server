fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    server.run();
}

struct Server {
    addr: String,
}
impl Server {
    fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    fn run(&mut self) {}
}

struct Requests {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
