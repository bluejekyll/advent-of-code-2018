use std::ops::Deref;

pub fn strings_differ_by_1<S: IntoIterator<Item = char>>(s1: S, s2: S) -> Option<String> {
    let (id, i) = s1.into_iter().zip(s2.into_iter()).enumerate().fold(
        (String::new(), 0),
        |(mut id, _), (i, (id1, id2))| {
            if id1 == id2 {
                id.push(id1);
            }

            (id, i)
        },
    );

    // if the strings differ by more than 1, it's not a match
    //   i was an enumeration, so it's off by one for the list
    if (i + 1) - id.len() == 1 {
        Some(id)
    } else {
        None
    }
}

pub fn find_strings_differ_by_1<D: Deref<Target = str>>(strings: &[D]) -> Option<String> {
    let outer = strings.iter();
    let inner = strings.iter();

    for (i, search) in outer.enumerate() {
        let i = i + 1; // we want to skip the check on the first, which is the same string
        for find in inner.clone().skip(i) {
            if let Some(found) = strings_differ_by_1(search.chars(), find.chars()) {
                return Some(found);
            }
        }
    }

    None
}

#[test]
fn test_strings_differ_by_1() {
    assert!(strings_differ_by_1("fgij".chars(), "fgij".chars()).is_none(),);
    assert_eq!(
        strings_differ_by_1("fghij".chars(), "fguij".chars()).unwrap(),
        "fgij"
    );
    assert!(strings_differ_by_1("fghijk".chars(), "fguijl".chars()).is_none())
}

#[test]
fn test_day_2() {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open("tests/test-data/day-2-input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    // we could if we want make an Iterator that uses the file, multiple seeks into it, but let's just use more memory...
    let strings: Vec<String> = reader
        .lines()
        .map(|r| r.expect("failed to read line"))
        .collect::<Vec<String>>();

    let found = find_strings_differ_by_1(strings.as_ref());

    println!("found is: {:?}", found);
}
