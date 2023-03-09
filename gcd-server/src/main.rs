use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/compute", web::post().to(post_gcd))
    });

    server
        .bind("127.0.0.1:5500")
        .expect("port in use")
        .run()
        .await
        .expect("error starting server")
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="UTF-8">
                        <meta http-equiv="X-UA-Compatible" content="IE=edge">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <title>GCD Calculator</title>
                    </head>
                    <body>
                        <h1>GCD Server</h1>
                        <form action="/compute" method="post">
                        <input type="text" name="num1"/>
                        <input type="text" name="num2"/>
                        <button type="submit">Compute GCD</button>
                    </body>
                    </form>
                </html>
            "#,
        )
}


use serde::Deserialize;
mod math;

#[derive(Deserialize)]
struct RequestBody {
    num1: u64,
    num2: u64
}

async fn post_gcd(form: web::Form<RequestBody>) -> HttpResponse {
    if (form.num1 < 1 || form.num2 < 1) {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("both numbers must be greater than or equal to 1")
    }

    let response = format!(
        "The greatest common divisor between numbers {} and {} is <b>{}</b>\n",
        form.num1,
        form.num2,
        math::greatest_common_divisor(form.num1, form.num2)
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}
