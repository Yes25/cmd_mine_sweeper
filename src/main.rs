use std::{env, io};
use rand::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut hight:usize = 0;
    let mut width:usize = 0;

    let num_mines = 5;

    match  (args.get(1).unwrap()).parse::<usize>()  {
        Err(_) => println!("hight is not given in the correct Form: Must be an Integer."),
        Ok(val) => {hight = val}  
    }

    match  (args.get(2).unwrap()).parse::<usize>()  {
        Err(_) => println!("width is not given in the correct Form: Must be an Integer."),
        Ok(val) => {width = val}  
    }

    let mines_pos = get_mines_pos(num_mines, width, hight);
    let mines = init_game_board(width, hight, &mines_pos);
    let mut opened = vec![vec![false; width]; hight];

    loop {
        print!("{}[2J", 27 as char);
        if check_win(&opened, &mines_pos) {
            print_game_board(&opened, &mines, true);    
            println!("Gewonnen!");
            break;
        }
        print_game_board(&opened, &mines, false);
        println!("Öffne Feld oder (Q)uit?");

        match get_pos(width, hight) {
            None => break,
            Some((row,col)) => {
                opened[row][col] = true;
                let val = mines[row][col];
                if val == -1 {
                    print!("{}[2J", 27 as char);
                    print_game_board(&opened, &mines, true);
                    println!("Explodiert!");
                    break;
                }
            }
        }
    }
}

fn get_pos(row_len: usize, col_len: usize) -> Option<(usize, usize)> {
    let row_out: usize;
    let col_out: usize;
    let mut command = String::new();
    loop {
        command = "".to_string();
        // print!("Inut row: ");
        io::stdin().read_line(&mut command)
                    .expect("Konnte die Eingabe nicht lesen");
        if &command[0..1] == "Q" {
            return None;
        }
        if let Some(row) = check_input(&command, row_len) {
            row_out = row;
            break;
        }
        print!("Die Eingabe war nicht korrekt");
    }
    loop {
        command = "".to_string();
        // print!("Inut col: ");
        io::stdin().read_line(&mut command)
                    .expect("Konnte die Eingabe nicht lesen");
        if &command[0..1] == "Q" {
            return None;
        }
        if let Some(col) = check_input(&command, col_len) {
            col_out = col;
            break;
        }
        print!("Die Eingabe war nicht korrekt");
    }
    println!();
    return Some((row_out, col_out));
}

fn check_input( command: &String, max_size: usize) -> Option<usize> {
    if let Ok(val) = &command[0..1].parse::<usize>() {
        if *val < max_size {
            return Some(*val);
        } else {
            println!("Die Eingabe ist zu groß oder zu klein")
        }
    }
    println!("Die Eingabe ist kleiner als 0 oder kein Integer");
    return None;
}

fn print_game_board(opened: &Vec<Vec<bool>>, mines: &Vec<Vec<i32>>, expl:bool) {
    let rows = opened.len();
    let cols = opened[0].len();
    
    print!("  | ");
    for i in 0..cols {
        print!("{} ", i);
    }
    println!();
    print!("---");
    for _i in 0..cols {
        print!("--");
    }
    println!();

    for m in 0..rows {
        print!("{} | ", m);
        for n in 0..cols {
            if !expl {
                if !opened[m][n] {
                    print!("# ");
                } else {
                    print!("{} ", mines[m][n]);
                }
            } else {
                let val = mines[m][n];
                if val == -1 {
                    print!("* ")
                } else {
                    print!("{} ", val);
                }
            }
        }
        println!("");
    }
}

fn init_game_board(width: usize, hight: usize, mines_pos: &Vec<(usize,usize)>) -> Vec<Vec<i32>> {
    let mut mines = vec![vec![0; width]; hight];

    for (row,col) in mines_pos {
        mines[*row][*col] = -1;
    }

    mines = count_mines(mines);

    mines
}

fn get_mines_pos(num_mines: i32, width: usize, hight: usize) -> Vec<(usize, usize)> {
    let mut mines: Vec<(usize, usize)> = Vec::new();
    for _ in 0..num_mines{
        let row = rand::thread_rng().gen_range(0..hight);
        let col = rand::thread_rng().gen_range(0..width);
        mines.push((row,col));
    }
    mines
}

fn count_mines(mut mines: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let num_rows = mines.len();
    let num_cols = mines[0].len(); 

    for row in 0..num_rows {
        for col in 0..num_cols {
            let mut sum_mines = 0;
            if mines[row][col] != -1 {
                // oben mitte
                if row > 0 {
                    if let Some(row) = mines.get(row-1) {
                        if let Some(val) = row.get(col) {
                            if *val == i32::from(-1) {
                                sum_mines += 1;
                            }
                        }
                    }
                }
                // oben links
                if row > 0 && col > 0 {
                    if let Some(row) = mines.get(row-1) {
                        if let Some(val) = row.get(col-1) {
                            if *val == i32::from(-1) {
                                sum_mines += 1;
                            }
                        }
                    }
                }
                // links
                if col > 0 {
                    if let Some(row) = mines.get(row) {
                        if let Some(val) = row.get(col-1) {
                            if *val == i32::from(-1) {
                                sum_mines += 1;
                            }
                        }
                    }
                }
                //unten links
                if col > 0 {
                    if let Some(row) = mines.get(row+1) {
                        if let Some(val) = row.get(col-1) {
                            if *val == i32::from(-1) {
                                sum_mines += 1;
                            }
                        }
                    }
                }
                //unten mitte
                if let Some(row) = mines.get(row+1) {
                    if let Some(val) = row.get(col) {
                        if *val == i32::from(-1) {
                            sum_mines += 1;
                        }
                    }
                }
                //unten rechts
                if let Some(row) = mines.get(row+1) {
                    if let Some(val) = row.get(col+1) {
                        if *val == i32::from(-1) {
                            sum_mines += 1;
                        }
                    }
                }
                //mitte rechts
                if let Some(row) = mines.get(row) {
                    if let Some(val) = row.get(col+1) {
                        if *val == i32::from(-1) {
                            sum_mines += 1;
                        }
                    }
                }
                //oben rechts
                if row > 0 {
                    if let Some(row) = mines.get(row-1) {
                        if let Some(val) = row.get(col+1) {
                            if *val == i32::from(-1) {
                                sum_mines += 1;
                            }
                        }
                    }
                }
                mines[row][col] = sum_mines;
            }
        }
    }
    mines
}

fn check_win(opened: &Vec<Vec<bool>>, mines_pos: &Vec<(usize, usize)>) -> bool {
    for row in 0..opened.len() {
        for col in 0..opened[0].len() {
            // if pos is not opened yet
            if !opened[row][col] {
                // check if pos is in mines
                let mut in_pos = false;
                for (row_mine,col_mine) in mines_pos {
                    if *row_mine == row && *col_mine == col {
                        in_pos = true;
                    }
                }
                // if pos not in mines_pos -> other fields not opened -> not won jet
                if !in_pos {
                    return false
                }
            }
        }
    }
    return true
}
