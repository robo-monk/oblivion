use crate::Buffer;
use termion::event::Key;

struct InputEvent {
    key: Key,
}

pub enum Mode {
    Normal(NormalMode),
    Insert(InsertMode),
    Visual(VisualMode),
}

trait ModeHandler {
    fn handle_input(&mut self, input: InputEvent, buffer: &mut Buffer);
}

pub struct NormalMode {}

impl ModeHandler for NormalMode {
    fn handle_input(&mut self, input: InputEvent, buffer: &mut Buffer) {
        // let k = input.key;

        // match input.key {
        //     Key::Char('q') => break,
        //     Key::Char(c) => {
        //         current_row += 1;
        //         if current_row >= max_width {
        //             current_row = 1;
        //             current_col += 1;
        //         }

        //         // println!("{}", c);

        //         write!(
        //             stdout,
        //             "{}{}{}",
        //             cursor::Goto(current_row, current_col),
        //             c,
        //             cursor::BlinkingBlock
        //         );
        //         // stdout.flush().unwrap();
        //     }
        //     _ => {}
        // }
    }
}

pub struct InsertMode {}

pub struct VisualMode {}
