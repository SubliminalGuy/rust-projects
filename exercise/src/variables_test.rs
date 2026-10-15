// TODO 1: return value widened to an f64, so a later division keeps its
//   fractional part. to_float(7) -> 7.0
fn to_float(value: i32) -> f64 {
    let x = value as f64;
    x
}

// TODO 2: return the average of total over count as an f64 whose
//   fractional part survives. Convert BEFORE dividing.
//   average_of(250, 4) -> 62.5, average_of(7, 2) -> 3.5
fn average_of(total: i32, count: i32) -> f64 {
    let a = total as f64;
    let b = count as f64;
    a / b
}

// TODO 3: parse text into an i32. The caller guarantees it holds a valid
//   number, so .unwrap() is acceptable here.
//   parse_score("42") -> 42, parse_score("-7") -> -7
fn parse_score(text: &str) -> i32 {
    let c = text.parse::<i32>().unwrap();
    c
}

// --- Build checks: these must all pass. Do not edit below this line. ---
fn main() {
    assert_eq!(to_float(7), 7.0, "to_float(7) should be the f64 7.0");
    assert_eq!(
        average_of(250, 4),
        62.5,
        "average_of(250, 4) should be exactly 62.5, not 62.0"
    );
    assert_eq!(
        average_of(7, 2),
        3.5,
        "average_of(7, 2) should be exactly 3.5 - the remainder must survive"
    );
    assert_eq!(
        parse_score("42"),
        42,
        "parse_score('42') should be the number 42"
    );
    assert_eq!(
        parse_score("-7"),
        -7,
        "parse_score('-7') should be the number -7"
    );

    println!("All checks passed.");
    println!("Average of 250 over 4: {}", average_of(250, 4));
    println!("Parsed score: {}", parse_score("42"));
}
