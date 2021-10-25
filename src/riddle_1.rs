use crate::Riddle;
use regex::Regex;
use std::fmt::Debug;
use std::ops::{Div, Mul};
use std::str::FromStr;

/// A regular expression to parse riddle 1 with named capture groups.
const RIDDLE_1_REGEX: &str = r"(?x)
    (If|if)\s+
    (?P<x>\d+/*\d*)\s+of\s+
    (?P<y>\d+/*\d*)\s+is\s+
    (?P<r>\d+/*\d*)\s*,\s+then\s+what\s+is\s+
    (?P<x2>\d+/*\d*)\s+of\s+
    (?P<y2>\d+/*\d*)?";

// Any type that implements Mul and Div traits can make up the riddle
pub struct Riddle1<T> {
    // if x * y = r , then find x2 * y2
    pub x: T,
    pub y: T,
    pub r: T,
    pub x2: T,
    pub y2: T,
}

impl<T> Riddle<T> for Riddle1<T>
where
    T: Mul<T, Output = T> + Div<T, Output = T> + Debug + FromStr,
    // FromStr trait is used to convert captured group to underlying type of riddle.
    <T as FromStr>::Err: Debug,
{
    fn parse(riddle: String) -> Self {
        let re = Regex::new(RIDDLE_1_REGEX).unwrap();
        let capture = re.captures(&riddle).expect("invalid capture sequence");

        let attribute_value = |name| {
            capture
                .name(name)
                .unwrap()
                .as_str()
                .split("/")
                .map(|x| x.parse::<T>().unwrap())
                // convert fraction into underlying type
                .reduce(|a, b| a / b)
                .unwrap()
        };

        Riddle1 {
            x: attribute_value("x"),
            y: attribute_value("y"),
            r: attribute_value("r"),
            x2: attribute_value("x2"),
            y2: attribute_value("y2"),
        }
    }

    fn solve(self) -> T {
        let denominator = self.x * self.y;
        let numerator = self.r * self.x2 * self.y2;
        numerator / denominator
    }

    fn show_result(riddle: String, result: T) {
        println!("Riddle: {}", riddle);
        println!("Solution: {:?}", result);
    }
}

impl Default for Riddle1<f64> {
    fn default() -> Riddle1<f64> {
        Riddle1 {
            x: 1.0 / 2.0,
            y: 5.0,
            r: 3.0,
            x2: 1.0 / 3.0,
            y2: 10.0,
        }
    }
}
