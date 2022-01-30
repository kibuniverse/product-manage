#[macro_use]
extern crate rocket;
extern crate base64;

use base64::decode;
use rocket::{
    catch, catchers,
    http::Status,
    request::{FromRequest, Outcome},
    serde::json::{serde_json::json, Value},
};
pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    // target Basic base64
    // aim username password
    fn form_header(auth_str: &str) -> Option<BasicAuth> {
        let split_vec = auth_str.split_whitespace().collect::<Vec<_>>();
        match split_vec.len() {
            2 => match split_vec[0] {
                "Basic" => Self::parse_base64(split_vec[1]),
                _ => None,
            },
            _ => None,
        }
    }
    fn parse_base64(base64_str: &str) -> Option<BasicAuth> {
        let decoded = decode(base64_str).ok()?;
        let decode_str = String::from_utf8(decoded).ok()?;
        let split_vec = decode_str.split(":").collect::<Vec<_>>();
        let (username, password) = (split_vec[0].to_string(), split_vec[1].to_string());
        Some(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::form_header(header_auth) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/hello")]
fn index() -> Value {
    json!("商品管理")
}

#[get("/list")]
fn product(auth: BasicAuth) -> Value {
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
