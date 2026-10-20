use std::{io, time::Duration};

use ratatui::layout::Alignment;
use ratatui::prelude::StatefulWidget;
use ratatui::text::{Line, Span, Text};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect, Size},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};
use std::hash::Hash;
use std::hash::Hasher;
use tui_scrollview::{ScrollView, ScrollViewState};

struct BacklogMessage {
    timestamp: &'static str,
    sigil: &'static str,
    nick: &'static str,
    message: &'static str,
}

impl BacklogMessage {
    fn new(
        timestamp: &'static str,
        sigil: &'static str,
        nick: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            timestamp,
            sigil,
            nick,
            message,
        }
    }
}

struct BacklogEvent {
    timestamp: &'static str,
    nick: &'static str,
    content: &'static str,
}

impl BacklogEvent {
    fn new(timestamp: &'static str, nick: &'static str, content: &'static str) -> Self {
        Self {
            timestamp,
            nick,
            content,
        }
    }
}

enum BacklogEntry {
    BacklogMessage(BacklogMessage),
    BacklogEvent(BacklogEvent),
}

impl BacklogEntry {
    fn message(
        timestamp: &'static str,
        sigil: &'static str,
        nick: &'static str,
        message: &'static str,
    ) -> Self {
        Self::BacklogMessage(BacklogMessage::new(timestamp, sigil, nick, message))
    }

    fn event(timestamp: &'static str, nick: &'static str, content: &'static str) -> Self {
        Self::BacklogEvent(BacklogEvent::new(timestamp, nick, content))
    }
}

fn messages() -> Vec<BacklogEntry> {
    vec![
        BacklogEntry::message(
            "06:35",
            "",
            "john",
            "don't think building space ships and submarines is illegal",
        ),
        BacklogEntry::message(
            "06:36",
            "",
            "john",
            "even an ICBM is probably legal as long as u don't attempt to make a warhead or sell the plans to enemy states",
        ),
        BacklogEntry::message("06:55", "+", "whitt", "*vomits*"),
        BacklogEntry::message(
            "06:55",
            "+",
            "vomitchan",
            " ≨ 鹭 䖇^哮^㺯⋕∤鋇⊘玷⊽^⊸^⋒囟≽ https://some.com/link.txt  嬈 ^⋈䙇⋟㢗^⊨≎囟^⋖^蔰㲷晧㔇⋕^⊂^",
        ),
        BacklogEntry::message("06:56", "%", "masser", "what are good halloween movies?"),
        BacklogEntry::message("07:12", "", "quad", "that one"),
        BacklogEntry::message("07:12", "", "quad", "john: there it's a national sport"),
        BacklogEntry::message(
            "07:13",
            "+",
            "radptr",
            "according to a random list I found online, the first 3 are pretty good",
        ),
        BacklogEntry::message("07:25", "%", "masser", "i mean movies for holloween"),
        BacklogEntry::message("07:25", "%", "masser", "not the holloween franchise itself"),
    ]
    /*
    vec![
        BacklogEntry::message(
            "04:47",
            "@",
            "MinceR",
            "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?",
        ),
        BacklogEntry::message(
            "04:47",
            "",
            "cns",
            "MinceR: irssi vagy weechat uit preferalod? esetleg mindketto? vagy hide buflist/nicklist ha nincs eleg hely?",
        ),
        BacklogEntry::message(
            "04:48",
            "@",
            "MinceR",
            "nekem a weechat ui nagy reszt jobban tetszett, de vegul meg is az irssinel maradtam",
        ),
        BacklogEntry::message(
            "04:48",
            "@",
            "MinceR",
            "a buflist es a nicklist is tetszett, bar a fuggoleges buflistben nem sok mindent latni egyszerre",
        ),
        BacklogEntry::message(
            "04:48",
            "@",
            "MinceR",
            "viszont az is a ui resze, hogy egy halozat serverlistaja egy vesszovel elvalasztott lista egy config stringben, es az annyira nem jott be",
        ),
        BacklogEntry::message("04:48", "", "cns", "megcsinalom a main view mockupot"),
        BacklogEntry::message("04:49", "", "cns", "toml lesz a config sztem"),
        BacklogEntry::message("04:49", "", "cns", "vagy hocon"),
        BacklogEntry::message(
            "04:49",
            "@",
            "MinceR",
            "az meg meg a weechatbol is hianyzott, hogy hierarchikus legyen a buflist, tehat hogy a network ala legyen rendelve a channel",
        ),
        BacklogEntry::message("04:49", "", "cns", "az lesz ofc"),
        BacklogEntry::message("04:49", "", "cns", "nem tudom miert nem olyan weechatben"),
        BacklogEntry::message(
            "04:49",
            "@",
            "MinceR",
            "gondolom, tul kozelrol kovettek az irssit",
        ),
        BacklogEntry::message(
            "04:50",
            "@",
            "cns",
            "arra gondoltam meg hogy lehetne egy overview screen ahol kiirja az osszes buffert + hany olvasatlan uzeneted van",
        ),
        BacklogEntry::message(
            "04:50",
            "@",
            "MinceR",
            "az hatarozottan elonyos a weechatben, hogy ujra tudja formazni az egesz buffert, pl. ha theme-et valtasz",
        ),
        BacklogEntry::message("04:50", "@", "MinceR", "az irssi nem tud ilyet"),
        BacklogEntry::message(
            "04:50",
            "",
            "cns",
            "es akkor egyszeruen atvaltasz arra binddel",
        ),
        BacklogEntry::message("04:50", "@", "MinceR", "jol hangzik"),
    ]
     */
}

struct State {}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut state = State {};

    loop {
        terminal.draw(|frame| render(frame, &mut state))?;

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    ratatui::run(run)
}

fn render(frame: &mut Frame, state: &mut State) {
    let block = Block::bordered()
        .title(" Welcome to #somechannel. ")
        .borders(Borders::ALL)
        .padding(Padding::uniform(1));

    let inner = block.inner(frame.area());

    let mut scroll_view = ScrollView::new(Size::new(inner.width, inner.height));
    let mut scroll_state = ScrollViewState::default();

    let paragraphs: Vec<_> = messages()
        .iter()
        .map(|message| match message {
            BacklogEntry::BacklogMessage(message) => {
                let content_width = inner
                    .width
                    .saturating_sub(message.sigil.len() as u16)
                    .saturating_sub(message.nick.len() as u16)
                    .saturating_sub(message.timestamp.len() as u16)
                    .saturating_sub(3);

                let timestamp = Paragraph::new(message.timestamp);
                let sigil = message.sigil;
                let nick = message.nick;
                let message = Paragraph::new(message.message).wrap(Wrap { trim: false });

                let height = message
                    .line_count(content_width)
                    .clamp(1, u16::MAX as usize) as u16;

                (timestamp, sigil, nick, message, content_width, height)
            }
            BacklogEntry::BacklogEvent(event) => {
                let content_width = inner.width.saturating_sub(1);

                let timestamp = Paragraph::new(event.timestamp);
                let content = Paragraph::new(event.content).wrap(Wrap { trim: false });
                let height = content
                    .line_count(content_width)
                    .clamp(1, u16::MAX as usize) as u16;

                (timestamp, "-!-", event.nick, content, content_width, height)
            }
        })
        .collect();

    let max_nick_len = messages().iter().fold(0u16, |accum, item| match item {
        BacklogEntry::BacklogMessage(item) => {
            let styling_len = 3 + item.sigil.len() as u16;
            let curr_len = item.nick.len() as u16 + styling_len;
            if curr_len > accum { curr_len } else { accum }
        }
        BacklogEntry::BacklogEvent(item) => {
            let styling_len = 3u16;
            let curr_len = item.nick.len() as u16 + styling_len;
            if curr_len > accum { curr_len } else { accum }
        }
    });

    let mut y = 0_u16;
    for (timestamp, sigil, nick, message, width, height) in paragraphs {
        let layout = Layout::horizontal(vec![
            Constraint::Length(6),
            Constraint::Length((nick.len() + sigil.len() + 3) as u16),
            Constraint::Fill(1),
        ])
        .split(Rect::new(0, y, width, height));

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        nick.hash(&mut hasher);
        let hash = hasher.finish() as usize;

        let colors = [
            Color::Cyan,
            Color::Red,
            Color::Yellow,
            Color::Green,
            Color::Magenta,
        ];

        let color = colors[hash % colors.len()];

        let sigil = Span::styled(sigil, Style::default().fg(Color::White));

        scroll_view.render_widget(timestamp, layout[0]);
        scroll_view.render_widget(
            Paragraph::new(Text::from(vec![Line::from(vec![
                Span::raw("<"),
                sigil,
                Span::styled(nick, Style::default().fg(color).bold()),
                Span::raw("> "),
            ])]))
            .alignment(Alignment::Right),
            layout[1],
        );
        scroll_view.render_widget(message, layout[2]);

        y = y.saturating_add(height);
    }

    scroll_view.render(inner, frame.buffer_mut(), &mut scroll_state);
    frame.render_widget(block, frame.area());
}
