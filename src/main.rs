use std::{cmp::min, io::{self, Result, Write}, thread::sleep, time::Duration};

use crossterm::{cursor::MoveTo, event::{Event, KeyCode, KeyEvent, read}, execute, queue, style::Print, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size}};

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    let (width, height): (u16, u16) = size()?;
    let tile_width = min(height*2, width)/8;
    let mut frame = String::new();
    load_frame(&mut frame, tile_width);
    queue!(stdout, Clear(ClearType::All), MoveTo(0,0), Print(&frame))?;
    stdout.flush()?;
    'main_loop: loop {
        match read()? {
            Event::Resize(width, height) => {
                let tile_width = min(height*2, width)/8;
                load_frame(&mut frame, tile_width);
            },
            Event::Key(KeyEvent{code, ..}) => {
                match code {
                    KeyCode::Char('q') => break 'main_loop,
                    _ => {}
                }
            },
            _ => {}
        }
        queue!(stdout, Clear(ClearType::All), MoveTo(0,0), Print(&frame))?;
        stdout.flush()?;
        sleep(Duration::from_millis(16));
    }
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

fn load_frame(frame: &mut String, tile_width: u16) {
    frame.clear();
    let mut is_white = true;
    for _ in 0..8 {
        for _ in 0..(tile_width/2) {
            for _ in 0..8 {
                for _ in 0..tile_width{
                    frame.push(if is_white {'█'} else {' '});
                }
                is_white = !is_white;
            }
            frame.push('\n');
            frame.push('\r');
        }
        is_white = !is_white;
    }
}

