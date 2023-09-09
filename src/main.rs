#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use permute_mmo_rs_ui::PermuteMMO;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    env_logger::init();

    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "PermuteMMO - Rust Edition",
        native_options,
        Box::new(|cc| Box::new(PermuteMMO::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "permute_mmo_rs", // hardcode it
                web_options,
                Box::new(|cc| Box::new(PermuteMMO::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });

}
