use std::ops::Deref;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Count2And3 {
    twos: usize,
    threes: usize,
}

fn has_2_or_3<I: IntoIterator<Item = char>>(chars: I) -> Count2And3 {
    let mut counts = [0usize; 26]; // index by char

    // making bad assumption here that all chars are lowercase and ascii
    for c in chars {
        assert!(c >= 'a' && c <= 'z');

        let index = (c as u8 - 'a' as u8) as usize;
        counts[index] += 1;
    }

    counts
        .into_iter()
        .fold(Count2And3 { twos: 0, threes: 0 }, |mut result, count| {
            if *count == 2 {
                result.twos = 1;
            } else if *count == 3 {
                result.threes = 1;
            }

            result
        })
}

fn count_2_and_3<D: Deref<Target = str>, I: IntoIterator<Item = D>>(strings: I) -> Count2And3 {
    strings
        .into_iter()
        .fold(Count2And3 { twos: 0, threes: 0 }, |mut count, s| {
            let has2or3 = has_2_or_3(s.chars());

            count.twos += has2or3.twos;
            count.threes += has2or3.threes;

            count
        })
}

pub fn checksum<D: Deref<Target = str>, I: IntoIterator<Item = D>>(strings: I) -> usize {
    let count = count_2_and_3(strings);

    count.twos * count.threes
}

#[test]
fn test_has_2_or3() {
    assert_eq!(
        has_2_or_3("abcdef".chars()),
        Count2And3 { twos: 0, threes: 0 }
    );

    assert_eq!(
        has_2_or_3("bababc".chars()),
        Count2And3 { twos: 1, threes: 1 }
    );

    assert_eq!(
        has_2_or_3("abbcde".chars()),
        Count2And3 { twos: 1, threes: 0 }
    );

    assert_eq!(
        has_2_or_3("abcccd".chars()),
        Count2And3 { twos: 0, threes: 1 }
    );

    assert_eq!(
        has_2_or_3("aabcdd".chars()),
        Count2And3 { twos: 1, threes: 0 }
    );

    assert_eq!(
        has_2_or_3("abcdee".chars()),
        Count2And3 { twos: 1, threes: 0 }
    );

    assert_eq!(
        has_2_or_3("ababab".chars()),
        Count2And3 { twos: 0, threes: 1 }
    );
}

#[test]
fn test_count_2_and_3() {
    let inventory = [
        "abcdef" as &str,
        "bababc",
        "abbcde",
        "abcccd",
        "aabcdd",
        "abcdee",
        "ababab",
    ];
    let count = count_2_and_3(inventory.iter().cloned());

    assert_eq!(count.twos, 4);
    assert_eq!(count.threes, 3);
}

#[test]
fn test_checksum() {
    let inventory = [
        "abcdef" as &str,
        "bababc",
        "abbcde",
        "abcccd",
        "aabcdd",
        "abcdee",
        "ababab",
    ];

    assert_eq!(checksum(inventory.iter().cloned()), 12);
}

#[test]
fn test_day_2() {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open("tests/test-data/day-2-input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    let checksum = checksum(reader.lines().map(|r| r.expect("failed to read line")));

    println!("checksum is: {}", checksum);
}
