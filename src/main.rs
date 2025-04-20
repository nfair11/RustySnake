use crossterm::{
    cursor::{self},
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
};

use rand::Rng;
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;
#[derive(Clone)]
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
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn main() {
    // Initialize the terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    let mut snake: Vec<Point> = Vec::new();
    snake.push(Point { x: 10, y: 10 });
    snake.push(Point { x: 9, y: 10 });
    snake.push(Point { x: 8, y: 10 });

    let mut gameboard: Vec<Vec<Board>> = vec![vec![Board::Empty; 21]; 21];

    let mut dir = Direction::None;
    for i in 0..21 {
        for j in 0..21 {
            if i == 0 || i == 20 || j == 0 || j == 20 {
                gameboard[i][j] = Board::Wall;
            } else {
                gameboard[i][j] = Board::Empty;
            }
        }
    }
    let mut rng = rand::thread_rng();
    let mut food_x = 13; 
    let mut food_y = 16;
    gameboard[food_y][food_x] = Board::Food;
    stdout.execute(cursor::Hide).unwrap();
    // Game loop
    loop {
        let mut last_event: Option<KeyCode> = None;
        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                last_event = Some(key_event.code);
            }
        }
        if let Some(key_event) = last_event {
            match key_event {
                KeyCode::Esc => break,
                KeyCode::Up => dir = Direction::Up,
                KeyCode::Down => dir = Direction::Down,
                KeyCode::Left => dir = Direction::Left,
                KeyCode::Right => dir = Direction::Right,
                _ => {}
            }
        }
        // Move the cursor to the top-left
        for i in 0..snake.len() {
            let s = &snake[i];
            gameboard[s.y as usize][s.x as usize] = Board::Snake;
        }
        // Draw the game state (example: a moving snake)
        for i in 0..21 {
            for j in 0..21 {
                stdout.execute(cursor::MoveTo(j as u16 * 2, (i+1) as u16)).unwrap();

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
        let head = snake[0].clone();
        let tail = snake[snake.len()-1].clone();
        match dir {
            Direction::Up => {
                snake.insert(0, Point { x: head.x, y: head.y - 1 });
                gameboard[tail.y as usize][tail.x as usize] = Board::Empty;
                snake.pop();
            }
            Direction::Down => {
                snake.insert(0, Point { x: head.x, y: head.y + 1 });
                gameboard[tail.y as usize][tail.x as usize] = Board::Empty;
                snake.pop();
            }
            Direction::Left => {
                snake.insert(0, Point { x: head.x - 1, y: head.y });
                gameboard[tail.y as usize][tail.x as usize] = Board::Empty;
                snake.pop();
            
            }
            Direction::Right => {
                snake.insert(0, Point { x: head.x + 1, y: head.y });
                gameboard[tail.y as usize][tail.x as usize] = Board::Empty;
                snake.pop();
            }
            Direction::None => {}
        }
        if head.x as usize == food_x && head.y as usize == food_y {
            snake.push(tail.clone());
            gameboard[food_y as usize][food_x as usize] = Board::Empty;
            loop {
                food_x = rng.gen_range(1..20);
                food_y = rng.gen_range(1..20);
                if snake.iter().all(|s| s.x != food_x as i32 || s.y != food_y as i32) {
                    break;
                }
            }
            gameboard[food_y as usize][food_x as usize] = Board::Food;
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
