use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::{
    app::App,
    message::{SystemKind, UiMessage},
    theme::Theme,
};

pub fn draw_ui(frame: &mut Frame, app: &App, theme: &Theme) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let message_area_height = areas[1].height;
    let max_scroll = max_scroll(app.messages.len(), message_area_height);
    let scroll = if app.auto_scroll {
        max_scroll
    } else {
        app.scroll.min(max_scroll)
    };

    let messages_text = if app.messages.is_empty() {
        vec![Line::from("")]
    } else {
        app.messages
            .iter()
            .map(|msg| render_message(msg, theme, &app.nick))
            .collect::<Vec<_>>()
    };

    let header = build_header(app, theme);
    let messages = Paragraph::new(messages_text)
        .style(Style::default().bg(theme.bg))
        .block(
            Block::default()
                .title(Span::styled(
                    " Chat ",
                    Style::default()
                        .fg(theme.title)
                        .bg(theme.bg)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.panel_border)),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(theme.text).bg(theme.bg))
        .block(
            Block::default()
                .title(Span::styled(
                    " Input ",
                    Style::default()
                        .fg(theme.title)
                        .bg(theme.bg)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.input_border)),
        );

    let help = Paragraph::new(build_footer(theme)).style(Style::default().bg(theme.bg));

    frame.render_widget(
        Block::default().style(Style::default().bg(theme.bg)),
        frame.area(),
    );
    frame.render_widget(header, areas[0]);
    frame.render_widget(messages, areas[1]);
    frame.render_widget(input, areas[2]);
    frame.render_widget(help, areas[3]);

    let input_width = areas[1].width.saturating_sub(2);
    let input_len = app.input.chars().count() as u16;
    let cursor_x = areas[2]
        .x
        .saturating_add(1)
        .saturating_add(input_len.min(input_width.saturating_sub(1)));
    let cursor_y = areas[2].y.saturating_add(1);

    frame.set_cursor_position((cursor_x, cursor_y));
}

fn build_header<'a>(app: &App, theme: &Theme) -> Paragraph<'a> {
    let online = app
        .online_count
        .map(|count| count.to_string())
        .unwrap_or_else(|| "?".to_string());

    let mode_label = if app.auto_scroll { "LIVE" } else { "SCROLLED" };
    let mode_color = if app.auto_scroll {
        theme.status
    } else {
        theme.timestamp
    };

    Paragraph::new(Line::from(vec![
        Span::styled(
            " server ",
            Style::default()
                .fg(theme.header_bg)
                .bg(theme.header_accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {}  ", app.addr),
            Style::default().fg(theme.header_fg).bg(theme.header_bg),
        ),
        Span::styled(
            " handle: ",
            Style::default()
                .fg(theme.header_bg)
                .bg(theme.status)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {}  ", app.nick),
            Style::default().fg(theme.header_fg).bg(theme.header_bg),
        ),
        Span::styled(
            " online: ",
            Style::default()
                .fg(theme.header_bg)
                .bg(theme.system)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {}  ", online),
            Style::default().fg(theme.header_fg).bg(theme.header_bg),
        ),
        Span::styled(
            " mode: ",
            Style::default()
                .fg(theme.header_bg)
                .bg(mode_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {} ", mode_label),
            Style::default().fg(theme.header_fg).bg(theme.header_bg),
        ),
    ]))
    .style(Style::default().bg(theme.header_bg))
}

fn build_footer(theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            "Enter",
            Style::default()
                .fg(theme.status)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("=send  ", Style::default().fg(theme.muted)),
        Span::styled(
            "Esc",
            Style::default()
                .fg(theme.error)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("=/quit  ", Style::default().fg(theme.muted)),
        Span::styled(
            "↑↓",
            Style::default()
                .fg(theme.timestamp)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("=scroll  ", Style::default().fg(theme.muted)),
        Span::styled(
            "PgUp/PgDn",
            Style::default()
                .fg(theme.timestamp)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("=page  ", Style::default().fg(theme.muted)),
        Span::styled(
            "End",
            Style::default()
                .fg(theme.status)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("=bottom  ", Style::default().fg(theme.muted)),
        Span::styled("/who /quit", Style::default().fg(theme.system)),
    ])
}

pub fn render_message(msg: &UiMessage, theme: &Theme, my_nick: &str) -> Line<'static> {
    match msg {
        UiMessage::System(kind, body) => render_system_message(*kind, body, theme),
        UiMessage::Error(body) => Line::from(vec![
            Span::styled(
                "!! ",
                Style::default()
                    .fg(theme.error)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(body.clone(), Style::default().fg(theme.error)),
        ]),
        UiMessage::Chat { from, body } => {
            let name_color = if from == my_nick {
                theme.self_name
            } else {
                theme.other_name
            };

            Line::from(vec![
                Span::styled(
                    from.clone(),
                    Style::default().fg(name_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(": ", Style::default().fg(theme.muted)),
                Span::styled(body.clone(), Style::default().fg(theme.text)),
            ])
        }
        UiMessage::Raw(body) => {
            Line::from(Span::styled(body.clone(), Style::default().fg(theme.text)))
        }
    }
}

fn render_system_message(kind: SystemKind, body: &str, theme: &Theme) -> Line<'static> {
    let (prefix, color) = match kind {
        SystemKind::Info => ("• ", theme.system),
        SystemKind::Join => ("+ ", theme.status),
        SystemKind::Leave => ("- ", theme.timestamp),
        SystemKind::Online => ("@ ", theme.system),
        SystemKind::Connected => ("* ", theme.status),
        SystemKind::Disconnected => ("x ", theme.error),
    };

    Line::from(vec![
        Span::styled(
            prefix,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(body.to_string(), Style::default().fg(color)),
    ])
}

pub fn max_scroll(message_count: usize, height: u16) -> u16 {
    let visible_lines = height.saturating_sub(2) as usize;

    message_count
        .saturating_sub(visible_lines)
        .try_into()
        .unwrap_or(u16::MAX)
}
