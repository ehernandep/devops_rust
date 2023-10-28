mod routes;
mod models;
use actix_web::test;
use actix_http::StatusCode;
use actix_cors::Cors;
use actix_web::{HttpServer, middleware::Logger, App, http::header};
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};
use dotenv::dotenv;
use routes::{health_route::health_checker_handler, config::config};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the db is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };

    println!("Server started successfully 🚀!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(actix_web::web::Data::new(AppState {db: pool.clone()}))
            .service(health_checker_handler)
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use std::borrow::Cow;
    use dotenv::dotenv;
    use std::env;
    
    #[actix_rt::test]
    async fn test_health_checker_handler() {
        // Create a test server instance
        let mut app = test::init_service(App::new().service(health_checker_handler)).await;

        // Perform a GET request to the health check route
        let req = test::TestRequest::get().uri("/api/healthchecker").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Assert the response
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_database_connection() {
        dotenv().ok();
        // Create a test database and set its URL
        let test_database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // Attempt to connect to the test database
        let pool = PgPoolOptions::new()
            .max_connections(1) // Adjust the number of connections as needed
            .connect(&test_database_url)
            .await;

        // Assert that the database connection is successful
        assert!(pool.is_ok());
    }

}