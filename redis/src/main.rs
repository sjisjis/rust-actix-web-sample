use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, Result};
use log::error;
use r2d2_redis::{r2d2, redis::Commands, RedisConnectionManager};
use serde_derive::Deserialize;
use std::{ops::DerefMut, time::Duration};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(long)]
    workers: usize,
    #[structopt(long)]
    web_port: u16,
    #[structopt(long)]
    redis: String,
}

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Pool(r2d2::Error),
    Redis(r2d2_redis::redis::RedisError),
    BlockingCanceled,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Io: {}", e),
            Self::Pool(e) => write!(f, "Pool: {}", e),
            Self::Redis(e) => write!(f, "Rdis: {}", e),
            Self::BlockingCanceled => write!(f, "BlockingCanceled"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Self::Pool(e)
    }
}

impl From<r2d2_redis::redis::RedisError> for Error {
    fn from(e: r2d2_redis::redis::RedisError) -> Self {
        Self::Redis(e)
    }
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        error!("{}", self);
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<actix_web::error::BlockingError<Error>> for Error {
    fn from(e: actix_web::error::BlockingError<Error>) -> Error {
        match e {
            actix_web::error::BlockingError::Error(e) => e,
            actix_web::error::BlockingError::Canceled => Error::BlockingCanceled,
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), crate::Error> {

    let config = Config::from_args();
    
    let redis = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .test_on_check_out(false)
        .build(RedisConnectionManager::new(format!("redis://{}", config.redis))?)?;

    HttpServer::new(move || {
        App::new()
            .data(redis.clone())
            .service(web::resource("/get").route(web::get().to(get)))
            .service(web::resource("/set").route(web::get().to(set)))
            .service(web::resource("/watcher").route(web::get().to(watcher)))
    })
    .bind(("0.0.0.0", config.web_port))?
    .workers(config.workers)
    .run()
    .await
    .map_err(crate::Error::from)?;

    Ok(())
}

#[derive(Deserialize)]
pub struct IdRequest {
    id: String,
    text: Option<String>,
}

async fn get(info: web::Query<IdRequest>, db: web::Data<r2d2::Pool<RedisConnectionManager>>) -> Result<HttpResponse, Error> {
    let id =  format!("{}",info.id);
    let txt = info.text.as_ref().map(|t| format!("{}", t)).unwrap_or_else(|| "null".into());
    web::block(move || db.get()?.get::<_, Option<String>>(&info.id).map_err(Error::from))
        .await
        .map(|v| {
            if let Some(v) = v {
                HttpResponse::Ok()
                    .header("ContentEncoding", "Gzip")
                    .content_type("text/html; charset=utf-8")
                    .body(format!("<p>{}<br><b>{}</b></p>", v, txt))
            } else {
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(format!("<p>not id:{}</p>", id))
            }
        })
        .map_err(Error::from)
}

async fn set(info: web::Query<IdRequest>, db: web::Data<r2d2::Pool<RedisConnectionManager>>) -> Result<HttpResponse, Error> {
    web::block(move || {
        r2d2_redis::redis::cmd("SET")
            .arg(format!("{}", info.id))
            .arg(format!("set {}", info.id))
            .query::<String>(db.get()?.deref_mut())
            .map(|_| "OK")
            .map_err(Error::from)
    })
    .await
    .map(|v| HttpResponse::Ok().body(v))
    .map_err(Error::from)
}

async fn watcher(pool: web::Data<r2d2::Pool<RedisConnectionManager>>) -> Result<HttpResponse, Error> {
    web::block(move || {
        r2d2_redis::redis::cmd("PING")
            .query::<String>(pool.get()?.deref_mut())
            .map(|_| "OK")
            .map_err(Error::from)
    })
    .await
    .map(|v| HttpResponse::Ok().body(v))
    .map_err(Error::from)
}
