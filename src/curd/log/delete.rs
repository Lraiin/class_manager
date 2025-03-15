use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    description: String
}

#[get("/describe/remove")]
async fn delete_describe(info: web::Query<Info>) -> impl Responder {
    let conn = Connection::open("./data/data.db").unwrap();
    match conn.execute("DELETE FROM describe WHERE description = ?1", [info.description.clone()]) {
        Ok(_) => HttpResponse::Ok().json("删除成功!"), 
        Err(_) => HttpResponse::InternalServerError().json("删除失败！")
    }
}
