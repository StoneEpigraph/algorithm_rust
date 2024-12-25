fn is_safe(board: &[i32], row: i32, col: i32) -> bool {
    for r in 0..row {
        let c = board[r as usize];
        // 检查列是否冲突
        if c == col {
            return false;
        }
        // 检查对角线是否冲突
        if (r as i32 + c as i32) == (row + col) || (r as i32 - c as i32) == (row - col) {
            return false;
        }
    }
    true
}

fn solve_nqueens_util(board: &mut [i32], row: i32) {
    if row == 8 {
        // 找到了一个有效的解，打印出来
        for r in 0..8 {
            for c in 0..8 {
                if board[r as usize] == c {
                    print!("Q ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        println!();
        return;
    }

    for col in 0..8 {
        if is_safe(board, row, col) {
            board[row as usize] = col;
            solve_nqueens_util(board, row + 1);
        }
    }
}

fn solve_nqueens() {
    let mut board: Vec<i32> = vec![0; 8];
    solve_nqueens_util(&mut board, 0);
}

fn main() {
    solve_nqueens();
}