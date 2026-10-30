use crossterm::event::{
    self, Event,
    KeyCode::{self, Tab},
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Clear, Gauge, List, ListState, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table,
    },
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
    cpu::{
        cpu6502::Cpu6502,
        instructions::{Instruction, get_instruction, operand_len},
    },
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
    let mut last_tick: Instant = Instant::now();
    let mut inspect_start_address: u16 = 0;
    let mut parser: Parser = vt100::Parser::new(42, 80, 40);
    let mut list_state: ListState = ListState::default().with_selected(Some(0));
    let mut state: AppState = AppState::Menu;
    let mut paused: bool = false;
    let mut search_adress: bool = false;
    let mut search_address_buf: String = String::new();

    loop {
        match &mut state {
            AppState::Menu => {
                terminal.draw(|frame| render_menu(frame, &mut list_state))?;
            }
            AppState::Apple1(apple1) => {
                terminal.draw(|frame| {
                    render_machine(
                        frame,
                        &parser,
                        &apple1.cpu,
                        &mut apple1.bus,
                        "APPLE I",
                        paused,
                        inspect_start_address,
                        search_adress,
                        &search_address_buf,
                    )
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
                                    parser = vt100::Parser::new(42, 80, 40);
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        AppState::Apple1(apple1) => {
                            if search_adress {
                                match key.code {
                                    KeyCode::Char(c) => {
                                        if c.is_ascii_hexdigit() && search_address_buf.len() < 4 {
                                            search_address_buf.push(c.to_ascii_uppercase());
                                        }
                                    }
                                    KeyCode::Backspace => {
                                        search_address_buf.pop();
                                    }
                                    KeyCode::Enter => {
                                        if let Ok(addr) =
                                            u16::from_str_radix(&search_address_buf, 16)
                                        {
                                            inspect_start_address = addr & !3;
                                        }
                                        search_address_buf.clear();
                                        search_adress = false;
                                    }
                                    KeyCode::Esc => search_adress = false,

                                    _ => {}
                                }
                            } else {
                                match key.code {
                                    KeyCode::Esc => state = AppState::Menu,
                                    KeyCode::F(1) => paused = !paused,
                                    KeyCode::F(2) => apple1.cpu.cycle(&mut apple1.bus),
                                    KeyCode::F(5) => {
                                        apple1.reset();
                                        parser = vt100::Parser::new(42, 80, 40);
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
                                    KeyCode::PageDown => {
                                        inspect_start_address =
                                            inspect_start_address.wrapping_add(40);
                                    }
                                    KeyCode::PageUp => {
                                        inspect_start_address =
                                            inspect_start_address.wrapping_sub(40);
                                    }
                                    _ => {}
                                }
                            }
                            if key.code == KeyCode::F(6) {
                                search_adress = !search_adress;
                            }
                        }
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
    bus: &mut B,
    machine_name: &str,
    paused: bool,
    inspect_start_address: u16,
    search_address: bool,
    search_address_buf: &String,
) {
    let root_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ]);
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
        Span::styled(" F6 ", Style::default().bg(Color::Cyan).fg(Color::Black)),
        Span::raw(" Search Memory  "),
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
        Constraint::Percentage(23),
        Constraint::Fill(1),
        Constraint::Percentage(18),
    ])
    .spacing(1);
    let [left, middle, right] = main_area.layout(&horizontal);
    let vertical = Layout::vertical([Constraint::Percentage(15), Constraint::Fill(1)]).spacing(1);
    let vertical_rev =
        Layout::vertical([Constraint::Fill(1), Constraint::Percentage(40)]).spacing(1);
    let [top_l, bottom_l] = left.layout(&vertical);
    let [top_r, bottom_r] = right.layout(&vertical_rev);

    render_registers(frame, top_l, cpu);

    render_stack(frame, top_r, cpu, bus);

    render_memory(frame, bottom_l, cpu, bus, inspect_start_address);

    render_instructions(frame, bottom_r, cpu, bus);

    let screen = parser.screen();
    let pseudo_term = PseudoTerminal::new(screen).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Terminal"),
    );

    frame.render_widget(pseudo_term, middle);

    if search_address {
        let pop_vertical = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(25),
            Constraint::Fill(1),
        ]);
        let pop_horizontal = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(35),
            Constraint::Fill(1),
        ]);

        let [_, popup_ver, _] = middle.layout(&pop_vertical);
        let [_, popup, _] = popup_ver.layout(&pop_horizontal);

        let popup_block = Block::bordered()
            .title(" Search Address ")
            .title_alignment(Alignment::Center);
        let centered_area = popup.centered(Constraint::Percentage(60), Constraint::Percentage(20));

        let paragraph = Paragraph::new(format!("0x{}█", search_address_buf))
            .block(popup_block)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, centered_area);
    }
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
        "OP: {:?} Addressing: {:?}",
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

fn render_stack<B: Bus>(frame: &mut Frame, area: Rect, cpu: &Cpu6502, bus: &mut B) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Stack ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ]);
    let [info, gauge, body] = inner.layout(&vertical);

    let widths = [
        Constraint::Length(6),
        Constraint::Length(4),
        Constraint::Fill(1),
    ];

    let accent = Style::new().fg(Color::Yellow).bold();
    let dim = Style::new().fg(Color::DarkGray);

    let depth = 0xFFu8.wrapping_sub(cpu.sp) as u16;
    let pct = depth * 100 / 255;
    let sp_addr = 0x0100u16 | cpu.sp as u16;

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("SP "),
            Span::styled(format!("{sp_addr:04X}"), accent),
            Span::styled(format!("   depth {depth}/255"), dim),
        ])),
        info,
    );

    frame.render_widget(
        Gauge::default()
            .gauge_style(Style::new().fg(Color::Green).bg(Color::DarkGray))
            .percent(pct),
        gauge,
    );

    let header = Row::new(["ADDR", "DATA"]).style(Style::new().fg(Color::Cyan).bold());

    let mut rows: Vec<Row> = vec![
        Row::new([
            format!("▶ {sp_addr:04X}"),
            String::new(),
            "next free".to_string(),
        ])
        .style(accent),
    ];

    for i in 1..=depth {
        let addr = sp_addr + i;
        let style = if i == 1 {
            accent
        } else {
            Style::new().fg(Color::White)
        };

        rows.push(
            Row::new([format!("{addr:04X}"), format!("{:02X}", bus.read(addr))]).style(style),
        );
    }

    rows.push(Row::new(["stale"]).style(accent));

    let used = 3 + depth as usize;
    let budget = (body.height as usize).saturating_sub(used + 1);
    let mut shown = 0;

    for i in 1..=budget {
        let addr = sp_addr - i as u16;
        if addr < 0x0100 {
            break;
        }

        rows.push(
            Row::new([
                format!("{addr:04X}"),
                format!("{:02X}", bus.read(addr)),
                String::new(),
            ])
            .style(dim),
        );
        shown += 1;
    }

    if (sp_addr - 0x0100) as usize > shown {
        rows.push(Row::new(["  ⋮"]).style(dim));
    }

    frame.render_widget(Table::new(rows, widths).header(header), body);
}

fn render_instructions<B: Bus>(frame: &mut Frame, area: Rect, cpu: &Cpu6502, bus: &mut B) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Instructions ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    let [pc, body] = inner.layout(&vertical);

    let widths = [
        Constraint::Length(6),
        Constraint::Length(8),
        Constraint::Fill(1),
    ];

    let accent = Style::new().fg(Color::Yellow).bold();
    let dim = Style::new().fg(Color::DarkGray);
    let current_len = operand_len(&cpu.opcode_state.current_opcode.addressing);
    let mut previous_len = current_len;

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("PC "),
            Span::styled(format!("{0:04X}", cpu.pc), accent),
        ])),
        pc,
    );

    let mut rows: Vec<Row> = vec![
        Row::new([
            format!("▶ {0:04X}", cpu.pc),
            (cpu.pc..cpu.pc.wrapping_add(current_len as u16))
                .map(|addr| format!("{:02X}", bus.read(addr)))
                .collect::<Vec<String>>()
                .join(" "),
            format!(
                "{:?} {}",
                cpu.opcode_state.current_opcode.operation,
                cpu.opcode_state.current_opcode.addressing.short_name()
            ),
        ])
        .style(accent),
    ];

    for _i in 0..body.height as usize {
        let temp_pc = cpu.pc.wrapping_add(previous_len as u16);
        let next_instruction = get_instruction(bus.read(temp_pc));

        let row = Row::new([
            format!("{0:04X}", cpu.pc.wrapping_add(previous_len as u16)),
            (cpu.pc.wrapping_add(previous_len as u16)
                ..temp_pc.wrapping_add(operand_len(&next_instruction.addressing) as u16))
                .map(|addr| format!("{:02X}", bus.read(addr)))
                .collect::<Vec<String>>()
                .join(" "),
            format!(
                "{:?} {}",
                next_instruction.operation,
                next_instruction.addressing.short_name()
            ),
        ])
        .style(dim);

        previous_len += operand_len(&next_instruction.addressing);

        rows.push(row);
    }

    frame.render_widget(Table::new(rows, widths), body);
}

fn render_memory<B: Bus>(
    frame: &mut Frame,
    area: Rect,
    cpu: &Cpu6502,
    bus: &mut B,
    inspect_start_address: u16,
) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Memory ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let widths = [
        Constraint::Length(6),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ];

    let rows: Vec<Row> = (0..inner.height)
        .map(|row_idx| {
            let row_addr = inspect_start_address.wrapping_add(row_idx * 4);

            let mut cells = vec![format!("{:04X}:", row_addr)];

            for offset in 0..4 {
                let byte_val = bus.read(row_addr.wrapping_add(offset));
                cells.push(format!("{:02X}", byte_val));
            }

            for offset in 0..4 {
                let byte_val = bus.read(row_addr.wrapping_add(offset));
                if (0x20..0x7E).contains(&byte_val) {
                    cells.push(format!("{}", byte_val as char));
                } else {
                    cells.push("..".into());
                }
            }

            Row::new(cells)
        })
        .collect();

    frame.render_widget(Table::new(rows, widths), inner);

    let mut vertical = ScrollbarState::new(0xFFFF).position(inspect_start_address as usize);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut vertical,
    );
}
