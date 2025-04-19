use crossterm::{
    cursor::{self},
    event,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

struct Point {
    x: i32,
    y: i32,
}
#[derive(Clone)]
enum Board {
    Empty,
    Wall,
    Food,
    Snake,
}

fn main() {
    // Initialize the terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    let mut snake: Vec<Point> = Vec::new();
    snake.push(Point { x: 10, y: 10 });
    let mut gameboard: Vec<Vec<Board>> = vec![vec![Board::Empty; 21]; 21];
    for i in 0..21 {
        for j in 0..21 {
            if i == 0 || i == 20 || j == 0 || j == 20 {
                gameboard[i][j] = Board::Wall;
            } else {
                gameboard[i][j] = Board::Empty;
            }
        }
    }
    gameboard[13][16] = Board::Food;
    stdout.execute(cursor::Hide).unwrap();
    // Game loop
    for frame in 0..100 {
        // Move the cursor to the top-left
        for i in 0..snake.len() {
            let s = &snake[i];
            gameboard[s.y as usize][s.x as usize] = Board::Snake;
        }
        // Draw the game state (example: a moving snake)
        for i in 0..21 {
            for j in 0..21 {
                stdout.execute(cursor::MoveTo(j as u16 * 2, i as u16)).unwrap();

                match gameboard[i][j] {
                    Board::Empty => 
                    if ((i+j)%2) == 1 {
                        print!("\x1b[48;5;240m  \x1b[0m")
                    } else {
                        print!("\x1b[48;5;15m  \x1b[0m")
                    },
                    Board::Wall => print!("\x1b[48;5;28m  \x1b[0m"),
                    Board::Food => print!("🍎"),
                    Board::Snake => print!("\x1b[48;5;2m  \x1b[0m"),
                }
            }
        }

        // Sleep to simulate frame duration
        sleep(Duration::from_millis(200));
    }

    // Restore terminal state
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    stdout.execute(cursor::Show).unwrap();
    println!("Game Over!");
}
