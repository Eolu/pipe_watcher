use std::thread;
use std::io::{BufReader, BufRead};
use ipipe::*;

fn main() -> Result<()>
{
    let threads = std::env::args().skip(1).map(|name| 
    {
        thread::spawn(move ||
        {
            let pipe = if name.contains('/') || name.contains('\\')
            {
                Pipe::open(std::path::Path::new(&name), OnCleanup::NoDelete).unwrap()
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
    }).collect::<Vec<std::thread::JoinHandle<()>>>();

    for thread in threads
    {
        thread.join().unwrap();
    }
    
    Ok(())
}
