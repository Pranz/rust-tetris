use std::net;

pub enum ConnectionType{
    Server,
    Client(net::UdpSocket,net::SocketAddr),
    None
}
