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
