pub struct Advent;

fn evolve(board: &Vec<Vec<u8>>, board2: &mut Vec<Vec<u8>>) {
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            board2[r][c] = {
                let mut occupied_neighbors = 0;

                for r2 in
                    if r > 0 { r - 1 } else { r }..=if r < board.len() - 1 { r + 1 } else { r }
                {
                    for c2 in if c > 0 { c - 1 } else { c }..=if c < board[r].len() - 1 {
                        c + 1
                    } else {
                        c
                    } {
                        if !(c == c2 && r == r2) {
                            let occupado: i32 = (board[r2][c2] == b'#').into();
                            occupied_neighbors += occupado;
                        }
                    }
                }

                if board[r][c] == b'#' && occupied_neighbors >= 4 {
                    b'L'
                } else if board[r][c] == b'L' && occupied_neighbors == 0 {
                    b'#'
                } else {
                    board[r][c]
                }
            }
        }
    }
}

fn raycast_neighbors(board: &Vec<Vec<u8>>, r: usize, c: usize) -> i32 {
    let mut occupied_neighbors = 0;

    for ray in [
        (-1i32, -1i32),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    {
        let mut v: (i32, i32) = (c as i32, r as i32);
        loop {
            v = (v.0 + ray.0, v.1 + ray.1);
            if 0 <= v.0
                && (v.0 as usize) < board[r].len()
                && 0 <= v.1
                && (v.1 as usize) < board.len()
            {
                match board[v.1 as usize][v.0 as usize] {
                    b'#' => {
                        occupied_neighbors += 1;
                        break;
                    }
                    b'L' => break,
                    _ => {}
                }
            } else {
                break;
            }
        }
    }

    occupied_neighbors
}

fn evolve2(board: &Vec<Vec<u8>>, board2: &mut Vec<Vec<u8>>) {
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            board2[r][c] = {
                let occupied_neighbors = raycast_neighbors(board, r, c);

                if board[r][c] == b'#' && occupied_neighbors >= 5 {
                    b'L'
                } else if board[r][c] == b'L' && occupied_neighbors == 0 {
                    b'#'
                } else {
                    board[r][c]
                }
            }
        }
    }
}

fn num_occupied(board: &Vec<Vec<u8>>) -> usize {
    board
        .iter()
        .map(|r| r.iter().filter(|&&c| c == b'#').count())
        .sum()
}

fn print_board(board: &Vec<Vec<u8>>) {
    println!(
        "{}\n",
        board
            .iter()
            .map(|r| std::str::from_utf8(r).unwrap())
            .collect::<Vec<_>>()
            .join("\n")
    )
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        11
    }

    fn main1(input: &String) -> String {
        let mut board1 = input
            .lines()
            .map(|line| line.into())
            .collect::<Vec<Vec<u8>>>();
        let mut board2 = board1.clone();
        let mut prev_occupied = 0;
        for i in 0.. {
            let occupied = if i % 2 == 0 {
                evolve(&board1, &mut board2);
                num_occupied(&board2)
            } else {
                evolve(&board2, &mut board1);
                num_occupied(&board1)
            };
            if occupied == prev_occupied {
                return format!("{}", occupied);
            }
            prev_occupied = occupied
        }
        panic!();
    }

    fn main2(input: &String) -> String {
        let mut board1 = input
            .lines()
            .map(|line| line.into())
            .collect::<Vec<Vec<u8>>>();
        let mut board2 = board1.clone();
        let mut prev_occupied = 0;
        for i in 0.. {
            let occupied = if i % 2 == 0 {
                evolve2(&board1, &mut board2);
                num_occupied(&board2)
            } else {
                evolve2(&board2, &mut board1);
                num_occupied(&board1)
            };
            if occupied == prev_occupied {
                return format!("{}", occupied);
            }
            prev_occupied = occupied
        }
        panic!();
    }
}
