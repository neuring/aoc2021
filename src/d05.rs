use anyhow::{anyhow, Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        let len = dx.abs().max(dy.abs());

        let div = num::integer::gcd(dx, dy);

        let dx = dx / div;
        let dy = dy / div;

        assert!([-1, 0, 1].contains(&dx));
        assert!([-1, 0, 1].contains(&dy));

        (0..=len).map(move |t| Point {
            x: self.start.x + dx * t,
            y: self.start.y + dy * t,
        })
    }
}
fn parse_point(point: &str) -> anyhow::Result<Point> {
    let mut coords = point.trim().split(',');

    let x = coords.next().ok_or(anyhow!("Missing x coordinate."))?;
    let x = x
        .parse::<i32>()
        .with_context(|| format!("Invalid x coord '{}'", x))?;
    let y = coords.next().ok_or(anyhow!("Missing y coordinate."))?;
    let y = y
        .parse::<i32>()
        .with_context(|| format!("Invalid y coord '{}'", y))?;

    Ok(Point { x, y })
}

fn parse_line(line: &str) -> anyhow::Result<Line> {
    let mut points = line.trim().split("->");

    let start = points
        .next()
        .map(parse_point)
        .ok_or(anyhow!("Missing start point."))?
        .context("When parsing start point.")?;
    let end = points
        .next()
        .map(parse_point)
        .ok_or(anyhow!("Missing end point."))?
        .context("When parsing end point.")?;

    Ok(Line { start, end })
}

fn parse(input: &str) -> anyhow::Result<Vec<Line>> {
    input
        .trim()
        .lines()
        .map(parse_line)
        .enumerate()
        .map(|(line_nr, line)| {
            line.with_context(|| format!("In line nr {}", line_nr + 1))
        })
        .collect::<Result<Vec<_>, _>>()
}

struct Field {
    width: usize,
    height: usize,

    data: Vec<u8>,
}

impl Field {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> &mut u8 {
        let width = self.width as i32;
        let height = self.height as i32;

        assert!(0 <= x);
        assert!(0 <= y);
        assert!(x < width);
        assert!(y < height);

        &mut self.data[(x + y * width) as usize]
    }

    fn place_line(&mut self, line: &Line) {
        for Point { x, y } in line.iter_points() {
            let v = self.get_mut(x, y);
            *v = v.saturating_add(1);
        }
    }

    fn iter_points(&self) -> impl Iterator<Item = u8> + '_ {
        self.data.iter().copied()
    }
}

fn solve(text: &str, line_filter: impl Fn(&Line) -> bool) -> anyhow::Result<usize> {
    let lines = parse(text)?;
    let lines: Vec<_> = lines.into_iter().filter(line_filter).collect();

    let width = lines
        .iter()
        .flat_map(|l| [l.start.x, l.end.x])
        .max()
        .ok_or(anyhow!("No lines in input"))?
        + 1;
    let height = lines
        .iter()
        .flat_map(|l| [l.start.y, l.end.y])
        .max()
        .ok_or(anyhow!("No lines in input"))?
        + 1;

    assert!(width >= 0);
    assert!(height >= 0);

    let mut field = Field::new(width as usize, height as usize);

    for line in &lines {
        field.place_line(line);
    }

    let result = field.iter_points().filter(|&i| i >= 2).count();
    Ok(result)
}

pub fn part1(text: &str) -> anyhow::Result<usize> {
    solve(text, |line| line.is_horizontal() || line.is_vertical())
}

pub fn part2(text: &str) -> anyhow::Result<usize> {
    solve(text, |_| true)
}
