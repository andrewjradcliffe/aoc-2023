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
        use super::super::*;
        use super::*;

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
            "e" | "o" | "r" | "x" | "n" | "t" => true,
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

    fn first(line: &str) -> Option<u8> {
        let mut left: usize = 0;
        for (i, c) in line.char_indices() {
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
        // This valid for UTF-8; if we assume ASCII, then `line.len() + 1`.
        let mut right: usize = line.chars().count() + 1;

        for (i, c) in line.char_indices().rev() {
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
        use super::super::*;
        use super::*;

        #[test]
        fn parse_line_works() {
            assert_eq!(parse_line("two1nine"), 29);
            assert_eq!(parse_line("eightwothree"), 83);
            assert_eq!(parse_line("abcone2threexyz"), 13);
            assert_eq!(parse_line("xtwone3four"), 24);
            assert_eq!(parse_line("4nineeightseven2"), 42);
            assert_eq!(parse_line("zoneight234"), 14);
            assert_eq!(parse_line("7pqrstsixteen"), 76);
            assert_eq!(parse_line("bqccqhbdgeight7"), 87);
            assert_eq!(parse_line("t1"), 11);
            assert_eq!(parse_line("gckhqpb6twoqnjxqplthree2fourkspnsnzxlz1"), 61);
            assert_eq!(parse_line("fmpvqkxgeightthreebdrng9tdcffvsfctwo"), 82);
            assert_eq!(parse_line("sevenlptpdhtjpgxconedvtrrnngn8"), 78);
            assert_eq!(parse_line("threeznnnbtfive5tmdfxtwothree3ndjcszrb"), 33);
            assert_eq!(
                parse_line("kkvtwone5sevenfcfnngpmjktrpxk7djgzmdthreehpp"),
                23
            );
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

// This is far too tedious without metaprogramming.
// N.B. negation on unsigned integers is not a robust hash function.
// I would expect problems if the character set were not limited to ASCII.
pub mod part2_alt {

    const OFFSET: u32 = '0' as u32;
    const BASE: u32 = 10;

    // pattern hashes, n = 1
    const Z: u32 = ('z' as u32).wrapping_neg();
    const O: u32 = ('o' as u32).wrapping_neg();
    const T: u32 = ('t' as u32).wrapping_neg();
    const F: u32 = ('f' as u32).wrapping_neg();
    const S: u32 = ('s' as u32).wrapping_neg();
    const E: u32 = ('e' as u32).wrapping_neg();
    const N: u32 = ('n' as u32).wrapping_neg();

    macro_rules! pattern_hash {
        ($var:ident, $lhs:ident, $c:literal) => {
            const $var: u32 = $lhs.wrapping_sub($c as u32);
        };
    }

    // n = 2
    pattern_hash!(ZE, Z, 'e');
    pattern_hash!(ON, O, 'n');
    pattern_hash!(TW, T, 'w');
    pattern_hash!(TH, T, 'h');
    pattern_hash!(FO, F, 'o');
    pattern_hash!(FI, F, 'i');
    pattern_hash!(SI, S, 'i');
    pattern_hash!(SE, S, 'e');
    pattern_hash!(EI, E, 'i');
    pattern_hash!(NI, N, 'i');

    //  n = 3
    pattern_hash!(ZER, ZE, 'r');
    pattern_hash!(ONE, ON, 'e');
    pattern_hash!(TWO, TW, 'o');
    pattern_hash!(THR, TH, 'r');
    pattern_hash!(FOU, FO, 'u');
    pattern_hash!(FIV, FI, 'v');
    pattern_hash!(SIX, SI, 'x');
    pattern_hash!(SEV, SE, 'v');
    pattern_hash!(EIG, EI, 'g');
    pattern_hash!(NIN, NI, 'n');

    // n = 4
    pattern_hash!(ZERO, ZER, 'o');
    pattern_hash!(THRE, THR, 'e');
    pattern_hash!(FOUR, FOU, 'r');
    pattern_hash!(FIVE, FIV, 'e');
    pattern_hash!(SEVE, SEV, 'e');
    pattern_hash!(EIGH, EIG, 'h');
    pattern_hash!(NINE, NIN, 'e');

    // n = 5
    pattern_hash!(THREE, THRE, 'e');
    pattern_hash!(SEVEN, SEVE, 'n');
    pattern_hash!(EIGHT, EIGH, 't');

    // reverse pattern hashes, n = 1
    const R: u32 = ('r' as u32).wrapping_neg();
    const X: u32 = ('x' as u32).wrapping_neg();

    // n = 2
    pattern_hash!(RO, O, 'r');
    pattern_hash!(WO, O, 'w');
    pattern_hash!(NE, E, 'n');
    pattern_hash!(EE, E, 'e');
    pattern_hash!(UR, R, 'u');
    pattern_hash!(VE, E, 'v');
    pattern_hash!(IX, X, 'i');
    pattern_hash!(EN, N, 'e');
    pattern_hash!(HT, T, 'h');

    // n = 3
    pattern_hash!(ERO, RO, 'e');
    pattern_hash!(REE, EE, 'r');
    pattern_hash!(OUR, UR, 'o');
    pattern_hash!(IVE, VE, 'i');
    pattern_hash!(VEN, EN, 'v');
    pattern_hash!(GHT, HT, 'g');
    pattern_hash!(INE, NE, 'i');

    // n = 4
    pattern_hash!(HREE, REE, 'h');
    pattern_hash!(EVEN, VEN, 'e');
    pattern_hash!(IGHT, GHT, 'i');

    macro_rules! check_seq {
        ($($var:ident),+ ; $($c:literal),+ ) => {
            $( ($var == $c as u32) )&+
        }
    }

    fn check3(c_0: u32, c_1: u32, c_2: u32) -> Option<u32> {
        if check_seq!(c_0, c_1, c_2 ; 'o', 'n', 'e') {
            Some(1)
        } else if check_seq!(c_0, c_1, c_2 ; 't', 'w', 'o') {
            Some(2)
        } else if check_seq!(c_0, c_1, c_2 ; 's', 'i', 'x') {
            Some(6)
        } else {
            None
        }
    }

    fn check4(c_0: u32, c_1: u32, c_2: u32, c_3: u32) -> Option<u32> {
        if check_seq!(c_0, c_1, c_2, c_3 ; 'z', 'e', 'r', 'o') {
            Some(0)
        } else if check_seq!(c_0, c_1, c_2, c_3 ; 'f', 'o', 'u', 'r') {
            Some(4)
        } else if check_seq!(c_0, c_1, c_2, c_3 ; 'f', 'i', 'v', 'e') {
            Some(5)
        } else if check_seq!(c_0, c_1, c_2, c_3 ; 'n', 'i', 'n', 'e') {
            Some(9)
        } else {
            None
        }
    }

    fn check5(c_0: u32, c_1: u32, c_2: u32, c_3: u32, c_4: u32) -> Option<u32> {
        if check_seq!(c_0, c_1, c_2, c_3, c_4 ; 't', 'h', 'r', 'e', 'e') {
            Some(3)
        } else if check_seq!(c_0, c_1, c_2, c_3, c_4 ; 's', 'e', 'v', 'e', 'n') {
            Some(7)
        } else if check_seq!(c_0, c_1, c_2, c_3, c_4 ; 'e', 'i', 'g', 'h', 't') {
            Some(8)
        } else {
            None
        }
    }

    macro_rules! is_hash_eq {
        ($h:ident ; $($rhs:ident),+) => {
            $( ($h == $rhs) )|+
        }
    }

    fn first(s: &str) -> Option<u32> {
        let mut c_0: u32 = 0;
        let mut c_1: u32 = 0;
        let mut c_2: u32 = 0;
        let mut c_3: u32 = 0;
        let mut c_4: u32 = 0;
        let mut h: u32 = 0;

        let mut j: usize = 0;

        for c in s.chars() {
            let u = c as u32;
            let x = u.wrapping_sub(OFFSET);
            if x < BASE {
                return Some(x);
            } else {
                if j == 0 {
                    h = h.wrapping_sub(u);
                    c_0 = u;
                    j += 1;
                } else if j == 1 {
                    h = h.wrapping_sub(u);
                    c_1 = u;
                    j += 1;
                } else if j == 2 {
                    h = h.wrapping_sub(u);
                    c_2 = u;
                    j += 1;
                } else if j == 3 {
                    h = h.wrapping_sub(u);
                    c_3 = u;
                    j += 1;
                } else if j == 4 {
                    h = h.wrapping_sub(u);
                    c_4 = u;
                    j += 1;
                } else if j == 5 {
                    h += c_0;
                    h = h.wrapping_sub(u);
                    c_0 = c_1;
                    c_1 = c_2;
                    c_2 = c_3;
                    c_3 = c_4;
                    c_4 = u;
                }
                while j > 0 {
                    if j == 5 {
                        if is_hash_eq!(h ; THREE, SEVEN, EIGHT) {
                            if let Some(x) = check5(c_0, c_1, c_2, c_3, c_4) {
                                return Some(x);
                            } else {
                                h += c_0;
                                c_0 = c_1;
                                c_1 = c_2;
                                c_2 = c_3;
                                c_3 = c_4;
                                c_4 = 0;
                                j -= 1;
                            }
                        } else {
                            h += c_0;
                            c_0 = c_1;
                            c_1 = c_2;
                            c_2 = c_3;
                            c_3 = c_4;
                            c_4 = 0;
                            j -= 1;
                        }
                    } else if j == 4 {
                        // Same concept, but `break` on `THRE`, `SEVE`, `EIGH`
                        if is_hash_eq!(h ; THRE, SEVE, EIGH) {
                            break;
                        } else if is_hash_eq!(h ; ZERO, FOUR, FIVE, NINE) {
                            if let Some(x) = check4(c_0, c_1, c_2, c_3) {
                                return Some(x);
                            } else {
                                h += c_0;
                                c_0 = c_1;
                                c_1 = c_2;
                                c_2 = c_3;
                                c_3 = 0;
                                j -= 1;
                            }
                        } else {
                            h += c_0;
                            c_0 = c_1;
                            c_1 = c_2;
                            c_2 = c_3;
                            c_3 = 0;
                            j -= 1;
                        }
                    } else if j == 3 {
                        // Same concept as j == 4
                        if is_hash_eq!(h ; ZER, THR, FOU, FIV, SEV, EIG, NIN) {
                            break;
                        } else if is_hash_eq!(h ; ONE, TWO, SIX) {
                            if let Some(x) = check3(c_0, c_1, c_2) {
                                return Some(x);
                            } else {
                                h += c_0;
                                c_0 = c_1;
                                c_1 = c_2;
                                c_2 = 0;
                                j -= 1;
                            }
                        } else {
                            h += c_0;
                            c_0 = c_1;
                            c_1 = c_2;
                            c_2 = 0;
                            j -= 1;
                        }
                    } else if j == 2 {
                        if is_hash_eq!(h ; ZE, ON, TW, TH, FO, FI, SI, SE, EI, NI) {
                            break;
                        } else {
                            h += c_0;
                            c_0 = c_1;
                            c_1 = 0;
                            j -= 1;
                        }
                    } else if j == 1 {
                        if is_hash_eq!(h ; Z, O, T, F, S, E, N) {
                            break;
                        } else {
                            h = 0;
                            c_0 = 0;
                            j -= 1;
                        }
                    }
                }
            }
        }
        None
    }

    fn last(s: &str) -> Option<u32> {
        let mut c_0: u32 = 0;
        let mut c_1: u32 = 0;
        let mut c_2: u32 = 0;
        let mut c_3: u32 = 0;
        let mut c_4: u32 = 0;
        let mut h: u32 = 0;

        let mut j: usize = 0;

        for c in s.chars().rev() {
            let u = c as u32;
            let x = u.wrapping_sub(OFFSET);
            if x < BASE {
                return Some(x);
            } else {
                if j == 0 {
                    h = h.wrapping_sub(u);
                    c_4 = u;
                    j += 1;
                } else if j == 1 {
                    h = h.wrapping_sub(u);
                    c_3 = u;
                    j += 1;
                } else if j == 2 {
                    h = h.wrapping_sub(u);
                    c_2 = u;
                    j += 1;
                } else if j == 3 {
                    h = h.wrapping_sub(u);
                    c_1 = u;
                    j += 1;
                } else if j == 4 {
                    h = h.wrapping_sub(u);
                    c_0 = u;
                    j += 1;
                } else if j == 5 {
                    h += c_4;
                    h = h.wrapping_sub(u);
                    c_4 = c_3;
                    c_3 = c_2;
                    c_2 = c_1;
                    c_1 = c_0;
                    c_0 = u;
                }
                while j > 0 {
                    if j == 5 {
                        if is_hash_eq!(h ; THREE, SEVEN, EIGHT) {
                            if let Some(x) = check5(c_0, c_1, c_2, c_3, c_4) {
                                return Some(x);
                            } else {
                                h += c_4;
                                c_4 = c_3;
                                c_3 = c_2;
                                c_2 = c_1;
                                c_1 = c_0;
                                c_0 = 0;
                                j -= 1;
                            }
                        } else {
                            h += c_4;
                            c_4 = c_3;
                            c_3 = c_2;
                            c_2 = c_1;
                            c_1 = c_0;
                            c_0 = 0;
                            j -= 1;
                        }
                    } else if j == 4 {
                        if is_hash_eq!(h ; HREE, EVEN, IGHT) {
                            break;
                        } else if is_hash_eq!(h ; ZERO, FOUR, FIVE, NINE) {
                            if let Some(x) = check4(c_1, c_2, c_3, c_4) {
                                return Some(x);
                            } else {
                                h += c_4;
                                c_4 = c_3;
                                c_3 = c_2;
                                c_2 = c_1;
                                c_1 = 0;
                                j -= 1;
                            }
                        } else {
                            h += c_4;
                            c_4 = c_3;
                            c_3 = c_2;
                            c_2 = c_1;
                            c_1 = 0;
                            j -= 1;
                        }
                    } else if j == 3 {
                        // Same concept as j == 4
                        if is_hash_eq!(h ; ERO, REE, OUR, IVE, VEN, GHT, INE) {
                            break;
                        } else if is_hash_eq!(h ; ONE, TWO, SIX) {
                            if let Some(x) = check3(c_2, c_3, c_4) {
                                return Some(x);
                            } else {
                                h += c_4;
                                c_4 = c_3;
                                c_3 = c_2;
                                c_2 = 0;
                                j -= 1;
                            }
                        } else {
                            h += c_4;
                            c_4 = c_3;
                            c_3 = c_2;
                            c_2 = 0;
                            j -= 1;
                        }
                    } else if j == 2 {
                        if is_hash_eq!(h ; RO, WO, NE, EE, UR, VE, IX, EN, HT) {
                            break;
                        } else {
                            h += c_4;
                            c_4 = c_3;
                            c_3 = 0;
                            j -= 1;
                        }
                    } else if j == 1 {
                        if is_hash_eq!(h ; O, E, R, X, N, T) {
                            break;
                        } else {
                            h = 0;
                            c_4 = 0;
                            j -= 1;
                        }
                    }
                }
            }
        }
        None
    }

    pub fn parse_line(line: &str) -> u8 {
        let first = first(line);
        let last = last(line);
        match (first, last) {
            (Some(d_1), Some(d_0)) => d_1 as u8 * 10 + d_0 as u8,
            (Some(d_0), None) => d_0 as u8 * 10 + d_0 as u8,
            _ => 0,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use super::*;

        #[test]
        fn first_works() {
            assert_eq!(first("oneight"), Some(1));
            assert_eq!(first("two1nine"), Some(2));
            assert_eq!(first("eightwothree"), Some(8));
            assert_eq!(first("abcone2threexyz"), Some(1));
            assert_eq!(first("xtwone3four"), Some(2));
            assert_eq!(first("4nineeightseven2"), Some(4));
            assert_eq!(first("zoneight234"), Some(1));
            assert_eq!(first("7pqrstsixteen"), Some(7));
            assert_eq!(first("bqccqhbdgeight7"), Some(8));
            assert_eq!(first("t1"), Some(1));
            assert_eq!(first("gckhqpb6twoqnjxqplthree2fourkspnsnzxlz1"), Some(6));
            assert_eq!(first("fmpvqkxgeightthreebdrng9tdcffvsfctwo"), Some(8));
            assert_eq!(first("sevenlptpdhtjpgxconedvtrrnngn8"), Some(7));
            assert_eq!(first("threeznnnbtfive5tmdfxtwothree3ndjcszrb"), Some(3));
            assert_eq!(
                first("kkvtwone5sevenfcfnngpmjktrpxk7djgzmdthreehpp"),
                Some(2)
            );
            assert_eq!(first("24"), Some(2));
            assert_eq!(first("oneninexqdseven4threefive"), Some(1));
            assert_eq!(first("fcsoneightnmtgzbbnflnnlk5two"), Some(1));
            assert_eq!(first("1qlcg4seven"), Some(1));
            assert_eq!(first("fivefivefourfiveeight7eightwods"), Some(5));
            assert_eq!(first("nine5threeninezgjcpssevenone"), Some(9));
            assert_eq!(first("vrctfpbp2bdknhtwothree68ckzlgkghponeightg"), Some(2));
            assert_eq!(first("mseven7six4five19hjd"), Some(7));
            assert_eq!(first("21zffhnksmjj1rcdpkcrznine"), Some(2));
            assert_eq!(first("threekp1onefrfjbrmmpmsdsvfour"), Some(3));
        }

        #[test]
        fn last_works() {
            assert_eq!(last("oneight"), Some(8));
            assert_eq!(last("two1nine"), Some(9));
            assert_eq!(last("eightwothree"), Some(3));
            assert_eq!(last("abcone2threexyz"), Some(3));
            assert_eq!(last("xtwone3four"), Some(4));
            assert_eq!(last("4nineeightseven2"), Some(2));
            assert_eq!(last("zoneight234"), Some(4));
            assert_eq!(last("7pqrstsixteen"), Some(6));
            assert_eq!(last("bqccqhbdgeight7"), Some(7));
            assert_eq!(last("t1"), Some(1));
            assert_eq!(last("gckhqpb6twoqnjxqplthree2fourkspnsnzxlz1"), Some(1));
            assert_eq!(last("fmpvqkxgeightthreebdrng9tdcffvsfctwo"), Some(2));
            assert_eq!(last("sevenlptpdhtjpgxconedvtrrnngn8"), Some(8));
            assert_eq!(last("threeznnnbtfive5tmdfxtwothree3ndjcszrb"), Some(3));
            assert_eq!(
                last("kkvtwone5sevenfcfnngpmjktrpxk7djgzmdthreehpp"),
                Some(3)
            );
            assert_eq!(last("24"), Some(4));
            assert_eq!(last("oneninexqdseven4threefive"), Some(5));
            assert_eq!(last("fcsoneightnmtgzbbnflnnlk5two"), Some(2));
            assert_eq!(last("1qlcg4seven"), Some(7));
            assert_eq!(last("fivefivefourfiveeight7eightwods"), Some(2));
            assert_eq!(last("nine5threeninezgjcpssevenone"), Some(1));
            assert_eq!(last("vrctfpbp2bdknhtwothree68ckzlgkghponeightg"), Some(8));
            assert_eq!(last("mseven7six4five19hjd"), Some(9));
            assert_eq!(last("21zffhnksmjj1rcdpkcrznine"), Some(9));
            assert_eq!(last("threekp1onefrfjbrmmpmsdsvfour"), Some(4));
        }

        #[test]
        fn parse_line_works() {
            assert_eq!(parse_line("two1nine"), 29);
            assert_eq!(parse_line("eightwothree"), 83);
            assert_eq!(parse_line("abcone2threexyz"), 13);
            assert_eq!(parse_line("xtwone3four"), 24);
            assert_eq!(parse_line("4nineeightseven2"), 42);
            assert_eq!(parse_line("zoneight234"), 14);
            assert_eq!(parse_line("7pqrstsixteen"), 76);
            assert_eq!(parse_line("bqccqhbdgeight7"), 87);
            assert_eq!(parse_line("t1"), 11);
            assert_eq!(parse_line("gckhqpb6twoqnjxqplthree2fourkspnsnzxlz1"), 61);
            assert_eq!(parse_line("fmpvqkxgeightthreebdrng9tdcffvsfctwo"), 82);
            assert_eq!(parse_line("sevenlptpdhtjpgxconedvtrrnngn8"), 78);
            assert_eq!(parse_line("threeznnnbtfive5tmdfxtwothree3ndjcszrb"), 33);
            assert_eq!(
                parse_line("kkvtwone5sevenfcfnngpmjktrpxk7djgzmdthreehpp"),
                23
            );
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
