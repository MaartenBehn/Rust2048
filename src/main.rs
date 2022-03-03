use rand::Rng;
use console::Term;

/*
modes: 
1: 304
2: 88
3: doesn't finsh
4: 320
*/

fn main() {

    let mode = 3;

    let size_x: usize = 4;
    let size_y: usize = 4;
    let mut field = create_field(size_x, size_y);
    let mut mov = -1;
    let stdout = Term::buffered_stdout();
    let mut lastMov = 0;
    let mut tries = 100;

    if mode == 0 {
        tries = 1;
    }

    let mut averageScore = 0;
    let mut cont = 0;

    for i in 0..tries {
        loop{
            let (pos_x, pos_y, val) = get_next_Incert(&field, size_x, size_y);
            if val == -1 {
                let score = get_score(&field, size_x, size_y);
                averageScore += score;
                println!("Score: {:?}", score);

                field = create_field(size_x, size_y);
                cont = 0;
                break;
            }
            field[pos_y][pos_x] = val;
    
            if mode == 0 {
                print_field(&field);
                if let Ok(character) = stdout.read_char() {
                    match character {
                        'w' => mov = 2,
                        'a' => mov = 0,
                        's' => mov = 3,
                        'd' => mov = 1,
                        _ => break,
                    }
                }
                print_move(mov);
            }
            else if mode == 1 {
                if lastMov == 0 {
                    mov = 2;
                }
                else if lastMov == 2 {
                    mov = 0;
                }
            }
            else if mode == 2 {
                if cont % 100 == 0 {
                    mov = 1;
                }
                else if lastMov == 0 {
                    mov = 2;
                }
                else if lastMov == 2 {
                    mov = 0;
                }
            }
            else if mode == 3 {
                let deepness = 10;
                let (best_mov, best_score) = search_for_best_move(&field, size_x, size_y, 0, deepness);
                mov = best_mov;
            }
            else if mode == 4 {
                let deepness = 7;
                if cont != 0 && cont % 10 == 0 {
                    let (best_mov, best_score) = search_for_best_move(&field, size_x, size_y, 0, deepness);
                    mov = best_mov;
                }
                else if lastMov == 0 {
                    mov = 2;
                }
                else {
                    mov = 0;
                }
            }
    
            field = move_field(field, size_x, size_y, mov);
            lastMov = mov;

            cont += 1;
            println!("{:?} ", cont);
        }
    }

    if mode != 0 {
        averageScore /= tries;
        println!("Avergae Score: {:?}", averageScore);
    }
}

fn search_for_best_move(field: &Vec<Vec<i32>>, size_x: usize, size_y: usize, deepness: i32, max_deepness: i32) -> (i8, i32) {

    let mut best_mov = 0;
    let mut best_score = 0;

    for i in 0..3 {
        let mut field_copy = field.clone();
        field_copy = move_field(field_copy, size_x, size_y, i);
        let (pos_x, pos_y, val) = get_next_Incert(&field_copy, size_x, size_y);

        let mut score = 0;
        if val == -1 || deepness >= max_deepness {

            score = get_score(&field_copy, size_x, size_y);
            
        }else{
            field_copy[pos_y][pos_x] = val;
            let (_, new_score) = search_for_best_move(&field_copy, size_x, size_y, deepness + 1, max_deepness);
            score = new_score;
        }

        if best_score < score {
            best_score = score;
            best_mov = i;
        }
    }

    return (best_mov, best_score);
}

fn create_field(size_x: usize, size_y: usize) -> Vec<Vec<i32>>{
    let array: Vec<Vec<i32>> = vec![vec![Default::default(); size_y]; size_x];
    array
}

fn print_field(field: &Vec<Vec<i32>>){
    println!("{:?}\n{:?}\n{:?}\n{:?}", &field[0], &field[1], &field[2], &field[3]);
}

fn get_next_Incert(field: &Vec<Vec<i32>>, size_x: usize, size_y: usize) -> (usize, usize, i32){

    let mut possibleMovesVec: Vec<Vec<usize>> = vec![vec![Default::default(); 2]; 0];
    for x in 0..size_x {
        for y in 0..size_y{
            if field[y][x] == 0 {
                let mut a: Vec<usize> = vec![Default::default(); 2];
                a[0] = x;
                a[1] = y;
                possibleMovesVec.push(a);
            }
        }
    }

    if possibleMovesVec.len() == 0{
        return (0, 0, -1);
    }

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..possibleMovesVec.len());

    let possible_vals: [i32; 2] = [2, 4];
    let val_index = rng.gen_range(0..2);
    let val = possible_vals[val_index];

    (possibleMovesVec[i][0], possibleMovesVec[i][1], val)
}

fn get_score(field: &Vec<Vec<i32>>, size_x: usize, size_y: usize) -> i32{

    let mut score = 0;
    for x in 0..size_x {
        for y in 0..size_y{
            score += field[y][x];
        }
    }
    score
}

// 0 <-
// 1 ->
// 2 \/
// 3 /\

fn print_move(m: i8){
    if m == 0{
        println!("\n<<<<\n")
    }
    else if m == 1 {
        println!("\n>>>>\n")
    }
    else if m == 2 {
        println!(" /\\ \n /\\ \n /\\ ")
    }
    else if m == 3 {
        println!(" \\/ \n \\/ \n \\/ ")
    }
}

fn move_field(mut field: Vec<Vec<i32>>, size_x: usize, size_y: usize, m: i8) -> Vec<Vec<i32>>{
    if m == 0 {
        for y in 0..size_y {
            let mut pos = 0;
            for x in 0..size_x{
                if field[y][x] == 0 || pos == x{
                    continue;
                }

                if  field[y][pos] == field[y][x] && x != pos {
                    field[y][pos] = field[y][x] * 2;
                    field[y][x] = 0;
                    pos += 1;
            
                }else{
                    if field[y][pos] != 0 {
                        pos += 1;
                    }
                    field[y][pos] = field[y][x];

                    if pos != x {
                        field[y][x] = 0;
                    }
                }
            }
        }
    }
    else if m == 1 {
        for y in 0..size_y {
            let mut pos = size_x -1;
            for x in (0..size_x).rev() {
                if field[y][x] == 0 || pos == x{
                    continue;
                }

                if  field[y][pos] == field[y][x] && x != pos {
                    field[y][pos] = field[y][x] * 2;
                    field[y][x] = 0;
                    pos -= 1;
            
                }else{
                    if field[y][pos] != 0 {
                        pos -= 1;
                    }
                    field[y][pos] = field[y][x];
                    if pos != x {
                        field[y][x] = 0;
                    }
                }
            }
        }
    }
    else if m == 2 {
        for x in 0..size_x {
            let mut pos = 0;
            for y in 0..size_y {
                if field[y][x] == 0 || pos == y{
                    continue;
                }

                if  field[pos][x] == field[y][x] && y != pos {
                    field[pos][x] = field[y][x] * 2;
                    field[y][x] = 0;
                    pos += 1;
            
                }else{
                    if field[pos][x] != 0 {
                        pos += 1;
                    }
                    field[pos][x] = field[y][x];

                    if pos != y {
                        field[y][x] = 0;
                    }
                }
            }
        }
    }
    else if m == 3 {
        for x in 0..size_x {
            let mut pos = size_y -1;
            for y in (0..size_y).rev() {
                if field[y][x] == 0 || pos == y{
                    continue;
                }

                if  field[pos][x] == field[y][x] && y != pos {
                    field[pos][x] = field[y][x] * 2;
                    field[y][x] = 0;
                    pos -= 1;
            
                }else{
                    if field[pos][x] != 0 {
                        pos -= 1;
                    }
                    field[pos][x] = field[y][x];

                    if pos != y {
                        field[y][x] = 0;
                    }
                }
            }
        }
    }

    field
}