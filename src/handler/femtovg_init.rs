use std::num::NonZeroU32;

use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use glutin::context::PossiblyCurrentContext;
use glutin::surface::Surface;
use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
#[allow(deprecated)]
use raw_window_handle::HasRawWindowHandle;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;
use winit::{dpi::PhysicalSize, window::Window};

pub fn init_canvas(
    event_loop: &ActiveEventLoop,
) -> (
    PossiblyCurrentContext,
    Canvas<OpenGl>,
    Window,
    Surface<WindowSurface>,
) {
    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let window_attr = WindowAttributes::default()
        .with_inner_size(PhysicalSize::new(1000., 600.))
        .with_title("Femotovg");
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attr));

    let (window, gl_config) = display_builder
        .build(event_loop, template, |mut configs| configs.next().unwrap())
        .unwrap();

    let window = window.unwrap();

    let gl_display = gl_config.display();

    #[allow(deprecated)]
    let context_attributes = ContextAttributesBuilder::new().build(Some(
        window
            .raw_window_handle()
            .expect("raw window handle failed"),
    ));

    let mut not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap()
    });

    #[allow(deprecated)]
    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window
            .raw_window_handle()
            .expect("raw window handle failed"),
        NonZeroU32::new(1000).unwrap(),
        NonZeroU32::new(600).unwrap(),
    );

    let surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let current_context = not_current_gl_context
        .take()
        .unwrap()
        .make_current(&surface)
        .unwrap();

    let renderer =
        unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s).cast()) }
            .expect("Cannot create renderer");

    let canvas = Canvas::new(renderer).expect("Cannot create canvas");

    (current_context, canvas, window, surface)
}
