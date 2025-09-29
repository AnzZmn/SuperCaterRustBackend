use actix::Message;
use serde::{Serialize, Deserialize};

#[derive(Message, Clone, Serialize, Deserialize)]
#[rtype(result="Option<Participant>")]
pub struct Participant{
      pub id: String,
      pub name: String,
      pub contact: String,
}


#[derive(Message)]
#[rtype(result="Option<Vec<Participant>>")]
pub struct GetJobQueue;


#[derive(Message, Clone, Serialize, Deserialize)]
#[rtype(result="Option<Vec<Participant>>")]
pub struct RemoveParticipant{
      id: String
}


