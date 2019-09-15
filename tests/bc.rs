#[macro_use]
extern crate bc;

#[test]
fn add() {
    let result = bc!("1+1").unwrap();

    assert_eq!(2, result.parse::<isize>().unwrap());
}

#[test]
fn sub() {
    let result = bc!("2-1").unwrap();

    assert_eq!(1, result.parse::<isize>().unwrap());
}

#[test]
fn multiply() {
    let result = bc!("2*3").unwrap();

    assert_eq!(6, result.parse::<isize>().unwrap());
}

#[test]
fn divide() {
    let result = bc!("5/2").unwrap();

    assert_eq!(2.5, result.parse::<f64>().unwrap());
}

#[test]
fn power() {
    let result = bc!("2^100").unwrap();

    assert_eq!(1267650600228229401496703205376, result.parse::<i128>().unwrap());
}

#[test]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Timeout")]
fn power_extreme() {
    bc_timeout!(1, "99999^99999").unwrap();
}
