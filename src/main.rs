#![feature(proc_macro_hygiene, decl_macro)]

mod all_headers;
mod metrics;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;
use crate::all_headers::AllHeaders;
use metrics::DEFAULT_BACKEND_APP_VER;
use rocket::http::{Header, HeaderMap, Status};
use rocket::response::status;
use rocket::response::status::Custom;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_prometheus::{prometheus, PrometheusMetrics};
use serde::Deserialize;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;

static COMPRESSED_DEPENDENCY_LIST: &[u8] = auditable::inject_dependency_list!();

#[get("/<file>")]
fn get_error(file: String, headers: AllHeaders) -> Custom<Template> {
    let mut variables = HashMap::new();
    variables.insert("user_agent", headers.get(String::from("User-Agent")));
    variables.insert("status_code", headers.get(String::from("X-Code")));
    variables.insert("accept_header", headers.get(String::from("X-Format")));
    variables.insert("original_uri", headers.get(String::from("X-Original-URI")));
    variables.insert(
        "backend_namespace",
        headers.get(String::from("X-Namespace")),
    );
    variables.insert("ingress_name", headers.get(String::from("X-Ingress-Name")));
    variables.insert("service_name", headers.get(String::from("X-Service-Name")));
    variables.insert("service_port", headers.get(String::from("X-Service-Port")));
    variables.insert("request_id", headers.get(String::from("X-Request-ID")));

    let status_code: u16;
    match variables.get("status_code") {
        Some(e) => status_code = e.unwrap_or(&String::from("1001")).parse().unwrap_or(1002),
        None => status_code = 1000,
    }
    return match status_code {
        1000 => {
            let t: Template = Template::render("dbrs-error-no-code", &variables);
            let s: Status = Status::new(status_code, "reason");
            status::Custom(s, t)
        }
        1001 => {
            let t: Template = Template::render("dbrs-error-unwrap", &variables);
            let s: Status = Status::new(500, "reason");
            status::Custom(s, t)
        }
        1002 => {
            let t: Template = Template::render("dbrs-error-parse", &variables);
            let s: Status = Status::new(500, "reason");
            status::Custom(s, t)
        }
        _ => {
            let error_template = format!("{}", file);
            let t: Template = Template::render(error_template, &variables);
            let s = Status::new(status_code, "reason");
            status::Custom(s, t)
        }
    };
}

fn main() {
    let prometheus = PrometheusMetrics::new();
    prometheus
        .registry()
        .register(Box::new(DEFAULT_BACKEND_APP_VER.clone()))
        .unwrap();
    let appdata_gauge =
        DEFAULT_BACKEND_APP_VER.with_label_values(&[env!("CARGO_PKG_VERSION"), env!("GIT_HASH")]);
    appdata_gauge.set(1.0);
    prometheus::gather();
    rocket::ignite()
        .attach(Template::fairing())
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount("/public", StaticFiles::from("public"))
        .mount("/", routes![get_error])
        .launch();
}
