use crate::tui::message::{SystemKind, UiMessage};

pub fn parse_message(line: String) -> UiMessage {
    if line.starts_with("Error: ") {
        return UiMessage::Error(line);
    }

    if line == "[disconnected]" {
        return UiMessage::System(SystemKind::Disconnected, line);
    }

    if line.starts_with("[read error]") {
        return UiMessage::Error(line);
    }

    if line.starts_with("Online (") {
        return UiMessage::System(SystemKind::Online, line);
    }

    if line.ends_with(" has joined the room") {
        return UiMessage::System(SystemKind::Join, line);
    }

    if line.ends_with(" has left the room") {
        return UiMessage::System(SystemKind::Leave, line);
    }

    if line == "Connected." {
        return UiMessage::System(SystemKind::Connected, line);
    }

    if line == "Type messages and press Enter."
        || line == "Commands: /who, /quit"
        || line == "Press Esc to quit."
    {
        return UiMessage::System(SystemKind::Info, line);
    }

    if let Some((from, body)) = line.split_once(": ") {
        if !from.is_empty() && !body.is_empty() {
            return UiMessage::Chat {
                from: from.to_string(),
                body: body.to_string(),
            };
        }
    }

    UiMessage::Raw(line)
}

pub fn parse_online_line(line: &str) -> Option<(usize, Vec<String>)> {
    let rest = line.strip_prefix("Online (")?;
    let (count_part, users_part) = rest.split_once("): ")?;
    let count = count_part.parse::<usize>().ok()?;

    let users = if users_part.trim().is_empty() {
        Vec::new()
    } else {
        users_part
            .split(", ")
            .map(|name| name.to_string())
            .collect::<Vec<_>>()
    };

    Some((count, users))
}
