# Rust Developer Project
This repository represent homeworks from a rust inclusive course.
Homeworks: 

## HW2 User API:
User can run the program with either cargo run as a dev mode or can compile the code and the run the binary. When running with cargo run the flag should be provided as `cargo run -- --<a-flag>/-<short-flag>` where `<>` is can be replaced with with flags such as `--lowercase/-l, --uppercase/-u, --no-spaces/-ns, --slugify/-s`.

## HW3 User API:
Improvement on the Previous work now error handling with `Result` and `eprintln!` is being added into the project. The code is modularized into seperate files and `main.rs` file being the entry point.
- Multiline read feature is added
- CSV string input parsing and pringing is added
- Input multiline csv and then get back as string from multiline input, it encountered with error if csv records are malformed.

## HW4 Add Multi Threading to the 
Improving on the previous work by implementing the mpsc based message passing multi threading where there is a recevier and sender. Main thread receive the command and and the input text and then send the command tommand aind input to another thread for processing then receive the result from the worker thread to view the result on the main thread.

# Useful Commands:
- Listening to a port with netcat cli: `nc -lk 8089`.