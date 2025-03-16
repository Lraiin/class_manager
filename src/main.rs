use std::env;
use std::process::Command;

use actix_files::Files;
use actix_web::{ dev::Service, App, HttpResponse, HttpServer };

mod curd;
use curd::{init, student, log};
use login::login_handler;

mod login;
mod deepseek;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match env::consts::OS {
        "windows" => {
            let _ = Command::new("cmd.exe").arg("/C").arg("start").arg("http://localhost:3737").spawn();
        },
        "linux" => {
            println!("Linux 版本不支持自动打开浏览器，请手动打开浏览器访问 http://localhost:3737, 服务器外网访问 请把 0.0.0.0 手动替换 IP 或域名。")
        },
        _ => {
            println!("MAC OS?");
        }
    }

    let logo = 
    r#"
                       __           _ _       
                      / / _ __ __ _(_|_)_ __  
                     / / | '__/ _` | | | '_ \ 
                    / /__| | | (_| | | | | | |
                    \____/_|  \__,_|_|_|_| |_|

        System: student management system  
        Version: 0.1.0
        Author: @Lraiin -> Lraiin@outlook.com
        Git Repository: https://atomgit.com/shazengchao/class_manager

        URL: 0.0.0.0:3737
    "#;
    
    println!("{}", logo);

    if std::path::Path::new("./data/").exists() {
        println!("[✅] 程序正常运行！服务运行中...");
    } else {
        println!("[❓] 检测为第一次运行服务，正在初始化服务...");
        match init::init_sql() {
            Ok(()) => println!("[✅] 初始化成功，正在运行服务..."),
            Err(e) => println!("[❌] 初始化失败: {}", e),
        }
    }

    HttpServer::new(|| {
        App::new()
            // 全局中间件
            .wrap_fn(|req, srv| {
                let path = req.path();
                let public_paths = [
                    "/login",
                    "/login.html",
                    "/assets/",
                    "/lib/",
                    "/js/",
                    "/favicon.ico"
                ];
            
                // 白名单路径直接放行
                if public_paths.iter().any(|p| path.starts_with(p)) {
                    return srv.call(req);
                }
            
                // 检查登录cookie
                let logged_in = req.cookies()
                    .map(|c| c.iter().any(|cookie| 
                        cookie.name() == "logged_in" && 
                        cookie.value() == "true"
                    ))
                    .unwrap_or(false);
            
                if !logged_in {
                    // 构造重定向响应
                    let redirect = HttpResponse::Found()
                        .append_header(("Location", "/login.html"))
                        .finish();
                    let response = req.into_response(redirect);
                    return Box::pin(async { Ok(response) });
                }
            
                // 已登录用户继续处理请求
                srv.call(req)
            })

            .service(login_handler)

            .service(student::read::student_list)
            .service(student::update::add_student)
            .service(student::delete::delete_student)
            .service(student::update::counting_credits)
            .service(student::update::bacth_add_student)
            
            .service(log::read::describe_list)
            .service(log::read::log_list)
            .service(log::update::add_describe)
            .service(log::delete::delete_describe)

            .service(deepseek::ai_analysis)

            .service(Files::new("/", "./web").index_file("index.html"))

    })
    .bind(("0.0.0.0", 3737))?
    .run()
    .await
}