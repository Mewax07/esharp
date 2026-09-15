use crate::server::Server;

pub(crate) mod math;
pub(crate) mod server;

fn main() {
    let server = Server::new();
    server.run();
}
