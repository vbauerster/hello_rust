// https://stackoverflow.com/questions/30012995/how-can-i-read-non-blocking-from-stdin
use crossbeam_channel::{bounded, Receiver, RecvTimeoutError};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
struct Answer {
    is_correct: bool,
    spent_time: Duration,
}

impl Answer {
    pub fn new(is_correct: bool, spent_time: Duration) -> Self {
        Answer {
            is_correct,
            spent_time,
        }
    }
}

fn start_stdin_reader_thread(buf: String) -> (Receiver<Answer>, JoinHandle<io::Result<()>>) {
    let (sender, receiver) = bounded(0);

    let handle = thread::spawn(move || {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        for (i, line) in buf.lines().enumerate() {
            let pair = line.split('\t').collect::<Vec<_>>();
            if pair.len() != 2 {
                panic!("invalid quiz at line: {} mm\n", i + 1);
            }
            print!("{}", pair[0]);
            io::stdout().flush()?;
            let mut buf = String::new();
            let start = Instant::now();
            handle.read_line(&mut buf)?;
            let answer = Answer::new(buf.trim() == pair[1], start.elapsed());
            if sender.send(answer).is_err() {
                break;
            }
        }
        Ok(())
    });

    (receiver, handle)
}

fn main() {
    let mut quiz_file = fs::File::open("quiz.tsv").unwrap();
    let mut buf = String::new();
    quiz_file.read_to_string(&mut buf).unwrap();
    let total_questions = buf.lines().count();

    let (receiver, handle) = start_stdin_reader_thread(buf);

    let mut score = 0;
    let mut dur = Duration::new(15, 0);

    loop {
        match receiver.recv_timeout(dur) {
            Ok(a) => {
                if a.is_correct {
                    score += 1;
                }
                dur = dur.checked_sub(a.spent_time).unwrap_or(Duration::new(0, 0));
            }
            Err(RecvTimeoutError::Timeout) => {
                // this case will never handle.join and it's ok
                println!("\nSorry, you ran out of time!");
                break;
            }
            Err(_) => {
                match handle.join() {
                    Ok(Ok(_)) => {
                        break;
                    }
                    Ok(Err(e)) => {
                        // e is std::io::Error here, it's result of the reader thread
                        eprintln!("something went wrong: {}", e);
                        std::process::exit(1);
                    }
                    Err(e) => {
                        eprintln!("reader thread halted in unexpected way: {:?}", e);
                        std::process::exit(1);
                    }
                };
            }
        };
    }
    println!("Score: {} / {}", score, total_questions);
}
