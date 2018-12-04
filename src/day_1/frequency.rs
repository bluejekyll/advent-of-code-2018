use std::collections::HashSet;
use std::io::{BufRead, Lines};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Frequency(isize);

impl Frequency {
    pub fn calibrate(self, deltas: &[isize]) -> Self {
        let mut set = HashSet::new();
        let mut freq = self.0;

        for (iterations, delta) in deltas.iter().cycle().enumerate() {
            assert!(iterations < 1_000_000); // making sure
            if set.contains(&freq) {
                return Frequency(freq);
            }

            set.insert(freq);
            freq += delta;
        }

        panic!("for loop should only return on repeated calibration");
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
fn test_calibrate() {
    assert_eq!(Frequency::default().calibrate(&[1, -1]).current(), 0);
    assert_eq!(
        Frequency::default().calibrate(&[3, 3, 4, -2, -4]).current(),
        10
    );
    assert_eq!(
        Frequency::default().calibrate(&[-6, 3, 8, 5, -6]).current(),
        5
    );
    assert_eq!(
        Frequency::default()
            .calibrate(&[7, 7, -2, -7, -4])
            .current(),
        14
    );
}

#[test]
fn test_day_1_part_2() {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("tests/test-data/day-1-input.txt").expect("could not open file");
    let reader = DeltaReader::from(BufReader::new(file));
    let deltas = reader.collect::<Vec<_>>();

    let frequency = Frequency::default().calibrate(&deltas);

    println!("calibrated frequency is: {}", frequency.current());
}
