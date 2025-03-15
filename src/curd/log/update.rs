use actix_web::{post, web, HttpResponse, Responder};
use rusqlite::Connection;
use serde::{ Deserialize, Serialize };

// 写入日志
#[derive(Debug, Serialize, Deserialize)]
pub struct LogStudent {
    pub id: i32,
    pub name: String,
    pub credits: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    time: String,
    id: i32,
    name: String,
    credits: f64,
    description: String
}

pub fn add_log(time: String, student: LogStudent, description: String) -> rusqlite::Result<()> {
    let conn = Connection::open("./data/log.db").unwrap();
    
    conn.execute(
        "INSERT INTO log (time, id, name, credits, description) VALUES (?1, ?2, ?3, ?4, ?5)", 
        (time, student.id, student.name, student.credits, description)
    )?;

    Ok(())
}

// 添加日志备注
#[derive(Debug, Serialize, Deserialize)]
struct LogDescribe {
    state: i8,
    description: String
}

#[post("/describe/add")]
pub async fn add_describe(info: web::Json<LogDescribe>) -> impl Responder {
    let conn = Connection::open("./data/data.db").unwrap();
    match info.state {
        0 | 1 => {
            conn.execute("INSERT INTO describe (state, description) VALUES (?1, ?2)", (info.state, info.description.clone()))
                .expect("state: 0 | 1 / 保存日志失败！");
            HttpResponse::Ok().body("添加成功！")
        },

        _ => {
            HttpResponse::BadRequest().body("请求参数错误！")
        }
    }
}