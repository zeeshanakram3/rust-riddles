mod riddle_1;
mod riddle_2;

#[cfg(test)]
mod tests;
use riddle_1::Riddle1;
use riddle_2::Riddle2;
use std::env::args;

/// A riddle should implement the Riddle trait
pub trait Riddle {
    type Output;
    fn parse(riddle: String) -> Self;
    fn solve(self) -> Self::Output;
    fn show_result(riddle: String, result: Self::Output);
}

const DEFAULT_RIDDLE_1: &str = "If 1/2 of 5 is 3, then what is 1/3 of 10?";
const DEFAULT_RIDDLE_2: &str = "What is 3/7 chicken, 2/3 cat and 2/4 goat?";

fn main() {
    let mut args = args().collect::<Vec<String>>().into_iter();
    args.next(); // Skip the first arg as its file name.

    // if no command line options are provided run the default riddle.
    if args.len() == 0 {
        let result_1 = Riddle1::<f64>::default().solve();
        let result_2 = Riddle2::default().solve();
        Riddle1::<f64>::show_result(DEFAULT_RIDDLE_1.to_string(), result_1);
        Riddle2::show_result(DEFAULT_RIDDLE_2.to_string(), result_2);
    }

    while let Some(i) = args.next() {
        match i.as_str() {
            "--riddle-1" => {
                let riddle = args.next().expect("riddle not provided");
                let result = Riddle1::<f64>::parse(riddle.clone()).solve();
                Riddle1::show_result(riddle, result);
            }
            "--riddle-2" => {
                let riddle = args.next().expect("riddle not provided");
                let result = Riddle2::parse(riddle.clone()).solve();
                Riddle2::show_result(riddle, result);
            }
            _ => {
                panic!("invalid command line option")
            }
        }
    }
}
