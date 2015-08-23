pub mod packet;



use core::mem;
use std::{net,sync,thread};
use std::error::Error;

use super::{server,Packet};
use data::input::Input;
use gamestate::PlayerId;

pub fn start(server_addr: net::SocketAddr,input_sender: sync::mpsc::Sender<(Input,PlayerId)>) -> Result<net::UdpSocket,()>{
	match net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),0)){
		Ok(socket) => {
			println!("Client: Connecting to {}...",server_addr);

			//Send connect packet to server
			try!(connect_server(&socket,server_addr,5));

			//Listen for packets from server in a new thread
			{let socket = socket.try_clone().unwrap();thread::spawn(move ||{
				let mut buffer = super::packet::buffer();

				//For each received packet
				while let Ok((buffer_size,address)) = socket.recv_from(&mut buffer){
					if server_addr != address{
						println!("Client: Not server who sent the packet ({} (Server) != {})",server_addr,address);
						continue;
					}

					if buffer_size > mem::size_of_val(&buffer){
						println!("Client: Server sent too big of a packet: {} bytes",buffer_size);
						continue;
					}

					//Deserialize packet
					match ::bincode::serde::deserialize(&buffer[..]){
						Ok(Packet{data,..}) => match data{
							//Received connection request established
							server::packet::Data::ConnectionEstablished{connection} => {
								println!("Client: Connection established to {} (Id: {})",address,connection);
							},

							//Received player input
							server::packet::Data::PlayerInput{input,..} => {
								input_sender.send((input,1)).unwrap();
							},

							//Received unimplemented TODO stuff
							data => println!("Client: {:?}",data),
						},

						//Received other stuff
						Err(e) => println!("Client: Receuived data but error: {}: {}",e,e.description())
					}
				}
			});}

			Ok(socket)
		},
		Err(e) => {
			println!("Client: Socket error: {:?}",e);
			Err(())
		}
	}
}

pub fn connect_server(socket: &net::UdpSocket,address: net::SocketAddr,mut retries: u8) -> Result<(),()>{
	loop{match socket.send_to(
		&*packet::Data::Connect{
			protocol_version: 1
		}.into_packet(0).serialize(),//TODO: Packet id and all other `into_packet`s
		address
	){
		Ok(_) => return Ok(()),
		Err(e) => {use std::io::ErrorKind::*;match e.kind(){
			Interrupted | TimedOut => {
				match retries.checked_sub(1){
					Some(n) => {
						retries = n;
						continue;
					},
					None => {
						println!("Client: Error when sending socket: Gave up, no more retries");
						return Err(());
					}
				}
			},
			kind => {
				println!("Client: {:?} error when sending socket",kind);
				return Err(());
			}
		}}
	}}
}
