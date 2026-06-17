use std::io::{self, stdout};

use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

use macmon_graph::{ColorGradient, ColorGraph, Direction, GradientTheme, RingBuffer, SymbolSet};

fn sample_wave(len: usize) -> Vec<f64> {
    (0..len)
        .map(|i| {
            let t = i as f64;
            let v = 50.0 + 35.0 * (t * 0.12).sin() + 15.0 * (t * 0.37).cos();
            v.clamp(0.0, 100.0)
        })
        .collect()
}

fn sample_spiky(len: usize) -> Vec<f64> {
    (0..len)
        .map(|i| {
            let t = i as f64;
            let base = 30.0 + 20.0 * (t * 0.08).sin();
            let spike = if i % 7 == 0 { 40.0 } else { 0.0 };
            (base + spike).clamp(0.0, 100.0)
        })
        .collect()
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;

    let wave = sample_wave(80);
    let spiky = sample_spiky(80);

    // Also build a RingBuffer to show the API
    let mut ring = RingBuffer::new(80);
    for &v in wave.iter().rev() {
        ring.push(v);
    }

    term.draw(|f| {
        let rows = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(f.area());

        // 1. Braille + CPU gradient (green→yellow→red)
        let g1 = ColorGraph::new()
            .data(ring.as_slice())
            .gradient(ColorGradient::from_theme(GradientTheme::Cpu))
            .symbol_set(SymbolSet::Braille)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(" Braille · CPU (green→yellow→red) "),
            );
        f.render_widget(g1, rows[0]);

        // 2. Block + Temp gradient (blue→purple→pink)
        let g2 = ColorGraph::new()
            .data(&wave)
            .gradient(ColorGradient::from_theme(GradientTheme::Temp))
            .symbol_set(SymbolSet::Block)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(" Block · Temp (blue→purple→pink) "),
            );
        f.render_widget(g2, rows[1]);

        // 3. TTY + Used gradient (dark red→rose→red) with spiky data
        let g3 = ColorGraph::new()
            .data(&spiky)
            .gradient(ColorGradient::from_theme(GradientTheme::Used))
            .symbol_set(SymbolSet::TTY)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(" TTY · Used (dark red→rose→red) "),
            );
        f.render_widget(g3, rows[2]);

        // 4. Single-row sparkline comparison: Braille
        let g4 = ColorGraph::new()
            .data(&wave)
            .gradient(ColorGradient::from_theme(GradientTheme::Free))
            .symbol_set(SymbolSet::Braille)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(" 1-row Braille · Free gradient "),
            );
        f.render_widget(g4, rows[3]);

        // 5. Single-row Block
        let g5 = ColorGraph::new()
            .data(&wave)
            .gradient(ColorGradient::from_theme(GradientTheme::Download))
            .symbol_set(SymbolSet::Block)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(" 1-row Block · Download gradient "),
            );
        f.render_widget(g5, rows[4]);

        // 6. LTR direction
        let g6 = ColorGraph::new()
            .data(&wave)
            .gradient(ColorGradient::from_theme(GradientTheme::Upload))
            .symbol_set(SymbolSet::Braille)
            .direction(Direction::LeftToRight)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(" LTR · Upload gradient "),
            );
        f.render_widget(g6, rows[5]);

        // Footer
        let footer = ratatui::widgets::Paragraph::new(
            " Press any key to exit. | macmon-graph demo — btop-style colored graphs for ratatui",
        )
        .style(Style::new().fg(Color::DarkGray));
        f.render_widget(footer, rows[6]);
    })?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q')
                || key.code == KeyCode::Esc
                || key.code != KeyCode::Null
            {
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
