use crate::board::Board;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    Result,
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use crate::direction::Direction;

mod board;
mod direction;
mod field;

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    const FIELD_WIDTH: u32 = 41;
    const FIELD_HEIGHT: u32 = 41;
    let mut board = Board::new(FIELD_WIDTH, FIELD_HEIGHT);

    execute!(
        stdout,
        EnterAlternateScreen,
        SetForegroundColor(Color::Green),
        Hide
    )?;

    let mut next_direction = Direction::None;

    const TICK_TIME: i128 = 200;
    let mut remaining_tick_time: i128 = TICK_TIME.clone();
    let mut tick_start = Instant::now();

    loop {
        if poll(Duration::from_millis(remaining_tick_time as u64))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {
                        let new_direction = Direction::from_key_code(code);
                        if new_direction.is_some() {
                            if !Direction::check_opposite(new_direction.unwrap(), board.direction) || board.snake_length <= 1 {
                                next_direction = new_direction.unwrap();
                            } else {
                                next_direction = board.direction;
                            }
                        }
                    }
                }
            }
        }

        remaining_tick_time = TICK_TIME - tick_start.elapsed().as_millis() as i128;

        if remaining_tick_time <= 0 {
            board.direction = next_direction;
            board.tick();

            queue!(stdout, Clear(ClearType::All))?;

            let mut i = 0;
            while let Some(line) = board.row_as_string(i) {
                queue!(stdout, MoveTo(0, i as u16), Print(line))?;
                i += 1;
            }

            queue!(
                stdout,
                MoveTo(0, (i + 1) as u16),
                Print("\nPress 'Esc' to exit...")
            )?;

            if board.game_over {
                queue!(
                    stdout,
                    MoveTo((FIELD_WIDTH * 2 + 3) as u16, 1),
                    Print("GAME OVER!")
                )?;
                let mut score: String = "Score: ".to_string();
                score.push_str(&*board.snake_length.to_string());
                queue!(
                    stdout,
                    MoveTo((FIELD_WIDTH * 2 + 3) as u16, 3),
                    Print(score)
                )?
            }

            stdout.flush()?;

            remaining_tick_time = TICK_TIME.clone();
            tick_start = Instant::now();
        }
    }

    execute!(stdout, ResetColor, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
