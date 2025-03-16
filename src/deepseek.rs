use actix_web:: { post, web, HttpResponse, Responder };
use deepseek_api_client::*;
use serde::{ Deserialize, Serialize };

use crate::curd::log::read::period_log;

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    deepseek_key: String,
    period: String,
}

#[post("/ai/analysis")]
pub async fn ai_analysis(info: web::Json<Info>) -> impl Responder {
    let period_sql = match info.period.as_str() {
        "一天内" => "time = DATE('now')".to_string(),
        "七天内" => "time BETWEEN DATE('now', '-6 days') AND DATE('now')".to_string(),
        "一月内" => "time BETWEEN DATE('now', '-29 days') AND DATE('now')".to_string(),
        _ => return HttpResponse::BadRequest().body("period 参数错误"),
    };

    let log_result = period_log(&period_sql);
    if let Err(e) = log_result {
        return HttpResponse::InternalServerError().body(format!("数据库中没有此时间段的记录: {}", e));
    }

    const SYSTEM_SETTING: &str = r#"
        你只是我的一个数据分析助手，帮我分析同学们这段时间在校的学习情况，
        请始终使用纯文本回答，不要使用任何 Markdown 格式（如 #、*、``` 等符号）或特殊转义符。
        
        要求：
        1. 分析详细程度：需包含最近表现趋势（扣分/加分比例）、改进建议
        2. 明确列出加分前五名和扣分最多的五名同学
        3. 语言风格：简明扼要，段落间用换行分隔
        
        注意：你只能回答与计分相关的问题，其他问题一律回复：
        "对不起，我只是计分数据分析助手，我不能回复你其它问题。"
    "#;

    let mut llm_completion = chat_completion(&info.deepseek_key);

    let msg = vec![
        Message {
            role: "system".to_owned(),
            content: SYSTEM_SETTING.to_owned(),
        },
        Message {
            role: "user".to_owned(),
            content: log_result.unwrap(),
        },
    ];

    let res = llm_completion(msg);
    let res_text = get_response_text(&res.await.unwrap(), 0);
    let replaced_text = res_text
        .map(|s| s.replace("\n", "<br>"))
        .unwrap_or_else(|| String::from(""));
    HttpResponse::Ok().json(replaced_text)
}

