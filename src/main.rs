
 


const ARRAY_TO_SOLVE : [[u8;6];5] = [[0,1,0,0,0,0],
[0,1,0,0,0,0],
[0,1,0,0,0,0],
[0,1,0,0,0,0],
[0,0,0,0,1,0]];

#[derive (PartialEq,Clone,Copy)]
enum State{
    Empty,
    Obstacle,
    Closed,
    Path,
}

const DELTA : [[i64;2];4] = [[-1,0],[0,-1],[1,0],[0,1]];

fn cell_string(state: State)->String{
    if state == State::Obstacle{
        return String::from("⛰️  ");
    }
    if state == State::Path{
        return String::from("🚓  ");
    }
    else {
        return String::from("0   ");
    }
}

fn read_board(array : [[u8;ARRAY_TO_SOLVE[0].len()];ARRAY_TO_SOLVE.len()])-> Vec<Vec<State>>{
    let mut board : Vec<Vec<State>> = Vec::new();
    for i in 0..array.len(){
        let mut row : Vec<State> = Vec::new();
        for j in 0..array[i].len(){
            if array[i][j] == 0 {
                row.push(State::Empty);
            }
            if array[i][j] == 1 {
                row.push(State::Obstacle);
            }
        }
        board.push(row);
    }
    return board;
}

fn print_board(board : Vec<Vec<State>>){
    for i in 0..board.len(){
        for j in 0..board[i].len(){
            if board[i][j] == State::Obstacle{
                print!{"⛰️   "};
            }
            if board[i][j] == State::Path{
                print!{"🚓  "};
            }
            if board[i][j] == State::Closed{
                print!("0   ");
            }
            if board[i][j] == State::Empty{
                print!("0   ");
            }
        }
        println!("");
    }
}

fn compare(a : &Vec<i64>, b: &Vec<i64>)->bool{
    let f1 : i64 = a[2] + a[3];
    let f2 : i64 = b[2] + b[3];
    if f1<f2{
        return true;
    }
    else {
        return false;
    }
}

fn heuristic(x1 : i64, y1: i64, x2:i64, y2:i64)->i64{
    return (x1-x2).abs() + (y1-y2).abs();
}

fn cell_sort(list : &mut Vec<Vec<i64>>)->Vec<Vec<i64>> {
    let mut temp_list : Vec<Vec<i64>> = Vec::new();
    let list_length = list.len();
    for i in 0..list_length{
        let mut smallest : Vec<i64> = Vec::new();
        let mut integer_of_smallest_value : usize = 0;
        if list.len()==1{
            smallest = list[0].clone();
            integer_of_smallest_value = 0;
        }
        if list.len()==2{
            if compare(&list[0],&list[1]){
                smallest = list[0].clone();
                integer_of_smallest_value = 0;
            }
            else {
                smallest = list[1].clone();
                integer_of_smallest_value = 1;
            }
        }
        if list.len()>2{
        for j in 0..list_length-i-1{
            if compare(&list[j],&list[j+1]){
                smallest = list[j].clone();
                integer_of_smallest_value = j;
            }
            else {
                smallest = list[j+1].clone();
                integer_of_smallest_value = j+1;
            }
        }
    }
        temp_list.push(smallest);
        list.remove(integer_of_smallest_value);
    }
    temp_list
}

fn check_valid_cell(x : i64, y : i64, board : &mut Vec<Vec<State>>)->bool{
    let x_on_grid : bool = x >= 0 && x < board.len().try_into().unwrap();
    let y_on_grid : bool = y >= 0 && y < board[0].len().try_into().unwrap();
    if x_on_grid && y_on_grid{
        let x_usize : usize = x.try_into().unwrap();
        let y_usize : usize = y.try_into().unwrap();
        if board[x_usize][y_usize] == State::Empty{
                return true;
            }
        }
    false
    }
    
fn add_to_open_list(list : &mut Vec<Vec<i64>>, x:i64,y:i64,g:i64,h:i64, board : &mut Vec<Vec<State>>){
    list.push(vec![x,y,g,h]);
    let x_usize : usize = x.try_into().unwrap();
    let y_usize : usize = y.try_into().unwrap();
    board[x_usize][y_usize] = State::Closed;
}

fn expand_neighbors(current_node : &Vec<i64>, goal : [i64;2], list : &mut Vec<Vec<i64>>, board: &mut Vec<Vec<State>>){
    let x : i64 = current_node[0];
    let y : i64 = current_node[1];
    let g : i64 = current_node[2];

    for i in 0..4 {
        let x2 : i64 = x + DELTA[i][0];
        let y2 : i64 = y + DELTA[i][1];

        if check_valid_cell(x2, y2, board){
            let g2 : i64 = g + 1;
            let h2 : i64 = heuristic(x2, y2, goal[0], goal[1]);
            add_to_open_list(list, x2, y2, g2, h2, board);
        }
    }
}

fn find_path(board : &mut Vec<Vec<State>>, initial_point : [i64;2], goal : [i64;2])->Vec<Vec<State>>{
    //Create vector of open nodes
    let mut open : Vec<Vec<i64>> = Vec::new();

    //initialize the starting node
    let x : i64 = initial_point[0];
    let y : i64 = initial_point[1];
    let g : i64 = 0;
    let h : i64 = heuristic(x, y, goal[0], goal[1]);
    //add starting node to the list of possible solution paths
    add_to_open_list(&mut open, x, y, g, h, board);

    while open.len() > 0{
        //get the next node by comparing f value, the manhattan distance to goal + steps gone already(g)
        let sorted_list_by_f_value : Vec<Vec<i64>> = cell_sort(&mut open);
        let current_cell : Vec<i64> = sorted_list_by_f_value[0].clone();
        let x : i64 = current_cell[0];
        let y : i64 = current_cell[1];
        //Add this node to path because of our algorithm prioratizing f value
        let x_usize : usize = x.try_into().unwrap();
        let y_usize : usize = y.try_into().unwrap();
        board[x_usize][y_usize] = State::Path;

        //check if we reached goal location
        if x == goal[0] && y == goal[1]{
            println!("The solution found is :   ");
            let solved_solution_at_last : Vec<Vec<State>> = board.clone();
            return solved_solution_at_last;
        }
    
        else {
            expand_neighbors(&current_cell, goal, &mut open, board);
        }
    }
    println!("The algorithm couldn't find a path from initial point to goal point! Sorry!");
    return Vec::new();
}

fn main() {
    println!("Hello, world!");



    // `s` is ASCII which represents each `char` as one byte
    let s = "hello";
    assert_eq!(s.len(), 5);

    // A `char` array with the same contents would be longer because
    // every `char` is four bytes
   

    // However, for non-ASCII strings, the difference will be smaller
    // and sometimes they are the same
    let s = "💖💖💖💖💖";

    let mountain: &str = "\u{1F3D4}";
    let car : & str = "🚗";
    assert_eq!(s.len(), 20);

    println!("{:?}",s);
    println!("{:?}",s);
    println!("{:?}",car);
    println!("{:?}",mountain);
    const THREE_HOURS_IN_SECONDS : u32 = 60*60*3;
    println!("{}",THREE_HOURS_IN_SECONDS);
    let mut counter : u32 = 0;
    let x = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}",x);
    let mut board = read_board(ARRAY_TO_SOLVE);
    let mut t : Vec<Vec<i64>> = vec![vec![2,3,4,5],vec![2,1,3,4],vec![3,4,5,6],vec![2,5,1,2],vec![3,1,2,4]];
    let sorted_list = cell_sort(&mut t);
    println!("{:?}",sorted_list);

    let solution_to_print : Vec<Vec<State>> =  find_path(&mut board, [0,0], [4,5]);

    print_board(solution_to_print);

}
