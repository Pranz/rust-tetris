pub mod packet;



use byte_conv::As;
use core::mem;
use rand::{Rng,StdRng};
use std::{net,sync,thread};
use std::error::Error;

use super::{client,Packet};
use data::{Input,Request};

pub fn start(host_addr: net::SocketAddr,request_sender: sync::mpsc::Sender<Request>) -> Result<(),()>{
	match net::UdpSocket::bind(host_addr){
		Ok(socket) => {
			println!("Server: Listening on {}...",host_addr);

			//Listen for packets from clients in a new thread
			thread::spawn(move ||{
				let mut _buffer: Packet<packet::Data> = unsafe{mem::uninitialized()};
				let mut buffer = unsafe{_buffer.as_bytes_mut()};
				let mut connection_id_gen = StdRng::new().unwrap();

				//For each received packet
				while let Ok((buffer_size,address)) = socket.recv_from(buffer){
					if buffer_size > buffer.len(){
						println!("Server: Client sent too big of a packet: {} bytes",buffer_size);
						continue;
					}

					//Deserialize packet
					match Packet::deserialize(buffer){
						Ok(Packet{data,..}) => match data{
							//Recevied connection request
							client::packet::Data::Connect{protocol_version} => {
								print!("Server: Connection request from {}... ",address);
								match protocol_version{
									1 => {
										let connection_id = connection_id_gen.gen::<u32>();
										println!("OK (As id: {})",connection_id);
										socket.send_to(
											&*packet::Data::ConnectionEstablished{
												connection: connection_id
											}.into_packet(0).serialize(),
											address
										).unwrap();
									},

									version => {
										println!("Server: Invalid version: {}",version);
										socket.send_to(&*packet::Data::ConnectionInvalid.into_packet(0).serialize(),address).unwrap();
									}
								}
							},

							//Received player input
							client::packet::Data::PlayerInput{input,..} => {
								request_sender.send(Request::Input{input: input,player: 0}).unwrap();
							},

							//Received player add reqeust
							client::packet::Data::PlayerCreateRequest{settings,..} => {
								request_sender.send(Request::PlayerAdd{settings: settings}).unwrap();

								socket.send_to(
									&*packet::Data::PlayerCreateResponse{
										player: 0,
										rng_seed: 0,
									}.into_packet(0).serialize(),
									address
								).unwrap();
							},

							//Received unimplemented TODO stuff
							data => println!("Server: {:?}",data),
						},

						//Received other stuff
						Err(e) => println!("Server: Received data but error: {}: {}",e,e.description())
					}
				}
			});
			Ok(())
		},
		Err(e) => {
			println!("Server:  Socket error: {:?}",e);
			Err(())
		}
	}
}
