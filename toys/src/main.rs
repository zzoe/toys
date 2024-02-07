#![windows_subsystem = "windows"]

#[cfg(not(target_arch = "wasm32"))]
mod desktop {
    use time::format_description::well_known::Rfc3339;
    use time::UtcOffset;
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::fmt::time::OffsetTime;

    use toys::App;

    pub fn launch() {
        tracing_subscriber::fmt()
            .with_ansi(false)
            .with_thread_ids(true)
            .with_max_level(LevelFilter::INFO)
            .with_timer(OffsetTime::new(
                UtcOffset::from_hms(8, 0, 0).unwrap(),
                Rfc3339,
            ))
            .init();

        toys::init();

        dioxus_desktop::launch_cfg(
            App,
            dioxus_desktop::Config::new()
                .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string()),
        );
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use toys::App;

    pub fn launch() {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
        toys::init();
        dioxus_web::launch(App)
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    desktop::launch();

    #[cfg(target_arch = "wasm32")]
    web::launch();
}
