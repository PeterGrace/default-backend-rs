#![feature(proc_macro_hygiene, decl_macro)]

mod all_headers;
mod metrics;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;
extern crate env_logger;
extern crate log;
use crate::all_headers::AllHeaders;
use metrics::DEFAULT_BACKEND_APP_VER;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_prometheus::{prometheus, PrometheusMetrics};

static COMPRESSED_DEPENDENCY_LIST: &[u8] = auditable::inject_dependency_list!();

#[get("/health")]
fn get_health() -> String {
    "{\"status\":\"ok\"}".to_string()
}

#[get("/")]
fn get_error(headers: AllHeaders) -> Custom<Template> {
    let status_code: u16;
    match headers.get(String::from("X-Code")) {
        Some(e) => status_code = e.parse().unwrap_or(1001),
        None => status_code = 1000,
    }
    return match status_code {
        1000 => {
            let t: Template = Template::render("unknown", &headers);
            let s: Status = Status::Ok;
            status::Custom(s, t)
        }
        1001 => {
            let t: Template = Template::render("dbrs-error-parse", &headers);
            let s: Status = Status::InternalServerError;
            status::Custom(s, t)
        }
        _ => {
            let t: Template = Template::render("error", &headers);
            let s = Status::new(status_code, "Unspecified");
            status::Custom(s, t)
        }
    };
}

fn prepare_rocket() -> Rocket {
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
        .mount("/", routes![get_error, get_health])
}

fn main() {
    match std::env::var("RUST_LOG") {
        Err(_) => std::env::set_var("RUST_LOG", "debug"),
        Ok(_) => (),
    }
    env_logger::init();

    prepare_rocket().launch();
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::http::{Header, Status};
    use rocket::local::Client;

    #[test]
    fn test_valid_x_code() {
        let rocket = prepare_rocket();
        let client = Client::new(rocket).unwrap();
        let h = Header::new("X-Code", "418");
        let response = client.get("/").header(h).dispatch();
        assert_eq!(response.status(), Status::new(418, "Unspecified"));
    }
    #[test]
    fn test_x_code_parse_error() {
        let rocket = prepare_rocket();
        let client = Client::new(rocket).unwrap();
        let h = Header::new("X-Code", "foobar");
        let response = client.get("/").header(h).dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
    }
    #[test]
    fn test_no_x_code() {
        let rocket = prepare_rocket();
        let client = Client::new(rocket).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
    #[test]
    fn test_metrics_fairing() {
        let rocket = prepare_rocket();
        let client = Client::new(rocket).unwrap();
        let response = client.get("/metrics").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
    #[test]
    fn test_staticfile_fairing() {
        let rocket = prepare_rocket();
        let client = Client::new(rocket).unwrap();
        let response = client.get("/public/bar").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
