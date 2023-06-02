use actix::{Actor, Context, System};

struct MyActor;

impl Actor for MyActor {

    type Context  = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("I am alive!");
        System::current().stop();
    }

}


fn main() {

    let system = System::new();

    let _addr = system.block_on(async { MyActor.start() });

    system.run().unwrap();

}


