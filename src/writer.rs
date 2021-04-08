use std::{io::Write, thread, thread::JoinHandle};
use ipipe::*;
use std::sync::mpsc::{self, Sender, Receiver, SendError, RecvError};
use getch::*;

mod shared;
use shared::*;

fn main() -> Result<()>
{
    let pipes = open_pipes();

    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();

    thread::spawn(move || input_watcher(tx));

    let senders = pipes.into_iter().map(pipe_write_thread)
        .collect::<Vec<(JoinHandle<()>, Sender<u8>)>>();
    
    loop
    {
        match rx.recv()
        {
            Ok(byte) => 
            {
                for tx in senders.iter().map(|(_, pipe)| pipe)
                {
                    if let Err(e) = tx.send(byte)
                    {
                        eprintln!("{:?}", e);
                    }
                }
            }
            Err(RecvError) => 
            {
                break;
            }
        }
    }

    for (thread, tx) in senders.into_iter()
    {
        drop(tx);
        match thread.join()
        {
            Err(_) => {}
            Ok(_) => {}
        }
    }

    Ok(())
}

fn pipe_write_thread(mut pipe: Pipe) -> (JoinHandle<()>, Sender<u8>)
{
    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    (thread::spawn(move ||
    {
        loop
        {
            match rx.recv()
            {
                Ok(byte) => 
                {
                    if let Err(e) = pipe.write(&[byte])
                    {
                        panic!("{}", e)
                    }
                }
                Err(_) => break
            }
        }
    }), tx)
}

fn open_pipes() -> Vec<Pipe>
{
    let open_threads = std::env::args().skip(1).map(|name| 
    {
        thread::spawn(move || open_pipe(&name))
    }).collect::<Vec<std::thread::JoinHandle<Pipe>>>();

    open_threads.into_iter().filter_map(|thread| 
    {
        match thread.join()
        {
            Ok(pipe) => Some(pipe),
            Err(e) => {eprintln!("{:?}", e); None}
        }
    }).collect()
}

fn input_watcher(tx: Sender<u8>) -> std::result::Result<(), SendError<u8>>
{
    let getch = Getch::new();
    loop
    {
        match getch.getch()
        {
            Ok(byte) => 
            {
                if byte == 3 || byte == 0
                {
                    break Ok(());
                }

                if let Err(e) = tx.send(byte)
                {
                    eprintln!("{:?}", e)
                }
            },
            Err(e) => eprintln!("{:?}", e)
        }
    }
}