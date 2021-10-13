use axum::{extract::Path, handler::get, http::StatusCode, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;

mod pb;
use pb::*;

// 参数使用serde做Deserialize，axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
  spec: String,
  url: String,
}

#[tokio::main]
async fn main() {
  // 初始化 tracing
  tracing_subscriber::fmt::init();

  // 构建路由
  let app = Router::new()
    // `GET /image` 会执行 generate 函数，并把spec和url传递过去
    .route("/image/:spec/:url", get(generate));
  
  // 运行 web 服务器
  let addr = "127.0.0.1:3000".parse().unwrap();
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn generate(Path(Params {spec, url}): Path<Params>) -> Result<String, StatusCode> {
  let url = percent_decode_str(&url).decode_utf8_lossy();
  let spec: ImageSpec = spec
    .as_str()
    .try_into()
    .map_err(|_| StatusCode::BAD_REQUEST)?;

  Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
