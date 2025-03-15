use actix_web::{ post, web, HttpResponse, Responder };
use rusqlite::{params, Connection};
use serde::{ Deserialize, Serialize };
use chrono::Local;

use crate::curd::log::update::add_log;
use crate::curd::log::update::LogStudent as LogStudent;

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    id: i32,
    name: String,
    credits: f64
}

#[post("/student/add")]
pub async fn add_student(student: web::Json<Student>) -> impl Responder {

    let s = Student {
        id: student.id,
        name: student.name.clone(),
        credits: student.credits
    };

    let conn = Connection::open("./data/data.db").unwrap();
    match conn.execute(
        "INSERT INTO student (id, name, credits) VALUES (?1, ?2, ?3)", 
        (&s.id, &s.name, &s.credits)
    ) {
        Ok(_) => HttpResponse::Created().json("添加成功!"), 
        Err(e) => {
            if e.to_string().contains("UNIQUE constraint failed") {
                HttpResponse::Conflict().json("学号已存在!")
            } else {
                HttpResponse::InternalServerError().json("服务器错误！")
            }
        }
    }
}

// ---------- ---------- ---------- o(￣┰￣*)ゞ ---------- ---------- ---------- 

#[derive(Debug, Serialize, Deserialize)]
struct UpdateCreditsQuery {
    id: i32,
    credits: f64,
    description: String
}

#[post("/student/count")]
pub async fn counting_credits(info: web::Json<UpdateCreditsQuery>) -> impl Responder {
    let conn = Connection::open("./data/data.db").unwrap();
    
    conn.execute(
        "UPDATE student SET credits = credits + ?1 WHERE id = ?2", 
        params![info.credits, info.id]
    ).unwrap();

    let add_log_student = match conn.query_row("SELECT * FROM student WHERE id = ?1", [info.id], |row| {
        Ok(LogStudent {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            credits: row.get(2).unwrap()
        })
    }) {
        Ok(s) => s,
        Err(e) => return HttpResponse::NotFound().body(format!("查无此人, {}", e))
    };

    let now_time = Local::now().format("%Y-%m-%d").to_string();

    match add_log(now_time, add_log_student, info.description.clone()) {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().body(format!("添加日志失败, {}", e))
    }


    HttpResponse::Ok().body("更新成功！")
}

// ---------- ---------- ---------- o(￣┰￣*)ゞ ---------- ---------- ---------- 
#[derive(Debug, Serialize, Deserialize)]
struct StudentQuery {
    id: i32,
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct ListQuery {
    student: Vec<StudentQuery>,
    init_credits: f64
}

// 批量添加学生
#[post("/student/batch_add")]
pub async fn bacth_add_student(list: web::Json<ListQuery>) -> impl Responder {
    let conn = Connection::open("./data/data.db").unwrap();
    let mut stmt = conn.prepare("INSERT INTO student (id, name, credits) VALUES (?1, ?2, ?3)").expect("conn.prepare ERROR");

    for student in list.student.iter() {
        stmt.execute(params![
            student.id,
            student.name,
            list.init_credits
        ]).expect("批量往数据库插入学生信息失败！");
    }

    HttpResponse::Ok().body("批量插入学生信息成功！")
}
