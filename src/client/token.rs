pub struct Token {
  identifier: String,
  password  : String,
  kind      : String,
  service   : String
}

impl Token {
  pub fn identifier(&self) -> &str {
    &self.identifier
  }
  
  pub fn kind(&self) -> &str {
    &self.kind
  }
  
  pub fn password(&self) -> &str {
    &self.password
  }
  
  pub fn service(&self) -> &str {
    &self.service
  }
}