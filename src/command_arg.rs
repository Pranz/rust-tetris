#![allow(non_camel_case_types)]

use core::str::FromStr;
use rustc_serialize::{Decodable,Decoder};
use std::net;

#[derive(Debug)]
pub struct Host(pub net::IpAddr);

#[derive(Debug,RustcDecodable)]
pub enum OnlineConnection{none,server,client}

#[derive(Debug)]
pub struct WindowSize(pub u32,pub u32);

#[derive(Debug,RustcDecodable)]
pub enum WindowMode{window,fullscreen}

impl Decodable for WindowSize{
	fn decode<D: Decoder>(d: &mut D) -> Result<Self,D::Error>{
		let str = try!(d.read_str());
		let str = &*str;
		let (w,h) = str.split_at(try!(str.find('x').ok_or_else(|| d.error("Invalid format: Missing 'x' in \"<WIDTH>x<HEIGHT>\""))));
		Ok(WindowSize(
			try!(FromStr::from_str(w).map_err(|_| d.error("Invalid format: <WIDTH> in (<SIZE> = <WIDTH>x<HEIGHT>) is not a valid positive integer"))),
			try!(FromStr::from_str(&h[1..]).map_err(|_| d.error("Invalid format: <HEIGHT> in (<SIZE> = <WIDTH>x<HEIGHT>) is not a valid positive integer")))
		))
	}
}

impl Decodable for Host{
	fn decode<D: Decoder>(d: &mut D) -> Result<Self,D::Error>{
		let str = try!(d.read_str());
		let str = &*str;

		Ok(Host(match net::IpAddr::from_str(str){
			Ok(addr) => addr,
			Err(_)   => try!(try!(try!(
				net::lookup_host(str).map_err(|_| d.error("Error when lookup_host")))
				.next().ok_or_else(|| d.error("No hosts when lookup_host")))
				.map_err(|_| d.error("Error when converting to IpAddr with lookup_host")))
				.ip()
		}))
	}
}
