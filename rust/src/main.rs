use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let mut v = vec![0u64];
    let mut handle = io::stdin().lock();
    loop {
        let mut buf = String::new();
        let s = handle.read_line(&mut buf)?;
        match s {
            0 => {
                break;
            }
            1 => {
                v.push(0);
            }
            _ => {
                *v.last_mut().unwrap() += buf.trim().parse::<u64>().unwrap();
            }
        }
    }
    v.sort_unstable();
    println!("{:?}", v.iter().last().unwrap());
    println!("{:?}", v.iter().rev().take(3).sum::<u64>());

    Ok(())
}

/*
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
 */
