extern crate termion;
use std::io::{stdin, stdout, Write};
use oblivion::renderer::TermionRenderer;
use termion::cursor::BlinkingBlock;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style, terminal_size};

use oblivion::{Buffer, Editor, InsertMode, Mode, NormalMode, VisualMode, Renderer, BufferInterface};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // write!(stdout, "{}{}Hey there!", color::Fg(color::Red), cursor::Goto(1,1)).unwrap();

    let (max_width, max_height) = terminal_size().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on...{}",
        termion::clear::All,
        cursor::Goto(1, 1),
        cursor::BlinkingBlock
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut content = String::new();
    let mut current_row = 1;
    let mut current_col = 1;

    // let editor = Editor::new();
// terminal_size().unwrap()
    // let renderer = Renderer::Terminal(
    //     TermionRenderer::new(terminal_size().unwrap())
    // );

    let renderer = TermionRenderer::new(terminal_size().unwrap());
    // renderer;
    // let mut buffer = Buffer::new(String::from("./src/main.rs"), Renderer::Terminal(renderer));
    // let mut buffer = Buffer::new(String::from("./src/main.rs"), Renderer::Terminal(renderer));
    let mut buffer = Buffer::new(String::from("./src/main.rs"), &renderer);
    // println!("{:?}", buffer);

    // write!(stdout, "{}", cursor::BlinkingBlock);

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => {
                buffer.append(c);
                // buffer
                // buffer.append(c);
                // renderer.
                // current_row += 1;
                // if current_row >= max_width {
                //     current_row = 1;
                //     current_col += 1;
                // }

                // println!("{}", c);

                // write!(
                //     stdout,
                //     "{}{}{}",
                //     cursor::Goto(current_row, current_col),
                //     c,
                //     cursor::BlinkingBlock
                // );
                // stdout.flush().unwrap();
            }
            _ => {}
        }

        write!(stdout, "{}", termion::cursor::BlinkingBlock).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::BlinkingBlock).unwrap();

    //println!("{}Red", color::Fg(color::Red));
    //println!("{}{} yo !", termion::clear::All, termion::cursor::Goto(1, 1));
}
