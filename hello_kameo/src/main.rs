use kameo::prelude::*;

struct Greet {
    pub m: String
}

#[derive(Actor)]
struct HelloActor;

impl Message<Greet> for HelloActor {
    type Reply = ();

    async fn handle(&mut self, msg: Greet, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        println!("Message received: {}", msg.m);
    }
}

#[tokio::main]
async fn main() {
    // 1. Spawn the actor: This starts the actor loop in the background
    let actor_ref = HelloActor::spawn(HelloActor);

    // 2. Send the message
    // We use .tell() to create the message envelope, and .send().await to deliver it
    actor_ref.tell(Greet { m: "Hello, world!".to_string() } ).send().await.unwrap();
    
}
