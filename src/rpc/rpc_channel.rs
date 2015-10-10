use std::fmt::Debug;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::thread::sleep_ms;

use mio::*;
use mio::tcp::*;

use protobuf::Message;

pub struct RpcCall<'a> 
{
  method: &'a str,
  request: &'a Message,
  response: &'a mut Message
}

pub trait RpcChannel 
{
  fn invoke(&mut self, call: &RpcCall);
  fn close(&mut self);
}

struct Request;

pub struct RpcChannelFactory;

impl RpcChannelFactory 
{
  pub fn new(addr: &str) -> Result<Box<RpcChannel>, HdfsErr> {
    
    let socket_addr: SocketAddr = FromStr::from_str(&addr).unwrap();
    let (sock, _) = TcpSocket::v4().unwrap().connect(&socket_addr).unwrap();
    
    Ok(Box::new(RpcChannelImpl {
      sock: sock,
      event_loop: EventLoop::new().unwrap(),
      requests: Arc::new(RwLock::new(HashMap::new()))
    }))
  }
}

struct RpcChannelImpl 
{
  sock: TcpStream,
  event_loop: EventLoop<RpcChannelHandler>,
  requests: Arc<RwLock<HashMap<u64, Request>>> 
}

impl RpcChannelImpl 
{
  fn connect(addr: SocketAddr) -> Result<RpcChannelImpl, HdfsErr> {
    Err(HdfsErr::UnknownHost)
  }
}

impl RpcChannel for RpcChannelImpl 
{
  fn invoke(&mut self, call: &RpcCall) 
  {
  }
  
  fn close(&mut self) 
  {
    self.event_loop.shutdown();
  }
}

struct RpcChannelHandler;

impl Handler for RpcChannelHandler {
  type Timeout = ();
  type Message = u32;
}

#[test]
pub fn test_channel() {
  let addr = format!("127.0.0.1:{}", 8020);
  let channel: Box<RpcChannel> = RpcChannelFactory::new(&addr).ok().unwrap();
  
  
}