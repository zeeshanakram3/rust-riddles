use crate::Riddle;
use regex::Regex;

/// A regular expression to parse riddle 2 with named capture groups.
const RIDDLE_2_REGEX: &str = r"(?x)
    (What|what)\s+is\s+
    (?P<ratio>\d+/\d+)\s+
    (?P<sequence>\w+)\s*,\s+
    (?P<ratio2>\d+/\d+)\s+
    (?P<sequence2>\w+)\s+and\s+
    (?P<ratio3>\d+/\d+)\s+
    (?P<sequence3>\w+)";

pub struct Riddle2 {
    pub ratio: usize,
    pub sequence: String,
    pub ratio2: usize,
    pub sequence2: String,
    pub ratio3: usize,
    pub sequence3: String,
}

impl Riddle<String> for Riddle2 {
    fn parse(riddle: String) -> Self {
        let re = Regex::new(RIDDLE_2_REGEX).unwrap();
        let capture = re.captures(&riddle).expect("invalid capture sequence");

        let ratio_sequence_tuple = |ratio, sequence| {
            // get ratio named capture group into a collection. 2/4 -> [2, 4]
            let ratio_vec = capture
                .name(ratio)
                .unwrap()
                .as_str()
                .split('/')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            // get sequence named capture group into a string
            let seq = capture.name(sequence).unwrap().as_str().to_string();
            if seq.len() == ratio_vec[1] {
                (ratio_vec[0], seq)
            } else {
                panic!("invalid riddle sequence")
            }
        };

        let seq_1 = ratio_sequence_tuple("ratio", "sequence");
        let seq_2 = ratio_sequence_tuple("ratio2", "sequence2");
        let seq_3 = ratio_sequence_tuple("ratio3", "sequence3");
        Riddle2 {
            ratio: seq_1.0,
            sequence: seq_1.1,
            ratio2: seq_2.0,
            sequence2: seq_2.1,
            ratio3: seq_3.0,
            sequence3: seq_3.1,
        }
    }

    fn solve(self) -> String {
        let a = &self.sequence[..self.ratio];
        let b = &self.sequence2[..self.ratio2];
        let c = &self.sequence3[..self.ratio3];
        a.to_owned() + b + c
    }

    fn show_result(riddle: String, result: String) {
        println!("Riddle: {}", riddle);
        println!("Solution: {:?}", result);
    }
}

impl Default for Riddle2 {
    fn default() -> Riddle2 {
        Riddle2 {
            ratio: 3,
            sequence: "chicken".to_string(),
            ratio2: 2,
            sequence2: "cat".to_string(),
            ratio3: 2,
            sequence3: "goat".to_string(),
        }
    }
}
