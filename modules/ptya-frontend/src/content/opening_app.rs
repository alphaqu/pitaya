use crate::content::app::AppLocation;
use crate::content::{ContentApps, ContentPanel};
use egui::{pos2, Color32, Id, LayerId, Mesh, Order, Pos2, Rect, Response, Stroke, TextureId, Ui, Vec2, Rounding, Align2, FontId};
use log::info;
use ptya_common::app::AppId;
use ptya_common::color::color::{ColorState, ColorType};
use ptya_common::settings::SPACING_SIZE;
use ptya_common::ui::animation::state::State;
use ptya_common::ui::animation::transition::Transition;
use ptya_common::System;

#[derive(Debug, Copy, Clone, Hash)]
pub enum OpenAppLocation {
    Existing(AppLocation),
    NewWidget(usize),
    // To prevent hash collision where the animation freaks out because the it becomes the same
    // FirstWidget means going from primary only to a single widget featured
    FirstWidget,
}

// Dragging an app summons this
pub struct DraggingApp {
    pub pos: Pos2,
    pub origin: Rect,
    pub rect: Transition<Rect>,
    pub state: State,

    pub released: bool,
    pub just_released: bool,

    pub app: AppId,
    pub new_location: Option<OpenAppLocation>,
}

impl DraggingApp {
    pub fn new(ui: &Ui, rect: Rect, app: AppId) -> DraggingApp {
        let id = app.egui_id().with("drag");
        let mut app = DraggingApp {
            pos: rect.center(),
            origin: rect,
            rect: Transition {
                state: State::basic(id.with("rect-state"), ui),
                from: rect,
                to: rect,
            },
            state: State::basic(id.with("state"), ui),
            released: false,
            just_released: false,
            app,
            new_location: None,
        };
        app.rect.state.reset_target(ui, 0.0, 1.0);
        app.state.reset_target(ui, 0.0, 1.0);

        app
    }

    pub fn tick_response(&mut self, ui: &mut Ui, response: &Response) {
        if response.drag_released() {
            self.released = true;
            self.just_released = true;
            self.state.set_target(ui, 0.0);
        }

        if !self.released {
            self.pos += response.drag_delta();
        }
    }

    pub fn update(
        &mut self,
        ui: &mut Ui,
        system: &System,
        apps: &mut ContentApps,
        content_rect: Rect,
    ) {
        if self.just_released {
            if let Some(location) = self.new_location {
                let area = self.rect.get(ui);
                let min = area.min.clamp(content_rect.min, content_rect.max);
                let max = area.max.clamp(content_rect.min, content_rect.max);
                let area = Rect::from_min_max(min, max);
                apps.open_app(ui, area, &self.app, location);
                // Make the app drag overlay scale down.
                self.rect
                    .set(ui, Rect::from_center_size(area.center(), Vec2::new(100.0, 100.0)));
            } else {
                self.rect.set(ui, self.origin);
            }
        }

        // If its not released yet, bind the overlay to the cursor.
        if !self.released {
            let size = Vec2::new(
                self.state.lerp(ui, &100.0, &200.0),
                self.state.lerp(ui, &100.0, &200.0),
            );
            self.rect.reset(ui, Rect::from_center_size(self.pos, size));
        }

        let rect = self.rect.get(ui);
        let painter = ui
            .ctx()
            .layer_painter(LayerId::new(Order::Tooltip, Id::null()));

        let state = ColorState::Idle;
        painter.rect(
            rect,
            Rounding::same(32.0),
            self.state
                .lerp(ui, &Color32::TRANSPARENT, &system.color.bg(5.0, ColorType::Secondary, state)),
            Stroke::none(),
        );
        {
            //let app = system.app.get_app(&self.app);
            //let texture_id = app.icon_handle.id();
            let size = rect.size().min(Vec2::new(60.0, 60.0));
            let icon = Rect::from_center_size(rect.center(), size);
         //  let mut mesh = Mesh::with_texture(texture_id);
         //  let color = self.state.lerp(ui, &Color32::TRANSPARENT, &system.color.fg(ColorType::Neutral, state));
         //  mesh.add_rect_with_uv(
         //      icon,
         //      Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
         //      color,
         //  );
         //  painter.add(mesh);

            let left_space = 60.0 + (SPACING_SIZE * 2.0);

          //  painter.text(rect.left_top() + Vec2::new(left_space, side_space), Align2::LEFT_CENTER, &app.info.name, FontId::proportional(30.0), color);
        }

        self.just_released = false;
    }

    pub fn draw_possible_placement(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        hidden: bool,
        location: OpenAppLocation,
        system: &System,
    ) -> bool {
        // primary_empty
        let painter = ui
            .ctx()
            .layer_painter(LayerId::new(Order::Tooltip, Id::null()));

        let id = ui.id().with("possible_placement").with(location);

        let touches = rect
            .expand(SPACING_SIZE / 2.0)
            .contains(self.pos);

        let mut hover_state = State::new(id, if touches && !self.released { 1.0 } else { 0.0 }, 1.0, ui);
        let color = system.color.bg(5.0, ColorType::Primary, ColorState::Idle);

        let from_color = if !hidden {
            color.linear_multiply(0.5)
        } else {
            Color32::TRANSPARENT
        };

        let to_color = if !hidden {
            color.linear_multiply(0.8)
        } else {
            color
        };

        let fill_color = hover_state.lerp(ui, &from_color, &to_color);
        let fill_color = self.state.lerp(ui, &Color32::TRANSPARENT, &fill_color);
        painter.rect(rect, Rounding::same(16.0), fill_color, Stroke::none());

        if touches && self.just_released {
            self.state.set_target(ui, 0.0);
            self.new_location = Some(location);
        }

        touches && !self.released
    }

    pub fn for_removal(&mut self, ui: &Ui) -> bool {
        self.released && self.state.get_progress(ui) == 0.0
    }
}
