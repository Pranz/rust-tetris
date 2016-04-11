//! Command argument types
//! The deserialized structures of input.

#![allow(non_camel_case_types,non_snake_case)]

use core::fmt;
use core::str::FromStr;
use docopt;
use opengl_graphics::OpenGL;
use rustc_serialize::{Decodable,Decoder};
use std::net;

docopt!(pub Args derive Debug,concat!("
Usage: ",PROGRAM_NAME!()," [options]
	   ",PROGRAM_NAME!()," --help

A game with tetrominos falling.

Options:
  -h, --help            Show this message
  -v, --version         Show version
  --credits             Show credits/staff
  --manual              Show instruction manual/guide for the game
  --online=CONNECTION   Available modes: none, server, client [default: none]
  --host=ADDR           Network address used for the online connection [default: 0.0.0.0]
  --port=N              Network port used for the online connection [default: 7374]
  --window-size=SIZE    Window size [default: 800x600]
  --window-mode=MODE    Available modes: window, fullscreen [default: window]
  --gl-backend=BACKEND  Not implemented yet. Available backends: sdl2, glfw, glutin [default: glutin]
  --gl-version=NN       Available versions: 20, 21, 30, 31, 32, 33, 40, 41, 42, 43, 44, 45 [default: 32]
"),
	flag_online     : OnlineConnection,
	flag_host       : Host,
	flag_port       : Port,
	flag_window_size: WindowSize,
	flag_window_mode: WindowMode,
	flag_gl_backend : GlBackend,
	flag_gl_version : GlVersion,
);

///Workaround for the creation of Args because the docopt macro is not making everything public
#[inline(always)]
pub fn Args_docopt() -> docopt::Docopt{Args::docopt()}

#[derive(Debug)]
pub struct Host(pub net::IpAddr);
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

pub type Port = u16;

#[derive(Debug,RustcDecodable)]
pub enum OnlineConnection{none,server,client}

#[derive(Debug)]
pub struct WindowSize(pub u32,pub u32);
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

#[derive(Debug,RustcDecodable)]
pub enum WindowMode{window,fullscreen}

#[derive(Debug,RustcDecodable)]
pub enum GlBackend{sdl2,glfw,glutin}

pub struct GlVersion(pub OpenGL);
impl fmt::Debug for GlVersion{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		write!(f,"{}",match self.0{
			OpenGL::V2_0 => "v2.0",
			OpenGL::V2_1 => "v2.1",
			OpenGL::V3_0 => "v3.0",
			OpenGL::V3_1 => "v3.1",
			OpenGL::V3_2 => "v3.2",
			OpenGL::V3_3 => "v3.3",
			OpenGL::V4_0 => "v4.0",
			OpenGL::V4_1 => "v4.1",
			OpenGL::V4_2 => "v4.2",
			OpenGL::V4_3 => "v4.3",
			OpenGL::V4_4 => "v4.4",
			OpenGL::V4_5 => "v4.5",
		})
	}
}
impl Decodable for GlVersion{
	fn decode<D: Decoder>(d: &mut D) -> Result<Self,D::Error>{
		let str = try!(d.read_str());
		let str = &*str;
		Ok(GlVersion(
			match try!(u8::from_str(str).map_err(|_| d.error("Invalid format: <NN> in (--gl-version=NN) is not a valid positive integer"))){
				20 => OpenGL::V2_0,
				21 => OpenGL::V2_1,
				30 => OpenGL::V3_0,
				31 => OpenGL::V3_1,
				32 => OpenGL::V3_2,
				33 => OpenGL::V3_3,
				40 => OpenGL::V4_0,
				41 => OpenGL::V4_1,
				42 => OpenGL::V4_2,
				43 => OpenGL::V4_3,
				44 => OpenGL::V4_4,
				45 => OpenGL::V4_5,
				_  => return Err(d.error("Invalid version number was given"))
			}
		))
	}
}
