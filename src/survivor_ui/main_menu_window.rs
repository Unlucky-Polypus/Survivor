use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets};

use crate::survivor_ui::window::{Action, Window};

const WINDOW_WIDTH: f32 = 300.;
const WINDOW_HEIGHT: f32 = 400.;
const WINDOW_POSITION: Vec2 = Vec2::new(800. / 2. - WINDOW_WIDTH / 2., 600. / 2. - WINDOW_HEIGHT / 2.);

pub(crate) struct MainMenuWindow {
    pub(crate) window: Window,
}

impl MainMenuWindow {
    pub(crate) async fn new() -> Self {
        Self {
            window: Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_POSITION).await,
        }
    }

    pub(crate) fn draw(&self) -> Option<Action> {
        root_ui().push_skin(&self.window.skin);
        
        root_ui().same_line(0.);
        
        let mut action: Option<Action> = None;

        root_ui().window(
            hash!(), self.window.position, vec2(self.window.width, 
                self.window.height), 
                |ui| {
            if widgets::Button::new("Play")
            .position(vec2(65.0, 15.0))
            .ui(ui) {
                action = Some(Action::Play);
            }
            if widgets::Button::new("Options")
            .position(vec2(40.0, 75.0))
            .ui(ui) {
                action = Some(Action::Options);
            }
            if widgets::Button::new("Quit")
            .position(vec2(65.0, 195.0))
            .ui(ui) {
                action = Some(Action::Quit);
            }
        });
        action
    }
}
