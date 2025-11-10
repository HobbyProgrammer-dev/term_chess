mod board;

use std::{cmp::min, io::{self, Result, Stdout, Write}, thread::sleep, time::Duration};

use crossterm::{cursor::MoveTo, event::{Event, KeyCode, KeyEvent, read}, execute, queue, style::Print, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size}};

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    let (width, height): (u16, u16) = size()?;
    let width = width - 2;
    let height = height - 2;
    let tile_width = min(height*2, width)/8;
    let mut frame = String::new();
    let mut game = board::GameBoard::new_standard();
    load_frame(&mut frame, tile_width);
    queue!(stdout, Clear(ClearType::All), MoveTo(0,0), Print(&frame))?;
    load_board(&mut stdout, &mut game, tile_width)?;
    stdout.flush()?;
    'main_loop: loop {
        let mut tile_width: u16 = 0;
        match read()? {
            Event::Resize(width, height) => {
                let width = width - 2;
                let height = height - 2;
                tile_width = min(height*2, width)/8;
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
        load_board(&mut stdout, &mut game, tile_width)?;
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
    frame.push('╔');
    for _tile_x in 0..8*tile_width {
        frame.push('═')
    }
    frame.push_str("╗\n\r");
    for _grid_y in 0..8 {
        for _tile_y in 0..(tile_width/2) {
            frame.push('║');
            for _grid_x in 0..8 {
                for _tile_x in 0..tile_width{
                    frame.push(if is_white {'█'} else {' '});
                }
                is_white = !is_white;
            }
            frame.push_str("║\n\r");
        }
        is_white = !is_white;
    }

    frame.push('╚');
    for _ in 0..8*tile_width {
        frame.push('═')
    }
    frame.push('╝');
}

fn load_board(stdout: &mut Stdout, game:&mut board::GameBoard, tile_width: u16) -> Result<()> {
    for grid_y in 0..8 {
        for grid_x in 0..8 {
            queue!(
                stdout,
                MoveTo(1 + tile_width / 2 + grid_x*tile_width, 1 + tile_width / 4 + grid_y*tile_width/2),
                Print(texture(&game.state[grid_y as usize][grid_x as usize])))?;
        }
    }
    Ok(())
}

fn texture(piece: &board::PieceType) -> &'static str {
    match piece {
        board::PieceType::KING(_color) => "K",
        board::PieceType::QUEEN(_color) => "Q",
        board::PieceType::BISHOP(_color) => "B",
        board::PieceType::KNIGHT(_color) => "N",
        board::PieceType::ROOK(_color) => "R",
        board::PieceType::PAWN(_color) => "P",
        board::PieceType::NULL => ""
    }
}
