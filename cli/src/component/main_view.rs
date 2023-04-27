use crate::component::mode::ModeSelector;
use crate::component::scene;
use crate::component::tabs::{AppTab, AppTabs};
use crate::component::{Component, Focus, Input};
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct MainView {
    tabs: AppTabs<AppTab>,
    mode_selector: ModeSelector,
    containers_scene: scene::Containers,
    wallet_scene: scene::Wallet,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            tabs: AppTabs::new(),
            mode_selector: ModeSelector::new(),
            containers_scene: scene::Containers::new(),
            wallet_scene: scene::Wallet::new(),
        }
    }
}

impl Input for MainView {
    fn on_input(&mut self, key: KeyCode) -> Option<Focus> {
        self.tabs.on_input(key);
        match self.tabs.selected()? {
            AppTab::Containers => {}
            AppTab::Wallet => {}
        }
        None
    }
}

impl<B: Backend> Component<B> for MainView {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ];
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.mode_selector.draw(f, main_chunks[0]);
        self.tabs.draw(f, main_chunks[1]);
        match self.tabs.selected() {
            Some(AppTab::Containers) => {
                self.containers_scene.draw(f, main_chunks[2]);
            }
            Some(AppTab::Wallet) => {
                self.wallet_scene.draw(f, main_chunks[2]);
            }
            None => {}
        }
    }
}
