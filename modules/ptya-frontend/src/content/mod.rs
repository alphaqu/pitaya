use std::mem::forget;
use crate::content::app::{AppPanel, AppResponse};
use crate::dropper::Placement;
use crate::AppDropper;
use egui::style::Margin;
use egui::{Frame, Pos2, Rect, Vec2};
use log::{debug, info};
use ptya_core::app::AppId;
use ptya_core::ui::{Pui, INTERACTIVE_SIZE, SPACING_SIZE, VISUAL_SIZE};
use ptya_core::System;

mod app;

pub const WIDGET_WIDTH: f32 = 440.0;
pub const WIDGET_ADD_HEIGHT: f32 = 120.0;

pub struct Content {
    primary: Option<AppPanel>,
    widgets: Vec<AppPanel>,
}

impl Content {
    pub fn new() -> Content {
        Content {
            primary: None,
            widgets: vec![],
        }
    }

    pub fn tick(&mut self, system: &System, dropper: &mut Option<AppDropper>) {
        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(SPACING_SIZE),
                outer_margin: Default::default(),
                rounding: Default::default(),
                shadow: Default::default(),
                fill: system.color.theme().bg,
                stroke: Default::default(),
            })
            .show(&system.egui_ctx.clone(), |ui| {
                let mut ui = Pui::new(ui, system, system.color.new_state());
                let rect = ui.max_rect();

                if let Some(dropper) = dropper {
                    if let Some(location) = dropper.just_dropped {
                        info!("Dropped app at {location:?}");
                        self.open_app(
                            &mut ui,
                            location,
                            Rect::from_center_size(
                                dropper.pos,
                                Vec2::new(VISUAL_SIZE, VISUAL_SIZE),
                            ),
                            dropper.id.clone(),
                        );
                    }
                }

                self.update_layout(&mut ui, rect, dropper);

                let mut new_dropper = None;
                if let Some(primary) = &mut self.primary {
                    match primary.draw(&mut ui, dropper) {
                        Ok(_) => {}
                        Err(AppResponse::Move) => {
                            new_dropper = Some(primary.id().clone());
                        }
                    }
                }

                for app in &mut self.widgets {
                    match app.draw(&mut ui, dropper) {
                        Ok(_) => {}
                        Err(AppResponse::Move) => {
                            new_dropper = Some(app.id().clone());
                        }
                    };
                };

                if let Some(id) = new_dropper {
                    if dropper.is_none() {
                        *dropper = Some(AppDropper::new(id));
                    }
                }
            });
    }

    #[allow(clippy::blocks_in_if_conditions)]
    fn update_layout(&mut self, ui: &mut Pui, rect: Rect, dropper: &mut Option<AppDropper>) {
        let mut widget_width = 0.0;
        // If there are any widgets or we are dropping make the widget area bigger.
        if !self.widgets.is_empty() || dropper.as_ref().map(|v| !v.dropped).unwrap_or(false) {
            widget_width = WIDGET_WIDTH;
        }

        // Primary layout
        {
            let mut rect = rect;
            rect.min.x += widget_width + (SPACING_SIZE / 2.0);

            if let Some(primary) = &mut self.primary {
                primary.set_rect(ui, rect);
            }

            if let Some(dropper) = dropper {
                let mut from = rect;
                if self.widgets.is_empty() {
                    from = Rect::from_min_size(Pos2::new(rect.min.x - widget_width, rect.min.y), Vec2::new(rect.width() + widget_width, rect.height()));
                }
                dropper.add_placement(Placement::new(
                    from,
                    rect,
                    rect.contains(dropper.pos),
                    AppLocation::Primary,
                ))
            }
        }

        // Widget layout
        {
            let mut widget_rect = rect;
            widget_rect.set_width(widget_width - (SPACING_SIZE / 2.0));
            let len = self.widgets.len();
            let widget_height = widget_rect.height() / len.max(1) as f32;

            // Calculate where the dropper would be changing a widget
            let hovering: Option<NewAppLocation> = dropper.as_mut().and_then(|v| {
                if !v.dropped && widget_rect.contains(v.pos) {
                    let pos = (v.pos.y - widget_rect.min.y) / widget_height;
                    let threshold = WIDGET_ADD_HEIGHT / widget_height;
                    let size = Vec2::new(widget_rect.width(), WIDGET_ADD_HEIGHT);
                    let idx = pos.round() as usize;

                    // check if its gonna yield the same result.
                    let mut no_change = false;
                    {
                        if let Some(widget) = self.widgets.get(idx) {
                            if widget.id() == &v.id {
                                no_change = true;
                            }
                        }

                        if let Some(widget) = self.widgets.get(idx.saturating_sub(1)) {
                            if widget.id() == &v.id {
                                no_change = true;
                            }
                        }
                    }

                    if !no_change && (pos - pos.round()).abs() < threshold {
                        let pos = (idx as f32 * widget_height) - (WIDGET_ADD_HEIGHT / 2.0);
                        let location = NewAppLocation::WidgetEdge(idx);
                        let rect = Rect::from_min_size(
                            Pos2::new(
                                widget_rect.min.x,
                                (widget_rect.min.y + pos)
                                    .clamp(widget_rect.min.y, widget_rect.max.y - size.y),
                            ),
                            size,
                        );

                        if !self.widgets.is_empty() {
                            v.add_placement(Placement::new(
                                Rect::from_min_size(
                                    {
                                        if idx == 0 {
                                            Pos2::new(widget_rect.min.x, widget_rect.min.y)
                                        } else if idx == len {
                                            Pos2::new(widget_rect.min.x, widget_rect.max.y)
                                        } else {
                                            rect.left_center()
                                        }
                                    },
                                    Vec2::new(widget_rect.width(), 0.0),
                                ),
                                rect,
                                true,
                                location,
                            ));
                        }

                        Some(location)
                    } else {
                        Some(NewAppLocation::Existing(AppLocation::Widget(pos as usize)))
                    }
                } else {
                    None
                }
            });

            if let Some(dropper) = dropper {
                if self.widgets.is_empty() {
                    dropper.add_placement(Placement::new(
                        Rect::from_min_size(widget_rect.min, Vec2::new(0.0, widget_rect.height())),
                        widget_rect,
                        widget_rect.contains(dropper.pos),
                        NewAppLocation::WidgetEdge(0),
                    ));
                }
            }

            for (i, widget) in self.widgets.iter_mut().enumerate() {
                let mut rect = Rect::from_min_max(
                    widget_rect.min,
                    widget_rect.min + Vec2::new(widget_rect.width(), widget_height),
                )
                .translate(Vec2::new(0.0, i as f32 * widget_height));

                if i != 0 {
                    rect.min.y += SPACING_SIZE / 2.0;
                }

                if i + 1 != len {
                    rect.max.y -= SPACING_SIZE / 2.0;
                }

                // Add space if there is about to be a new widget to be added.
                let mut hovered = false;
                match hovering {
                    // Check if we are being hovered
                    Some(NewAppLocation::Existing(AppLocation::Widget(id))) => {
                        if i == id {
                            hovered = true;
                        }
                    }
                    // check if we should give place for an edge
                    Some(NewAppLocation::WidgetEdge(edge)) => {
                        if i == edge {
                            rect.min.y += if i == 0 {
                                WIDGET_ADD_HEIGHT
                            } else {
                                WIDGET_ADD_HEIGHT / 2.0
                            } + SPACING_SIZE;
                        }

                        if i + 1 == edge {
                            rect.max.y -= if i == len - 1 {
                                WIDGET_ADD_HEIGHT
                            } else {
                                WIDGET_ADD_HEIGHT / 2.0
                            } + SPACING_SIZE;
                        }
                    }
                    _ => {}
                }

                widget.set_rect(ui, rect);
                if let Some(dropper) = dropper {
                    let rect = widget.get_rect(ui);
                    dropper.add_placement(Placement::new(
                        rect,
                        rect,
                        hovered,
                        AppLocation::Widget(i),
                    ));
                }
            }
        }
    }

    fn open_app(
        &mut self,
        ui: &mut Pui,
        location: NewAppLocation,
        mut summon_rect: Rect,
        id: AppId,
    ) {
        let existing = self.find_app(&id);
        if let Some(app) = existing {
            if let Some(value) = self.get_app_mut(app) {
                summon_rect = value.get_rect(ui);
            }
        }
        let app = AppPanel::new(ui.sys, id, summon_rect);
        match location {
            NewAppLocation::Existing(AppLocation::Primary) => {
                // If its a primary app then we can just remove and not worry about it.
                if let Some(existing) = existing {
                    self.remove_app(existing);
                }
                self.primary = Some(app);
            }
            NewAppLocation::Existing(AppLocation::Widget(id)) => {
                // if its a widget,
                // replace the existing widget and then remove the old location if its not this replaced location
                *self.widgets.get_mut(id).unwrap() = app;
                if let Some(existing) = existing {
                    if existing != AppLocation::Widget(id) {
                        self.remove_app(existing)
                    }
                }
            }
            NewAppLocation::WidgetEdge(edge) => {
                // If its a widget edge. We to first insert the widget.
                // And then remove an existing one. If its under the idx we can safely remove it.
                // If its above we need to add one because our new widget moved the entire array by one.
                self.widgets.insert(edge, app);

                if let Some(existing) = existing {
                    match existing {
                        AppLocation::Primary => self.remove_app(existing),
                        AppLocation::Widget(id) => {
                            if id < edge {
                                self.remove_app(existing)
                            } else {
                                self.remove_app(AppLocation::Widget(id + 1))
                            }
                        }
                    }
                }
            }
        }
    }

    fn find_app(&self, id: &AppId) -> Option<AppLocation> {
        if let Some(app) = &self.primary {
            if app.id() == id {
                return Some(AppLocation::Primary);
            }
        }

        for (i, app) in self.widgets.iter().enumerate() {
            if app.id() == id {
                return Some(AppLocation::Widget(i));
            }
        }

        None
    }

    fn set_app(&mut self, location: AppLocation, app: AppPanel) {
        match location {
            AppLocation::Primary => {
                self.primary = Some(app);
            }
            AppLocation::Widget(widget) => {
                *self.widgets.get_mut(widget).unwrap() = app;
            }
        }
    }

    fn remove_app(&mut self, location: AppLocation) {
        match location {
            AppLocation::Primary => {
                self.primary = None;
            }
            AppLocation::Widget(id) => {
                self.widgets.remove(id);
            }
        }
    }

    fn get_app_mut(&mut self, location: AppLocation) -> Option<&mut AppPanel> {
        match location {
            AppLocation::Primary => self.primary.as_mut(),
            AppLocation::Widget(widget) => self.widgets.get_mut(widget),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AppLocation {
    Primary,
    Widget(usize),
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum NewAppLocation {
    Existing(AppLocation),
    WidgetEdge(usize),
}

impl From<AppLocation> for NewAppLocation {
    fn from(app: AppLocation) -> Self {
        NewAppLocation::Existing(app)
    }
}
