pub mod packet;



use byte_conv::As;
use core::mem;
use std::{net,sync,thread};
use std::error::Error;

use super::{server,Packet};
use ::data::{player,Request};
use ::game::data::{WorldId,PlayerId};

pub fn start(server_addr: net::SocketAddr,request_sender: sync::mpsc::Sender<Request<PlayerId,WorldId>>) -> Result<net::UdpSocket,()>{
	match net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),0)){
		Ok(socket) => {
			println!("Client: Connecting to {}...",server_addr);

			//Send connect packet to server
			try!(connect_server(&socket,server_addr,5));

			//Listen for packets from server in a new thread
			{let socket = socket.try_clone().unwrap();thread::spawn(move ||{
				let mut _buffer: Packet<packet::Data> = unsafe{mem::uninitialized::<Packet<packet::Data>>()};
				let buffer = unsafe{_buffer.as_bytes_mut()};
				let mut connected = false;
				let mut connection_id;

				//For each received packet
				while let Ok((buffer_size,address)) = socket.recv_from(buffer){
					if server_addr != address{
						println!("Client: Not server who sent the packet ({} (Server) != {})",server_addr,address);
						continue;
					}

					if buffer_size > buffer.len(){
						println!("Client: Server sent too big of a packet: {} bytes",buffer_size);
						continue;
					}

					//Deserialize packet
					match Packet::deserialize(buffer){
						Ok(Packet{data,..}) => match data{
							//Received connection request established
							server::packet::Data::ConnectionEstablished{connection} if !connected => {
								println!("Client: Connection established to {} (Id: {})",address,connection);
								connected = true;
								connection_id = connection;

								//Request new player
								println!("Client: Request new player...");
								let settings = player::Settings{
									gravityfall_frequency: 1.0,
									fastfall_shadow      : true,
								};
								socket.send_to(
									&*packet::Data::Request{
										connection: connection_id,
										request: Request::PlayerAdd{
											settings: settings,
											world: 1
										}
									}.into_packet(0).serialize(),//TODO: Packet id and all other `into_packet`s
									address
								).unwrap();

								request_sender.send(Request::PlayerAdd{settings: settings,world: 1}).unwrap();
							},

							//Received player input
							server::packet::Data::PlayerInput{input,..} if connected => {
								request_sender.send(Request::PlayerInput{input: input,player: 0}).unwrap();
							},

							//Received player add response
							//server::packet::Data::PlayerCreateResponse{..} if connected => {},

							//Received unimplemented TODO stuff
							data => println!("Client: {:?} (Connected: {})",data,connected),
						},

						//Received other stuff
						Err(e) => println!("Client: Received data but error: {}: {}",e,e.description())
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
	//Send packet with retries
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
						println!("Client: Error when sending socket: Gave up");
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
