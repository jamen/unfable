mod renderer;
mod state;

use std::sync::Arc;

use glam::UVec2;
use rend3::util::output::OutputFrame;
use renderer::*;
use state::*;

use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

use native_dialog::FileDialog;

use thiserror::Error;

#[derive(Error, Debug)]
enum InitError {
    #[error("Failed to read settings.toml.")]
    FailedToReadSettings,
    #[error("Settings file not found.")]
    SettingsNotFound,
    #[error("Config directory not found.")]
    ConfigDirNotFound,
    #[error("Failed to parse settings.toml")]
    FailedToParseSettings,
    #[error("Failed to parse settings.toml at {0}:{1}")]
    FailedToParseSettingsAt(usize, usize),
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // TODO: File logger?
    env_logger::init();

    let fable_dir = match FileDialog::new().show_open_single_dir().unwrap() {
        Some(fable_dir) => fable_dir,
        None => return
    };

    let mut state = State::new(&fable_dir).unwrap_or_else(|err| {
        log::error!("Failed to make state");
        panic!("{:?}", err);
    });

    let mut state = Arc::new(state);

    let event_loop = EventLoop::new();

    let window_size = winit::dpi::LogicalSize::new(1024, 768);

    let window = WindowBuilder::new()
        .with_title("OpenAlbion")
        .with_inner_size(window_size)
        // .with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
        .with_resizable(true)
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let iad = rend3::create_iad(None, None, None).await.unwrap();

    let surface = unsafe { iad.instance.create_surface(&window) };

    let preferred_format = surface.get_preferred_format(&iad.adapter).unwrap_or(wgpu::TextureFormat::Rgba8Unorm);

    rend3::configure_surface(
        &surface,
        &iad.device,
        preferred_format,
       [window_size.width as u32, window_size.height as u32].into(),
        wgpu::PresentMode::Mailbox,
    );

    let aspect_ratio = Some(window_size.width as f32 / window_size.height as f32);

    let renderer = rend3::Renderer::new(iad, aspect_ratio).unwrap();

    let mut main_render_routine = MainRenderRoutine::new(Arc::clone(&state));

    let output_frame = OutputFrame::from_surface(&surface).unwrap();

    renderer.render(&mut main_render_routine, output_frame);

    window.set_visible(true);

    event_loop.run(move |event, _, mut control_flow| {
        state.handle_event(&event, &mut control_flow);

        match event {
            Event::MainEventsCleared => {
                let output_frame = OutputFrame::from_surface(&surface).unwrap();

                renderer.render(&mut main_render_routine, output_frame);
            }
            _ => {}
        }
    })
}