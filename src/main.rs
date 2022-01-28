use rand::Rng;

fn main() {
    let size_x: usize = 4;
    let size_y: usize = 4;
    let mut field = create_field(size_x, size_y);

    /*
    for i in 0..6 {
        let (pos_x, pos_y, val) = get_next_Incert(&field, size_x, size_y);
        field[pos_x][pos_y] = val;
    }
    */

    field[0][0] = 4;
    field[0][1] = 4;
    field[0][2] = 8;
    field[0][3] = 0;

    field[1][0] = 4;
    field[1][1] = 0;
    field[1][2] = 2;
    field[1][3] = 0;


    print_field(&field);

    field = move_field(field, size_x, size_y, 0);

    print_field(&field);
}

fn create_field(size_x: usize, size_y: usize) -> Vec<Vec<i32>>{
    let array: Vec<Vec<i32>> = vec![vec![Default::default(); size_y]; size_x];
    array
}

fn print_field(field: &Vec<Vec<i32>>){
    println!("{:?}\n{:?}\n{:?}\n{:?}\n", &field[0], &field[1], &field[2], &field[3]);
}

fn get_next_Incert(field: &Vec<Vec<i32>>, size_x: usize, size_y: usize) -> (usize, usize, i32){

    let mut rng = rand::thread_rng();
    let mut pos_x = 0;
    let mut pos_y = 0;
    for i in 0..1000 {
        pos_x = rng.gen_range(0..size_x);
        pos_y = rng.gen_range(0..size_y);

        if field[pos_x][pos_y] == 0 {
            break;
        }
    }

    let possible_vals: [i32; 2] = [2, 4];
    let val_index = rng.gen_range(0..2);
    let val = possible_vals[val_index];

    (pos_x, pos_y, val)
}

// 0 <-
// 1 ->
// 2 \/
// 3 /\

fn move_field(mut field: Vec<Vec<i32>>, size_x: usize, size_y: usize, m: i8) -> Vec<Vec<i32>>{
    if m == 0 {
        for y in 0..size_y {
            let mut pos = 0;
            for x in 0..size_x{
                if field[y][x] == 0 || pos == x{
                    continue;
                }

                move_action!(field, x, y, pos, y, pos);
            }
        }
    }
    

    field
}

macro_rules! move_action{
       (mut field: Vec<Vec<i32>>, x: usize, y: usize, x_pos: usize, y_pos: usize, mut pos: usize)=>{
           {
            if field[y_pos][x_pos] == field[y][x] && x != pos {
                field[y_pos][x_pos] = field[y][x] * 2;
                field[y][x] = 0;
                pos += 1;
        
            }else{
                if field[y_pos][x_pos] != 0 {
                    pos += 1;
                }
                field[y_pos][x_pos] = field[y][x];
                field[y][x] = 0;
            }
           }
       }
   }

   /*
fn move_action(mut field: Vec<Vec<i32>>, x: usize, y: usize, x_pos: usize, y_pos: usize, mut pos: usize) -> (Vec<Vec<i32>>, usize) {
    if field[y_pos][x_pos] == field[y][x] && x != pos {
        field[y_pos][x_pos] = field[y][x] * 2;
        field[y][x] = 0;
        pos += 1;

    }else{
        if field[y_pos][x_pos] != 0 {
            pos += 1;
        }
        field[y_pos][x_pos] = field[y][x];
        field[y][x] = 0;
    }

    return (field, pos)
}
*/
