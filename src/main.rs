use eyre::{bail, eyre, Result};
use lazy_static::lazy_static;
use nom::IResult;
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

    let day3exinput = parseday3(
        r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
    );
    let day3input = parseday3(include_str!("day3"));
    println!("day3pt1ex: {}", day3pt1(&day3exinput));
    println!("day3pt1: {}", day3pt1(&day3input));
    println!("day3pt2ex: {}", day3pt2(&day3exinput));
    println!("day3pt2: {}", day3pt2(&day3input));

    let day4exinput = parseday4(
        r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
"#,
    )
    .unwrap()
    .1;
    let day4input = parseday4(include_str!("day4")).unwrap().1;
    println!("day4pt1ex: {}", day4pt1(&day4exinput.0, &day4exinput.1));
    println!("day4pt1: {}", day4pt1(&day4input.0, &day4input.1));
    println!("day4pt2ex: {}", day4pt2(&day4exinput.0, &day4exinput.1));
    println!("day4pt2: {}", day4pt2(&day4input.0, &day4input.1));
}

#[derive(Default, Clone)]
struct BingoBoard {
    board: Vec<Vec<BingoPlace>>,
}

impl BingoBoard {
    fn new_from(xs: Vec<Vec<BingoPlace>>) -> Self {
        BingoBoard { board: xs }
    }

    fn is_winner(&self) -> bool {
        for i in 0..5 {
            // x axis
            if (0..5).all(|x| self.board[i][x].is_selected()) {
                return true;
            }
            // y axis
            if (0..5).all(|y| self.board[y][i].is_selected()) {
                return true;
            }
        }
        false
    }

    fn mark(&mut self, n: usize) -> bool {
        for x in 0..5 {
            for y in 0..5 {
                if BingoPlace::Unselected(n) == self.board[y][x] {
                    // XXX: I'm assuming that numbers don't repeat in a board like in normal bingo
                    self.board[y][x].select();
                    return true;
                }
            }
        }
        false
    }

    fn unmarked_sum(&self) -> usize {
        self.board
            .iter()
            .flat_map(|row| row.iter())
            .map(|place| {
                if let BingoPlace::Unselected(x) = place {
                    *x
                } else {
                    0
                }
            })
            .sum()
    }

    fn parse(raw: &str) -> IResult<&str, BingoBoard> {
        use nom::character::complete::{digit1, newline, space0, space1};
        use nom::multi::separated_list1;

        let mut board = Vec::with_capacity(5);
        let mut s = raw;
        for _ in 0..5 {
            let (input, _) = space0(s)?;
            let (input, row) = separated_list1(space1, digit1)(input)?;
            let (input, _) = newline(input)?;

            let vals: Vec<BingoPlace> = row
                .iter()
                .map(|x| BingoPlace::Unselected(x.parse::<usize>().unwrap()))
                .collect();

            board.push(vals);
            s = input;
        }

        Ok((s, BingoBoard::new_from(board)))
    }
}

#[derive(PartialEq, Clone)]
enum BingoPlace {
    Selected(usize),
    Unselected(usize),
}

impl BingoPlace {
    fn is_selected(&self) -> bool {
        if let &BingoPlace::Selected(_) = self {
            true
        } else {
            false
        }
    }

    fn select(&mut self) {
        if let BingoPlace::Unselected(v) = *self {
            *self = BingoPlace::Selected(v)
        }
    }
}

fn parseday4(raw: &str) -> IResult<&str, (Vec<usize>, Vec<BingoBoard>)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, newline};
    use nom::multi::{count, separated_list1};

    let (input, calls) = separated_list1(tag(","), digit1)(raw)?;
    let (input, _) = count(newline, 2)(input)?;
    let (input, boards) = separated_list1(newline, BingoBoard::parse)(input)?;

    let calls = calls.iter().map(|x| x.parse::<usize>().unwrap()).collect();

    Ok((input, (calls, boards)))
}

fn day4pt1(numbers: &[usize], boards: &[BingoBoard]) -> usize {
    let mut boards: Vec<BingoBoard> = boards.iter().map(|b| b.clone()).collect();
    for call in numbers {
        for b in boards.iter_mut() {
            b.mark(*call);
            if b.is_winner() {
                let unselected = b.unmarked_sum();
                return unselected * call;
            }
        }
    }
    panic!("no winner");
}

fn day4pt2(numbers: &[usize], boards: &[BingoBoard]) -> usize {
    let mut boards: Vec<BingoBoard> = boards.iter().map(|b| b.clone()).collect();
    for call in numbers {
        boards.iter_mut().for_each(|b| {
            b.mark(*call);
        });
        let mut unwon = boards.iter().filter(|b| !b.is_winner());
        let last = unwon.next().unwrap();
        if unwon.next().is_none() {
            return day4pt1(numbers, &[last.clone()]);
        }
    }
    panic!("no single final choice");
}

fn parseday3(raw: &str) -> Vec<u16> {
    raw.lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect()
}

fn day3pt1(xs: &[u16]) -> usize {
    let (gamma, epsilon) = gamma_epsilon(xs);

    gamma * epsilon
}

fn gamma_epsilon(xs: &[u16]) -> (usize, usize) {
    let len = xs.len();
    let counts = xs.iter().fold([0usize; 16], |acc, x| {
        let mut a = acc;
        for i in 0..16 {
            a[i] += ((x >> i) & 0x1) as usize;
        }
        a
    });

    let gamma = counts.iter().enumerate().fold(0, |gamma, (i, x)| {
        if x >= &(len - x) {
            // 1 is most common
            gamma | (0x1 << i)
        } else {
            // 0 is most common
            gamma
        }
    });

    let bits = (1..=16).find(|&n| gamma < 2usize.pow(n)).unwrap() - 1;
    let mask = 2usize.pow(bits + 1) - 1;
    let epsilon = gamma ^ mask;

    (gamma, epsilon)
}

fn day3pt2(xs: &[u16]) -> usize {
    let (gamma, _) = gamma_epsilon(xs);
    let digits = (1..=16).find(|&n| gamma < 2usize.pow(n)).unwrap();

    let ox_rating = {
        let mut ox_pool: Vec<u16> = xs.into();
        for i in (0..digits).rev() {
            if ox_pool.len() == 1 {
                break;
            }
            if ox_pool.is_empty() {
                panic!("exhausted pool")
            }
            let (gamma, _) = gamma_epsilon(&ox_pool);
            let val = ((gamma >> i) & 0x1) as u16;

            ox_pool = ox_pool
                .drain(..)
                .filter(|x| ((x >> i) & 0x1) == val)
                .collect();
        }
        ox_pool.pop().unwrap()
    };

    let co_rating = {
        let mut co_pool: Vec<u16> = xs.into();
        for i in (0..digits).rev() {
            if co_pool.len() == 1 {
                break;
            }
            if co_pool.is_empty() {
                panic!("exhausted pool")
            }
            let (gamma, _) = gamma_epsilon(&co_pool);
            let val = ((gamma >> i) & 0x1) as u16;

            co_pool = co_pool
                .drain(..)
                .filter(|x| ((x >> i) & 0x1) != val)
                .collect();
        }
        if co_pool.len() != 1 {
            dbg!(&co_pool);
            panic!("pool size unexpected");
        }
        co_pool.pop().unwrap()
    };

    ox_rating as usize * co_rating as usize
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
