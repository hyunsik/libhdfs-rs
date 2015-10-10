#[allow(non_camel_case_types)]
pub enum Action 
{
  /// '---'
  NONE,
  /// '--x'
  EXECUTE,
  /// '-w-'
  WRITE,
  /// '-wx'
  WRITE_EXECUTE,
  /// 'r--'
  READ,
  /// 'r-x'
  READ_EXECUTE,
  /// 'rw-'
  READ_WRITE,
  /// 'rwx'
  ALL  
}

pub struct Permission 
{
  user_action : Action,
  group_action: Action,
  other       : Action,
  stick_bit   : bool
}