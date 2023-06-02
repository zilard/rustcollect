use actix::prelude::*;
use anyhow::Result;
//use futures::prelude::*;
//use tokio::time::{sleep, Duration};



struct MyActor {}


impl Actor for MyActor {

    type Context = Context<Self>;

    // Called when an actor gets polled the first time.
    fn started(&mut self, ctx: &mut Self::Context) {

        // Sends the message to self.
        ctx.notify(SmallTask {});

    }

}




fn main() -> Result<()> {

    let sys = actix::System::new();

    sys.block_on(async {
        let _addr = MyActor {}.start();
    });

    sys.run()?;

    Ok(())

}


struct SmallTask {}


impl Message for SmallTask {
    type Result = ();
}




impl Handler<SmallTask> for MyActor {

    type Result = ();

    fn handle(&mut self, _msg: SmallTask, ctx: &mut Context<Self>) -> Self::Result {

        let fut = Box::pin(async {
            println!("Easy task done!");
        });

        // convert normal future into an ActorFuture
        let actor_future = fut.into_actor(self);
 
        // spawns a future into the context
        ctx.spawn(actor_future);

    }

}



