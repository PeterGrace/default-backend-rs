use once_cell::sync::Lazy;
use rocket::Route;
use rocket_prometheus::prometheus::{opts, Encoder, GaugeVec, TextEncoder};

pub(crate) static DEFAULT_BACKEND_APP_VER: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        opts!(
            "default_backend_app_info",
            "static app labels that potentially only change at restart"
        ),
        &["crate_version", "git_hash"],
    )
    .expect("foobar")
});
