mod elements;
mod expert;
mod header;
mod main_view;
mod normal;
mod scene;
mod settings;
mod tabs;

pub use main_view::MainView;

use crate::state::AppState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub trait Component<B: Backend> {
    type State;

    /// A context reference a mutable to modify the frame.
    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State);
}

#[derive(Debug, Clone, Copy)]
pub enum MoveFocus {
    /// Entering into a component.
    In,
    /// Exiting out of a component.
    Out,
    Up,
    Down,
    Next,
    Prev,
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentEvent {
    Key(KeyEvent),
    Focus,
}

pub trait Input {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<MoveFocus>;

    fn focus(&mut self, state: &mut AppState) {
        self.on_event(ComponentEvent::Focus, state);
    }
}
