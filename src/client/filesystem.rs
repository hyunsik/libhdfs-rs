use common::HdfsResult;


use super::BlockLocation;
use super::FileStatus;
use super::FileSystemStats;
use super::Path;
use super::Permission;

pub trait FileSystem 
{
  fn connect(&self) -> HdfsResult<()>;
  
  fn connect_to(&self, uri: &str) -> HdfsResult<()>;
  
  fn connect_with(
    &self, 
    uri: &str, 
    username: &str,
    token: &str) -> HdfsResult<()>;
  
  fn disconnect(&self);
  
  fn default_replica_num(&self) -> i32;
  
  fn default_block_size(&self) -> i64;
  
  fn home_dir(&self) -> &str;
  
  fn delete(&self, path: &str, recursive: bool) -> HdfsResult<()>;
  
  fn mkdir(&self, path: &str) -> HdfsResult<()>;
  
  fn mkdirs(&self, path: &str, permission: &Permission) -> HdfsResult<()>;
  
  fn get_file_status(&self, path: &str) -> HdfsResult<FileStatus>;
  
  fn get_file_block_locations(
    &self, 
    path: &str, 
    start: i64, 
    len: i64) -> Vec<BlockLocation>;
  
  // TODO - shoud return incremental iterator
  fn list_dirs(&self, path: &str) -> HdfsResult<()>;
  
  fn list_status(&self, path: &str) -> HdfsResult<Vec<FileStatus>>;
  
  fn set_owner(&self, path: &str, username: &str, groupname: &str) -> HdfsResult<()>;
  
  fn set_time(&self, path: &str, mtime: i64, atime: i64) -> HdfsResult<()>;
  
  fn set_perm(&self, path: &str, perm: &Permission) -> HdfsResult<()>;
  
  fn set_replica_num(&self, path: &str, replica_num: i16) -> HdfsResult<()>;
  
  fn rename(&self, src: &str, dst: &str) -> HdfsResult<()>;
  
  fn set_working_dir(&self, path: &str) -> HdfsResult<()>;
  
  fn working_dir(&self) -> HdfsResult<String>;
  
  fn exist(&self, path: &str) -> bool;
  
  fn stats(&self) -> HdfsResult<FileSystemStats>;
  
  fn truncate(&self, src: &str, size: i64) -> HdfsResult<()>;
  
  fn delegation_token_with(&self, renewer: &str) ->  HdfsResult<String>;
  
  fn delegation_token(&self) ->  HdfsResult<String>;
  
  fn renew_delegation_token(&self,  token: &str) ->  HdfsResult<()>;
  
  fn cancel_delegation_token(&self, token: &str) ->  HdfsResult<()>;
}