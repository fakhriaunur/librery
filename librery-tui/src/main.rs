use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Event {
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel::<Event>();

    let tx_clone = tx.clone();
    thread::spawn(move || loop {
        tx_clone.send(Event::Tick).unwrap();
        thread::sleep(Duration::from_millis(100));
    });

    let item_id = String::new();
    let patron_id = String::new();
    let status_message = String::from("Enter item ID and patron ID to checkout");
    let mut tick_count = 0u32;

    loop {
        tick_count += 1;
        if tick_count > 1000 {
            break;
        }
        terminal.draw(|f: &mut Frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(5),
                ])
                .split(f.area());

            let item_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan))
                .title("Item ID");
            let item_paragraph = Paragraph::new(item_id.as_str()).block(item_block);
            f.render_widget(item_paragraph, chunks[0]);

            let patron_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan))
                .title("Patron ID");
            let patron_paragraph = Paragraph::new(patron_id.as_str()).block(patron_block);
            f.render_widget(patron_paragraph, chunks[1]);

            let status_block = Block::default().borders(Borders::ALL).title("Status");
            let status_paragraph = Paragraph::new(status_message.as_str()).block(status_block);
            f.render_widget(status_paragraph, chunks[2]);

            let help_block = Block::default().title("Help").borders(Borders::ALL);
            let help_paragraph =
                Paragraph::new("Press Enter to checkout, Esc to quit").block(help_block);
            f.render_widget(help_paragraph, chunks[3]);
        })?;

        if let Ok(event) = rx.recv_timeout(Duration::from_millis(50)) {
            match event {
                Event::Tick => {}
            }
        }
    }

    Ok(())
}
