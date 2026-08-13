use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex::Center, Layout, VerticalAlignment::Top},
    widgets::{Block, BorderType},
};
use std::io;

mod bus;
mod cpu;

fn main() -> io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .spacing(1);
    let [left, middle, right] = frame.area().layout(&horizontal);
    let vertical = Layout::vertical([Constraint::Percentage(40), Constraint::Fill(1)]).spacing(1);
    let [topL, bottomL] = left.layout(&vertical);
    let [topR, bottomR] = right.layout(&vertical);

    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Registers"),
        topL,
    );
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Stack"),
        topR,
    );
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Memory"),
        bottomL,
    );
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Instructions"),
        bottomR,
    );
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Terminal"),
        middle,
    );
}
