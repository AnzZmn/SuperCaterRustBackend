use actix_web::{get, post, web::{self, Json}, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::{CredenClaims, RequestLogin};


#[post("/init_jwt_gen")]
pub async fn init_jwt_gen(info: web::Json<RequestLogin>) -> impl Responder {
    // Check the Credentials in info
    //if crendentials Match

      // declare the expiration
      let expiration = Utc::now()
            .checked_add_signed(Duration::days(10))
            .unwrap()
            .timestamp();

    
    if info.contact == "admin" && info.o_time_p == 0000{
      let claims = CredenClaims{
            sub: info.contact.clone(),
            exp: expiration as usize
      };
      //Init JWT Token
      let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("super-secret-key".as_ref())).unwrap();

      // Send Response
      HttpResponse::Ok().json(serde_json::json!({"token":token}))
    } else {
      //else
      // Give Unathorized status
        HttpResponse::Unauthorized().body("Invalid Credentials")
    }
    
     
    
}