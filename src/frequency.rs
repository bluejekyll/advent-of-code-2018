use std::io::{BufRead, Lines};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Frequency(isize);

impl Frequency {
    pub fn calibrate<I: IntoIterator<Item = isize>>(self, changes: I) -> Self {
        let new = changes
            .into_iter()
            .fold(self.0, |current, delta| current + delta);

        Frequency(new)
    }

    pub fn current(self) -> isize {
        self.0
    }
}

pub struct DeltaReader<R: BufRead>(Lines<R>);

impl<R: BufRead> DeltaReader<R> {
    pub fn from(read: R) -> Self {
        DeltaReader(read.lines())
    }
}

impl<R: BufRead> Iterator for DeltaReader<R> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        use std::str::FromStr;

        match self.0.next() {
            Some(Ok(s)) => Some(isize::from_str(&s).expect("bad string, expected +N or -N")),
            Some(Err(e)) => panic!("error reading from stream: {}", e),
            None => None,
        }
    }
}

#[test]
fn basic_test() {
    let freq = Frequency::default();

    assert_eq!(freq.calibrate(vec![0]).current(), 0);
    assert_eq!(freq.calibrate(vec![-1]).current(), -1);
    assert_eq!(freq.calibrate(vec![1]).current(), 1);
}

#[test]
fn parse_int() {
    use std::str::FromStr;

    assert_eq!(isize::from_str("-1").unwrap(), -1);
    assert_eq!(isize::from_str("+1").unwrap(), 1);
}

#[test]
fn test_day_1_input() {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("tests/test-data/day-1-input.txt").expect("could not open file");
    let reader = DeltaReader::from(BufReader::with_capacity(8, file));

    let frequency = Frequency::default();
    let frequency = frequency.calibrate(reader);

    println!("calibrated frequency is: {}", frequency.current());
}
