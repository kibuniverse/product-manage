#[macro_use]
extern crate rocket;
use rocket::{
    catch, catchers,
    serde::json::{serde_json::json, Value},
};

struct user {
    username: String,
    password: String,
}
#[get("/hello")]
fn index() -> Value {
    json!("商品管理")
}

#[get("/list")]
fn product() -> Value {
    json!("list")
}

#[delete("/<id>")]
fn delete_product(id: i32) -> Value {
    println!("id: {} product delete success", id);
    json!("delete success")
}

#[catch(404)]
fn not_found_url() -> Value {
    json!("not found")
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/product", routes![product, delete_product])
        .register("/", catchers!(not_found_url))
}
