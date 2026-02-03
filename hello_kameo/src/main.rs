use kameo::prelude::*;

struct Greet {
    pub m: String
}

#[derive(Actor)]
struct HelloActor;

impl Message<Greet> for HelloActor {
    type Reply = ();

    async fn handle(&mut self, msg: Greet, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        println!("Actor received: {}", msg.m);
    }
}

#[tokio::main]
async fn main() {
    let actor_ref = HelloActor::spawn(HelloActor);

    actor_ref.tell(Greet { m: "Hello, world!".to_string() } ).send().await.unwrap();
    
}
