use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, List, ListState, Paragraph},
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_term::{
    vt100::{self, Parser},
    widget::PseudoTerminal,
};

use crate::{
    bus::Bus,
    machines::apple_1::apple1::{self, Apple1},
};

mod bus;
mod cpu;
mod machines;

enum AppState {
    Menu,
    Apple1(Apple1),
}

fn main() -> io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut last_tick = Instant::now();

    let mut parser = vt100::Parser::new(24, 80, 0);
    let mut list_state = ListState::default().with_selected(Some(0));
    let mut state = AppState::Menu;

    loop {
        match &mut state {
            AppState::Menu => {
                terminal.draw(|frame| render_menu(frame, &mut list_state))?;
            }
            AppState::Apple1(apple1) => {
                terminal.draw(|frame| render_machine(frame, &parser))?;
            }
        }

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match &mut state {
                        AppState::Menu => match key.code {
                            KeyCode::Esc => return Ok(()),
                            KeyCode::Down => list_state.select_next(),
                            KeyCode::Up => list_state.select_previous(),
                            KeyCode::Enter => match list_state.selected().unwrap() {
                                0 => {
                                    state = AppState::Apple1(Apple1::new());
                                    last_tick = Instant::now();
                                    parser = vt100::Parser::new(24, 80, 0);
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        AppState::Apple1(apple1) => match key.code {
                            KeyCode::Esc => state = AppState::Menu,
                            KeyCode::Char('n') => {
                                for _ in 0..50 {
                                    apple1.cpu.cycle(&mut apple1.bus);
                                }
                            }
                            KeyCode::Char(c) if c.is_ascii() => {
                                apple1.bus.console_write(c.to_ascii_uppercase() as u8);
                            }
                            KeyCode::Enter => {
                                apple1.bus.console_write(0x0D);
                            }
                            KeyCode::Backspace => {
                                apple1.bus.console_write(0x5F);
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
        match &mut state {
            AppState::Apple1(apple1) => {
                apple1.consume_cycles(last_tick.elapsed());
                last_tick = Instant::now();

                while let Some(byte) = apple1.bus.console_read() {
                    match byte {
                        b'\r' => parser.process(b"\r\n"),
                        0x5F => parser.process(b"\x08 \x08"),
                        _ => parser.process(&[byte]),
                    }
                }
            }
            _ => {}
        }
    }
}

fn render_menu(frame: &mut Frame, list_state: &mut ListState) {
    let vertical = Layout::vertical([
        Constraint::Percentage(33),
        Constraint::Percentage(15),
        Constraint::Percentage(52),
    ])
    .spacing(1);
    let [top, middle, bottom] = frame.area().layout(&vertical);

    let title_text = " 
    ██████╗         ██████╗  ███████╗ ██████╗ ██████╗ 
    ██╔═████╗       ██╔═══╝  ██╔════╝██╔═████╗╚════██╗
    ██║██╔██║██  ██ ███████╗ ███████╗██║██╔██║ █████╔╝
    ████╔╝██║ ╚██╔╝ ██╔═══██╗╚════██║████╔╝██║██╔═══╝ 
    ╚██████╔╝██╔╝██╗╚██████╔╝███████║╚██████╔╝███████╗
     ╚═════╝ ╚═╝ ╚═╝ ╚═════╝ ╚══════╝ ╚═════╝ ╚══════╝
                                                   ";

    let title = Text::from(title_text);

    let title_paragraph = Paragraph::new(title).style(Style::default().fg(Color::Green).bold());

    let welcome = Paragraph::new("Welcome to 0x6502\n Please choose a system:")
        .style(Style::default().fg(Color::Red).bold())
        .centered();
    frame.render_widget(title_paragraph, top);
    frame.render_widget(welcome, middle);

    let items = ["Apple I"];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Style::new().red().italic())
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, bottom, list_state);
}

fn render_machine(frame: &mut Frame, parser: &Parser) {
    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .spacing(1);
    let [left, middle, right] = frame.area().layout(&horizontal);
    let vertical = Layout::vertical([Constraint::Percentage(40), Constraint::Fill(1)]).spacing(1);
    let vertical_rev =
        Layout::vertical([Constraint::Fill(1), Constraint::Percentage(40)]).spacing(1);
    let [top_l, bottom_l] = left.layout(&vertical);
    let [top_r, bottom_r] = right.layout(&vertical_rev);

    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Registers"),
        top_l,
    );
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Stack"),
        top_r,
    );
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Memory"),
        bottom_l,
    );
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Instructions"),
        bottom_r,
    );
    let screen = parser.screen();
    let pseudo_term = PseudoTerminal::new(screen).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Terminal"),
    );

    frame.render_widget(pseudo_term, middle);
}
