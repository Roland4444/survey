// use actix_web::{web, App, HttpResponse, HttpServer, Result};
// use serde::Deserialize;
// use std::sync::Mutex;

// #[derive(Debug, Deserialize, Clone)]
// struct SurveyResponse {
//     name: Option<String>,
//     linux_experience: String,
//     attitude: String,
//     comments: Option<String>,
//     daily_tasks: Vec<String>,
//     testing_willingness: bool,
//     contact_preference: Option<String>,
// }

// struct AppState {
//     responses: Mutex<Vec<SurveyResponse>>,
// }

// impl AppState {
//     fn new() -> Self {
//         Self {
//             responses: Mutex::new(Vec::new()),
//         }
//     }
// }

// // Простая HTML форма без JavaScript
// const INDEX_HTML: &str = r#"
// <!DOCTYPE html>
// <html lang="ru">
// <head>
//     <meta charset="UTF-8">
//     <meta name="viewport" content="width=device-width, initial-scale=1.0">
//     <title>Опрос: Ваше мнение о ПО</title>
//     <style>
//         body { 
//             font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
//             max-width: 600px; 
//             margin: 40px auto; 
//             padding: 20px; 
//             background: #f5f5f5; 
//         }
//         .container { 
//             background: white; 
//             padding: 30px; 
//             border-radius: 10px; 
//             box-shadow: 0 2px 10px rgba(0,0,0,0.1); 
//         }
//         h1 { color: #2c3e50; margin-bottom: 10px; }
//         .description { color: #7f8c8d; margin-bottom: 25px; line-height: 1.5; }
//         .form-group { margin-bottom: 20px; }
//         label { display: block; margin-bottom: 8px; font-weight: 600; color: #34495e; }
//         input[type="text"], textarea, select { 
//             width: 100%; 
//             padding: 10px; 
//             border: 1px solid #bdc3c7; 
//             border-radius: 5px; 
//             font-size: 14px; 
//             box-sizing: border-box;
//         }
//         textarea { height: 80px; resize: vertical; }
//         .checkbox-group { margin: 10px 0; }
//         .checkbox-group label { font-weight: normal; margin-left: 5px; }
//         .optional { color: #7f8c8d; font-style: italic; }
//         .btn { 
//             background: #3498db; 
//             color: white; 
//             padding: 12px 30px; 
//             border: none; 
//             border-radius: 5px; 
//             cursor: pointer; 
//             font-size: 16px; 
//             margin-top: 10px;
//         }
//         .btn:hover { background: #2980b9; }
//         .success { 
//             background: #2ecc71; 
//             color: white; 
//             padding: 20px; 
//             border-radius: 5px; 
//             text-align: center; 
//             margin: 20px 0; 
//         }
//         .error { 
//             background: #e74c3c; 
//             color: white; 
//             padding: 10px; 
//             border-radius: 5px; 
//             margin: 10px 0; 
//         }
//     </style>
// </head>
// <body>
//     <div class="container">
//         <h1>📊 Опрос: Ваше мнение о программном обеспечении</h1>
        
//         <div class="description">
//             <p>Изучаем возможности оптимизации рабочих процессов через использование альтернативного ПО, включая Linux и open-source решения.</p>
//             <p><strong>Ваше мнение важно</strong> для принятия взвешенных решений. Опрос анонимен — указывать имя необязательно.</p>
//         </div>

//         <!-- Сообщение об успехе, если есть -->
//         {{SUCCESS_MESSAGE}}

//         <form method="post" action="/submit">
//             <div class="form-group">
//                 <label>Ваше имя <span class="optional">(необязательно)</span></label>
//                 <input type="text" name="name" placeholder="Можно оставить пустым">
//             </div>

//             <div class="form-group">
//                 <label>Опыт работы с Linux <span style="color: red;">*</span></label>
//                 <select name="linux_experience" required>
//                     <option value="">-- Выберите вариант --</option>
//                     <option value="none">Нет опыта</option>
//                     <option value="basic">Базовый (пользовался немного)</option>
//                     <option value="advanced">Продвинутый (регулярно использую)</option>
//                     <option value="expert">Эксперт (администрирую системы)</option>
//                 </select>
//             </div>

//             <div class="form-group">
//                 <label>Ваше отношение к возможному переходу на Linux <span style="color: red;">*</span></label>
//                 <select name="attitude" required>
//                     <option value="">-- Выберите вариант --</option>
//                     <option value="positive">Положительное, готов участвовать</option>
//                     <option value="neutral">Нейтральное, мне все равно</option>
//                     <option value="concerned">Есть опасения по поводу сложностей</option>
//                     <option value="negative">Отрицательное, не поддерживаю идею</option>
//                 </select>
//             </div>

//             <div class="form-group">
//                 <label>Какие задачи выполняете на компьютере? <span class="optional">(можно выбрать несколько)</span></label>
//                 <div class="checkbox-group">
//                     <input type="checkbox" name="daily_tasks" value="documents" id="task1">
//                     <label for="task1">Работа с документами (Word, Excel)</label>
//                 </div>
//                 <div class="checkbox-group">
//                     <input type="checkbox" name="daily_tasks" value="email" id="task2">
//                     <label for="task2">Электронная почта</label>
//                 </div>
//                 <div class="checkbox-group">
//                     <input type="checkbox" name="daily_tasks" value="1c" id="task3">
//                     <label for="task3">Работа с 1С</label>
//                 </div>
//                 <div class="checkbox-group">
//                     <input type="checkbox" name="daily_tasks" value="browser" id="task4">
//                     <label for="task4">Веб-браузер</label>
//                 </div>
//                 <div class="checkbox-group">
//                     <input type="checkbox" name="daily_tasks" value="specialized" id="task5">
//                     <label for="task5">Специализированный софт</label>
//                 </div>
//             </div>

//             <div class="form-group">
//                 <label>Готовы ли вы участвовать в тестировании?</label>
//                 <div class="checkbox-group">
//                     <input type="checkbox" name="testing_willingness" value="true" id="testing">
//                     <label for="testing">Да, готов участвовать в пилотном тестировании</label>
//                 </div>
//             </div>

//             <div class="form-group">
//                 <label>Предпочтительный способ обратной связи <span class="optional">(необязательно)</span></label>
//                 <input type="text" name="contact_preference" placeholder="Email, telegram, личная беседа...">
//             </div>

//             <div class="form-group">
//                 <label>Ваши комментарии и предложения</label>
//                 <textarea name="comments" placeholder="Что вам нравится/не нравится в текущем ПО? Какие сложности видите?"></textarea>
//             </div>

//             <button type="submit" class="btn">Отправить ответ</button>
//         </form>

//         <div style="margin-top: 30px; text-align: center; color: #7f8c8d; font-size: 14px;">
//             <p>Спасибо за ваше время! 💙</p>
//         </div>
//     </div>
// </body>
// </html>
// "#;

// const SUCCESS_HTML: &str = r#"
// <!DOCTYPE html>
// <html lang="ru">
// <head>
//     <meta charset="UTF-8">
//     <meta name="viewport" content="width=device-width, initial-scale=1.0">
//     <title>Спасибо за участие!</title>
//     <style>
//         body { 
//             font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
//             max-width: 600px; 
//             margin: 40px auto; 
//             padding: 20px; 
//             background: #f5f5f5; 
//             text-align: center;
//         }
//         .container { 
//             background: white; 
//             padding: 40px; 
//             border-radius: 10px; 
//             box-shadow: 0 2px 10px rgba(0,0,0,0.1); 
//         }
//         .success { 
//             background: #2ecc71; 
//             color: white; 
//             padding: 30px; 
//             border-radius: 5px; 
//             margin: 20px 0; 
//             font-size: 18px;
//         }
//         .btn { 
//             background: #3498db; 
//             color: white; 
//             padding: 12px 30px; 
//             border: none; 
//             border-radius: 5px; 
//             cursor: pointer; 
//             font-size: 16px; 
//             margin-top: 20px;
//             text-decoration: none;
//             display: inline-block;
//         }
//     </style>
// </head>
// <body>
//     <div class="container">
//         <div class="success">
//             <h2>✅ Спасибо за ваш ответ!</h2>
//             <p>Ваше мнение очень важно для нас и поможет сделать нашу работу лучше.</p>
//         </div>
        
//         <a href="/" class="btn">Вернуться к форме</a>
//     </div>
// </body>
// </html>
// "#;

// // Обработчик главной страницы
// async fn index() -> HttpResponse {
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(INDEX_HTML)
// }

// // Обработчик отправки формы
// async fn submit_response(
//     form: web::Form<SurveyResponse>,
//     data: web::Data<AppState>,
// ) -> HttpResponse {
//     let response = form.into_inner();
    
//     // Логируем ответ
//     log::info!("Получен новый ответ: опыт - {}, отношение - {}, тестирование - {}",
//         response.linux_experience,
//         response.attitude,
//         response.testing_willingness
//     );

//     // Сохраняем ответ
//     {
//         let mut responses = data.responses.lock().unwrap();
//         responses.push(response);
//     }

//     // Возвращаем страницу благодарности
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(SUCCESS_HTML)
// }

// // Страница с результатами
// async fn results(data: web::Data<AppState>) -> Result<HttpResponse> {
//     let responses = data.responses.lock().unwrap();
    
//     let mut html = String::from(r#"
//     <!DOCTYPE html>
//     <html>
//     <head>
//         <title>Результаты опроса</title>
//         <style>
//             body { font-family: Arial, sans-serif; margin: 40px; }
//             .response { border: 1px solid #ccc; padding: 15px; margin: 10px 0; border-radius: 5px; }
//             .anonymous { color: #666; font-style: italic; }
//             .stats { background: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }
//         </style>
//     </head>
//     <body>
//         <h1>Результаты опроса</h1>
//         <div class="stats">
//             <h3>Всего ответов: {count}</h3>
//             <a href="/">Вернуться к форме</a>
//         </div>
//     "#.replace("{count}", &responses.len().to_string()));

//     for (i, response) in responses.iter().enumerate() {
//         let name_display = match &response.name {
//             Some(name) if !name.trim().is_empty() => name.clone(),
//             _ => "Аноним".to_string(),
//         };

//         html.push_str(&format!(r#"
//         <div class="response">
//             <h3>Ответ #{number} - {name}</h3>
//             <p><strong>Опыт с Linux:</strong> {experience}</p>
//             <p><strong>Отношение к переходу:</strong> {attitude}</p>
//             <p><strong>Задачи:</strong> {tasks}</p>
//             <p><strong>Готов к тестированию:</strong> {testing}</p>
//             <p><strong>Контакты:</strong> {contact}</p>
//             <p><strong>Комментарии:</strong> {comments}</p>
//         </div>
//         "#,
//         number = i + 1,
//         name = name_display,
//         experience = response.linux_experience,
//         attitude = response.attitude,
//         tasks = if response.daily_tasks.is_empty() { 
//             "не указано".to_string() 
//         } else { 
//             response.daily_tasks.join(", ") 
//         },
//         testing = if response.testing_willingness { "Да" } else { "Нет" },
//         contact = response.contact_preference.as_deref().unwrap_or("не указано"),
//         comments = response.comments.as_deref().unwrap_or("нет")
//         ));
//     }

//     html.push_str("</body></html>");
    
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(html))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init();

//     let app_state = web::Data::new(AppState::new());

//     println!("🚀 Сервер опроса запускается на http://localhost:8080");
//     println!("📊 Форма опроса доступна по основному URL");
//     println!("📈 Результаты доступны по http://localhost:8080/results");

//     HttpServer::new(move || {
//         App::new()
//             .app_data(app_state.clone())
//             .route("/", web::get().to(index))
//             .route("/submit", web::post().to(submit_response))
//             .route("/results", web::get().to(results))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }



// use actix_web::{web, App, HttpServer, Responder};

// async fn index() -> impl Responder {
//     "Hello world!"
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             // prefixes all resources and routes attached to it...
//             web::scope("/app")
//                 // ...so this handles requests for `GET /app/index.html`
//                 .route("/index.html", web::get().to(index)),
//         )
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }



// use actix_files as fs;
// use actix_web::{App, HttpServer};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(fs::Files::new("/static", ".").show_files_listing()))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }






use actix_web::{post, web, get, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_files::Files;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;
use std::fs;
use std::path::Path;
use urlencoding::decode;


const TEST_LOG_FILE: &str = "./default_test.log";
const PRODUCTION_LOG_FILE: &str = "./production.log";
const PRODUCTION_LOG_FILE_DIRECT: &str = "./productionF.log";



#[derive(Deserialize)]
struct FormData{
    name: Option<String>
}

fn decode_form_data(body: &str) -> String {
    let mut result = String::new();
    
    // Разбираем пары ключ=значение
    for pair in body.split('&') {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 {
            let key = decode(parts[0]).unwrap_or_else(|_| parts[0].into());
            let value = decode(parts[1]).unwrap_or_else(|_| parts[1].into());
            
            result.push_str(&format!("{}: {}\n", key, value));
        }
    }
    
    result
}

#[post("/debug")]
async fn debug_request(req: HttpRequest, payload: web::Payload) -> HttpResponse {
    println!("=== REQUEST INFO ===");
    println!("Method: {}", req.method());
    println!("URI: {}", req.uri());
    println!("Headers:");
    for (name, value) in req.headers() {
        println!("  {}: {:?}", name, value);
    }
    
    // Извлекаем тело
    let body = payload.to_bytes().await.unwrap();
    let body_str = String::from_utf8_lossy(&body);
    println!("Body: {}", body_str);
    println!("===================");

    HttpResponse::Ok().body("Запрос залогирован")
}


// #[post("/submit")]
// async fn submit_form(form: web::Form<FormData>) -> HttpResponse {
//     // let timestamp  = Utc :: now(). format("%Y-%m-%d %H:%M:%S");
//     // HttpResponse::InternalServerError().body("Ошибка при сохранении данных")
//     let name = form.name.clone().unwrap_or_else(|| "не указано".to_string());
//     println!("{}", name.to_string());

//     HttpResponse::Ok()
//                 .content_type("text/html; charset=utf-8")
//                 .body(name)

    
// }


#[post("/submit")]
async fn submit_form(req: HttpRequest, payload: web::Payload) -> HttpResponse {
    println!("=== REQUEST INFO ===");
    println!("Method: {}", req.method());
    println!("URI: {}", req.uri());
    println!("Headers:");
    for (name, value) in req.headers() {
        println!("  {}: {:?}", name, value);
    }
    
    // Извлекаем тело
    let body = payload.to_bytes().await.unwrap();
    let body_str = String::from_utf8_lossy(&body);
    let decoded_body = decode_form_data(&body_str).replace("+", " ");

    write_to_fileR(PRODUCTION_LOG_FILE, body_str.as_ref(), true);
    println!("Body: {}", &body_str);
    println!("===================");

    println!("DECODED: {}", &decoded_body);
    println!("===================");
    write_to_fileR(PRODUCTION_LOG_FILE_DIRECT, decoded_body.as_ref(), true);


    HttpResponse::Ok().body("Запрос залогирован")
}


fn read_from_fileR(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn delete_log_fileR(filename: &str) {
    fs::remove_file(filename).unwrap();
}



fn test_add(a: u32, b: u32) -> u32{
    return a+b;
}


fn write_to_fileR(filename: &str, strWrite: &str, append: bool) {
    if  !Path::new(filename).exists(){
        std::fs::File::create(filename).expect("create failed");
    }    
    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .open(filename)
        .unwrap();
    if let Err(e) =  write!(file, "{}", strWrite) {
        eprintln!("Couldn't write to file: {}", e);
    }    
}

#[post("/submit3")]
async fn create_user() -> HttpResponse {
    HttpResponse::Ok().body("fuck you2")
}


#[get("/submit")]
async fn create_userget() -> HttpResponse {
    HttpResponse::Ok().body("fuck you")
}

#[get("/submit2")]
async fn create_userget2() -> HttpResponse {
    HttpResponse::Ok().body("fuck you2")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Сервер запущен на http://localhost:8080");
    HttpServer::new(|| {
        App::new()
          //  .service(Files::new("/", "./static").index_file("index.html"))
          //  .service(create_user)

        /////    .service(Files::new("/","./static").index_file("index.html"))
            .service(debug_request)
            .service(create_userget)
            .service(submit_form)
            .service(create_userget2)
            .service(create_user)
            .service(Files::new("/","./static").index_file("index.html"))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = test_add(2, 9);
        assert_eq!(result, 11);
    }

    
    
    #[test]
    fn test_write_to_file() {
        delete_log_fileR(TEST_LOG_FILE);
        let content = "نظر شما برای اتخاذ تصمیمات آگاهانه بسیار حائز اهمیت است. این نظرسنجی ناشناس است — ذکر نام اختیاری می‌باشد.";
        write_to_fileR(TEST_LOG_FILE, content, true);
        let result = read_from_fileR(TEST_LOG_FILE);
        assert_eq!(result, content);
    }



    

}