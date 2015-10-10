use common::HdfsResult;

use super::path::Path;

pub trait FileSystem 
{
  fn mkdir(&self, path: &Path) -> HdfsResult<()>;
}