# Pipe Watcher

This binary can be used with the [ipipe](https://github.com/Eolu/ipipe) library to direct output easily.

Usage: `pipe_watcher [pipe_name...]`

Multiple pipe names may be specified.

In a terminal:
```
pipe_watcher my_pipe1
```

Then, in your Rust program:
```
use ipipe::*;

fn main()
{
    ipipe::init("my_pipe1").unwrap();
    pprintln!("my_pipe1", "A line sent to you from me!");
}
```

And that's it! You should see the output in your terminal.

### Pipe names:

The pipe_name argument can be parsed one of two ways:
- If the name does not contain a forward slash `/` or backaslash `\` character, it will be treated as an ipipe pipe name. That translates to `"\\.\pipe\pipe_name"` on Windows and `/tmp/pipe_name` on Unix systems. 
- If the name does contain a slash, it will be treated as a literal path. That means the path must be formatted correctly to work on the appropriate OS.