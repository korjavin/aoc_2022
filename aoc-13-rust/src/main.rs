use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Element {
    Int(i32),
    List(Vec<Element>),
}

// Parse a string with square brackets and numbers into Element.
fn parse(input: &str) -> Vec::<Element> {
    let mut stack = Vec::new();
    let mut current = Vec::<Element>::new();
    for c in input.chars() {
        match c {
            '[' => {
                stack.push(current);
                current = Vec::new();
            }
            ']' => {
                let mut last = stack.pop().unwrap();
                last.push(Element::List(current));
                current = last;
            }
            ',' => (),
            _ => {
                let num = c.to_digit(10).unwrap() as i32;
                current.push(Element::Int(num));
            }
        }
    }


    // Element::List(current)
    current
}

#[test]
fn test_parse() {
    assert_eq!(parse("1"), vec![ Element::Int(1) ]);

    assert_eq!(parse("[1,2,3]"), vec![
        Element::List(vec![
            Element::Int(1),
            Element::Int(2),
            Element::Int(3),
        ])
    ]);

    assert_eq!(parse("[[][]]"), vec![
        Element::List(vec![
            Element::List(vec![]),
            Element::List(vec![]),
        ])
    ]);

    assert_eq!(parse("[[1,2],[3]]"), vec![
        Element::List(vec![
            Element::List(vec![
                Element::Int(1),
                Element::Int(2),
            ]),
            Element::List(vec![
                Element::Int(3),
            ]),
        ])
    ]);
}

fn compare_vec_of_elements(a: Vec::<Element>, b: Vec::<Element>) -> bool {

    let mut a = a.iter();
    let mut b = b.iter();
    loop {
        match (a.next(),b.next()) {
            (Some(a), Some(b)) => {
                return compare(a,b);
            },
            (None, Some(_)) => {
                return true;
            },
            _ => {
                return false;
            }
        }
    }
}

fn compare(first: &Element, second: &Element) -> bool {
    match (first, second) {
        (Element::Int(a), Element::Int(b)) => a <= b,
        (Element::Int(a), Element::List(b)) =>
        {
            if b.len() == 0 {
                return false;
            }
            compare(&Element::List(vec![Element::Int(*a)]), &Element::List(vec![b[0].clone()]))
        }
        (Element::List(a), Element::Int(b)) =>
        {
            if a.len() == 0 {
                return true;
            }
            compare( &Element::List(vec![a[0].clone()]),&Element::List(vec![Element::Int(*b)]))
        },
        (Element::List(a), Element::List(b)) => {
            let mut a = a.iter();
            let mut b = b.iter();
            loop {
                match (a.next(), b.next()) {
                    (Some(x), Some(y)) => {
                        if !compare(x, y) {
                            return false;
                        }
                    }
                    (Some(x), None) => {
                        return false
                    }
                    (None, Some(_)) => return true,
                    (None, None) => return true,
                }
            }
        }
        _ => false,
    }
}
#[test]
fn test_compare_vec_of_elememts() {
    // assert_eq!(compare_vec_of_elements(parse("1"), parse("2")), true);
    // assert_eq!(compare_vec_of_elements(parse("2"), parse("1")), false);
    //
    // assert_eq!(compare_vec_of_elements(parse("[1,2,3,4]"), parse("[1,2,3,4]")), true);
    // assert_eq!(compare_vec_of_elements(parse("[1,2,3,5]"), parse("[1,2,3,4]")), false);
    //
    // assert_eq!(compare_vec_of_elements(parse("[1,2,3]"), parse("[1,2,3,4]")), true);
    // assert_eq!(compare_vec_of_elements(parse("[1,2,3]"), parse("[1,2]")), false);
    //
    // assert_eq!(compare_vec_of_elements(parse("[[1,2],[3]]"), parse("[[1,2],[3]]")), true);
    // assert_eq!(compare_vec_of_elements(parse("[[1,2]]"), parse("[[1,2],[3]]")), true);
    // assert_eq!(compare_vec_of_elements(parse("[[4,2],[3]]"), parse("[[1,2],[3]]")), false);
    //
    // assert_eq!(compare_vec_of_elements(
    //         parse("[1,1,3,1,1]"),
    //         parse("[1,1,5,1,1]")),
    //         true,
    //         );

    assert_eq!(compare_vec_of_elements(
            parse("[[1],[2,3,4]]"),
            parse("[[1],4]")),
            true,
            "strange one",
            );


    assert_eq!(compare_vec_of_elements(
            parse("[9]"),
            parse("[[8,7,6]]")),
            false,
            );


    assert_eq!(compare_vec_of_elements(
            parse("[[4,4],4,4]"),
            parse("[[4,4],4,4,4]")),
            true,
            );

    assert_eq!(compare_vec_of_elements(
            parse("[7,7,7,7]"),
            parse("[7,7,7]")),
            false,
            );

    assert_eq!(compare_vec_of_elements(
            parse("[]"),
            parse("[3]")),
            true,
            );

    assert_eq!(compare_vec_of_elements(
            parse("[[[]]]"),
            parse("[[]]")),
            false,
            );

    assert_eq!(compare_vec_of_elements(
            parse("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")),
            false,
            );

    assert_eq!(compare_vec_of_elements(
            parse("1"),
            parse("[]")),
            false,
            );

    assert_eq!(compare_vec_of_elements(
            parse("[]"),
            parse("1")),
            true,
            );
}





fn main() {

    let mut lines = io::stdin().lock().lines();
    let mut idx=0;
    let mut sum = 0;

    let mut input = Vec::new();
    
    loop {
        match (lines.next(), lines.next()) {
            (Some(a), Some(b)) => {

                input.push(parse(a.as_ref().unwrap().clone().as_str()));
                input.push(parse(b.as_ref().unwrap().clone().as_str()));
                idx+=1;
                if compare_vec_of_elements(parse(a.unwrap().as_str()), parse(b.unwrap().as_str())) {
                    sum+=idx;

                }
            }
            _ => break,
        }
        lines.next();
    }
    input.sort_by(|a,b| {
        if compare_vec_of_elements(a,b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    println!("{}", sum);
}
