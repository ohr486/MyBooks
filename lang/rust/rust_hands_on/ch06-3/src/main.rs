use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/:value", axum::routing::get(handle_index1))
        .route("/raw/:value", axum::routing::get(handle_index2))
        .route("/con/:value", axum::routing::get(handle_index3))
        .route("/if/:value", axum::routing::get(handle_index4))
        .route("/janken/:value", axum::routing::get(handle_index5))
        .route("/array", axum::routing::get(handle_index6))
        .route("/map", axum::routing::get(handle_index7))
        .route("/filter1", axum::routing::get(handle_index8))
        .route("/filter2", axum::routing::get(handle_index9))
        .route("/filter3", axum::routing::get(handle_index10))
        .route("/top2", axum::routing::get(handle_index11))
        .route("/if2", axum::routing::get(handle_index12));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct Myform {
    name: String,
    mail: String,
}

async fn handle_index1(axum::extract::Path(value): axum::extract::Path<usize>) -> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (value)");
    context.insert("value", &value);

    let output = tera.render("index1.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_index2(axum::extract::Path(value): axum::extract::Path<usize>) -> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (raw)");
    context.insert("value", &value);

    let output = tera.render("index2.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_index3(axum::extract::Path(value): axum::extract::Path<usize>) -> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (con)");
    context.insert("value", &value);

    let output = tera.render("index3.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_index4(axum::extract::Path(value): axum::extract::Path<usize>) -> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (if)");
    context.insert("value", &value);

    let output = tera.render("index4.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_index5(axum::extract::Path(value): axum::extract::Path<usize>) -> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (じゃんけん)");
    context.insert("value", &value);

    let output = tera.render("index5.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_index6() -> axum::response::Html<String> {
    let data = [
        Myform { name: "taro".to_string(), mail: "taro@yamada".to_string() },
        Myform { name: "hanako".to_string(), mail: "hanako@flower".to_string() },
        Myform { name: "sachiko".to_string(), mail: "sachiko@happy".to_string() },
        Myform { name: "jiro".to_string(), mail: "jiro@change".to_string() },
    ];

    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (配列)");
    context.insert("data", &data);

    let output = tera.render("index6.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_index7() -> axum::response::Html<String> {
    let mut map = std::collections::HashMap::new();
    map.insert("taro", ("taro@yamada", 39));
    map.insert("hanako", ("hanako@flower", 28));
    map.insert("sachiko", ("sachiko@happy", 17));

    let tera = tera::Tera::new("templates/*").unwrap();

    let mut context = tera::Context::new();
    context.insert("title", "Index page (マップ)");
    context.insert("data", &map);

    let output = tera.render("index7.html", &context);
    axum::response::Html(output.unwrap())
}

fn hello_filter(value: &tera::Value, _: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let s = tera::try_get_value!("hello_filter", "value", String, value);
    Ok(tera::Value::String(format!("こんにちは、{}さん!", s)))
}

async fn handle_index8() -> axum::response::Html<String> {
    let mut tera = tera::Tera::new("templates/*").unwrap();
    tera.register_filter("hello", hello_filter);

    let mut context = tera::Context::new();
    context.insert("title", "Index page (hello filter)");
    context.insert("name", "山田タロー");

    let output = tera.render("index8.html", &context);
    axum::response::Html(output.unwrap())
}

fn sample_filter(value: &tera::Value, _: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let data = [
        ("taro", "taro@yamada", 39, "male"),
        ("hanako", "hanako@flower", 28, "female"),
        ("sachiko", "sachiko@happy", 17, "female"),
        ("jiro", "jiro@change", 6, "male")
    ];
    let n = tera::try_get_value!("sample_filter", "value", usize, value);
    let item = data[n];
    Ok(tera::Value::String(format!("{}({},{})<{}>.", item.0, item.3, item.2, item.1)))
}

async fn handle_index9() -> axum::response::Html<String> {
    let mut tera = tera::Tera::new("templates/*").unwrap();
    tera.register_filter("sample", sample_filter);

    let mut context = tera::Context::new();
    context.insert("title", "Index page (sample filter)");
    context.insert("id", &1);

    let output = tera.render("index9.html", &context);
    axum::response::Html(output.unwrap())
}

fn calc_filter(_: &tera::Value, map: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let price = map.get("price").unwrap().as_f64().unwrap();
    let tax = map.get("tax").unwrap().as_f64().unwrap();
    let res = price * tax;
    Ok(tera::Value::String(format!("price: {} * tax: {} = {}", price, tax, res)))
}

async fn handle_index10() -> axum::response::Html<String> {
    let mut tera = tera::Tera::new("templates/*").unwrap();
    tera.register_filter("calc", calc_filter);

    let mut context = tera::Context::new();
    context.insert("title", "Index page (calc filter)");

    let output = tera.render("index10.html", &context);
    axum::response::Html(output.unwrap())
}

async fn handle_index11() -> axum::response::Html<String> {
    let mut params = std::collections::HashMap::new();
    params.insert("title", "Index page (index hbs)");
    params.insert("message", "This is sample page message!");

    let mut handlebars = handlebars::Handlebars::new();
    let _ = handlebars.register_template_string("hello", include_str!("templates/index1.hbs"));

    let template = handlebars.render("hello", &params).unwrap();
    axum::response::Html(template)
}

async fn handle_index12() -> axum::response::Html<String> {
    let num = 1234;
    let params = serde_json::json!({
        "title": "Index Page (index hbs2)",
        "num": num,
        "flg": num % 2 == 0,
        "data": ["apple", "banana", "orange"]
    });

    let mut handlebars = handlebars::Handlebars::new();
    let _ = handlebars.register_template_string("hello", include_str!("templates/index2.hbs"));

    let template = handlebars.render("hello", &params).unwrap();
    axum::response::Html(template)
}
