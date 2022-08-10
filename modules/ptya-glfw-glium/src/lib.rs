#![warn(clippy::all)]
#![allow(clippy::single_match)]

extern crate core;

use std::ffi::c_void;
use std::os::raw::c_int;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::time::Instant;
// Re-export dependencies.
pub use egui;
pub use glfw;

mod painter;

pub use painter::Painter;

use egui::*;
use glfw::{Context, Glfw, init, WindowEvent};
use glium::backend::Backend;
use glium::SwapBuffersError;

#[cfg(not(feature = "clipboard"))]
mod clipboard;
mod input;

use clipboard::{
    ClipboardContext, // TODO: remove
    ClipboardProvider,
};
use crate::input::{EguiInputState};


pub struct EguiGlfwGlium {
    pub egui_ctx: egui::Context,
    input: EguiInputState,
    painter: Painter,

    events: Receiver<(f64, WindowEvent)>,
    pub context: Rc<glium::backend::Context>,
    internals: Rc<Internals>,
}

impl EguiGlfwGlium {
    pub fn new(window: glfw::Window, events: Receiver<(f64, WindowEvent)>) -> Self {
        let (width, height) = window.get_framebuffer_size();
        let native_pixels_per_point = window.get_content_scale().0;

        let internals = Rc::new(Internals {
            window
        });

        let context = unsafe {
            glium::backend::Context::new(internals.clone(), true, Default::default())
        }.expect("Critical Failure when creating opengl context.");

        let painter = Painter::new(&context);
        Self {
            egui_ctx: Default::default(),
            input: EguiInputState::new(egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::new(0f32, 0f32),
                    vec2(width as f32, height as f32) / native_pixels_per_point,
                )),
                pixels_per_point: Some(native_pixels_per_point),
                ..Default::default()
            }),
            painter,
            events,
            context:  unsafe {
                glium::backend::Context::new(internals.clone(), true, Default::default())
            }.expect("Critical Failure when creating opengl context."),
            internals
        }
    }

    /// Returns `true` if egui requests a repaint.
    ///
    /// Call [`Self::paint`] later to paint.
    pub fn run(
        mut self,
        mut run_ui: impl FnMut(&EguiGlfwGlium),
    )  {
        let mut repaint = true;
        let start_time = Instant::now();
        loop {
            self.input.input.time = Some(start_time.elapsed().as_secs_f64());
            self.input.input.pixels_per_point = Some(self.input.input.pixels_per_point.unwrap_or(1.0));
            unsafe {
                glfw::ffi::glfwPollEvents();
            }
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Close => {
                        unsafe { glfw::ffi::glfwSetWindowShouldClose(self.internals.window.window_ptr(), true as c_int) }
                    },
                    _ => {
                        repaint = true;
                        input::handle_event(event, &mut self.input);
                    }
                }
            }

            self.egui_ctx.begin_frame(self.input.input.take());


            if repaint {
                repaint = false;

                run_ui(&self);
                let output = self.egui_ctx.end_frame();
                if output.needs_repaint {
                    repaint = true;
                }

                if !output.platform_output.copied_text.is_empty() {
                    input::copy_to_clipboard(&mut self.input, output.platform_output.copied_text);
                }


                if let Some(rect) =  self.input.input.screen_rect {
                    let mut frame = glium::Frame::new(self.context.clone(), (rect.width() as u32, rect.height() as u32));
                    let clipped_primitives = self.egui_ctx.tessellate(output.shapes);
                    self.painter.paint_and_update_textures(
                        &self.context,
                        &self.internals,
                        &mut frame,
                        self.egui_ctx.pixels_per_point(),
                        &clipped_primitives,
                        &output.textures_delta,
                    );
                    frame.finish().unwrap();
                }

                //self.internals.swap_buffers().unwrap();
            }
        }
    }
}


pub(crate) struct Internals {
    window: glfw::Window,
}

unsafe impl Backend for Internals {
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        unsafe {
            glfw::ffi::glfwSwapBuffers(self.window.window_ptr());
        }
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        self.window.glfw.get_proc_address_raw(symbol)
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let size = self.window.get_framebuffer_size();
        (size.0 as u32, size.1 as u32)
    }

    fn is_current(&self) -> bool {
        self.window.is_current()
    }

    unsafe fn make_current(&self) {
        glfw::ffi::glfwMakeContextCurrent(self.window.window_ptr());
    }
}