pub struct KerberosName 
{
  name:  String,
  host:  Option<String>,
  realm: Option<String>,
  principal: String,
}

impl KerberosName 
{
  pub fn name(&self) -> &str {
    &self.name
  }
  
  pub fn host(&self) -> Option<&str> {
    self.host.as_ref().map(String::as_ref)
  }
  
  pub fn realm(&self) -> Option<&str> {
    self.realm.as_ref().map(String::as_ref)
  }
  
  pub fn principal(&self) -> &str {
    &self.principal
//    let mut strbuf = String::new();
//    
//    strbuf.push_str(&self.name);
//    
//    if let Some(ref host) = self.host {
//      strbuf.push_str("/");
//      strbuf.push_str(host);
//    }
//    
//    if let Some(ref realm) = self.realm {
//      strbuf.push_str("@");
//      strbuf.push_str(realm);
//    }
//    
//    strbuf
  }
}