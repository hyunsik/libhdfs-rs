use permission::Permission;

pub struct FileStatus 
{
  is_dir      : bool,
  atime       : i64,
  blocksize   : i64,
  length      : i64,
  mtime       : i64,
  permmission : Permission,
  replications: i16,
  owner       : String,
  group       : String,
  path        : String,
  symlink     : String
}