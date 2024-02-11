use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use mysql::Pool;
use std::convert::Infallible;
use mysql::prelude::Queryable;
use dotenv::dotenv;

async fn handle_request(req: Request<Body>) -> std::result::Result<Response<Body>, Infallible> {
    let uri = req.uri();
    let path = uri.path();
    
    if let Some(slug) = path.strip_prefix("/") {
        let url = match get_url_from_database(slug).await {
            Ok(url) => url,
            Err(_) => return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()),
        };
        
        return Ok(Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", url)
            .body(Body::empty())
            .unwrap());
    }

    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::empty())
        .unwrap())
}

async fn get_url_from_database(slug: &str) -> Result<String, Box<dyn std::error::Error>> {
    let db_url = format!("mysql://{}:{}@{}:{}/{}", 
                std::env::var("DATABASE_USER").unwrap_or("root".to_string()),
                std::env::var("DATABASE_PASSWORD").unwrap_or("".to_string()),
                std::env::var("DATABASE_HOST").unwrap_or("localhost".to_string()),
                std::env::var("DATABASE_PORT").unwrap_or("3306".to_string()),
                std::env::var("DATABASE_NAME").unwrap_or("redirector".to_string())
            );
    let pool = Pool::new(db_url)?;
    let mut conn = pool.get_conn()?;
    let query = format!("SELECT url FROM urls WHERE slug = '{}'", slug);
    let url: Option<String> = conn.query_first(query)?;
    match url {
        Some(url) => Ok(url),
        None => Err("Slug not found".into())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let addr = format!("{}:{}", 
                            std::env::var("SERVER_HOST").unwrap_or("localhost".to_string()), 
                            std::env::var("SERVER_PORT").unwrap_or("8080".to_string())
                        ).parse().unwrap();
    // ([127, 0, 0, 1], 8080).into();
    
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
