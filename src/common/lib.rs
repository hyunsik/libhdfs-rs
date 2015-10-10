use std::result::Result;

pub enum HdfsErr 
{
  UnknownHost,       // cannot resolve the host name
  ConnectionTimeout, // no TCP handshaking for a given time
  ConnectionRefused, // no service listening on the port
}

pub type HdfsResult<T> = Result<T, HdfsErr>;