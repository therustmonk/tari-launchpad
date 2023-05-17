use crate::component::elements::block_with_title;
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::onboarding::Message;
use crate::state::AppState;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::widgets::{Paragraph, Wrap};

pub struct MessageWidget {
    msg: Option<Message>,
}

impl MessageWidget {
    pub fn new(msg: Option<Message>) -> Self {
        Self { msg }
    }
}

impl Input for MessageWidget {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for MessageWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let block = block_with_title(None, false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .horizontal_margin(3)
            .vertical_margin(1)
            .split(inner_rect);
        let text = self
            .msg
            .as_ref()
            .map(|msg| msg.text.as_ref())
            .unwrap_or("...");
        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        f.render_widget(paragraph, chunks[0]);
    }
}
