pub mod packet;



use core::mem;
use rand::{Rng,StdRng};
use std::{net,sync,thread};
use std::error::Error;

use super::{client,Packet};
use data::input::Input;
use gamestate::PlayerId;

pub fn start(host_addr: net::SocketAddr,input_sender: sync::mpsc::Sender<(Input,PlayerId)>) -> Result<(),()>{
	match net::UdpSocket::bind(host_addr){
		Ok(socket) => {
			println!("Server: Listening on {}...",host_addr);

			//Listen for packets from clients in a new thread
			thread::spawn(move ||{
				let mut buffer = super::packet::buffer();
				let mut connection_id_gen = StdRng::new().unwrap();

				//For each received packet
				while let Ok((buffer_size,address)) = socket.recv_from(&mut buffer){
					if buffer_size > mem::size_of_val(&buffer){
						println!("Server: Client sent too big of a packet: {} bytes",buffer_size);
						continue;
					}

					//Deserialize packet
					match ::bincode::serde::deserialize(&buffer[..]){
						Ok(Packet{data,..}) => match data{
							//Recevied connection request
							client::packet::Data::Connect{protocol_version} => {
								print!("Server: Connection request from {}... ",address);
								match protocol_version{
									1 => {
										println!("OK");
										socket.send_to(
											&*packet::Data::ConnectionEstablished{
												connection: connection_id_gen.gen::<u32>()
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
								input_sender.send((input,1)).unwrap()
							},

							//Received unimplemented TODO stuff
							data => println!("Server: {:?}",data),
						},

						//Received other stuff
						Err(e) => println!("Server: Receuived data but error: {}: {}",e,e.description())
					}
				}
			});
			Ok(())
		},
		Err(e) => {
			println!("Server socket error: {:?}",e);
			Err(())
		}
	}
}
