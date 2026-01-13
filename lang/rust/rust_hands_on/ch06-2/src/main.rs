use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/top1", axum::routing::get(handle_index1))
        .route("/top2", axum::routing::get(handle_index2))
        .route("/post2", axum::routing::post(handle_post2));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_index1() -> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (top index)");
    context.insert("message", "これはサンプルです。");

    let output = tera.render("index1.html", &context);
    axum::response::Html(output.unwrap())
}

#[derive(Serialize, Deserialize)]
struct Myform {
    name: String,
    mail: String,
}

async fn handle_index2() -> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (top index)");
    context.insert("message", "これはサンプルです。");

    let output = tera.render("index2.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_post2(axum::Form(myform): axum::Form<Myform>) -> axum::response::Html<String> {
    let msg = format!("I am {}<{}>.", myform.name, myform.mail);
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (form)");
    context.insert("message", &msg);

    let output = tera.render("index2.html", &context);
    axum::response::Html(output.unwrap())
}
