use actix::prelude::*;
use anyhow::Result;
use futures::prelude::*;
use tokio::time::{sleep, Duration};


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct MyOtherActor {}

impl Actor for MyOtherActor {

    type Context = Context<Self>;

    // Called when an actor gets polled the first time.
    fn started(&mut self, ctx: &mut Self::Context) {

        ctx.notify(SmallTask {});

    }

}

// ---------------------------

struct MyActor {}

impl Actor for MyActor {

    type Context = Context<Self>;

    // Called when an actor gets polled the first time.
    fn started(&mut self, ctx: &mut Self::Context) {

        ctx.notify(ComplexTask {});    // 'ComplexTask' FIRST!

        // Sends the message to self.
        ctx.notify(SmallTask {});

    }

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


fn main() -> Result<()> {

    let sys = actix::System::new();


    sys.block_on(async {

        let _my_actor_addr = MyActor {}.start();
        sleep(Duration::from_secs(2)).await;
        let _my_other_actor_addr = MyOtherActor {}.start();

    });

    sys.run()?;

    Ok(())

}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct SmallTask {}

impl Message for SmallTask {
    type Result = ();
}



struct ComplexTask {}

impl Message for ComplexTask {
    type Result = ();
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


impl Handler<SmallTask> for MyActor {

    type Result = ();

    fn handle(&mut self, _msg: SmallTask, ctx: &mut Context<Self>) -> Self::Result {

        let fut = Box::pin(async {
            println!("Easy task done!");
        });

        // convert normal future into an ActorFuture
        let actor_future = fut.into_actor(self);
 
        // spawns a future into the context
        ctx.wait(actor_future);

    }

}


impl Handler<ComplexTask> for MyActor {

    type Result = ();

    fn handle(&mut self, _msg: ComplexTask, ctx: &mut Context<Self>) -> Self::Result {

        let fut = Box::pin({
            sleep(Duration::from_secs(3)).then(|_| async {
                println!("Complex task done!");
            })
        });

        let actor_fut = fut.into_actor(self);

        ctx.wait(actor_fut);

    }

}


impl Handler<SmallTask> for MyOtherActor {

    type Result = ();

    fn handle(&mut self, _msg: SmallTask, ctx: &mut Context<Self>) -> Self::Result {

        let fut = Box::pin(async {
            println!("OTHER - Easy task done!");
        });

        let actor_future = fut.into_actor(self);

        ctx.wait(actor_future);

    }

}


