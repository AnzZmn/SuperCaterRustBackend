use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct RequestLogin{
      pub contact: String,
      pub o_time_p: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CredenClaims{
      pub sub: String,
      pub exp: usize,
}



