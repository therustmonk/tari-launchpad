mod message;

use crate::component::{Component, ComponentEvent, Frame, Input, Pass};
use crate::state::onboarding::OnboardingAction;
use crate::state::{AppState, Focus};
use message::MessageWidget;
use std::time::{Duration, Instant};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Gauge, Paragraph};

pub struct OnboardingScene {
    wink: Option<Instant>,
}

impl OnboardingScene {
    pub fn new() -> Self {
        Self {
            wink: Some(Instant::now()),
        }
    }
}

impl Input for OnboardingScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if let Some(wink) = self.wink {
            if wink.elapsed() >= Duration::from_secs(5) {
                self.wink.take();
                state.redraw();
            }
        } else {
            self.wink = Some(Instant::now());
            state.redraw();
        }

        match event.pass() {
            Pass::Enter => {
                state.bus.send(OnboardingAction::Next);
            }
            Pass::Leave => {
                state.focus_on(Focus::Root);
            }
            _ => {}
        }
    }
}

impl OnboardingScene {
    fn get_progress(&self, _state: &AppState) -> u16 {
        100
    }
}

impl<B: Backend> Component<B> for OnboardingScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(rect);
        let constraints = [
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(h_chunks[1]);

        let constraints = [Constraint::Min(0), Constraint::Length(1)];
        let view_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(v_chunks[1]);

        let msg = state.bus.state().onboarding.messages.last().cloned();
        let message = MessageWidget::new(msg);
        message.draw(f, view_chunks[0], state);

        let constraints = [
            Constraint::Min(0),
            Constraint::Length(5),
            Constraint::Length(5),
        ];
        let line_chinks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(view_chunks[1]);

        let gauge = Gauge::default()
            .label("")
            .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Reset))
            .percent(self.get_progress(state));
        f.render_widget(gauge, line_chinks[0]);
        let style = Style::default().fg(Color::White);
        let bot_state = if self.wink.is_some() {
            "[o o]"
        } else {
            "[- -]"
        };
        let text = vec![Spans::from(Span::styled(bot_state, style))];
        let bot = Paragraph::new(text);
        f.render_widget(bot, line_chinks[2]);
    }
}
