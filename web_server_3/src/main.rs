use std::sync::Mutex;
use actix_web::{get, post, web, HttpServer, Responder, App};
use actix_files::NamedFile;
use serde_json::json;

struct AppState {
  pub click_count: Mutex<u32>
}

#[get("/")]
async fn index() -> impl Responder {
  NamedFile::open_async("F:\\thebluetropics\\Projects\\rust_practice\\web_server_3\\src\\index.html").await
}

#[get("/api/current-click-count")]
async fn current_click_count(data: web::Data<AppState>) -> String {
  let response_json = json!({
    "current_click_count": data.click_count
  });
  serde_json::to_string(&response_json).unwrap()
}

#[post("/api/click")]
async fn click(data: web::Data<AppState>) -> String {
  let mut click_count = data.click_count.lock().unwrap();
  *click_count += 1;
  let response_json = json!({
    "current_click_count": *click_count
  });
  serde_json::to_string(&response_json).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .app_data(web::Data::new(AppState { click_count: Mutex::new(0) }))
      .service(index)
      .service(current_click_count)
      .service(click)
  })
  .bind(("127.0.0.1", 3000))
  .unwrap()
  .run()
  .await
}
