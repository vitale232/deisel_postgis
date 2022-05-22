use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel_postgis::{establish_connection, models::Location};

use diesel::prelude::*;
use geojson::Feature;

#[get("/points")]
async fn get_points() -> impl Responder {
    use diesel_postgis::schema::locations::dsl::*;

    let results = locations
        .filter(is_active.eq(true))
        .limit(10)
        .load::<Location>(&establish_connection())
        .expect("Errors");
    web::Json::<Vec<Location>>(results)
}

#[get("/geojson")]
async fn get_geojson() -> impl Responder {
    use diesel_postgis::schema::locations::dsl::*;

    let results = locations
        .filter(is_active.eq(true))
        .limit(10)
        .load::<Location>(&establish_connection())
        .expect("Errors");
    let values: Vec<Feature> = results.into_iter().map(|l| l.to_geojson()).collect();
    web::Json::<Vec<Feature>>(values)
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = String::from("localhost");
    let port = 8080;

    let host_url = format!("http://{}:{}", host, port);
    println!("Starting server on host {host_url:?}");

    HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .service(get_points)
            .service(get_geojson)
    })
    .bind((host, port))?
    .run()
    .await
}
