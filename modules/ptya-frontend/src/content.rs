pub mod app;
pub mod opening_app;

use app::{AppLocation, AppPanel};
use egui::epaint::Shadow;
use egui::style::Margin;
use egui::{Context, Frame, Id, LayerId, Order, Pos2, Rect, Rounding, Sense, Stroke, Ui, Vec2};
use opening_app::{DraggingApp, OpenAppLocation};
use ptya_common::app::AppId;
use ptya_common::color::color::{ColorState, ColorType};
use ptya_common::settings::{SPACING_SIZE, WIDGET_ADD_SIZE, WIDGET_WIDTH};
use ptya_common::System;

pub struct ContentPanel {
    pub id: Id,
    pub apps: ContentApps,
    pub dragging_app: Option<DraggingApp>,
}

impl ContentPanel {
    pub fn new() -> Self {
        Self {
            id: Id::null(),
            apps: ContentApps::new(),
            dragging_app: None,
        }
    }

    pub fn update(&mut self, ctx: &Context, system: &mut System) {
        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(SPACING_SIZE),
                outer_margin: Default::default(),
                rounding: Default::default(),
                shadow: Default::default(),
                fill: Default::default(),
                stroke: Default::default(),
            })
            .show(ctx, |ui| {
                self.id = ui.id();

                let content_rect = ui.max_rect();

                // Primary rectangle area setup
                self.update_primary(ui, system, content_rect);
                self.update_widgets(ui, system);

                if let Some(dragged) = &mut self.dragging_app {
                    dragged.update(ui, system, &mut self.apps, content_rect);
                    if dragged.for_removal(ui) {
                        self.dragging_app = None;
                    }
                }

                for widget in &mut self.apps.widgets {
                    widget.draw(ui, system, &mut self.dragging_app);
                }

                if let Some(primary) = &mut self.apps.primary {
                    primary.draw(ui, system, &mut self.dragging_app);
                }

                self.apps.finalize();
            });
    }

    fn update_widgets(&mut self, ui: &mut Ui, system: &mut System) {
        let widgets = self.apps.widgets.len() as f32;
        let rect = ui.max_rect();
        let widget_height = (rect.height() - (SPACING_SIZE * (widgets - 1.0))) / widgets;

        let mut widget_rect = Rect::from_min_size(rect.min, Vec2::new(WIDGET_WIDTH, widget_height));

        let len = self.apps.widgets.len();
        let mut above_active = false;
        for (i, app) in self.apps.widgets.iter_mut().enumerate() {
            let mut current_widget_rect = widget_rect;

            widget_rect = widget_rect.translate(Vec2::new(0.0, SPACING_SIZE + widget_height));

            if above_active {
                current_widget_rect.min.y += (WIDGET_ADD_SIZE + SPACING_SIZE) / 2.0;
                above_active = false;
            }

            if let Some(dragged) = &mut self.dragging_app {
                let add_size = Vec2::new(WIDGET_WIDTH, WIDGET_ADD_SIZE);
                // First Top
                if i == 0 {
                    let top_rect = Rect::from_min_size(current_widget_rect.min, add_size);
                    #[rustfmt::skip]
                    if dragged.draw_possible_placement(ui, top_rect, true, OpenAppLocation::NewWidget(0),  system) {
                        current_widget_rect.min.y += WIDGET_ADD_SIZE + SPACING_SIZE;
                    }
                }

                let mut add_space = WIDGET_ADD_SIZE + SPACING_SIZE;
                if i != len - 1 {
                    // the one below will also correct
                    add_space /= 2.0;
                }

                let bottom_rect =
                    Rect::from_min_size(widget_rect.min - Vec2::new(0.0, add_space), add_size);

                #[rustfmt::skip]
                if dragged.draw_possible_placement(ui, bottom_rect, true, OpenAppLocation::NewWidget(i + 1), system) {
                    current_widget_rect.max.y -= add_space;
                    above_active = true;
                }
            }

            app.set_rect(current_widget_rect, ui);
            if let Some(dragged) = &mut self.dragging_app {
                dragged.draw_possible_placement(
                    ui,
                    app.rect(ui),
                    false,
                    OpenAppLocation::Existing(AppLocation::Widget(i)),
                    system,
                );
            }
        }
    }

    fn update_primary(&mut self, ui: &mut Ui, system: &mut System, content_rect: Rect) {
        let mut primary_rect = content_rect;
        if !self.apps.widgets.is_empty() {
            // If there are widgets make the primary smaller to allow for the widgets to exist
            primary_rect.min.x += WIDGET_WIDTH;
            primary_rect.min.x += SPACING_SIZE;
        } else if let Some(dragged) = &mut self.dragging_app {
            // If there no widgets but we are dragging a new app in.
            // Expand the area and allow the add widget button to exist.
            let mut rect = content_rect;
            rect.set_width(WIDGET_ADD_SIZE);

            if dragged.draw_possible_placement(
                ui,
                rect,
                self.apps.primary.is_some(),
                OpenAppLocation::FirstWidget,
                system,
            ) || self.apps.primary.is_none()
            {
                primary_rect.min.x += SPACING_SIZE + WIDGET_ADD_SIZE;
            }
        }

        let mut placement_rect = primary_rect;
        if let Some(primary) = &mut self.apps.primary {
            primary.set_rect(primary_rect, ui);
            placement_rect = primary.rect(ui);
        }

        if let Some(dragged) = &mut self.dragging_app {
            dragged.draw_possible_placement(
                ui,
                placement_rect,
                false,
                OpenAppLocation::Existing(AppLocation::Primary),
                system,
            );
        };
    }
}

pub struct ContentApps {
    pub widgets: Vec<AppPanel>,
    pub primary: Option<AppPanel>,
}

impl ContentApps {
    pub fn new() -> ContentApps {
        ContentApps {
            widgets: vec![],
            primary: None,
        }
    }

    pub fn find_open_app(&self, app: &AppId) -> Option<AppLocation> {
        self.widgets
            .iter()
            .enumerate()
            .find_map(|(i, widget)| (widget.app_id() == app).then(|| AppLocation::Widget(i)))
            .or_else(|| {
                self.primary
                    .as_ref()
                    .and_then(|primary| (primary.app_id() == app).then(|| AppLocation::Primary))
            })
    }

    pub fn open_app(
        &mut self,
        ui: &Ui,
        mut summon_area: Rect,
        app_id: &AppId,
        location: OpenAppLocation,
    ) {
        let old_location = self.find_open_app(app_id);
        if let Some(old_location) = old_location {
            if let Some(app) = self.get_app_mut(old_location) {
                println!("Replacing app {app_id:?} from {old_location:?} to {location:?}");
                summon_area = app.rect(ui);
                app.mark_removal();
            }
        } else {
            println!("Opening app {app_id:?} at {location:?}");
        }

        let panel = AppPanel::new(ui, app_id.clone(), summon_area);

        match location {
            OpenAppLocation::Existing(existing) => {
                self.set_app(existing, panel);
            }
            OpenAppLocation::NewWidget(widget) => {
                if widget > self.widgets.len() {
                    self.widgets.push(panel);
                } else {
                    self.widgets.insert(widget, panel);
                }
            }
            OpenAppLocation::FirstWidget => {
                self.widgets.push(panel);
            }
        }
    }

    pub fn set_app(&mut self, location: AppLocation, app_panel: AppPanel) {
        match location {
            AppLocation::Primary => {
                self.primary = Some(app_panel);
            }
            AppLocation::Widget(widget) => match self.widgets.get_mut(widget) {
                None => {
                    if widget > self.widgets.len() {
                        self.widgets.push(app_panel);
                    } else {
                        self.widgets.insert(widget, app_panel);
                    }
                }
                Some(value) => {
                    *value = app_panel;
                }
            },
        }
    }

    pub fn get_app_mut(&mut self, location: AppLocation) -> Option<&mut AppPanel> {
        match location {
            AppLocation::Primary => self.primary.as_mut(),
            AppLocation::Widget(widget) => self.widgets.get_mut(widget),
        }
    }

    pub fn get_app(&self, location: AppLocation) -> Option<&AppPanel> {
        match location {
            AppLocation::Primary => self.primary.as_ref(),
            AppLocation::Widget(widget) => self.widgets.get(widget),
        }
    }

    pub fn finalize(&mut self) {
        self.widgets.drain_filter(|v| v.for_removal());
        if let Some(primary) = &mut self.primary {
            if primary.for_removal() {
                self.primary = None;
            }
        }
    }
}
