use std::thread;
use std::io::{Write, Read, stdout};
use ipipe::*;

mod shared;
use shared::*;

fn main() -> Result<()>
{
    let threads = std::env::args().skip(1).map(|name| 
    {
        thread::spawn(move ||
        {
            let mut stdout = stdout();
            let pipe = open_pipe(&name);
            loop
            {
                for byte in pipe.clone().bytes()
                {
                    match byte
                    {
                        Ok(byte) => 
                        {
                            print!("{}", byte as char);
                            match stdout.flush()
                            {
                                Ok(_) => {}
                                Err(e) => eprintln!("{:?}", e)
                            }
                        },
                        Err(e) => eprintln!("{:?}", e)
                    }
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
