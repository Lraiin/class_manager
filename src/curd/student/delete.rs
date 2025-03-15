use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    id: i32
}

#[get("/student/delete")]
pub async fn delete_student(info: web::Query<Info>) -> impl Responder {
    let conn = Connection::open("./data/data.db").unwrap();

    match conn.execute("DELETE FROM student WHERE id = ?1", [info.id]) {
        Ok(_) => HttpResponse::Ok().json("删除成功!"), 
        Err(_) => HttpResponse::InternalServerError().json("删除学生时,发生了错误！")
    }
    
}