use app::{model, update, view};
use async_std::task::block_on;
use nannou::{
    wgpu::{Backends, DeviceDescriptor, Limits},
    App,
};

mod app;
mod vek_extension;

// From https://github.com/tomoyanonymous/nannou-web-template
async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()
        .device_descriptor(device_desc)
        .view(view)
        .build_async()
        .await
        .unwrap();
}

async fn run_app() {
    let app = nannou::app::Builder::new_async(|app| {
        Box::new(async move {
            create_window(app).await;
            model(app)
        })
    })
    .backends(Backends::PRIMARY | Backends::GL)
    .update(update);

    app.run_async().await;
}

fn main() {
    // Async needed to run on web

    block_on(async {
        run_app().await;
    });
}
