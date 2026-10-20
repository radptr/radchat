use std::{io, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Bar, BarChart, Block, Borders, Gauge, LineGauge, Padding, Paragraph, canvas::Label},
};

const TITLES: [&str; 3] = [
    "Something.Short.rar",
    "Something.Really.Long.And.Radical.tar",
    "SOmething.Even.Longer.And.Even.More.Radical.Than.The.Last.Radical.Thing.mkv",
];

const ENTRIES: usize = TITLES.len();

fn main() -> io::Result<()> {
    ratatui::run(run)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScrollDirection {
    Forward,
    Backward,
}

struct State {
    scroll_offsets: Vec<usize>,
    scroll_direction: Vec<ScrollDirection>,
    scroll_pause: Vec<usize>,
    last_area: Option<Rect>,
    download_progress: Vec<u64>,
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut state = State {
        scroll_offsets: vec![0; ENTRIES],
        scroll_direction: vec![ScrollDirection::Forward; ENTRIES],
        scroll_pause: vec![0; ENTRIES],
        last_area: None,
        download_progress: vec![0; ENTRIES],
    };

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

fn render(frame: &mut Frame, state: &mut State) {
    let mut tform_titles: Vec<String> = Vec::with_capacity(ENTRIES);
    let max_length = (usize::from(frame.area().width) / 5).max(1);
    // let max_length = if max_length < 40 { 40 } else { max_length };

    if let Some(area) = state.last_area
        && area != frame.area()
    {
        state.scroll_offsets = vec![0; ENTRIES];
        state.scroll_direction = vec![ScrollDirection::Forward; ENTRIES];
        state.scroll_pause = vec![0; ENTRIES];
    }

    for progress in &mut state.download_progress {
        *progress += rand::random_range(0..=3);

        if *progress > 100 {
            *progress = 100;
        }
    }

    for (i, title) in TITLES.iter().enumerate() {
        if title.chars().count() > max_length {
            let tform: String = title
                .chars()
                .skip(state.scroll_offsets[i])
                .take(max_length)
                .collect();

            tform_titles.push(tform);

            if state.scroll_pause[i] > 0 {
                state.scroll_pause[i] -= 1;
                continue;
            }

            let delta = title.len() - max_length;

            if state.scroll_direction[i] == ScrollDirection::Forward {
                state.scroll_offsets[i] += 1;
            } else {
                state.scroll_offsets[i] -= 1;
            }

            if state.scroll_offsets[i] == delta {
                state.scroll_direction[i] = ScrollDirection::Backward;
                state.scroll_pause[i] = 5;
                continue;
            } else if state.scroll_offsets[i] == 0 {
                state.scroll_direction[i] = ScrollDirection::Forward;
                state.scroll_pause[i] = 5;
                continue;
            }
        } else {
            tform_titles.push(title.to_string());
        }
    }

    state.last_area = Some(frame.area());

    let block = Block::bordered()
        .title(" XDCC Transfer ")
        .borders(Borders::ALL)
        .padding(Padding::uniform(1));

    let columns = Layout::horizontal([Constraint::Length(max_length as u16), Constraint::Fill(1)])
        .split(block.inner(frame.area()));

    let title_rows = Layout::vertical([1; ENTRIES]).spacing(1).split(columns[0]);

    let gauge_rows = Layout::vertical([1; ENTRIES])
        .horizontal_margin(2)
        .spacing(1)
        .split(columns[1]);

    for i in 0..ENTRIES {
        frame.render_widget(Paragraph::new(tform_titles[i].as_str()), title_rows[i]);

        frame.render_widget(
            LineGauge::default()
                .ratio((state.download_progress[i] as f64) / 100.0)
                .filled_style(Style::new().green().on_black().bold())
                .filled_symbol(ratatui::symbols::line::THICK_HORIZONTAL)
                .label(format!("{:2}% ", state.download_progress[i])),
            gauge_rows[i],
        );
    }

    frame.render_widget(block, frame.area());
}
