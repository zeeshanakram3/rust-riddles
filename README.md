# rust-riddles

#### Building the project
`cargo build`


#### Running the default riddles
Default riddles are:
Riddle 1: "If 1/2 of 5 is 3, then what is 1/3 of 10?"
Riddle 2: "What is 3/7 chicken, 2/3 cat and 2/4 goat?"
`./target/debug/rust-riddles`

#### Running custom riddles
Regex crate is used to parse the riddle. Make sure to pass valid riddle string
##### Riddle 1
`./target/debug/rust-riddles --riddle-1 "If 2/3 of 4 is 13, then what is 5/6 of 11?"`
##### Riddle 2
`./target/debug/rust-riddles --riddle-2 "What is 3/4 good, 2/6 global and 1/5 event?"`
##### Both riddles
`./target/debug/rust-riddles --riddle-1 "If 2/3 of 4 is 13, then what is 5/6 of 11?" --riddle-2 "What is 3/4 good, 2/6 global and 1/5 event?"`



#### Run tests
`cargo test`