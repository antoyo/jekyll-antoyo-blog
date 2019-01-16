extern crate mio;

use std::io;

use mio::{Events, Poll, Ready, PollOpt, Token};
use mio::unix::EventedFd;

const STDIN_FD: i32 = 0;
const STDIN_TOKEN: Token = Token(0);

fn main() -> io::Result<()> {
    let mut sum = 0;
    let poll = Poll::new()?;
    poll.register(&EventedFd(&STDIN_FD), STDIN_TOKEN, Ready::readable(), PollOpt::edge())?;
    let mut events = Events::with_capacity(1);
    'event_loop:
    loop {
        poll.poll(&mut events, None)?;
        for event in &events {
            match event.token() {
                STDIN_TOKEN => {
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).unwrap();
                    for word in line.split_whitespace() {
                        let num = word.parse::<i64>().unwrap();
                        sum += num;
                    }
                    break 'event_loop;
                },
                _ => unreachable!(),
            }
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}
