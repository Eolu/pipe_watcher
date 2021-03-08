use std::thread;
use std::io::{BufReader, BufRead};
use ipipe::*;

fn main() -> Result<()>
{
    for thread in std::env::args().skip(1).map(|name| 
    {
        thread::spawn(move ||
        {
            let pipe = if name.contains('/') || name.contains('\\')
            {
                Pipe::open(&std::path::PathBuf::from(name), OnCleanup::NoDelete).unwrap()
            }
            else
            {
                Pipe::with_name(&name).unwrap()
            };
            loop
            {
                for line in BufReader::new(pipe.clone()).lines()
                {
                    println!("{}", line.unwrap());
                }
            }
        })
    })
    {
        thread.join().unwrap();
    }
    Ok(())
}
