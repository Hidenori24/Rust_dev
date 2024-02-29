use actix_web::{get, App, HttpResponse, HttpServer};
use thiserror::Error;

// エラーをまとめるenumを定義
// actix_web::ResponseErrorとして使うために deriveマクロでDebugを付与している必要がある．
#[derive(Error, Debug)]
enum MyError {}

impl ResponseError for MyError {}

// MyErrorは RespondeErrorを実装しているので indexの戻り値に MyErrorを使える
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";
    // HttpResponse::Ok()はステータスコード200を持つ HttpResponseBuilder という構造体を返す
    // HttpResponseBuilder の body()という関数にレスポンスのボディを渡すと HttpResponse が帰ってくる．
    // 戻り値が Resultなので Ok で包む．
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
