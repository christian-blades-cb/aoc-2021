use eyre::{bail, eyre, Result};
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let day1input = parseday1(include_str!("day1"));
    let day1exinput = parseday1(
        r#"199
200
208
210
200
207
240
269
260
263
"#,
    );
    println!("day1pt1ex: {}", day1pt1(&day1exinput));
    println!("day1pt1: {}", day1pt1(&day1input));
    println!("day1pt2ex: {}", day1pt2(&day1exinput));
    println!("day1pt2: {}", day1pt2(&day1input));

    let day2exinput = parseday2(
        r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#,
    );
    let day2input = parseday2(include_str!("day2"));
    println!("day2pt1ex: {}", day2pt1(&day2exinput));
    println!("day2pt1: {}", day2pt1(&day2input));
    println!("day2pt2ex: {}", day2pt2(&day2exinput));
    println!("day2pt2: {}", day2pt2(&day2input));
}

lazy_static! {
    static ref RE_DIRECTION: Regex = Regex::new(r#"(\w+) (\d+)"#).unwrap();
}

enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn parse(line: &str) -> Result<(Direction, isize)> {
        let c = RE_DIRECTION
            .captures(line)
            .ok_or_else(|| eyre!("line does not match"))?;
        let dir = match c.get(1).unwrap().as_str() {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            invalid => bail!("unexpected direction {}", invalid),
        };
        let unit = c.get(2).unwrap().as_str().parse::<isize>()?;
        Ok((dir, unit))
    }
}

fn parseday2(x: &str) -> Vec<(Direction, isize)> {
    x.lines().map(|l| Direction::parse(l).unwrap()).collect()
}

fn day2pt1(xs: &[(Direction, isize)]) -> isize {
    let (h, d) = xs
        .iter()
        .fold((0isize, 0isize), |(h, d), (dir, unit)| match dir {
            Direction::Forward => (h + unit, d),
            Direction::Up => (h, d - unit),
            Direction::Down => (h, d + unit),
        });
    h * d
}

fn day2pt2(xs: &[(Direction, isize)]) -> isize {
    let (_, h, d) = xs.iter().fold(
        (0isize, 0isize, 0isize),
        |(aim, h, d), (dir, unit)| match dir {
            Direction::Forward => (aim, h + unit, d + (aim * unit)),
            Direction::Up => (aim - unit, h, d),
            Direction::Down => (aim + unit, h, d),
        },
    );
    h * d
}

fn parseday1(x: &str) -> Vec<usize> {
    x.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

fn day1pt1(xs: &[usize]) -> usize {
    xs.iter()
        .zip(xs.iter().skip(1))
        .fold(0, |acc, (a, b)| acc + if b > a { 1 } else { 0 })
}

fn day1pt2(xs: &[usize]) -> usize {
    let window_sums: Vec<usize> = xs
        .iter()
        .zip(xs.iter().skip(1))
        .zip(xs.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();
    window_sums
        .iter()
        .zip(window_sums.iter().skip(1))
        .fold(0, |acc, (a, b)| acc + if b > a { 1 } else { 0 })
}
