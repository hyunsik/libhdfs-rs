pub struct BlockLocation 
{
  corrupt        : bool,
  length         : i64,
  /// Offset of the block in the file
  offset         : i64,
  /// Datanode hostnames 
  hosts          : Vec<String>,
  ///  Datanode IP:xferPort for accessing the block
  names          : Vec<String>,
  /// Full path name in network topology
  topology_paths : Vec<String>,
}