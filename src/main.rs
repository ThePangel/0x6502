use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
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
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Terminal"),
        middle,
    );
}
