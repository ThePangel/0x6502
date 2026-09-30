use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
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
    cpu::cpu6502::Cpu6502,
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
    let mut paused = false;

    loop {
        match &mut state {
            AppState::Menu => {
                terminal.draw(|frame| render_menu(frame, &mut list_state))?;
            }
            AppState::Apple1(apple1) => {
                terminal.draw(|frame| {
                    render_machine(frame, &parser, &apple1.cpu, &apple1.bus, "APPLE I", paused)
                })?;
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
                            KeyCode::F(1) => paused = !paused,
                            KeyCode::F(2) => apple1.cpu.cycle(&mut apple1.bus),
                            KeyCode::F(5) => apple1.reset(),
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
                if !paused {
                    apple1.consume_cycles(last_tick.elapsed());
                }
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

fn render_machine<B: Bus>(
    frame: &mut Frame,
    parser: &Parser,
    cpu: &Cpu6502,
    bus: &B,
    machine_name: &str,
    paused: bool,
) {
    let root_layout = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Fill(1)]);
    let [header_area, _, main_area] = frame.area().layout(&root_layout);

    let header_layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(machine_name.len() as u16 + 16),
    ]);
    let [head_l, head_r] = header_area.layout(&header_layout);

    let keybinds = Line::from(vec![
        Span::raw(" "),
        Span::styled(" F1 ", Style::default().bg(Color::Cyan).fg(Color::Black)),
        Span::raw(" Play/Pause  "),
        Span::styled(" F2 ", Style::default().bg(Color::Cyan).fg(Color::Black)),
        Span::raw(" Step  "),
        Span::styled(" F5 ", Style::default().bg(Color::Cyan).fg(Color::Black)),
        Span::raw(" Reset  "),
        Span::styled(" ESC ", Style::default().bg(Color::Red).fg(Color::White)),
        Span::raw(" Quit "),
    ]);
    frame.render_widget(Paragraph::new(keybinds), head_l);

    let (state, color) = if !paused {
        (" > RUNNING ", Color::LightGreen)
    } else {
        (" || PAUSED ", Color::Red)
    };

    let machine = Line::from(vec![
        Span::styled(machine_name, Style::new().fg(Color::Green).bold()),
        Span::raw("  "),
        Span::styled(
            state,
            Style::new().fg(Color::Rgb(20, 22, 30)).bg(color).bold(),
        ),
        Span::raw(" "),
    ])
    .left_aligned();
    frame.render_widget(Paragraph::new(machine), head_r);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(21),
        Constraint::Percentage(58),
        Constraint::Percentage(21),
    ])
    .spacing(1);
    let [left, middle, right] = main_area.layout(&horizontal);
    let vertical = Layout::vertical([Constraint::Percentage(15), Constraint::Fill(1)]).spacing(1);
    let vertical_rev =
        Layout::vertical([Constraint::Fill(1), Constraint::Percentage(40)]).spacing(1);
    let [top_l, bottom_l] = left.layout(&vertical);
    let [top_r, bottom_r] = right.layout(&vertical_rev);

    render_registers(frame, top_l, cpu);

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

fn render_registers(frame: &mut Frame, area: Rect, cpu: &Cpu6502) {
    let f_states = |bit: u8, ch: char| {
        let color = if cpu.p & bit != 0 {
            Color::LightGreen
        } else {
            Color::DarkGray
        };

        Span::styled(ch.to_string(), Style::default().fg(color))
    };
    let line_1 = Line::from(format!("A:{:02X} Y:{:02X} X:{:02X}", cpu.a, cpu.y, cpu.x,));

    let line_2 = Line::from(format!("PC:{:04X} SP:{:02X}", cpu.pc, cpu.sp,));

    let flags = Line::from(vec![
        Span::raw(format!("P:{:02X} Flags: [", cpu.p)),
        f_states(0x80, 'N'),
        f_states(0x40, 'V'),
        Span::styled("-", Style::default().fg(Color::DarkGray)),
        f_states(0x10, 'B'),
        f_states(0x08, 'D'),
        f_states(0x04, 'I'),
        f_states(0x02, 'Z'),
        f_states(0x01, 'C'),
        Span::raw("]"),
    ]);

    let cycle_state = Line::from(format!(
        "Cycles: {:?}  OP cycles: {:?}",
        cpu.cycles, cpu.opcode_state.opcode_cycle
    ));

    let opcode_state = Line::from(format!(
        "OPCode: {:?} Addressing: {:?}",
        cpu.opcode_state.current_opcode.operation, cpu.opcode_state.current_opcode.addressing,
    ));

    frame.render_widget(
        Paragraph::new(vec![line_1, line_2, flags, cycle_state, opcode_state]).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("CPU STATE"),
        ),
        area,
    );
}
