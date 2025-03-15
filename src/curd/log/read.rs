use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct DescribeQuery {
    state: i8 
}

#[derive(Debug, Serialize, Deserialize)]
struct Describe {
    state: i8,
    description: String
}

#[get("/describe/list")]
pub async fn describe_list(info: web::Query<DescribeQuery>) -> impl Responder {
    let conn = Connection::open("./data/data.db").unwrap();

    match info.state {
        0 | 1 => {
            let mut stmt = conn.prepare(format!("SELECT * FROM describe WHERE state = {}", info.state).as_str())
                .expect("curd/student/read.rs | pub async fn student_list() -> 查询失败！");

            let describe_iter = stmt.query_map([], |row| {
                Ok(Describe {
                    state: row.get(0)?,
                    description: row.get(1)?
                })
            }).expect("curd/student/read.rs | pub async fn student_list() -> 迭代失败！");

            let mut describe_vec = Vec::new();
            for describe in describe_iter {
                describe_vec.push(describe.unwrap());
            }

            HttpResponse::Ok().json(describe_vec)
        },

        _ => {
            HttpResponse::BadRequest().body("请求参数错误！")
        }
    }
}


// ---------- ---------- ---------- o(￣┰￣*)ゞ ---------- ---------- ----------
#[derive(Debug, Serialize, Deserialize)]
struct Log {
    time: String,
    id: i32,
    name: String,
    credits: f64,
    description: String
}


#[get("/log/list")]
pub async fn log_list() -> impl Responder {
    let conn = Connection::open("./data/log.db").unwrap();
    
    // ORDER BY rowid DESC 倒叙查询
    let mut stmt = conn.prepare("SELECT * FROM log ORDER BY rowid DESC").unwrap();
    let log_iter = stmt.query_map([], |row| {
        Ok(Log {
            time: row.get(0)?,
            id: row.get(1)?,
            name: row.get(2)?,
            credits: row.get(3)?,
            description: row.get(4)?
        })
    }).expect("curd/student/read.rs | pub async fn student_list() -> 迭代失败！");

    let mut log_vec = Vec::new();
    for log in log_iter {
        log_vec.push(log.unwrap());
    }
    HttpResponse::Ok().json(log_vec)
}