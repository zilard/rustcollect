use tokio::registry::{FromRegistry, Registry};
use actix::*;

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<MyMessage> for MyActor {
    type Result = ();

    fn handle(&mut self, msg: MyMessage, _: &mut Context<Self>) {
        // Handle the message
    }
}

fn main() {
    let system = System::new("my-system");
    let addr = MyActor.start();
    // Register the actor with a registry
    Registry::register(addr, "my-actor");
    let addr = MyActor::from_registry().unwrap();
    // Send a message to the actor
    addr.do_send(MyMessage);
}



