use crate::component::elements::block_with_title;
use crate::component::{Component, ComponentEvent, Frame, Input, MoveFocus};
use crate::state::AppState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;

pub struct Wallet {}

impl Wallet {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for Wallet {
    fn on_event(&mut self, _event: ComponentEvent, state: &mut AppState) -> Option<MoveFocus> {
        None
    }
}

impl<B: Backend> Component<B> for Wallet {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let block = block_with_title(Some("Wallet"), false);
        f.render_widget(block, rect);
    }
}
