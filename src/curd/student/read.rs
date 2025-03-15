use actix_web::{get, HttpResponse, Responder};
use rusqlite::Connection;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    id: i32,
    name: String,
    credits: f64
}

#[get("/student/list")]
pub async fn student_list() -> impl Responder {
    let conn = Connection::open("./data/data.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM student").expect("curd/student/read.rs | pub async fn student_list() -> 查询失败！");

    let student_iter = stmt.query_map([], |row| {
        Ok(Student {
            id: row.get(0)?,
            name: row.get(1)?,
            credits: row.get(2)?
        })
    }).expect("curd/student/read.rs | pub async fn student_list() -> 迭代失败！");

    let mut student_vec = Vec::new();
    for student in student_iter {
        student_vec.push(student.unwrap());
    }

    HttpResponse::Ok().json(student_vec)
}