mod from;
mod result;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::io::Result;

async fn tesk(info: web::Json<from::create>) -> impl Responder {
    println!("{:?}", info);
    let mut rek = HashMap::new();
    for i in 0..100000 {
        rek.insert(i, i);
    }

    HttpResponse::Ok().json(result::Map { map: rek })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().route("/tesk", web::post().to(tesk)))
        .bind("127.0.0.1:8080")?
        .run()
        .await;
    Ok(())
}
