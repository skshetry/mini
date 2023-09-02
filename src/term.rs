use std::fmt::Display;
use std::io::{stdout, Stdout, Write};

use crossterm::event::{read, Event};
use crossterm::{cursor, style, terminal, QueueableCommand};

use crossterm::style::Color;

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct Terminal {
    pub size: Size,
    stdout: Stdout,
}

impl Default for Terminal {
    fn default() -> Self {
        let size = terminal::size().unwrap();
        terminal::enable_raw_mode().unwrap();

        Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            stdout: stdout(),
        }
    }
}

impl Terminal {
    pub fn clear_screen(&mut self) -> std::io::Result<&Self> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?;
        Ok(self)
    }
    pub fn clear_current_line(&mut self) -> std::io::Result<&Self> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?;
        Ok(self)
    }

    pub fn show_cursor(&mut self) -> std::io::Result<&Self> {
        self.stdout.queue(cursor::Show)?;
        Ok(self)
    }

    pub fn hide_cursor(&mut self) -> std::io::Result<&Self> {
        self.stdout.queue(cursor::Hide)?;
        Ok(self)
    }

    pub fn move_position(&mut self, x: u16, y: u16) -> std::io::Result<&Self> {
        self.stdout.queue(cursor::MoveTo(x, y))?;
        Ok(self)
    }

    pub fn move_to_top(&mut self) -> std::io::Result<&Self> {
        self.move_position(0, 0)
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }

    pub fn print<T: Display + Sized>(&mut self, line: &T) -> std::io::Result<&Self> {
        self.stdout.queue(style::Print(&line))?;
        Ok(self)
    }

    pub fn set_fg_color(&mut self, color: Color) -> std::io::Result<&Self> {
        self.stdout.queue(style::SetForegroundColor(color))?;
        Ok(self)
    }

    pub fn set_bg_color(&mut self, color: Color) -> std::io::Result<&Self> {
        self.stdout.queue(style::SetBackgroundColor(color))?;
        Ok(self)
    }

    pub fn reset_color(&mut self) -> std::io::Result<&Self> {
        self.stdout.queue(style::ResetColor)?;
        Ok(self)
    }

    pub fn read() -> std::io::Result<Event> {
        read()
    }
}
