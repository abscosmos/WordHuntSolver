use std::cmp::Ordering;
use std::fmt;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct FoundWord {
    pub word: String,
    pub path: Vec<u8>
}

impl FoundWord {
    pub fn as_string_with_arrows(&self) -> String {
        let mut board = ['*'; 16];
        self.path
            .windows(2)
            .for_each(|s| {
                let arrow = match s[0] as i8 -s[1] as i8 {
                    1 => '\u{2190}', // left
                    -1 => '\u{2192}', // right
                    4 => '\u{2191}', // up
                    -4 => '\u{2193}', // down
                    5 => '\u{2196}', // nw
                    -5 => '\u{2198}', // se
                    3 => '\u{2197}', // ne
                    -3 => '\u{2199}', // ne
                    _ => panic!("invalid move"),
                };
                board[s[0] as usize] = arrow;
            });

        if let Some(&i) = self.path.first() {
            board[i as usize] = (board[i as usize] as u32 + 64).try_into().unwrap();
        }
        if let Some(&i) = self.path.last() {
            board[i as usize] = '\u{2297}';
        }

        let board = board.iter()
            .chunks(4)
            .into_iter()
            .map(|row| row.into_iter().join(" "))
            .join("\n");

        format!("{}:\n{board}", self.word)
    }

    pub fn as_string_with_numbers(&self) -> String {
        let mut board = ['*'; 16];
        self.path.iter()
            .enumerate()
            .for_each(|(mut i, &n)| {
                if i > 8 { i += 39 }
                board[n as usize] = (i as u8 + b'1') as char;
            });

        let board = board.iter()
            .chunks(4)
            .into_iter()
            .map(|row| row.into_iter().join(" "))
            .join("\n");

        format!("{}:\n{board}\n", self.word)
    }
}

impl PartialOrd for FoundWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            match self.word.len().cmp(&other.word.len()) {
                Ordering::Equal => self.word.cmp(&other.word),
                ord => ord,
            }
        )
    }
}

impl fmt::Display for FoundWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string_with_arrows())
    }
}