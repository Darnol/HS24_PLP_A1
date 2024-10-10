# Dominik Arnold 11-747-680

# Run
Each task / subtask is contained in a separate folder. Each folder contains the `Cargo.toml` and a `src/main.rs` file.
To run a task, navigate to the folder, for example `cd A1_T1_hello_world` and run `cargo run`.

## Run the compiled executable
As stated in the assignment, I also included for each task folder an executable in
```
<TASK_FOLDER>/target/debug/<BINARY>
```
where <BINARY> has the same name as <TASK_FOLDER>. Running the binaries of the second task `A1_T2_fizz_buzz` would therefore require
```bash
cd A1_T2_fizz_buzz/target/debug
./A1_T2_fizz_buzz // On macos or linux
A1_T2_fizz_buzz // On windows
```

# Organization
I decided to make separate cargo folders for each subtask in task 4. So there are three folders with T4 in it, one for each subtask.

# LLM Usage
I used ChatGPT 4o and also Github Copilot.  
I tried to indicate with comments whenever a certain snipped was proposed by AI which I didn't come up with by myself after reading documentation.  
In general, ChatGPT could be helpful when learning the concepts of Rust. Apart from the book and the documentation, it sometimes helped to clarify concepts or especially to explain syntax structures. However, I found that the proposed code snippets were faulty too often.