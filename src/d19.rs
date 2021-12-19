use std::collections::HashSet;

use itertools::iproduct;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};

fn parse_point(input: &str) -> Vector3<i32> {
    Vector3::from_iterator(
        input.trim().split(",").map(|v| v.parse::<i32>().unwrap()),
    )
}

fn parse_scanner_points(input: &str) -> Vec<Vector3<i32>> {
    input.trim().lines().skip(1).map(parse_point).collect()
}

fn parse(input: &str) -> Vec<Vec<Vector3<i32>>> {
    input
        .trim()
        .split("\n\n")
        .map(parse_scanner_points)
        .collect()
}

fn collect_distance_set(a: &[Vector3<i32>]) -> Vec<HashSet<i32>> {
    a.iter()
        .enumerate()
        .map(|(i, p)| {
            a.iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, o)| (p - o).dot(&(p - o)))
                .collect()
        })
        .collect()
}

fn try_align_recursive(
    i: usize,
    distances_a: &[HashSet<i32>],
    distances_b: &[HashSet<i32>],
    current_assignment: &mut Vec<Option<usize>>,
    taken: &mut Vec<bool>,
    result: &mut Vec<Vec<Option<usize>>>,
) {
    assert_eq!(current_assignment.len(), i);
    assert!(i <= distances_a.len());

    if i == distances_a.len() {
        if current_assignment.iter().filter(|i| i.is_some()).count() >= 12 {
            result.push(current_assignment.clone());
        }
        return;
    }

    for j in 0..distances_b.len() {
        if taken[j] {
            continue;
        } else if distances_a[i].intersection(&distances_b[j]).count() >= 11 {
            current_assignment.push(Some(j));
            taken[j] = true;

            try_align_recursive(
                i + 1,
                distances_a,
                distances_b,
                current_assignment,
                taken,
                result,
            );

            taken[j] = false;
            current_assignment.pop();
        }
    }

    current_assignment.push(None);
    try_align_recursive(
        i + 1,
        distances_a,
        distances_b,
        current_assignment,
        taken,
        result,
    );
    current_assignment.pop();
}

fn try_align(a: &[Vector3<i32>], b: &[Vector3<i32>]) -> Vec<Vec<Option<usize>>> {
    let distances_a = collect_distance_set(a);
    let distances_b = collect_distance_set(b);

    /*
    for (i, da) in distances_a.iter().enumerate() {
        println!("d({}, {}, {}) = {:?}", a[i][0], a[i][1], a[i][2], da);
    }
    println!();
    for (i, da) in distances_b.iter().enumerate() {
        println!("d({}, {}, {}) = {:?}", b[i][0], b[i][1], b[i][2], da);
    }
    */

    let mut result = Vec::new();

    try_align_recursive(
        0,
        &distances_a,
        &distances_b,
        &mut Vec::new(),
        &mut vec![false; b.len()],
        &mut result,
    );

    result
}

fn is_valid_alignment(
    a: &[Vector3<i32>],
    b: &[Vector3<i32>],
    alignment: &[Option<usize>],
) -> Option<Matrix4<i32>> {
    let pairs = || {
        alignment
            .iter()
            .enumerate()
            .filter_map(|(i, t)| t.map(|o| (a[i], b[o])))
    };

    'transform: for transform in all_transforms() {
        let mut pairs = pairs();

        let (a, b) = pairs.next().unwrap();
        let transformed_b = transform * b;

        let distance = transformed_b - a;

        for (a, b) in pairs {
            let transformed_b = transform * b;
            if transformed_b - a != distance {
                continue 'transform;
            } else {
            }
        }

        let t = transform.insert_row(3, 0);
        let mut t = t.insert_column(3, 0);

        let translation = a - transformed_b;

        t[(0, 3)] = translation[0];
        t[(1, 3)] = translation[1];
        t[(2, 3)] = translation[2];
        t[(3, 3)] = 1;

        return Some(t);
    }

    return None;
}

fn all_transforms() -> impl Iterator<Item = Matrix3<i32>> {
    let cos = |i: i32| match i.rem_euclid(4) {
        0 => 1,
        1 => 0,
        2 => -1,
        3 => 0,
        _ => unreachable!(),
    };
    let sin = move |i: i32| cos(i - 3);

    let x_rotations = (0..4)
        .map(move |i| Matrix3::new(1, 0, 0, 0, cos(i), -sin(i), 0, sin(i), cos(i)));

    (0..4)
        .map(move |i| Matrix3::new(cos(i), 0, sin(i), 0, 1, 0, -sin(i), 0, cos(i)))
        .chain(
            [1, -1].map(|i| {
                Matrix3::new(cos(i), -sin(i), 0, sin(i), cos(i), 0, 0, 0, 1)
            }),
        )
        .flat_map(move |m| x_rotations.clone().map(move |x_rot| x_rot * m))
}

fn calc_all_scanner_transforms(input: &[Vec<Vector3<i32>>]) -> Vec<Matrix4<i32>> {
    let mut done = vec![false; input.len()];
    let mut d = Vec::new();

    let mut transforms: Vec<Matrix4<i32>> = vec![Matrix4::identity(); input.len()];

    done[0] = true;
    d.push(0);

    let mut i = 0;

    while i < d.len() {
        let a_idx = d[i];

        let a = &input[a_idx];

        'outer_for: for b_idx in input.iter().enumerate().map(|(i, _)| i) {
            if done[b_idx] {
                continue 'outer_for;
            }

            let b = &input[b_idx];
            let alignment = try_align(&a, &b);

            for alignment in alignment {
                if let Some(transform) = is_valid_alignment(a, b, &alignment) {
                    let total_transform = transforms[a_idx] * transform;

                    transforms[b_idx] = total_transform;
                    done[b_idx] = true;
                    d.push(b_idx);
                    continue 'outer_for;
                }
            }
        }

        i += 1;
    }

    assert!(done.iter().all(|i| *i), "{:?}", done);

    transforms
}

pub fn part1(text: &str) -> anyhow::Result<usize> {
    let input = parse(text);

    let transforms = calc_all_scanner_transforms(&input);

    let mut points = HashSet::new();

    for (idx, scanner) in input.into_iter().enumerate() {
        for p in scanner {
            points.insert(transforms[idx] * p.insert_row(3, 1));
        }
    }

    Ok(points.len())
}

pub fn part2(text: &str) -> anyhow::Result<i32> {
    let input = parse(text);

    let transforms = calc_all_scanner_transforms(&input);

    let scanner_pos: Vec<_> = transforms
        .iter()
        .map(|t| t * Vector4::<i32>::new(0, 0, 0, 1))
        .collect();

    let max = iproduct!(&scanner_pos, &scanner_pos)
        .map(|(a, b)| (a - b).abs().iter().sum::<i32>())
        .max()
        .unwrap();

    Ok(max)
}
