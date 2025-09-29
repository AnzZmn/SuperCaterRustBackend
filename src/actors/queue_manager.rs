use actix::{Actor, Context, Handler, MessageResult};
use crate::actors::messages::{GetJobQueue, Participant};
pub struct JobQueue{
      queue_data: Vec<Participant>,
}

impl JobQueue {
      pub fn new() -> Self{
          JobQueue { queue_data: Vec::new() }
      }
}

impl Actor for JobQueue  {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("New Job Queue Initialized")
    }

}

impl Handler<Participant> for JobQueue {
    type Result = MessageResult<Participant>;
    fn handle(&mut self, msg: Participant, _ctx: &mut Self::Context) -> Self::Result {
        println!("Recieved a new Participant with name :{} and contact: {}", msg.name, msg.contact);
        let new_participant = Participant{
            id: msg.id,
            name: msg.name,
            contact: msg.contact,
        };
        self.queue_data.push(new_participant.clone());

        MessageResult(Some(new_participant))
    }
}

impl Handler<GetJobQueue> for JobQueue {
    type Result = MessageResult<GetJobQueue>;
    fn handle(&mut self, _msg: GetJobQueue, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(Some(self.queue_data.clone()))
    }
}
