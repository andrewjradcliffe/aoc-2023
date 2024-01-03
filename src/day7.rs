use std::convert::TryFrom;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    // J, // for part 2
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Card::*;
        match s {
            "2" => Ok(Two),
            "3" => Ok(Three),
            "4" => Ok(Four),
            "5" => Ok(Five),
            "6" => Ok(Six),
            "7" => Ok(Seven),
            "8" => Ok(Eight),
            "9" => Ok(Nine),
            "T" => Ok(T),
            "J" => Ok(J),
            "Q" => Ok(Q),
            "K" => Ok(K),
            "A" => Ok(A),
            _ => Err(s.to_string()),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Card::*;
        match c {
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            'T' => Ok(T),
            'J' => Ok(J),
            'Q' => Ok(Q),
            'K' => Ok(K),
            'A' => Ok(A),
            _ => Err(c.to_string()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
fn classify(cards: &[Card; 5]) -> HandType {
    use Card::*;
    use HandType::*;
    let mut count = [0u8; 13];
    for card in cards {
        match card {
            Two => count[0] += 1,
            Three => count[1] += 1,
            Four => count[2] += 1,
            Five => count[3] += 1,
            Six => count[4] += 1,
            Seven => count[5] += 1,
            Eight => count[6] += 1,
            Nine => count[7] += 1,
            T => count[8] += 1,
            J => count[9] += 1,
            Q => count[10] += 1,
            K => count[11] += 1,
            A => count[12] += 1,
        }
    }
    count.sort_unstable();
    match count[8..13] {
        [1, 1, 1, 1, 1] => HighCard,
        [0, 1, 1, 1, 2] => OnePair,
        [0, 0, 1, 2, 2] => TwoPair,
        [0, 0, 1, 1, 3] => ThreeOfAKind,
        [0, 0, 0, 2, 3] => FullHouse,
        [0, 0, 0, 1, 4] => FourOfAKind,
        [0, 0, 0, 0, 5] => FiveOfAKind,
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn classify_wildcard(cards: &[Card; 5]) -> HandType {
    use Card::*;
    use HandType::*;
    let mut count = [0u8; 13];
    for card in cards {
        match card {
            Two => count[0] += 1,
            Three => count[1] += 1,
            Four => count[2] += 1,
            Five => count[3] += 1,
            Six => count[4] += 1,
            Seven => count[5] += 1,
            Eight => count[6] += 1,
            Nine => count[7] += 1,
            T => count[8] += 1,
            J => count[9] += 1,
            Q => count[10] += 1,
            K => count[11] += 1,
            A => count[12] += 1,
        }
    }
    let n = count[9].clone();
    count.sort_unstable();
    if n == 5 || n == 4 {
        FiveOfAKind
    } else if n == 3 {
        match count[8..13] {
            [0, 0, 1, 1, 3] => FourOfAKind,
            [0, 0, 0, 2, 3] => FiveOfAKind,
            _ => unreachable!(),
        }
    } else if n == 2 {
        match count[8..13] {
            [0, 1, 1, 1, 2] => ThreeOfAKind,
            [0, 0, 1, 2, 2] => FourOfAKind,
            [0, 0, 0, 2, 3] => FiveOfAKind,
            _ => unreachable!(),
        }
    } else if n == 1 {
        match count[8..13] {
            [1, 1, 1, 1, 1] => OnePair,
            [0, 1, 1, 1, 2] => ThreeOfAKind,
            [0, 0, 1, 2, 2] => FullHouse,
            [0, 0, 1, 1, 3] => FourOfAKind,
            [0, 0, 0, 1, 4] => FiveOfAKind,
            _ => unreachable!(),
        }
    } else {
        match count[8..13] {
            [1, 1, 1, 1, 1] => HighCard,
            [0, 1, 1, 1, 2] => OnePair,
            [0, 0, 1, 2, 2] => TwoPair,
            [0, 0, 1, 1, 3] => ThreeOfAKind,
            [0, 0, 0, 2, 3] => FullHouse,
            [0, 0, 0, 1, 4] => FourOfAKind,
            [0, 0, 0, 0, 5] => FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

// Because we have carefully set up `Ord` for the `Card` and `HandType`,
// and, furthermore, because the order of the fields in `Hand` is `HandType`
// _then_ [Card; 5] (which itself will subject to lexicographical comparison),
// we can derive `PartialOrd` and `Ord`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    ty: HandType,
    cards: [Card; 5],
}

impl From<[Card; 5]> for Hand {
    fn from(cards: [Card; 5]) -> Self {
        // let ty = classify_wildcard(&cards); // for part 2
        let ty = classify(&cards);
        Self { cards, ty }
    }
}
impl FromStr for Hand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            Err(s.to_string())
        } else {
            let mut cards = [Card::Two; 5];
            for (i, c) in s.char_indices() {
                let card: Card = c.try_into()?;
                cards[i] = card;
            }
            Ok(Self::from(cards))
        }
    }
}

pub fn parse_hand_bids(s: &str) -> Result<Vec<(Hand, u64)>, String> {
    let mut v = Vec::new();
    for line in s.lines() {
        if let Some((hand, bid)) = line.split_once(' ') {
            let hand = hand.parse::<Hand>()?;
            let bid = bid.trim().parse::<u64>().map_err(|e| e.to_string())?;
            v.push((hand, bid));
        } else {
            return Err(line.to_string());
        }
    }
    Ok(v)
}
pub fn total_winnings(v: &mut Vec<(Hand, u64)>) -> u64 {
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    v.iter()
        .map(|x| x.1)
        .zip(1u64..)
        .fold(0u64, |acc, (rank, bid)| rank * bid + acc)
}

pub fn hand_bids_from_path<T: AsRef<Path>>(path: T) -> Result<Vec<(Hand, u64)>, String> {
    let s = fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
    parse_hand_bids(&s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;
    use HandType::*;

    #[test]
    fn camelcard_ord() {
        let mut v = vec![
            A, K, Q, J, T, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ];
        v.sort_unstable();
        assert_eq!(
            v,
            vec![Two, Three, Four, Five, Six, Seven, Eight, Nine, T, J, Q, K, A]
        );
    }

    #[test]
    fn handtype_ord() {
        let mut v = vec![
            FiveOfAKind,
            FourOfAKind,
            FullHouse,
            ThreeOfAKind,
            TwoPair,
            OnePair,
            HighCard,
        ];
        v.sort_unstable();
        assert_eq!(
            v,
            vec![
                HighCard,
                OnePair,
                TwoPair,
                ThreeOfAKind,
                FullHouse,
                FourOfAKind,
                FiveOfAKind
            ]
        );
    }

    #[test]
    fn classify_works() {
        let cards = [A, A, A, A, A];
        assert_eq!(classify(&cards), FiveOfAKind);

        let cards = [A, A, Eight, A, A];
        assert_eq!(classify(&cards), FourOfAKind);

        let cards = [Two, Three, Three, Three, Two];
        assert_eq!(classify(&cards), FullHouse);

        let cards = [T, T, T, Nine, Eight];
        assert_eq!(classify(&cards), ThreeOfAKind);

        let cards = [Two, Three, Four, Three, Two];
        assert_eq!(classify(&cards), TwoPair);

        let cards = [A, Two, Three, A, Four];
        assert_eq!(classify(&cards), OnePair);

        let cards = [Two, Three, Four, Five, Six];
        assert_eq!(classify(&cards), HighCard);
    }

    #[test]
    fn hand_from() {
        let h = Hand::from([T, T, T, Nine, Eight]);
        assert_eq!(h.ty, ThreeOfAKind);
        assert_eq!(h.cards, [T, T, T, Nine, Eight]);
    }

    #[test]
    fn hand_from_str() {
        let lhs = "AAAAA".parse::<Hand>().unwrap();
        let rhs = Hand::from([A, A, A, A, A]);
        assert_eq!(lhs, rhs);

        let lhs = "AA8AA".parse::<Hand>().unwrap();
        let rhs = Hand::from([A, A, Eight, A, A]);
        assert_eq!(lhs, rhs);

        let lhs = "23332".parse::<Hand>().unwrap();
        let rhs = Hand::from([Two, Three, Three, Three, Two]);
        assert_eq!(lhs, rhs);

        let lhs = "TTT98".parse::<Hand>().unwrap();
        let rhs = Hand::from([T, T, T, Nine, Eight]);
        assert_eq!(lhs, rhs);

        let lhs = "23432".parse::<Hand>().unwrap();
        let rhs = Hand::from([Two, Three, Four, Three, Two]);
        assert_eq!(lhs, rhs);

        let lhs = "A23A4".parse::<Hand>().unwrap();
        let rhs = Hand::from([A, Two, Three, A, Four]);
        assert_eq!(lhs, rhs);

        let lhs = "23456".parse::<Hand>().unwrap();
        let rhs = Hand::from([Two, Three, Four, Five, Six]);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn hand_ord() {
        let lhs = Hand::from([Three, Three, Three, Three, Two]);
        let rhs = Hand::from([Two, A, A, A, A]);
        assert_eq!(lhs.cmp(&rhs), std::cmp::Ordering::Greater);

        let lhs = Hand::from([Seven, Seven, Eight, Eight, Eight]);
        let rhs = Hand::from([Seven, Seven, Seven, Eight, Eight]);
        assert_eq!(lhs.cmp(&rhs), std::cmp::Ordering::Greater);
    }

    #[test]
    fn total_winnings_works() {
        let mut v = vec![
            (Hand::from([Three, Two, T, Three, K]), 765),
            (Hand::from([T, Five, Five, J, Five]), 684),
            (Hand::from([K, K, Six, Seven, Seven]), 28),
            (Hand::from([K, T, J, J, T]), 220),
            (Hand::from([Q, Q, Q, J, A]), 483),
        ];
        assert_eq!(total_winnings(&mut v), 6440);
    }

    #[test]
    fn parse_hand_bids_works() {
        static TEST: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let lhs = parse_hand_bids(TEST).unwrap();
        let rhs = vec![
            (Hand::from([Three, Two, T, Three, K]), 765),
            (Hand::from([T, Five, Five, J, Five]), 684),
            (Hand::from([K, K, Six, Seven, Seven]), 28),
            (Hand::from([K, T, J, J, T]), 220),
            (Hand::from([Q, Q, Q, J, A]), 483),
        ];
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn classify_wildcard_works() {
        let cards = [T, Five, Five, J, Five];
        assert_eq!(classify_wildcard(&cards), FourOfAKind);

        let cards = [K, T, J, J, T];
        assert_eq!(classify_wildcard(&cards), FourOfAKind);

        let cards = [Q, Q, Q, J, A];
        assert_eq!(classify_wildcard(&cards), FourOfAKind);
    }
}
