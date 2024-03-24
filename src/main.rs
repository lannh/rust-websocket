use actix::prelude::*;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use server::MyWs;
mod server;
use actix_cors::Cors;

// /// Implement the handler for WebSocket messages
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         if let Ok(ws::Message::Binary(bin)) = msg {
//             // Handle binary WebSocket message
//         }
//     }
// }
/// WebSocket handshake and start `MyWs` actor
async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(MyWs {}, &r, stream)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/ws", web::get().to(ws_index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}