use actix_web::{http::header, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Result};
use mysql::*;
use std::{num::NonZeroUsize, sync::Mutex};
use mysql::prelude::Queryable;
use dotenv::dotenv;
use lru::LruCache;

struct AppState {
    pool: Pool,
    lru: lru::LruCache<String, String>,
}


async fn redirect(req: HttpRequest, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse> {
    let slug = req.match_info().get("slug").unwrap_or_default();

    let mut locked_data = data.lock().unwrap();
    let mut conn = locked_data.pool.get_conn().unwrap();
    
    match locked_data.lru.get(slug) {
        Some(x) => Ok(HttpResponse::PermanentRedirect()
                       .append_header((header::LOCATION, header::HeaderValue::from_str(x)?))
                       .finish()),
        None => {
            let query = "SELECT url FROM urls WHERE slug = ?";
            let option_url: Option<String> = conn.exec_map(&query, (&slug,), |url: Row| url.get::<String, usize>(0)).unwrap().last().unwrap_or(&None).clone();

            match option_url {
                Some(url) => {
                    locked_data.lru.push(slug.to_string(), url.clone());
                    Ok(HttpResponse::PermanentRedirect()
                    .append_header((header::LOCATION, header::HeaderValue::from_str(url.as_str())?))
                    .finish())
                },
                None => Ok(HttpResponse::NotFound().finish())
            }
        }

    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = format!("mysql://{}:{}@{}:{}/{}", 
                std::env::var("DATABASE_USER").unwrap_or("root".to_string()),
                std::env::var("DATABASE_PASSWORD").unwrap_or("".to_string()),
                std::env::var("DATABASE_HOST").unwrap_or("localhost".to_string()),
                std::env::var("DATABASE_PORT").unwrap_or("3306".to_string()),
                std::env::var("DATABASE_NAME").unwrap_or("redirector".to_string())
            );
    let pool = Pool::new(
        Opts::from_url(&db_url).unwrap()
    )
    .unwrap();

    let server_url = format!("{}:{}", std::env::var("SERVER_HOST").unwrap_or("localhost".to_string()), std::env::var("SERVER_PORT").unwrap_or("8080".to_string()));

    let data = Data::new(Mutex::new(AppState { pool: pool.clone(), lru: LruCache::new(NonZeroUsize::new(20).unwrap()) }));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/{slug}", web::get().to(redirect))
    })
    .bind(server_url)?
    .run()
    .await
}
