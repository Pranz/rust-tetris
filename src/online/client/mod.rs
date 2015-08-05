pub mod packet;



use num::traits::FromPrimitive;
use std::{net,sync,thread};

use data::input::Input;
use gamestate::PlayerId;

pub fn start(server_addr: net::SocketAddr,input_sender: sync::mpsc::Sender<(Input,PlayerId)>) -> Result<net::UdpSocket,()>{
	match net::UdpSocket::bind((net::Ipv4Addr::new(0,0,0,0),0)){
        Ok(socket) => {
            use core::mem;
			use endian_type::types::*;


            println!("Client: Connecting to {}...",server_addr);

            //Send connect packet
            {let mut retry = 0;loop{match socket.send_to(
                packet::Connect{
                    protocol_version: u16_le::from(1)
                }.into_packet().as_bytes(),
                server_addr
            ){
                Ok(_) => break,
                Err(e) => {use std::io::ErrorKind::*;match e.kind(){
                    Interrupted | TimedOut => {
                        if retry>5{
                            println!("Client: Error when sending socket: Retried {} times",retry);
                            return Err(());
                        }else{
                            retry+=1;
                            continue;
                        }
                    },
                    kind => {
                        println!("Client: {:?} error when sending socket",kind);
                        return Err(());
                    }
                }}
            }}}

            //Listen for server packets
            {let socket = socket.try_clone().unwrap();thread::spawn(move ||{
                use super::server::packet::*;

                let mut buffer: PacketBytes = [0; SIZE];
                while let Ok((buffer_size,address)) = socket.recv_from(&mut buffer){
                    //First byte is the packet type
                    match Type::from_packet_bytes(&buffer[..]){
                        //Recevied connection request established
                        Some(Type::ConnectionEstablished) if buffer_size==mem::size_of::<super::Packet<Type,ConnectionEstablished>>() => {
                            let packet = ConnectionEstablished::from_packet_bytes(&buffer[..buffer_size]);
                            println!("Client: Connection established to {} (Id: {})",address,Into::<u32>::into(packet.connection_id));
                        },

                        //Recevied player input
                        Some(Type::PlayerInput) if buffer_size==mem::size_of::<super::Packet<Type,PlayerInput>>() => {
                            let packet = PlayerInput::from_packet_bytes(&buffer[..buffer_size]);
                            match Input::from_u8(packet.input){
                                Some(input) => input_sender.send((input,1)).unwrap(),
                                None => ()
                            }
                        },

                        //Received unimplemented TODO stuff
                        Some(ty) => println!("Client: {:?}: {:?} (Size: {})",ty,buffer,buffer_size),

                        //Received other stuff
                        None => ()
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
