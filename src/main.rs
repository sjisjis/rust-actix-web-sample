extern crate r2d2_redis;
use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_derive::Deserialize;
use std::ops::DerefMut;

#[derive(Deserialize)]
pub struct IdRequest {
    id: String,
    text: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let redis_pool = r2d2::Pool::builder().build(manager).unwrap();
    HttpServer::new(move || App::new()
        .data(redis_pool.clone())
        .service(web::resource("/get").route(web::get().to(get)))
        .service(web::resource("/set").route(web::get().to(set)))
        .service(web::resource("/watcher").route(web::get().to(watcher)))
    ).bind("localhost:8080")?
    .run()
    .await
}

async fn get(
    web::Query(info): web::Query<IdRequest>,
    db: web::Data<r2d2::Pool<RedisConnectionManager>>,
) -> impl Responder {
    let mut conn = db.get().unwrap();
    let rg: String = conn.get(&info.id).expect(&format!("no data id:{}", &info.id));
    let mut txt: String = "not text".to_string();
    if info.text.is_some() {
        txt = info.text.unwrap();
    }
    println!("redis get {}, text:{}", rg, txt);

    HttpResponse::Ok()
        .header("ContentEncoding", "Gzip")
        .header("Content-Type", "text/html")
        .body(format!("<p>{}<br><b>{}</b></p>", rg, txt))
}

async fn set(
    web::Query(info): web::Query<IdRequest>,
    db: web::Data<r2d2::Pool<RedisConnectionManager>>,
) -> impl Responder {
    let mut conn = db.get().unwrap();
    let rs: String = r2d2_redis::redis::cmd("SET").arg(format!("{}", info.id)).arg(format!("set {}", info.id)).query::<String>(conn.deref_mut()).unwrap();
    println!("reds set {}", rs);
    HttpResponse::Ok().content_type("text/plain").body(format!("set {};",info.id))
}

async fn watcher(
    db: web::Data<r2d2::Pool<RedisConnectionManager>>
) -> impl Responder {
    let mut conn = db.get().unwrap();
    let mut ck: String = r2d2_redis::redis::cmd("PING").query::<String>(conn.deref_mut()).unwrap();
    println!("reds check {}", ck);
    if ck == "PONG" {
        ck = "OK".to_string();
    } else {
        ck = "NG".to_string();
    }
    HttpResponse::Ok().body(format!("{}", ck))
}