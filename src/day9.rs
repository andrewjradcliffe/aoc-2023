use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn diff(x: &[i32]) -> Vec<i32> {
    x.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn extrapolate_fwd(v: &[i32]) -> i32 {
    let n = v.len();
    if n > 1 {
        let d = diff(v);
        if d.iter().all(|x| *x == 0) {
            v[n - 1].clone()
        } else {
            v[n - 1].clone() + extrapolate_fwd(&d)
        }
    } else {
        0
    }
}

pub fn extrapolate_back(v: &[i32]) -> i32 {
    let n = v.len();
    if n > 1 {
        let d = diff(v);
        if d.iter().all(|x| *x == 0) {
            v[0].clone()
        } else {
            v[0].clone() - extrapolate_back(&d)
        }
    } else {
        0
    }
}

pub fn parse_line(s: &str) -> Result<Vec<i32>, String> {
    let mut v = Vec::new();
    for x in s.split_whitespace() {
        v.push(x.parse::<i32>().map_err(|e| e.to_string())?);
    }
    Ok(v)
}

pub fn sum_extrapolated_from_path<T: AsRef<Path>>(path: T) -> Result<(i32, i32), String> {
    let f = File::open(path.as_ref()).map_err(|e| e.to_string())?;
    let mut f = BufReader::new(f);
    let mut s = String::with_capacity(1024);
    let mut sum_fwd: i32 = 0;
    let mut sum_back: i32 = 0;
    while f.read_line(&mut s).map_err(|e| e.to_string())? != 0 {
        s.pop();
        let v = parse_line(&s)?;
        sum_fwd += extrapolate_fwd(&v);
        sum_back += extrapolate_back(&v);
        s.clear();
    }
    Ok((sum_fwd, sum_back))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrapolate_fwd_works() {
        let v = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(extrapolate_fwd(&v), 18);

        let v = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(extrapolate_fwd(&v), 28);

        let v = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate_fwd(&v), 68);
    }

    #[test]
    fn extrapolate_back_works() {
        let v = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(extrapolate_back(&v), -3);

        let v = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(extrapolate_back(&v), 0);

        let v = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate_back(&v), 5);
    }
}
