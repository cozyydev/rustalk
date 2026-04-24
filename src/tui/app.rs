use crate::tui::message::UiMessage;

pub struct App {
    pub addr: String,
    pub nick: String,
    pub messages: Vec<UiMessage>,
    pub input: String,
    pub should_quit: bool,
    pub scroll: u16,
    pub auto_scroll: bool,
    pub online_count: Option<usize>,
    pub online_users: Vec<String>,
}

impl App {
    pub fn new(addr: String, nick: String) -> Self {
        Self {
            addr,
            nick,
            messages: Vec::new(),
            input: String::new(),
            should_quit: false,
            scroll: 0,
            auto_scroll: true,
            online_count: None,
            online_users: Vec::new(),
        }
    }

    pub fn push_message(&mut self, msg: UiMessage) {
        self.messages.push(msg);

        if self.messages.len() > 1000 {
            let drain_count = self.messages.len() - 1000;
            self.messages.drain(0..drain_count);
        }
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.auto_scroll = false;
        self.scroll = self.scroll.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: u16, max_scroll: u16) {
        self.scroll = self.scroll.saturating_add(amount).min(max_scroll);
        if self.scroll >= max_scroll {
            self.scroll = max_scroll;
            self.auto_scroll = true;
        }
    }

    pub fn scroll_to_bottom(&mut self, max_scroll: u16) {
        self.scroll = max_scroll;
        self.auto_scroll = true;
    }

    pub fn clamp_scroll(&mut self, max_scroll: u16) {
        if self.auto_scroll {
            self.scroll = max_scroll;
        } else {
            self.scroll = self.scroll.min(max_scroll);
        }
    }
}
