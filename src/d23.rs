use std::collections::{BinaryHeap, HashMap};

use anyhow::bail;
use itertools::{Either, Itertools};

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumIter)]
enum AmphiType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphiType {
    fn bucket_idx(&self) -> usize {
        match self {
            AmphiType::Amber => 0,
            AmphiType::Bronze => 1,
            AmphiType::Copper => 2,
            AmphiType::Desert => 3,
        }
    }

    fn move_cost(&self) -> u32 {
        match self {
            AmphiType::Amber => 1,
            AmphiType::Bronze => 10,
            AmphiType::Copper => 100,
            AmphiType::Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Empty,
    Amphi(AmphiType),
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Amphi(AmphiType::Amber) => 'A',
            Tile::Amphi(AmphiType::Bronze) => 'B',
            Tile::Amphi(AmphiType::Copper) => 'C',
            Tile::Amphi(AmphiType::Desert) => 'D',
        }
    }
}

fn parse(grid: &str) -> anyhow::Result<Grid<Tile>> {
    let height = grid.trim().lines().count();
    let width = grid.trim().lines().next().unwrap().trim().chars().count();

    let grid: Result<Vec<Tile>, _> = grid
        .trim()
        .lines()
        .flat_map(|l| l.chars().pad_using(width, |_| '#'))
        .map(|c| match c {
            '.' => Ok(Tile::Empty),
            ' ' | '#' => Ok(Tile::Wall),
            'A' => Ok(Tile::Amphi(AmphiType::Amber)),
            'B' => Ok(Tile::Amphi(AmphiType::Bronze)),
            'C' => Ok(Tile::Amphi(AmphiType::Copper)),
            'D' => Ok(Tile::Amphi(AmphiType::Desert)),
            _ => bail!("Illegal tile '{}'.", c),
        })
        .collect();

    let grid = grid?;

    Ok(Grid::from_rows_columns(width, height, grid))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: Vec<Option<AmphiType>>,
    buckets: Vec<Vec<AmphiType>>,
    max_bucket_size: usize,
}

fn build_initial_state(grid: &Grid<Tile>) -> State {
    let mut amphis: Vec<_> = grid
        .iter_coords::<i32>()
        .filter_map(|a| match a.2 {
            Tile::Amphi(amphi) => Some((a.0, a.1, amphi)),
            _ => None,
        })
        .collect();

    amphis.sort_by_key(|(x, y, _)| (*x, -*y));

    let mut buckets = Vec::new();
    for (_, iter) in &amphis.into_iter().group_by(|(x, _, _)| *x) {
        buckets.push(iter.map(|(_, _, amphi)| *amphi).collect::<Vec<_>>());
    }

    let hallway_len = buckets.len() * 2 + 3;

    State {
        hallway: vec![None; hallway_len],
        max_bucket_size: buckets[0].len(),
        buckets,
    }
}

fn bucket_to_hallway_pos(bucket_idx: usize) -> usize {
    bucket_idx * 2 + 2
}

fn is_pausable_space(&hallway_pos: &usize) -> bool {
    hallway_pos < 2 || hallway_pos % 2 == 1 || hallway_pos > 8
}

fn bucket_amphi_type(bucket_idx: usize) -> AmphiType {
    match bucket_idx {
        0 => AmphiType::Amber,
        1 => AmphiType::Bronze,
        2 => AmphiType::Copper,
        3 => AmphiType::Desert,
        _ => unreachable!(),
    }
}

fn start_end_iter(start: usize, end: usize) -> impl Iterator<Item = usize> {
    if start <= end {
        Either::Left(start..end + 1)
    } else {
        Either::Right((end..start + 1).rev())
    }
}

/// returns cost of state transition and next state.
fn adjacent_states(state: &State) -> Vec<(u32, State)> {
    let mut result = Vec::new();

    // Move out of buckets
    for (bucket_idx, bucket) in state.buckets.iter().enumerate() {
        if bucket.iter().all(|&a| a == bucket_amphi_type(bucket_idx)) {
            continue;
        }

        if let Some(top_amphi) = bucket.last() {
            let hallway_pos = bucket_to_hallway_pos(bucket_idx);

            let positions = (hallway_pos..state.hallway.len())
                .take_while(|&i| state.hallway[i].is_none())
                .chain(
                    (0..hallway_pos)
                        .rev()
                        .take_while(|&i| state.hallway[i].is_none()),
                )
                .filter(is_pausable_space);

            for pos in positions {
                let vertical_len = (state.max_bucket_size
                    - state.buckets[bucket_idx].len()
                    + 1) as u32;
                let horizontal_len = (hallway_pos as i64 - pos as i64).abs() as u32;
                let cost = (vertical_len + horizontal_len) * top_amphi.move_cost();

                let mut new_state = state.clone();
                new_state.buckets[bucket_idx].pop();
                new_state.hallway[pos] = Some(*top_amphi);
                result.push((cost, new_state));
            }
        }
    }

    // Move into bucket
    for (pos, amphi) in state
        .hallway
        .iter()
        .enumerate()
        .filter_map(|(pos, a)| a.map(|a| (pos, a)))
    {
        let amphi_bucket_idx = amphi.bucket_idx();
        let bucket_pos = bucket_to_hallway_pos(amphi_bucket_idx);

        // Bucket is ready for entering. (No other amphi that need to move out first.)
        if state.buckets[amphi_bucket_idx]
            .iter()
            .any(|&a| a != bucket_amphi_type(amphi_bucket_idx))
        {
            continue;
        }

        // Other amphi in the way
        if start_end_iter(pos, bucket_pos)
            .skip(1)
            .any(|i| state.hallway[i].is_some())
        {
            continue;
        }

        let horizontal_len = (pos as i64 - bucket_pos as i64).abs() as u32;
        let vertical_len =
            (state.max_bucket_size - state.buckets[amphi_bucket_idx].len()) as u32;
        let cost = (horizontal_len + vertical_len) * amphi.move_cost();

        let mut new_state = state.clone();
        new_state.hallway[pos] = None;
        new_state.buckets[amphi_bucket_idx].push(amphi);
        result.push((cost, new_state));
    }

    result
}

#[derive(Ord, Eq)]
struct QueueEntry {
    id: u64,
    cost: u32,
}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost).map(|a| a.reverse())
    }
}

struct Solver {
    state_to_id: HashMap<State, u64>,
    id_to_state: HashMap<u64, State>,
    next_state_id: u64,

    target_state: State,

    visited: HashMap<u64, u32>, // stores cost for visited.
    queue: BinaryHeap<QueueEntry>,
}

impl Solver {
    fn new(start_state: State, target_state: State) -> Self {
        let mut solver = Self {
            target_state,
            state_to_id: HashMap::new(),
            id_to_state: HashMap::new(),
            next_state_id: 1,
            visited: HashMap::new(),
            queue: BinaryHeap::new(),
        };

        let id = solver.add_state(start_state);
        solver.queue.push(QueueEntry { id, cost: 0 });

        solver
    }

    fn add_state(&mut self, state: State) -> u64 {
        if self.state_to_id.contains_key(&state) {
            return self.state_to_id[&state];
        }

        let id = self.next_state_id;
        self.next_state_id += 1;

        self.state_to_id.insert(state.clone(), id);
        self.id_to_state.insert(id, state);

        id
    }

    fn search(&mut self) -> u32 {
        while let Some(QueueEntry { id, cost }) = self.queue.pop() {
            if self
                .visited
                .get(&id)
                .map(|&known_smallest_cost| known_smallest_cost < cost)
                .unwrap_or(false)
            {
                continue;
            }

            let state = self.id_to_state.get(&id).unwrap();
            //eprintln!("----------------------------------------------");
            //dbg!(id);
            //dbg!(cost);
            //dbg!(state);

            for (move_cost, next_state) in adjacent_states(state) {
                let new_state_cost = move_cost + cost;

                //dbg!(new_state_cost);
                //dbg!(&next_state);

                if &next_state == &self.target_state {
                    return new_state_cost;
                }

                let next_state_id = self.add_state(next_state);
                //dbg!(next_state_id);

                if self
                    .visited
                    .get(&next_state_id)
                    .map(|&known_smallest_cost| known_smallest_cost > new_state_cost)
                    .unwrap_or(true)
                {
                    self.visited.insert(next_state_id, new_state_cost);
                    self.queue.push(QueueEntry {
                        id: next_state_id,
                        cost: new_state_cost,
                    });
                }
            }
        }

        unreachable!("Target state could not be found")
    }
}

fn create_target_state(initial_state: &State) -> State {
    let mut target_state = initial_state.clone();

    target_state.hallway.iter_mut().for_each(|i| *i = None);

    for (bucket_idx, bucket) in target_state.buckets.iter_mut().enumerate() {
        bucket.fill(bucket_amphi_type(bucket_idx));
    }

    target_state
}

fn solve(input: &str) -> anyhow::Result<u32> {
    let grid = parse(input)?;

    let initial_state = build_initial_state(&grid);
    let target_state = create_target_state(&initial_state);

    //dbg!(&initial_state);
    //dbg!(&target_state);

    let mut solver = Solver::new(initial_state, target_state);

    let result = solver.search();

    Ok(result)
}

pub fn part1(input: &str) -> anyhow::Result<u32> {
    solve(input)
}

pub fn part2(input: &str) -> anyhow::Result<u32> {
    solve(input)
}
