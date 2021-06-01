use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use log::{error, info};
use sqlx::mysql::MySqlPool;
use structopt::StructOpt;

mod model;
pub use model::{UpdateRequest, User, UserRequest};
mod error;
pub use error::Error;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(long)]
    workers: usize,
    #[structopt(long)]
    web_port: u16,
    #[structopt(long)]
    mysql: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        GET /userss -> list of all todos
        POST /user -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
        GET /user/{id} -> show one todo with requested id
        PUT /update/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /delete/{id} -> delete todo with requested id
    "#
    )
}

async fn user(id: web::Path<i32>, db_pool: web::Data<MySqlPool>) -> Result<HttpResponse, Error> {
    let result = User::find_by_id(*id, db_pool.get_ref()).await;
    info!("{:?}", result);
    Ok::<_, Error>(HttpResponse::Ok().body(format!("{:?}", result)))
}

async fn users(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = User::find_all(db_pool.get_ref()).await;
    info!("{:?}", result);
    Ok::<_, Error>(HttpResponse::Ok().body(format!("{:?}", result)))
}

async fn create(
    user: web::Json<UserRequest>,
    db_pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, Error> {
    let result = User::create(user.into_inner(), db_pool.get_ref()).await;
    info!("{:?}", result);
    match result {
        Ok(user) => Ok::<_, Error>(
            HttpResponse::Ok()
                .content_type("application/javascript")
                .json(user)
        ),
        Err(err) => Ok::<_, Error>(HttpResponse::BadRequest().body(format!("{}", err))),
    }
}

async fn update(
    id: web::Path<u32>,
    user: web::Json<UpdateRequest>,
    db_pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, Error> {
    let result = User::update(*id, user.into_inner(), db_pool.get_ref()).await;
    info!("{:?}", result);
    match result {
        Ok(user) => Ok::<_, Error>(HttpResponse::Ok().json(user)),
        Err(err) => Ok::<_, Error>(HttpResponse::BadRequest().body(format!("err:{}", err))),
    }
}

async fn delete_put(
    id: web::Path<u32>,
    db_pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, Error> {
    let result = User::delete_put(*id, db_pool.get_ref()).await;
    info!("{:?}", result);
    match result {
        Ok(user) => Ok::<_, Error>(HttpResponse::Ok().json(user)),
        Err(err) => Ok::<_, Error>(HttpResponse::BadRequest().body(format!("err:{}", err))),
    }
}

async fn delete(id: web::Path<u32>, db_pool: web::Data<MySqlPool>) -> Result<HttpResponse, Error> {
    let result = User::delete(*id, db_pool.get_ref()).await;
    println!("{:?}", result);
    match result {
        Ok(rows) => {
            if rows > 0 {
                Ok::<_, Error>(HttpResponse::Ok()
                    .body(format!("Successfully deleted id:{} record(s)", rows)))
            } else {
                Ok::<_, Error>(HttpResponse::BadRequest().body("User not found"))
            }
        }
        _ => Ok::<_, Error>(HttpResponse::BadRequest().body("User not found")),
    }
}

#[actix_web::main]
async fn main() -> Result<(), crate::Error> {
    env_logger::init();
    let config = Config::from_args();

    let pool = MySqlPool::connect(&format!("mysql://{}", config.mysql)).await?;
    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/user/{id}").route(web::get().to(user)))
            .service(web::resource("/users").route(web::get().to(users)))
            .service(web::resource("/create").route(web::post().to(create)))
            .service(web::resource("/update/{id}").route(web::put().to(update)))
            .service(web::resource("/delete/{id}").route(web::put().to(delete_put)))
            .service(web::resource("/delete/physics/{id}").route(web::delete().to(delete)))
            .route("/alive", {
                web::get().to(|pool: web::Data<MySqlPool>| async move {
                    let row: (i16,) = sqlx::query_as("SELECT ?;")
                        .bind(150_i16)
                        .fetch_one(&**pool)
                        .await?;

                    if row.0 != 150_i16 {
                        error!("{}", format!("mysql://{:#?}", row.0));
                        return Ok::<_, Error>(
                            HttpResponse::RequestTimeout().body("NG"),
                        );
                    }

                    Ok::<_, Error>(HttpResponse::Ok().body("OK"))
                })
            })
    })
    .bind(("0.0.0.0", config.web_port))?
    .workers(config.workers)
    .run()
    .await
    .map_err(crate::Error::from)?;

    Ok(())
}
