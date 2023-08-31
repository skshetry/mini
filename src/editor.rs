use crate::term::Terminal;

pub struct Editor {}

impl Editor {
    #[allow(clippy::unused_self)]
    pub fn run(&self) -> std::io::Result<()> {
        use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

        let mut term = Terminal::default()?;
        loop {
            term.hide_cursor()?;
            term.move_to_top()?;
            for _ in 0..term.size.height - 1 {
                term.clear_current_line()?;
                term.print(&"~\r\n")?;
            }

            term.show_cursor()?;
            term.flush()?;
            #[allow(clippy::single_match)]
            match Terminal::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => break,
                _ => (),
            }
        }
        Ok(())
    }
}
