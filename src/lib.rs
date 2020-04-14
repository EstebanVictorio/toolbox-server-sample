#[macro_use]
extern crate actix_web;
use serde::{ Serialize, Deserialize };
use actix_web::{
  App,
  web,
  Result,
  middleware,
  HttpServer,
  HttpRequest,
};
mod toolshop_rack;
pub use toolshop_rack::builder as Toolshop;
pub use toolshop_rack::types as ToolTypes;




#[derive(Serialize)]
struct IndexResponse {
  // message: String,
  tool_name: String,
}

#[derive(Deserialize)]
struct ToolInfo {
    name: String,
}

#[post("/")]
fn index(req: web::Json<ToolInfo>) -> Result<web::Json<IndexResponse>> {
  let toolshop: Toolshop::Toolbox = Toolshop::Toolbox::new();
  let requested_tool: &str = &req.name;
  let tool: Toolshop::Tool =
    toolshop
      .yield_tool(&requested_tool)
      .unwrap_or_else(|| Toolshop::Tool {
        name: String::from("Tool not available"),
        tool_type: ToolTypes::ToolType::NOEXIST,
      });
  

  Ok(web::Json(IndexResponse {
    // message: hello.to_owned(),
    tool_name: tool.name,
  }))
}


impl MessageApp {
  pub fn new(port: u16) -> Self {
    MessageApp { port }
  }

  pub fn run(&self) -> std::io::Result<()> {
    println!("Starting http server: 127.0.0.1:{}", self.port);
    HttpServer::new(move || {
      App::new()
        .wrap(middleware::Logger::default())
        .service(index)
      })
      .bind(("127.0.0.1", self.port))?
      .workers(8)
      .run()
  }
}

pub struct MessageApp {
  port: u16,
}
