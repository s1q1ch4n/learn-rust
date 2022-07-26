use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // Self可以代替Server结构体名字
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => {}
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
