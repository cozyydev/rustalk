#[derive(Debug, Clone)]
pub enum UiMessage {
    System(SystemKind, String),
    Error(String),
    Chat { from: String, body: String },
    Raw(String),
}

#[derive(Debug, Clone, Copy)]
pub enum SystemKind {
    Info,
    Join,
    Leave,
    Online,
    Connected,
    Disconnected,
}
