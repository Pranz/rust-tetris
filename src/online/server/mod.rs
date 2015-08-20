pub mod packet;


use num::traits::FromPrimitive;
use rand::{Rng,StdRng};
use std::{net,sync,thread};

use data::input::Input;
use endian_type::types::*;
use gamestate::PlayerId;

pub fn start(host_addr: net::SocketAddr,input_sender: sync::mpsc::Sender<(Input,PlayerId)>) -> Result<(),()>{
	match net::UdpSocket::bind(host_addr){
		Ok(socket) => {
			use core::mem;

			println!("Server: Listening on {}...",host_addr);
			thread::spawn(move ||{
				use online::client::packet::*;

				let mut buffer: PacketBytes = [0; SIZE];
				let mut connection_id_gen = StdRng::new().unwrap();
				while let Ok((buffer_size,address)) = socket.recv_from(&mut buffer){
					//First byte is the packet type
					match Type::from_packet_bytes(&buffer[..]){
						//Recevied connection request
						Some(Type::Connect) if buffer_size==mem::size_of::<super::Packet<Type,Connect>>() => {
							let packet = Connect::from_packet_bytes(&buffer[..buffer_size]);
							print!("Server: Connection request from {}... ",address);
							match packet.protocol_version.into(){
								1 => {
									println!("OK");
									socket.send_to(
										packet::ConnectionEstablished{
											connection_id: u32_le::from(connection_id_gen.gen::<u32>())
										}.into_packet(u16_le::from(0)).as_bytes(),
										address
									).unwrap();
								},

								version => {
									println!("Server: Invalid version: {}",version);
									socket.send_to(packet::ConnectionInvalid.into_packet(u16_le::from(0)).as_bytes(),address).unwrap();
								}
							}
						},

						//Received player input
						Some(Type::PlayerInput) if buffer_size==mem::size_of::<super::Packet<Type,PlayerInput>>() => {
							let packet = PlayerInput::from_packet_bytes(&buffer[..buffer_size]);
							match Input::from_u8(packet.input){
								Some(input) => input_sender.send((input,1)).unwrap(),
								None => ()
							}
						},

						//Received unimplemented TODO stuff
						Some(ty) => println!("Server: {:?}: {:?} (Size: {})",ty,buffer,buffer_size),

						//Received other stuff
						None => ()
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
