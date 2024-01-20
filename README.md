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

## HW Lesson: 9: Sync TCP Server
A syncronized TCP server and a clint. The client is capable of sending a Mesage can contain an Image, File content and Text Message and also a Quit Command.
The server ca receive them as bytes and then decodes the Rust Serialized object with `bincode` crate. Also logging is add. A successful response log will be as follows:

```
[2023-11-21T12:05:25Z INFO  rustration] Server config is created from default: 127.0.0.1:8089
[2023-11-21T12:05:25Z INFO  server] Server listening on 127.0.0.1:8089
[2023-11-21T12:05:32Z INFO  server] 7fc2fb78-c48c-47a1-9d3a-d1a8d4d83f00 Received image
...
```

# Useful Commands:
- Listening to a port with netcat cli: `nc -lk 8089`.

## HW Lesson 11: Rust Ecosystem
- Project is divided into lib, client and server
    - Server runs as binary
    - Client runs from the main file
    - Library has shared code
- Env Logger along with logger is being added
- UUID is being used to diffierenciate each request
Following log will be shown when running the Applciation.
```
[2023-11-21T12:05:25Z INFO  rustration] Server config is created from default: 127.0.0.1:8089
[2023-11-21T12:05:25Z INFO  server] Server listening on 127.0.0.1:8089
[2023-11-21T12:05:32Z INFO  server] 7fc2fb78-c48c-47a1-9d3a-d1a8d4d83f00 Received image
[2023-11-21T12:05:32Z INFO  server] 7fc2fb78-c48c-47a1-9d3a-d1a8d4d83f00 Message length: 2175276
[2023-11-21T12:05:32Z INFO  server] a14e1b95-a0cc-4f94-ae6f-50c7fd40b3f4 Received text
[2023-11-21T12:05:32Z INFO  server] a14e1b95-a0cc-4f94-ae6f-50c7fd40b3f4 Message length: 25
[2023-11-21T12:05:32Z INFO  server] cede40a7-15fb-4360-857b-575fc637f6cd Received file: test_1.txt
[2023-11-21T12:05:32Z INFO  server] cede40a7-15fb-4360-857b-575fc637f6cd Message length: 52
[2023-11-21T12:05:32Z INFO  server] bda20fc2-1310-42b3-a238-ba42d57efa5f Received quit message
[2023-11-21T12:05:32Z INFO  server] bda20fc2-1310-42b3-a238-ba42d57efa5f Message length: 4
```
