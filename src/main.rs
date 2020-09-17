#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;
use rocket_contrib::templates::Template;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::{HeaderMap, Status, Header};
use serde::Deserialize;
use std::collections::HashMap;
use rocket::response::status;
use rocket::response::status::Custom;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct AllHeaders(HashMap<String, String>);
impl AllHeaders {
    fn get(&self, key: String) -> Option<&String> {
        self.0.get(key.as_str())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AllHeaders {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<AllHeaders, ()> {
        let mut hash = HashMap::new();
        for header in request.headers().iter() {
            hash.insert(header.name().to_string(), header.value().to_string());
        }
        return Outcome::Success(AllHeaders(hash));
    }
}


#[get("/<id>")]
fn get_something(id: String, headers: AllHeaders) -> Custom<Template>
{
    let context = headers;
    let mut status_code: u16 = 0;
    let mut variables = HashMap::new();
    match context.get(String::from("X-Code")) {
        Some(e) => status_code = e.parse().unwrap(),
        None => status_code = 500
    }
    variables.insert("user_agent", context.get(String::from("User-Agent")));
    variables.insert("status_code", context.get(String::from("X-Code")));
    variables.insert("accept_header", context.get(String::from("X-Format")));
    variables.insert("original_uri", context.get(String::from("X-Original-URI")));
    variables.insert("backend_namespace", context.get(String::from("X-Namespace")));
    variables.insert("ingress_name", context.get(String::from("X-Ingress-Name")));
    variables.insert("service_name", context.get(String::from("X-Service-Name")));
    variables.insert("service_port", context.get(String::from("X-Service-Port")));
    variables.insert("request_id", context.get(String::from("X-Request-ID")));
    let t = Template::render("default", &variables);
    let s = Status::new(status_code, "reason");
    status::Custom(s, t)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![get_something])
        .launch();
}
