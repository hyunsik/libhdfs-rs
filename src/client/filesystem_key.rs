use super::UserInfo;

pub struct FileSystemKey
{
  authority: String,
  host     : String,
  port     : String,
  scheme   : String,
  user     : UserInfo
}