use actix_web::{web,get,HttpServer,App,Responder};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(||{
        App::new()
    }).bind("127.0.0.1",8000)
}
