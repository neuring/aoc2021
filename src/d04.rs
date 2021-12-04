use anyhow::anyhow;

#[derive(Clone)]
struct Board<T> {
    tiles: Vec<T>,
    size: u32,
}

impl<T: Clone> Board<T> {
    fn new(size: u32, value: T) -> Self {
        let tiles = vec![value; size as usize * size as usize];

        Self { tiles, size }
    }

    fn get(&self, x: i32, y: i32) -> Option<&T> {
        if 0 > x && x >= self.size as i32 {
            return None;
        }
        if 0 > y && y >= self.size as i32 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;
        let size = self.size as usize;

        Some(&self.tiles[x + y * size])
    }

    fn set(&mut self, x: i32, y: i32, value: T) {
        assert!(0 <= x);
        assert!(0 <= y);
        assert!(x < self.size as i32);
        assert!(y < self.size as i32);

        let x = x as usize;
        let y = y as usize;
        let size = self.size as usize;

        self.tiles[x + y * size] = value;
    }
}

impl Board<bool> {
    fn bingo_at(&self, x: u32, y: u32) -> bool {
        let x = x as i32;
        let y = y as i32;

        let dx1 = (x..self.size as _)
            .take_while(|&x| *self.get(x, y).unwrap_or(&false))
            .count();
        let dx2 = (0..x)
            .rev()
            .take_while(|&x| *self.get(x, y).unwrap_or(&false))
            .count();
        let vertical = dx1 + dx2;

        let dy1 = (y..self.size as _)
            .take_while(|&y| *self.get(x, y).unwrap_or(&false))
            .count();
        let dy2 = (0..y)
            .rev()
            .take_while(|&y| *self.get(x, y).unwrap_or(&false))
            .count();
        let horizontal = dy1 + dy2;

        vertical >= 5 || horizontal >= 5
    }
}

#[derive(Debug)]
struct BoardMap {
    map: Vec<Option<(u32, u32)>>,
}

#[derive(Debug)]
struct ParseResult {
    sequence: Vec<u32>,
    boards: Vec<BoardMap>,
}

fn parse_number_sequence(sequence: &str) -> anyhow::Result<Vec<u32>> {
    Ok(sequence
        .trim()
        .split(',')
        .map(|n| n.trim().parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?)
}

fn parse_bingo_board(board: &str, max_value: u32) -> anyhow::Result<BoardMap> {
    let board = board
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>())
                .enumerate()
                .map(move |(x, n)| -> anyhow::Result<(u32, u32, u32)> {
                    Ok((x as _, y as _, n?))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut result = vec![None; 1 + max_value as usize];

    board
        .into_iter()
        .for_each(|(x, y, n)| result[n as usize] = Some((x, y)));

    Ok(BoardMap { map: result })
}

fn parse(text: &str) -> anyhow::Result<ParseResult> {
    let mut elements = text.trim().split("\n\n");

    let sequence = elements
        .next()
        .ok_or(anyhow!("Missing number sequencea at beginning"))?;

    let sequence = parse_number_sequence(sequence)?;

    let max_num = sequence
        .iter()
        .copied()
        .max()
        .ok_or(anyhow!("Empty number sequence"))?;

    let boards = elements
        .map(|e| parse_bingo_board(e, max_num))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ParseResult { sequence, boards })
}

fn sum_unmarked(placed: &Board<bool>, map: &BoardMap) -> u32 {
    let sum_unmarked: u32 = map
        .map
        .iter()
        .enumerate()
        .filter_map(|(num, x)| x.as_ref().map(|x| (num, x)))
        .filter(|(_, &(x, y))| !*placed.get(x as _, y as _).unwrap())
        .map(|(num, _)| num as u32)
        .sum();

    sum_unmarked
}

pub fn entry1(text: &str) -> anyhow::Result<u32> {
    let ParseResult { sequence, boards } = parse(text)?;

    let placed = vec![Board::new(5, false); boards.len()];

    let mut boards: Vec<_> = placed.into_iter().zip(boards).collect();

    for num in sequence {
        for (board, map) in boards.iter_mut() {
            let Some((x, y)) = map.map[num as usize] else {
                continue
            };

            board.set(x as _, y as _, true);

            if board.bingo_at(x, y) {
                let sum_unmarked = sum_unmarked(&*board, map);

                return Ok(sum_unmarked * num);
            }
        }
    }

    Err(anyhow!("No bingo occured."))
}

pub fn entry2(text: &str) -> anyhow::Result<u32> {
    let ParseResult { sequence, boards } = parse(text)?;

    let placed = vec![Board::new(5, false); boards.len()];

    let mut boards: Vec<_> = placed.into_iter().zip(boards).collect();

    for num in sequence {
        let last_board = boards
            .drain_filter(|(placed, map)| {
                let Some((x, y)) = map.map[num as usize] else {
                    return false;
                };

                placed.set(x as _, y as _, true);

                placed.bingo_at(x, y)
            })
            .last();

        if boards.is_empty() {
            let result_board = last_board
                .expect("Once board is empty, it has to have removed the last.");
            let sum_unmarked = sum_unmarked(&result_board.0, &result_board.1);
            return Ok(sum_unmarked * num);
        }
    }

    Err(anyhow!("No bingo occured."))
}
