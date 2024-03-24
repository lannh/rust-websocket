use std::{sync::{Arc, Mutex}, time::Instant};
use actix::{
    fut,
    prelude::{Actor, Addr, Handler, StreamHandler},
    ActorContext, ActorFuture, AsyncContext, ContextFutureSpawner, WrapFuture
};
use mongodb::{ 
    bson::{ doc,  Bson, Document},//Bson
    Client,
    Collection 
};
use actix_web_actors::ws;
mod dbservice;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use self::dbservice::InterviewItem;



// const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);
// const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
// const FILE_PATH: &'static str = "table.html";

// pub struct FileWatcherWebsocket {
//     hb: Instant,
//     modified: SystemTime,
// }

// impl FileWatcherWebsocket {
//     pub fn new() -> Self {
//         let metadata = fs::metadata(FILE_PATH).unwrap();
//         let modified = metadata.modified().unwrap();

//         Self {
//             hb: Instant::now(),
//             modified,
//         }
//     }

//     fn hb(&self, ctx: &mut <Self as Actor>::Context) {
//         ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
//             if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
//                 println!("Websocket Client heartbeat failed, disconnecting!");

//                 ctx.stop();

//                 return;
//             }

//             let modified = fs::metadata(FILE_PATH).unwrap().modified().unwrap();
//             if modified.duration_since(act.modified).unwrap() > Duration::from_millis(0) {
//                 act.modified = modified;
//                 println!("Sending file changes event! {}", &FILE_PATH);
//                 ctx.text("file_changed")
//             }

//             ctx.ping(b"");
//         });
//     }
// }

// impl Actor for FileWatcherWebsocket {
//     type Context = ws::WebsocketContext<Self>;

//     fn started(&mut self, ctx: &mut Self::Context) {
//         self.hb(ctx);
//     }
// }

/// Define our WebSocket actor
#[derive(Clone)]
pub struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<String> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: String, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ok(ws::Message::Ping(msg)) => {
            //     self.hb = Instant::now();
            //     ctx.pong(&msg);
            // }
            // Ok(ws::Message::Pong(_)) => {
            //     self.hb = Instant::now();
            // }
            Ok(ws::Message::Text(text)) => {
                // ctx.text(text);
                // let ctx_ref: Rc<RefCell<&mut ws::WebsocketContext<MyWs>>> = Rc::new(RefCell::new(ctx));
                println!("received data: {} ", text);

                // let fut = async move {
                //     println!("before process data");

                //     handleTextMsg().await;
                //     // Serialize the vector of words into a JSON string
                //     // return serde_json::to_string(&resJson).unwrap();
                //     // Send the JSON response back to the client

                // };
                // let res = actix_rt::spawn(fut);

                let addr = ctx.address();
                let fut = async move{
                    println!("before process data");

                    let processed_data = handleTextMsg().await;

                    let json_data = json!(processed_data);
                    println!("json data: {} ", json_data.to_string());

                    addr.send(json_data.to_string()).await;
                };
                println!("after process data");

                // match futures::executor::block_on(res) {
                //     Ok(processed_data) => {
                //         println!("received data: {} ", text);

                //         let json_data = json!(processed_data);
                //         println!("json data: {} ", json_data.to_string());

                //         ctx.text(json_data.to_string());
                //     }
                //     Err(e) => {
                //         println!("Error while process data");
                //     }
                // }

                ctx.spawn(fut.into_actor(self));
                // actix::spawn(async move {
                //     handleTextMsg(Rc::clone(ctx_ref)).await;
                // });
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}



async fn handleTextMsg() -> Vec<InterviewItem>{
    let uri = "mongodb+srv://mockinterview:QqJS1wa0ae5kKy4N@cluster0.ssh6qwb.mongodb.net"; 

    let client = Client::with_uri_str(uri).await.unwrap();
    let coll: Collection<InterviewItem> = client.database("mock_interview").collection("interviews");
    let userId = "6591b7281947d121f6180b9c";
    let resJson: Vec<InterviewItem> = dbservice::getInterviewsDB(coll, userId.to_owned()).await;

    println!("done process data ");

    // // Serialize the vector of words into a JSON string
    // let json_response = serde_json::to_string(&resJson).unwrap();
    // // Send the JSON response back to the client
    // let mut ctx = ctx.borrow_mut();

    // ctx.text(json_response); 
    return resJson;
}
