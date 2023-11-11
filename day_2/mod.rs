//use crate::utils;

pub fn part_1(lines: &Vec<String>) -> i32 {
    let mut total = 0;

    for line in lines {
        let op_move = line.chars().nth(0).unwrap() as i32 - 64;
        let my_move = line.chars().nth(2).unwrap() as i32 - 87;

        if my_move == op_move {
            total += 3;
        } else if my_move - op_move == 1 || my_move - op_move == -2 {
            total += 6;
        }

        total += my_move;
    }
    total
}

//rock == 1, paper == 2, scissors == 3
//loss == 0, draw == 3, win == 6
// x == lose, y == draw, z == win

pub fn part_2(lines: &Vec<String>) -> i32 {
    let mut total = 0;

    for line in lines {
        let op_move = line.chars().nth(0).unwrap() as i32 - 64;
        let my_move = line.chars().nth(2).unwrap() as i32 - 87;

        match my_move {
            1 => {
                let mut req_move = op_move - 1;
                if req_move == 0 {
                    req_move = 3
                };
                total += req_move;
            }
            2 => total += op_move + 3,
            3 => {
                let mut req_move = op_move + 1;
                if req_move == 4 {
                    req_move = 1
                };
                total += req_move + 6;
            }
            _ => (),
        }
    }
    total
}
