use std::num::NonZeroU32;

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Renderer};
use glutin::surface::Surface;
use glutin::{context::PossiblyCurrentContext, display::Display};
use glutin_winit::DisplayBuilder;
#[allow(deprecated)]
use raw_window_handle::HasRawWindowHandle;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::event::WindowEvent; // this is the WindowEvent::CloseRequested
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowAttributes, WindowId};
use winit::{dpi::PhysicalSize, window::Window};

use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};

fn main() {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    let (context, gl_display, window, surface) = create_window(&event_loop);

    let renderer =
        unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s).cast()) }
            .expect("Cannot create renderer");

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(1000, 600, window.scale_factor() as f32);

    let default_mouse_pos = PhysicalPosition::new(0., 0.);

    let mut app = MyApplicationHandler {
        close_requested: false,
        mouse_position: default_mouse_pos,
        window,
        context,
        surface,
        canvas,
    };
    event_loop.run_app(&mut app).expect("run failed");
}

struct MyApplicationHandler {
    close_requested: bool,
    mouse_position: PhysicalPosition<f64>,
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    canvas: Canvas<OpenGl>,
}

impl ApplicationHandler for MyApplicationHandler {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = position;
                self.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                render(
                    &self.context,
                    &self.surface,
                    &self.window,
                    &mut self.canvas,
                    self.mouse_position,
                );
            }
            _ => println!("{:?} {:?}", window_id, event),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.close_requested {
            event_loop.exit();
        }
    }
}

fn create_window(
    event_loop: &EventLoop<()>,
) -> (
    PossiblyCurrentContext,
    Display,
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

    (
        not_current_gl_context
            .take()
            .unwrap()
            .make_current(&surface)
            .unwrap(),
        gl_display,
        window,
        surface,
    )
}

fn render<T: Renderer>(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<T>,
    square_position: PhysicalPosition<f64>,
) {
    // Make sure the canvas has the right size:
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, window.scale_factor() as f32);

    // clear canvas by filling with black
    canvas.clear_rect(0, 0, size.width, size.height, Color::black());

    // Make smol red rectangle
    canvas.clear_rect(
        square_position.x as u32,
        square_position.y as u32,
        30,
        30,
        Color::rgbf(1., 0., 0.),
    );

    // Tell renderer to execute all drawing commands
    canvas.flush();

    // Display what we've just rendered
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}
