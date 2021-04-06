This crate compiles 2 binaries can be used with the [ipipe](https://github.com/Eolu/ipipe) library to interact with named pipes. The `pipe_listener` binary reads the output of one or more named pipes and writes it to standard out. The `pipe_writer` binary reads the output of standard in and writes it to one or more named pipes.

These programs can easily be used in conjunction to redirect named-pipe I/O willy-nilly.

# To Install

`cargo install pipe_watcher`

# Pipe Listener

Usage: `pipe_listener [pipe_name...]`

Multiple pipe names may be specified.

In a terminal:
```
pipe_listener my_pipe1
```

Then, in your Rust program:
```rust
use ipipe::*;

fn main()
{
    ipipe::init("my_pipe1").unwrap();
    pprintln!("my_pipe1", "A line sent to you from me!");
}
```

And that's it! You should see the output in your terminal.

# Pipe Writer

Usage: `pipe_writer [pipe_name...]`

The reverse of the pipe listener binary.

In a terminal:
```
pipe_writer my_pipe1
```

In your Rust program:
```rust
use ipipe::*;
use std::io::{Read, BufReader};

fn main()
{
    let pipe = ipipe::init("my_pipe1").unwrap();
    loop
    {
        for line in BufReader:new(pipe.clone()).lines()
        {
            println!("{}", line);
        }
    }
}
```

Then start typing into the terminal, hit enter, and watch the magic happen. Note that these binaries are NOT line-buffered by default.

### Pipe names:

The pipe_name argument can be parsed one of two ways:
- If the name does not contain a forward slash `/` or backaslash `\` character, it will be treated as an ipipe pipe name. That translates to `"\\.\pipe\pipe_name"` on Windows and `/tmp/pipe_name` on Unix systems. 
- If the name does contain a slash, it will be treated as a literal path. That means the path must be formatted correctly to work on the appropriate OS.