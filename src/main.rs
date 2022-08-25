#![feature(drain_filter)]
#![feature(stmt_expr_attributes)]
extern crate egui;

use egui::style::{Spacing, WidgetVisuals, Widgets};
use egui::FontFamily::{Monospace, Proportional};
use egui::{
    Context, FontData, FontDefinitions, FontFamily, FontId, Frame, ImageData, Rounding, Stroke,
    Style, TextStyle, Vec2, Visuals,
};
// use glfw::{WindowMode};
use glium::backend::Facade;
use glutin::platform::unix::WindowBuilderExtUnix;
//use ptya_common::settings::INTERACTIVE_SIZE;
//use ptya_common::System;
//use ptya_frontend::Frontend;
use std::rc::Rc;
use log::warn;
use tokio::runtime::Handle;
use tokio::task::block_in_place;
use ptya_frontend::Frontend;
//use eframe::{App, NativeOptions, run_native};

//use ptya_glfw_glium::EguiGlfwGlium;
//use ptya_spotify::{Spotify, SpotifyAppData, SpotifyLogin};

fn main() {
    let display_size = 27.0;
    let target_display_size = 27.0;
    let ratio = target_display_size / display_size;

    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let display = create_display(&event_loop, (1920.0 * ratio), (1080.0 * ratio));

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);
    let mut pitaya = Pitaya::new(&egui_glium.egui_ctx, display.get_context(), ratio);

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            let mut quit = false;

            let repaint_after = egui_glium.run(&display, |egui_ctx| {
                pitaya.update(egui_ctx);
            });

            for (_, app) in pitaya.frontend.system.app.apps().iter_mut() {
                if app.id.is_none() {
                    app.id = Some(egui_glium.painter.register_native_texture(app.framebuffer.clone()));
                }

                if app.dirty {
                    if let Some(id) = app.id {
                        egui_glium.painter.replace_native_texture(id, app.framebuffer.clone());
                    } else {
                        warn!("App is dirty but does not have an id bound");
                    }
                    app.dirty = false;
                }
            }

            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if repaint_after {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                use glium::Surface as _;
                let mut target = display.draw();

               // let color = egui::Rgba::from_rgb(0.05, 0.05, 0.05);
                //target.clear_color(color[0], color[1], color[2], color[3]);

                // draw things behind egui here

                egui_glium.paint(&display, &mut target);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                egui_glium.on_event(&event);

                display.gl_window().window().request_redraw(); // TODO(emilk): ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached {
                ..
            }) => {
                display.gl_window().window().request_redraw();
            }
            _ => (),
        }
    });
}

fn create_display(
    event_loop: &glutin::event_loop::EventLoop<()>,
    width: f32,
    height: f32,
) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
        //  .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_inner_size(glutin::dpi::LogicalSize { width, height })
        .with_base_size(glutin::dpi::PhysicalSize { width, height })
        .with_title("Pitaya");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_multisampling(4)
        .with_srgb(true)
        .with_stencil_buffer(0);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}

pub struct Pitaya {
   // system: System,
    //     frontend: Option<Frontend>, //sidebar: SidebarPanel,
    //                                 //content: ContentPanel,
    frontend: Frontend,
}

impl Pitaya {
    pub fn new(ctx: &Context, opengl: &Rc<glium::backend::Context>, ratio: f32) -> Pitaya {
        ctx.set_visuals(Visuals::dark());
        ctx.set_pixels_per_point(ratio);
        //ctx.tessellation_options().feathering = false;
        Pitaya {
            // sidebar: SidebarPanel::new(&ctx, &system),
            // content: ContentPanel::new(),
            // system,
           // system: System::new(ctx.clone(), opengl.clone()).expect("Failed to initialize system"),
            frontend: Frontend::new(ctx.clone(), opengl.clone()).unwrap(),
        }
    }

    fn update(&mut self, ctx: &Context) {
        self.frontend.tick().unwrap();
        //if !self.system.is_loaded() {
        //    self.system.tick().unwrap();
        //    if self.system.is_loaded() {
        //        self.frontend = Some(Frontend::new(&mut self.system));
        //    }
        //} else if let Some(frontend) = &mut self.frontend {
        //    self.system.animation.tick(ctx);
        //    frontend.tick(&mut self.system);
        //}
        //self.sidebar
        //    .update(ctx, &mut self.system, &mut self.content);
        //self.content.update(ctx, &mut self.system);
        // if ctx.wants_keyboard_input() {}
    }
}
