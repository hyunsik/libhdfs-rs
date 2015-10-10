use std::collections::HashMap;
use kerberos_name::KerberosName;

pub struct UserInfo 
{
  effective_user: KerberosName,
  real_user: String,
  tokens: HashMap<(String, String), String>
}

impl UserInfo 
{
  pub fn real_user(&self) -> &str {
    &self.real_user
  }
  
  pub fn effective_user(&self) -> &str {
    &self.effective_user.name()
  }
  
  pub fn principal(&self) -> &str {
    &self.effective_user.principal()
  }
}