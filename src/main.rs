use actix_web::{http::header, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Result};
use mysql::*;
use std::sync::Mutex;
use mysql::prelude::Queryable;

struct AppState {
    pool: Pool,
}

async fn redirect(req: HttpRequest, data: web::Data<Mutex<AppState>>) -> Result<HttpResponse> {
    let slug = req.match_info().get("slug").unwrap_or_default();

    let mut conn = data.lock().unwrap().pool.get_conn().unwrap();

    let query = "SELECT url FROM urls WHERE slug = ?";
    let url: Option<String> = conn.exec_map(&query, (&slug,), |url: Row| url.get::<String, usize>(0)).unwrap().last().unwrap_or(&None).clone();
    if let Some(url) = url {
        Ok(HttpResponse::PermanentRedirect()
            .append_header((header::LOCATION, header::HeaderValue::from_str(url.as_str())?))
            .finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = Pool::new(
        Opts::from_url("mysql://root:0000@localhost:3306/redirector").unwrap(),
    )
    .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Mutex::new(AppState { pool: pool.clone() })))
            .route("/{slug}", web::get().to(redirect))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
