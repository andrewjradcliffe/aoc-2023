use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse_file<F, T: AsRef<Path>>(f: F, path: T) -> io::Result<u64>
where
    F: Fn(&str) -> u8,
{
    let file = File::open(path)?;
    let mut file = BufReader::new(file);
    // It is unlikely that a single line will be larger than 1 KiB
    let mut s = String::with_capacity(1024);
    let mut sum: u64 = 0;
    while file.read_line(&mut s)? != 0 {
        sum += f(&s) as u64;
        s.clear();
    }
    Ok(sum)
}

pub fn parse_lines<F, R: BufRead>(f: F, mut r: R) -> io::Result<u64>
where
    F: Fn(&str) -> u8,
{
    let mut s = String::with_capacity(1024);
    let mut sum: u64 = 0;
    while r.read_line(&mut s)? != 0 {
        sum += f(&s) as u64;
        s.clear();
    }
    Ok(sum)
}

pub mod part1 {
    // use super::*;

    fn decimal(c: char) -> Option<u8> {
        c.to_digit(10).map(|d| d as u8)
    }
    pub fn parse_line(line: &str) -> u8 {
        let mut iter = line.chars().filter(|c| c.is_ascii_digit());
        match (iter.next().and_then(decimal), iter.last().and_then(decimal)) {
            (Some(d_1), Some(d_0)) => d_1 * 10 + d_0,
            (Some(d_0), None) => d_0 * 10 + d_0,
            _ => 0,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::*;

        // Equivalent, but potentially slower due to more branching
        fn parse_line2(line: &str) -> u8 {
            let mut iter = line.chars().filter_map(decimal);
            match (iter.next(), iter.last()) {
                (Some(d_1), Some(d_0)) => d_1 * 10 + d_0,
                (Some(d_0), None) => d_0 * 10 + d_0,
                _ => 0,
            }
        }

        #[test]
        fn parse_line_works() {
            assert_eq!(parse_line("t1"), 11);
            assert_eq!(parse_line("four9two"), 99);
            assert_eq!(parse_line("43two6eight9"), 49);

            assert_eq!(parse_line2("t1"), 11);
            assert_eq!(parse_line2("four9two"), 99);
            assert_eq!(parse_line2("43two6eight9"), 49);
        }

        #[test]
        fn parse_lines_works() {
            let s = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
            assert_eq!(
                parse_lines(parse_line, BufReader::new(s.as_bytes())).unwrap(),
                142
            );
        }
    }
}

pub mod part2 {
    /*
    It would be much more efficient to implement this logic using a rolling hash.
    At present, this performs considerably more string comparisons than are
    necessary.
    */
    fn char1_matches(s: &str) -> bool {
        match s {
            "o" | "t" | "f" | "s" | "e" | "n" | "z" => true,
            _ => false,
        }
    }
    fn char2_matches(s: &str) -> bool {
        match s {
            "on" | "tw" | "th" | "fo" | "fi" | "si" | "se" | "ei" | "ni" | "ze" => true,
            _ => false,
        }
    }
    fn char3(s: &str) -> Option<Result<u8, ()>> {
        match s {
            "one" => Some(Ok(1)),
            "two" => Some(Ok(2)),
            "six" => Some(Ok(6)),
            "thr" | "fou" | "fiv" | "sev" | "eig" | "nin" | "zer" => Some(Err(())),
            _ => None,
        }
    }
    fn char4(s: &str) -> Option<Result<u8, ()>> {
        match s {
            "four" => Some(Ok(4)),
            "five" => Some(Ok(5)),
            "nine" => Some(Ok(9)),
            "zero" => Some(Ok(0)),
            "thre" | "seve" | "eigh" => Some(Err(())),
            _ => None,
        }
    }
    fn char5(s: &str) -> Option<u8> {
        match s {
            "three" => Some(3),
            "seven" => Some(7),
            "eight" => Some(8),
            _ => None,
        }
    }

    fn char1_matches_rev(s: &str) -> bool {
        match s {
            "e" | "o" | "r" | "x" | "n" | "t"  => true,
            _ => false,
        }
    }
    fn char2_matches_rev(s: &str) -> bool {
        match s {
            "ne" | "wo" | "ee" | "ur" | "ve" | "ix" | "en" | "ht" | "ro" => true,
            _ => false,
        }
    }
    fn char3_rev(s: &str) -> Option<Result<u8, ()>> {
        match s {
            "one" => Some(Ok(1)),
            "two" => Some(Ok(2)),
            "six" => Some(Ok(6)),
            "ree" | "our" | "ive" | "ven" | "ght" | "ine" | "ero" => Some(Err(())),
            _ => None,
        }
    }
    fn char4_rev(s: &str) -> Option<Result<u8, ()>> {
        match s {
            "four" => Some(Ok(4)),
            "five" => Some(Ok(5)),
            "nine" => Some(Ok(9)),
            "zero" => Some(Ok(0)),
            "hree" | "even" | "ight" => Some(Err(())),
            _ => None,
        }
    }

    fn first_last(line: &str) -> (Option<u8>, Option<u8>) {
        let mut iter = line.char_indices();
        let mut first: Option<u8> = None;
        let mut last: Option<u8> = None;

        let mut assign = |d: u8| {
            if first.is_none() {
                first = Some(d);
            } else {
                last = Some(d);
            }
        };
        let mut left: usize = 0;
        while let Some((i, c)) = iter.next() {
            let right = i + 1;
            if let Some(d) = c.to_digit(10).map(|d| d as u8) {
                assign(d);
                left = i + 1;
            } else {
                let mut size = right - left;
                loop {
                    if size == 6 {
                        left += 1;
                        size = right - left;
                    } else if size == 5 {
                        match line.get(left..right).and_then(char5) {
                            Some(d) => {
                                assign(d);
                                left = right;
                            }
                            _ => {
                                left += 1;
                            }
                        }
                        size = right - left;
                    } else if size == 4 {
                        match line.get(left..right).and_then(char4) {
                            Some(Ok(d)) => {
                                assign(d);
                                left = right;
                            }
                            Some(Err(())) => {
                                break;
                            }
                            _ => {
                                left += 1;
                            }
                        }
                        size = right - left;
                    } else if size == 3 {
                        match line.get(left..right).and_then(char3) {
                            Some(Ok(d)) => {
                                assign(d);
                                left = right;
                            }
                            Some(Err(())) => {
                                break;
                            }
                            _ => {
                                left += 1;
                            }
                        }
                        size = right - left;
                    } else if size == 2 {
                        if line.get(left..right).is_some_and(char2_matches) {
                            break;
                        } else {
                            left += 1;
                            size = right - left;
                        }
                    } else if size == 1 {
                        if line.get(left..right).is_some_and(char1_matches) {
                            break;
                        } else {
                            left += 1;
                            size = right - left;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        (first, last)
    }
    fn first(line: &str) -> Option<u8> {
        let mut iter = line.char_indices();
        let mut left: usize = 0;
        while let Some((i, c)) = iter.next() {
            let right = i + 1;
            if let Some(d) = c.to_digit(10).map(|d| d as u8) {
                return Some(d);
            } else {
                let mut size = right - left;
                loop {
                    if size == 6 {
                        left += 1;
                        size = right - left;
                    } else if size == 5 {
                        match line.get(left..right).and_then(char5) {
                            Some(d) => return Some(d),
                            _ => {
                                left += 1;
                            }
                        }
                        size = right - left;
                    } else if size == 4 {
                        match line.get(left..right).and_then(char4) {
                            Some(Ok(d)) => return Some(d),
                            Some(Err(())) => {
                                break;
                            }
                            _ => {
                                left += 1;
                            }
                        }
                        size = right - left;
                    } else if size == 3 {
                        match line.get(left..right).and_then(char3) {
                            Some(Ok(d)) => return Some(d),
                            Some(Err(())) => {
                                break;
                            }
                            _ => {
                                left += 1;
                            }
                        }
                        size = right - left;
                    } else if size == 2 {
                        if line.get(left..right).is_some_and(char2_matches) {
                            break;
                        } else {
                            left += 1;
                            size = right - left;
                        }
                    } else if size == 1 {
                        if line.get(left..right).is_some_and(char1_matches) {
                            break;
                        } else {
                            left += 1;
                            size = right - left;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        None
    }
    fn last(line: &str) -> Option<u8> {
        let mut iter = line.char_indices().rev();
        let mut right: usize = line.chars().count() + 1;
        while let Some((i, c)) = iter.next() {
            let left = i;
            if let Some(d) = c.to_digit(10).map(|d| d as u8) {
                return Some(d);
            } else {
                let mut size = right - left;
                loop {
                    if size == 6 {
                        right -= 1;
                        size = right - left;
                    } else if size == 5 {
                        match line.get(left..right).and_then(char5) {
                            Some(d) => return Some(d),
                            _ => {
                                right -= 1;
                            }
                        }
                        size = right - left;
                    } else if size == 4 {
                        match line.get(left..right).and_then(char4_rev) {
                            Some(Ok(d)) => return Some(d),
                            Some(Err(())) => {
                                break;
                            }
                            _ => {
                                right -= 1;
                            }
                        }
                        size = right - left;
                    } else if size == 3 {
                        match line.get(left..right).and_then(char3_rev) {
                            Some(Ok(d)) => return Some(d),
                            Some(Err(())) => {
                                break;
                            }
                            _ => {
                                right -= 1;
                            }
                        }
                        size = right - left;
                    } else if size == 2 {
                        if line.get(left..right).is_some_and(char2_matches_rev) {
                            break;
                        } else {
                            right -= 1;
                            size = right - left;
                        }
                    } else if size == 1 {
                        if line.get(left..right).is_some_and(char1_matches_rev) {
                            break;
                        } else {
                            right -= 1;
                            size = right - left;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        None
    }


    pub fn parse_line(line: &str) -> u8 {
        // let first = first(line);
        // let (first, last) = first_last(line);
        let first = first(line);
        let last = last(line);
        match (first, last) {
            (Some(d_1), Some(d_0)) => d_1 * 10 + d_0,
            (Some(d_0), None) => d_0 * 10 + d_0,
            _ => 0,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::*;

        #[test]
        fn parse_line_works() {
            assert_eq!(parse_line("two1nine"), 29);
            assert_eq!(parse_line("eightwothree"), 83);
            assert_eq!(parse_line("abcone2threexyz"), 13);
            assert_eq!(parse_line("xtwone3four"), 24);
            assert_eq!(parse_line("4nineeightseven2"), 42);
            assert_eq!(parse_line("zoneight234"), 14);
            assert_eq!(parse_line("7pqrstsixteen"), 76);
            assert_eq!(parse_line("bqccqhbdgeight7"),  87);
            assert_eq!(parse_line("t1"), 11);
            assert_eq!(parse_line("gckhqpb6twoqnjxqplthree2fourkspnsnzxlz1"), 61);
            assert_eq!(parse_line("fmpvqkxgeightthreebdrng9tdcffvsfctwo"), 82);
            assert_eq!(parse_line("sevenlptpdhtjpgxconedvtrrnngn8"), 78);
            assert_eq!(parse_line("threeznnnbtfive5tmdfxtwothree3ndjcszrb"), 33);
            assert_eq!(parse_line("kkvtwone5sevenfcfnngpmjktrpxk7djgzmdthreehpp"), 23);
            assert_eq!(parse_line("24"), 24);
            assert_eq!(parse_line("oneninexqdseven4threefive"), 15);
            assert_eq!(parse_line("fcsoneightnmtgzbbnflnnlk5two"), 12);
            assert_eq!(parse_line("1qlcg4seven"), 17);
            assert_eq!(parse_line("fivefivefourfiveeight7eightwods"), 52);
            assert_eq!(parse_line("nine5threeninezgjcpssevenone"), 91);
            assert_eq!(parse_line("vrctfpbp2bdknhtwothree68ckzlgkghponeightg"), 28);
            assert_eq!(parse_line("mseven7six4five19hjd"), 79);
            assert_eq!(parse_line("21zffhnksmjj1rcdpkcrznine"), 29);
            assert_eq!(parse_line("threekp1onefrfjbrmmpmsdsvfour"), 34);
        }

        #[test]
        fn parse_lines_works() {
            let s = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
            assert_eq!(
                parse_lines(parse_line, BufReader::new(s.as_bytes())).unwrap(),
                281
            );
        }
    }
}
