#![allow(dead_code)]

use std::ops::{Deref, DerefMut};

use ratatui::{prelude::Rect, widgets::*};

use super::Frame;

#[derive(Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub scroll_state: ScrollbarState,
    pub items: Vec<T>,

    loops: bool,
}

impl<T> Deref for StatefulList<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl<T> DerefMut for StatefulList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            scroll_state: ScrollbarState::default(),
            items: Vec::new(),
            loops: false,
        }
    }
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            scroll_state: ScrollbarState::default(),
            items,
            loops: false,
        }
    }
    pub fn replace(&mut self, items: Vec<T>) {
        self.items = items;
        if self.state.selected().is_none() {
            self.first();
        }
    }

    pub fn set_scrollloop(&mut self, scrollloop: bool) {
        self.loops = scrollloop;
    }

    pub fn next(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    if self.loops {
                        0
                    } else {
                        len - 1
                    }
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i as u16);
    }

    pub fn previous(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    if self.loops {
                        self.items.len() - 1
                    } else {
                        0
                    }
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i as u16);
    }

    pub fn first(&mut self) {
        self.state.select(Some(0));
        self.scroll_state = self.scroll_state.position(0_u16);
    }
    pub fn last(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        self.state.select(Some(len - 1));
        self.scroll_state = self.scroll_state.position((len - 1) as u16);
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn get(&self) -> Option<&T> {
        self.items.get(self.state.selected()?)
    }

    pub fn render(
        &mut self,
        f: &mut Frame,
        area: Rect,
        list_cb: impl Fn(&[T]) -> List,
        scrollbar: Option<Scrollbar>,
    ) {
        f.render_stateful_widget(list_cb(&self.items), area, &mut self.state);
        if let Some(scroll) = scrollbar {
            self.scroll_state = self.scroll_state.content_length(self.items.len() as u16);
            f.render_stateful_widget(scroll, area, &mut self.scroll_state);
        }
    }
}
