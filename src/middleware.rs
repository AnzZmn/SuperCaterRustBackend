use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::{ bearer::BearerAuth, AuthenticationError};
use actix_web_httpauth::extractors::bearer::Config;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims{
      pub sub: String,
      pub exp: usize,
}

pub async fn validator(req: ServiceRequest, creds: BearerAuth) -> Result<ServiceRequest,(Error, ServiceRequest)>{
    let token = creds.token();
    let decoding_key = DecodingKey::from_secret("super-secret-key".as_ref());
    let validation = Validation::default();
    let result = decode::<Claims>(token, &decoding_key, &validation);
    
    match result{
        Ok(data) => {
            req.extensions_mut().insert(data.claims);
            Ok(req)
        }
        Err(_) =>{
            let config= Config::default()
                .realm("Access Unauthorized")
                .scope("Contact");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}