use once_cell::sync::Lazy;
use rocket_prometheus::prometheus::{opts, GaugeVec};

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
