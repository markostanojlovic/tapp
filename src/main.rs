mod tap;
 
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tap::Tap;
use serde_json::Value;


#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(Tap::now(""))
}

#[get("/help")]
async fn help() -> impl Responder {
    HttpResponse::Ok().body("This is the simple web app to check when I took my meds.\n \
                            Here is how you use it:")
}

#[post("/tap")]
async fn user_tap(req_body: String) -> impl Responder {
    let body: Value = serde_json::from_str(&req_body).unwrap();
    println!("{:#?}", &body);
    HttpResponse::Ok().body(Tap::now(body["comment"].to_string().as_ref()))
}

#[get("/history")]
async fn history() -> impl Responder {
    HttpResponse::Ok().body(Tap::history())
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(help)
            .service(user_tap)
            .service(history)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 7575))?
    .run()
    .await
}
