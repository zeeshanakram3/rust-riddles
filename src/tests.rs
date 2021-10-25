use super::*;

#[test]
fn riddle1_test1_pass() {
    let riddle = Riddle1::<f64>::parse("If 1/2 of 5 is 3, then what is 1/3 of 10?".to_string());
    let result = riddle.solve();
    assert_eq!(result, 4.0);
}

#[test]
fn riddle1_test2_pass() {
    let riddle = Riddle1::<i32>::parse("If 2 of 5 is 3, then what is 3 of 10?".to_string());
    let result = riddle.solve();
    assert_eq!(result, 9);
}
#[test]
#[should_panic]
fn riddle1_test3_fail() {
    // fail because of `divide by zero`, as it will be parsed to i32 and 1/2 is zero.
    let riddle = Riddle1::<i32>::parse("If 1/2 of 5 is 3, then what is 1/3 of 10?".to_string());
    riddle.solve();
}

#[test]
fn riddle2_test1_pass() {
    let riddle = Riddle2::parse("What is 3/7 chicken, 2/3 cat and 2/4 goat?".to_string());
    let result = riddle.solve();
    assert_eq!(result, "chicago".to_string());
}

#[test]
fn riddle2_test2_pass() {
    let riddle = Riddle2::parse("What is 3/4 good, 2/6 global and 1/5 event?".to_string());
    let result = riddle.solve();
    assert_eq!(result, "google".to_string());
}

#[test]
#[should_panic]
fn riddle2_test3_fail() {
    // fail to parse due to invalid riddle string, it should be `3/4 good`
    Riddle2::parse("What is 3/5 good, 2/6 global and 1/5 event?".to_string());
}
