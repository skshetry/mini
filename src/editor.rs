use std::time::{Duration, Instant};

use crate::term::{Size, Terminal};
use crossterm::style::Color;
const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};
#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    status: StatusMessage,
    pub terminal: Terminal,
}

#[derive(Debug)]
struct StatusMessage {
    text: String,
    time: Instant,
}

impl From<String> for StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            terminal: Terminal::default(),
            should_quit: false,
            status: String::from("HELP: Ctrl-F = find | Ctrl-Q = quit").into(),
        }
    }
}

impl Editor {
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

        #[allow(clippy::single_match)]
        match Terminal::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                log::trace!("Received request to quit");
                self.should_quit = true;
                Ok(())
            } // quit on next loop
            Event::Resize(width, height) => {
                let size = Size { width, height };
                log::trace!(
                    "terminal resized from {:?} to {:?}",
                    self.terminal.size,
                    size
                );
                self.terminal.size = size;
                Ok(())
            }
            ev => {
                log::trace!("received {:?}", ev);
                Ok(())
            }
        }
    }

    pub fn draw_status_bar(&mut self) -> std::io::Result<()> {
        let width = self.terminal.size.width as usize;
        let mut message = format!("Status bar: {:?}", self.terminal.size);
        message.push_str(&" ".repeat(width.saturating_sub(message.len())));

        self.terminal.clear_current_line()?;
        self.terminal.set_bg_color(STATUS_BG_COLOR)?;
        self.terminal.set_fg_color(STATUS_FG_COLOR)?;
        self.terminal.print(&(message + "\r\n"))?;
        self.terminal.reset_color()?;
        Ok(())
    }

    pub fn draw_message_bar(&mut self) -> std::io::Result<()> {
        self.terminal.clear_current_line()?;
        let message = &self.status;
        if message.time.elapsed() < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size.width as usize);
            self.terminal.print(&text)?;
        }
        Ok(())
    }

    pub fn draw_rows(&mut self) -> std::io::Result<()> {
        let height = self.terminal.size.height - 2;

        for _ in 0..height {
            self.terminal.clear_current_line()?;
            self.terminal.print(&"~\r\n")?;
        }
        Ok(())
    }

    pub fn refresh_screen(&mut self) -> std::io::Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.move_to_top()?;

        if self.should_quit {
            self.terminal.clear_screen()?;
        } else {
            self.draw_rows()?;
            self.draw_status_bar()?;
            self.draw_message_bar()?;
        }

        self.terminal.show_cursor()?;
        self.terminal.flush()
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            self.handle_events()?;
        }
        Ok(())
    }
}
