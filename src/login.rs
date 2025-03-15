use actix_web::{cookie::Cookie, post, web, HttpResponse, Responder};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    user_name: String,
    password: String
}

#[post("/login")]
pub async fn login_handler(form: web::Json<LoginRequest>) -> impl Responder {
    let conn = Connection::open("./data/data.db").unwrap();

    let stored_password: Result<String, _> = conn.query_row(
        "SELECT password FROM user WHERE user_name = ?",
        params![form.user_name],
        |row| row.get::<_, String>(0),
    );

    match stored_password {
        Ok(password_from_db) if password_from_db == form.password => {
            // 登录成功设置 Cookie
            HttpResponse::Ok()
                .cookie(
                    Cookie::build("logged_in", "true")
                        .path("/")
                        .finish()
                )
                .body("登录成功!")
        }
        _ => HttpResponse::Unauthorized().body("用户名或密码错误"),
    }
}