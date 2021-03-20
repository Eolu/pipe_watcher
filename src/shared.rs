use ipipe::*;

pub fn open_pipe(name: &str) -> Pipe
{
    if name.contains('/') || name.contains('\\')
    {
        match Pipe::open(std::path::Path::new(&name), OnCleanup::NoDelete)
        {
            Ok(pipe) => pipe,
            Err(e) => {eprintln!("{:?}", e); panic!("Faild to open pipe")}
        }
    }
    else
    {
        match Pipe::with_name(&name)
        {
            Ok(pipe) => pipe,
            Err(e) => {eprintln!("{:?}", e); panic!("Faild to open pipe")}
        }
    }
}