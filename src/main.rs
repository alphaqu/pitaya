#![feature(drain_filter)]
#![feature(stmt_expr_attributes)]
extern crate egui;

use egui::style::{Spacing, WidgetVisuals, Widgets};
use egui::FontFamily::{Monospace, Proportional};
use egui::{Context, FontId, Frame, ImageData, Rounding, Stroke, Style, TextStyle, Vec2, Visuals};
// use glfw::{WindowMode};
use ptya_common::apps::app::{AppInfo, AppInstance};
use ptya_common::apps::AppContainer;
use ptya_common::settings::style::StyleSettings;
use ptya_common::ui::WidgetApp;
use ptya_common::System;
use std::collections::HashMap;
use std::rc::Rc;
use glium::backend::Facade;
use glium::framebuffer::RenderBuffer;
use glium::texture::UncompressedFloatFormat;
use glutin::dpi::LogicalPosition;
use glutin::platform::unix::WindowBuilderExtUnix;
use glutin::window::Fullscreen;
//use eframe::{App, NativeOptions, run_native};

use ptya_frontend::content::ContentPanel;
use ptya_frontend::sidebar::SidebarPanel;
//use ptya_glfw_glium::EguiGlfwGlium;
use ptya_map::Map;
//use ptya_spotify::{Spotify, SpotifyAppData, SpotifyLogin};

fn main() {
    let display_size = 27.0;
    let target_display_size = 15.6
        ;
    let ratio = target_display_size / display_size;




//
    //let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    //glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    //glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
    //glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    //glfw.window_hint(glfw::WindowHint::Resizable(true));
//
    //let (mut window, events) = glfw
    //    .create_window(
    //        (1920.0 * ratio) as u32,
    //        (1080.0 * ratio) as u32,
    //        "Pitaya",
    //        WindowMode::Windowed,
    //    )
    //    .unwrap();
//
    //window.set_char_polling(true);
    //window.set_cursor_pos_polling(true);
    //window.set_key_polling(true);
    //window.set_mouse_button_polling(true);
    //glfw::Context::make_current(&mut window);
//
    //let glium = EguiGlfwGlium::new(window, events);
    //let mut pitaya = Pitaya::new(glium.egui_ctx.clone(), ratio);
    //glium.run(|ctx|  {
    //    pitaya.update(&ctx.egui_ctx);
    //});

     // run_native(
    //              "Pitaya",
    //              NativeOptions {
    //                  initial_window_size: Some(Vec2::new(1920.0 * ratio, 1080.0 * ratio)),
    //                  ..NativeOptions::default()
    //              },
    //              Box::new(move |cc| {
    //                  cc.egui_ctx.set_visuals(Visuals::dark());
    //                  cc.egui_ctx.set_pixels_per_point(ratio);
    //                  let mut system = System::new(StyleSettings::pitaya_dark());
    //                  system.apps.load_app(AppContainer::new(
    //                      &cc.egui_ctx,
    //                      Map::app_info(),
    //                      Box::new(Map::new().unwrap()),
    //                  ));
    //                  system.apps.load_app(AppContainer::new(
    //                      &cc.egui_ctx,
    //                      WidgetApp::app_info(),
    //                      Box::new(WidgetApp::new()),
    //                  ));
    //                  system.apps.load_app(AppContainer::new(
    //                      &cc.egui_ctx,
    //                      Spotify::app_info(),
    //                      Box::new(Spotify::new(
    //                          &system,
    //                          SpotifyAppData {
    //                              login: Some(SpotifyLogin {
    //                                  username: "yan.gyunashyan@gmail.com".to_string(),
    //                                  password: "P%n2g$01@4SHBXlD2A67Cgwe".to_string(),
    //                              }),
    //                          },
    //                      )),
    //                  ));
    //
    //                  cc.egui_ctx.set_style(Style {
    //                      text_styles: [
    //                          (TextStyle::Heading, FontId::new(90.0, Proportional)),
    //                          (
    //                              TextStyle::Name("Heading2".into()),
    //                              FontId::new(75.0, Proportional),
    //                          ),
    //                          (
    //                              TextStyle::Name("Context".into()),
    //                              FontId::new(69.0, Proportional),
    //                          ),
    //                          (TextStyle::Body, FontId::new(35.0, Proportional)),
    //                          (TextStyle::Monospace, FontId::new(42.0, Proportional)),
    //                          (TextStyle::Button, FontId::new(35.0, Proportional)),
    //                          (TextStyle::Small, FontId::new(30.0, Proportional)),
    //                      ]
    //                      .into(),
    //                      visuals: Visuals {
    //                          widgets: Widgets {
    //                              noninteractive: WidgetVisuals {
    //                                  bg_fill: system.settings.style.bg_2,
    //                                  bg_stroke: Stroke::none(),
    //                                  rounding: Rounding::same(system.settings.rounding / 2.0),
    //                                  fg_stroke: Stroke::new(1.0, system.settings.style.fg_2),
    //                                  expansion: 0.0,
    //                              },
    //                              inactive: WidgetVisuals {
    //                                  bg_fill: system.settings.style.bg_2,
    //                                  bg_stroke: Stroke::new(0.5, system.settings.style.bg_2),
    //                                  rounding: Rounding::same(system.settings.rounding / 2.0),
    //                                  fg_stroke: Stroke::new(1.0, system.settings.style.fg_4),
    //                                  expansion: 0.0,
    //                              },
    //                              hovered: WidgetVisuals {
    //                                  bg_fill: system.settings.style.bg_3,
    //                                  bg_stroke: Stroke::new(1.0, system.settings.style.fg_0),
    //                                  rounding: Rounding::same(system.settings.rounding / 2.0),
    //                                  fg_stroke: Stroke::new(1.0, system.settings.style.fg_5),
    //                                  expansion: 0.0,
    //                              },
    //                              active: WidgetVisuals {
    //                                  bg_fill: system.settings.style.bg_4,
    //                                  bg_stroke: Stroke::new(1.0, system.settings.style.fg_2),
    //                                  rounding: Rounding::same(system.settings.rounding / 2.0),
    //                                  fg_stroke: Stroke::new(1.0, system.settings.style.fg_5),
    //                                  expansion: 0.0,
    //                              },
    //                              open: WidgetVisuals {
    //                                  bg_fill: system.settings.style.bg_4,
    //                                  bg_stroke: Stroke::new(1.0, system.settings.style.fg_2),
    //                                  rounding: Rounding::same(system.settings.rounding / 2.0),
    //                                  fg_stroke: Stroke::new(1.0, system.settings.style.fg_5),
    //                                  expansion: 0.0,
    //                              },
    //                          },
    //                          ..Visuals::dark()
    //                      },
    //                      spacing: Spacing {
    //                          item_spacing: Vec2::new(25.0, 25.0),
    //                          button_padding: Vec2::new(24.0, 12.0),
    //                          ..Spacing::default()
    //                      },
    //                      ..Style::default()
    //                  });
    //
    //                  Box::new(Pitaya {
    //                      sidebar: SidebarPanel::new(&cc.egui_ctx, &system),
    //                      content: ContentPanel::new(),
    //                      system,
    //                  })
    //              }),
    //          );
    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let display = create_display(&event_loop, (1920.0 * ratio), (1080.0 * ratio) );

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);
    let mut pitaya = Pitaya::new(&egui_glium.egui_ctx, display.get_context(), ratio);

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            let mut quit = false;

            let repaint_after = egui_glium.run(&display, |egui_ctx| {
                pitaya.update(egui_ctx);
            });

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

                let color = egui::Rgba::from_rgb(0.05, 0.05, 0.05);
                target.clear_color(color[0], color[1], color[2], color[3]);

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

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>, width: f32, height: f32) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()

      //  .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize {
            width,
            height,
        })
        .with_base_size(
            glutin::dpi::PhysicalSize {
                width,
                height,
            }
        )
        .with_title("Pitaya");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}

pub struct Pitaya {
    system: System,
    sidebar: SidebarPanel,
    content: ContentPanel,
}

impl Pitaya {
    pub fn new(ctx: &Context, opengl: &Rc<glium::backend::Context>, ratio: f32) -> Pitaya {
        ctx.set_visuals(Visuals::dark());
        ctx.set_pixels_per_point(ratio);
        let mut system = System::new(StyleSettings::pitaya_dark());
        system.apps.load_app(AppContainer::new(
            &ctx,
            Map::app_info(),
            AppInstance::EGui(Box::new(Map::new().unwrap())),
        ));

       //system.apps.load_app(AppContainer::new(
       //    &ctx,
       //    ptya_map_v2:: Map::app_info(),
       //    AppInstance::OpenGL {
       //        ctx: opengl.clone(),
       //        buffer: Rc::new(RenderBuffer::new(opengl, UncompressedFloatFormat::U8U8U8U8, 10, 10).unwrap()),
       //        app: Box::new(ptya_map_v2::Map::new(opengl).unwrap())
       //    },
       //));
        system.apps.load_app(AppContainer::new(
            &ctx,
            WidgetApp::app_info(),
            AppInstance::EGui(Box::new(WidgetApp::new())),
        ));
        //system.apps.load_app(AppContainer::new(
        //    &ctx,
        //    Spotify::app_info(),
        //    AppInstance::EGui(Box::new(Spotify::new(
        //        &system,
        //        SpotifyAppData {
        //            login: Some(SpotifyLogin {
        //            }),
        //        },
        //    ))),
        //));

        ctx.set_style(Style {
            text_styles: [
                (TextStyle::Heading, FontId::new(90.0, Proportional)),
                (
                    TextStyle::Name("Heading2".into()),
                    FontId::new(75.0, Proportional),
                ),
                (
                    TextStyle::Name("Context".into()),
                    FontId::new(69.0, Proportional),
                ),
                (TextStyle::Body, FontId::new(35.0, Proportional)),
                (TextStyle::Monospace, FontId::new(42.0, Proportional)),
                (TextStyle::Button, FontId::new(35.0, Proportional)),
                (TextStyle::Small, FontId::new(30.0, Proportional)),
            ]
                .into(),
            visuals: Visuals {
                widgets: Widgets {
                    noninteractive: WidgetVisuals {
                        bg_fill: system.settings.style.bg_2,
                        bg_stroke: Stroke::none(),
                        rounding: Rounding::same(system.settings.rounding / 2.0),
                        fg_stroke: Stroke::new(1.0, system.settings.style.fg_2),
                        expansion: 0.0,
                    },
                    inactive: WidgetVisuals {
                        bg_fill: system.settings.style.bg_2,
                        bg_stroke: Stroke::new(0.5, system.settings.style.bg_2),
                        rounding: Rounding::same(system.settings.rounding / 2.0),
                        fg_stroke: Stroke::new(1.0, system.settings.style.fg_4),
                        expansion: 0.0,
                    },
                    hovered: WidgetVisuals {
                        bg_fill: system.settings.style.bg_3,
                        bg_stroke: Stroke::new(1.0, system.settings.style.fg_0),
                        rounding: Rounding::same(system.settings.rounding / 2.0),
                        fg_stroke: Stroke::new(1.0, system.settings.style.fg_5),
                        expansion: 0.0,
                    },
                    active: WidgetVisuals {
                        bg_fill: system.settings.style.bg_4,
                        bg_stroke: Stroke::new(1.0, system.settings.style.fg_2),
                        rounding: Rounding::same(system.settings.rounding / 2.0),
                        fg_stroke: Stroke::new(1.0, system.settings.style.fg_5),
                        expansion: 0.0,
                    },
                    open: WidgetVisuals {
                        bg_fill: system.settings.style.bg_4,
                        bg_stroke: Stroke::new(1.0, system.settings.style.fg_2),
                        rounding: Rounding::same(system.settings.rounding / 2.0),
                        fg_stroke: Stroke::new(1.0, system.settings.style.fg_5),
                        expansion: 0.0,
                    },
                },
                ..Visuals::dark()
            },
            spacing: Spacing {
                item_spacing: Vec2::new(25.0, 25.0),
                button_padding: Vec2::new(24.0, 12.0),
                ..Spacing::default()
            },
            ..Style::default()
        });

        Pitaya {
            sidebar: SidebarPanel::new(&ctx, &system),
            content: ContentPanel::new(),
            system,
        }
    }

    fn update(&mut self, ctx: &Context) {
        self.sidebar.update(ctx, &self.system, &mut self.content);
        self.content.update(ctx, &mut self.system);
        if ctx.wants_keyboard_input() {
            
        }
    }
}