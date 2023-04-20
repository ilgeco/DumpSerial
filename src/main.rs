#![feature(iter_advance_by)]
#![feature(vec_into_raw_parts)]

use std::{collections::VecDeque, env, io::Read, process::exit, time::Duration};

fn copy_min(arr: &[u8], deq: &mut VecDeque<u8>, limit: usize) {
    let m = std::cmp::min(arr.len(), limit);
    let len = arr.len();
    let mut iterator = arr.iter();
    iterator
        .advance_by(len - m)
        .expect("checked that is bigger");
    deq.extend(iterator);
    let deq_len = deq.len();
    if deq_len > limit {
        deq.drain(0..deq_len - limit);
    }
}

fn main() {
    // Prints each argument on a separate line
    let mut args = env::args();
    let mut buffer = [0_u8; 16384];

    if args.len() < 4 {
        print!("Usage:\n\t[TTY]\t\t Pass the tty\n\t[Baud]\t\t Pass the baud rate\n\t[Last String] Pass the last string it's automatically add a \\n\n\t[Timeout]\t\t Optional timeout in seconds default 10 \n");
        return;
    }

    let tty_string = args.nth(1).unwrap();
    let tty_baud = u32::from_str_radix(&args.next().unwrap(), 10).unwrap();
    let tty_end = args.next().unwrap() + "\n";
    let tty_end_len = tty_end.len();

    let timeout =
        u32::from_str_radix(&args.next().unwrap_or("10".to_string()), 10).unwrap() * 1000 / 50;
    print!("0");
    let mut port = serialport::new(&tty_string, tty_baud)
        .open_native()
        .expect(format!("Failed to open port {}", &tty_string).as_str());
    print!("A");
    port.set_exclusive(false).unwrap();

    let mut end_buffer: VecDeque<u8> = VecDeque::new();
    let mut i = 0;

    loop {
        let res = port.read(&mut buffer);
        i += 1;
        if let Ok(x) = res {
            if x > 0 {
                unsafe {
                    let str_buffer = std::str::from_utf8_unchecked(&buffer[0..x]);

                    copy_min(str_buffer.as_bytes(), &mut end_buffer, tty_end_len);
                    print!("{}", &str_buffer);
                }
                i = 0;

                let vec = end_buffer.into();
                unsafe {
                    let tmp = String::from_utf8_unchecked(vec);

                    if tmp.ends_with(&tty_end) {
                        print!("B");
                        exit(0);
                    }
                    let (ptr, length, capacity) = tmp.into_raw_parts();
                    let vec = Vec::from_raw_parts(ptr, length, capacity);
                    end_buffer = vec.into();
                }
            }
        }
        if i > timeout {
            print!("C");
            exit(0);
        }
        buffer = [0_u8; 16384];
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_more() {
        let casa = "casa";
        let mut deq: VecDeque<u8> = VecDeque::new();
        let len = 15;
        deq.reserve(len);
        copy_min(casa.as_bytes(), &mut deq, len);

        let res = String::from_utf8(deq.into()).unwrap();

        assert!(res == "casa");
    }

    #[test]
    fn test_equal() {
        let casa = "casa";
        let mut deq: VecDeque<u8> = VecDeque::new();
        let len = 4;
        deq.reserve(len);
        copy_min(casa.as_bytes(), &mut deq, len);

        let res = String::from_utf8(deq.into()).unwrap();

        assert!(res == "casa");
    }

    #[test]
    fn test_less() {
        let casa = "casa";
        let mut deq: VecDeque<u8> = VecDeque::new();
        let len = 2;
        deq.reserve(len);
        copy_min(casa.as_bytes(), &mut deq, len);

        let res = String::from_utf8(deq.into()).unwrap();

        assert!(res == "sa");
    }

    #[test]
    fn test_one() {
        let casa = "casa";
        let mut deq: VecDeque<u8> = VecDeque::new();
        let len = 1;
        deq.reserve(len);
        copy_min(casa.as_bytes(), &mut deq, len);

        let res = String::from_utf8(deq.into()).unwrap();

        assert!(res == "a");
    }

    #[test]
    fn test_zero() {
        let casa = "casa";
        let mut deq: VecDeque<u8> = VecDeque::new();
        let len = 0;
        deq.reserve(len);
        copy_min(casa.as_bytes(), &mut deq, len);

        let res = String::from_utf8(deq.into()).unwrap();

        assert!(res == "");
    }

    #[test]
    fn test_concat() {
        let casa = "casa";
        let mut deq: VecDeque<u8> = VecDeque::new();
        let len = 7;
        deq.reserve(len);
        copy_min(casa.as_bytes(), &mut deq, len);
        copy_min(casa.as_bytes(), &mut deq, len);

        let res = String::from_utf8(deq.into()).unwrap();

        assert!(res == "asacasa");
    }

    #[test]
    fn test_long() {
        let casa = "frase molto lunga in un poema epico e vittoriano\n";
        let mut deq: VecDeque<u8> = VecDeque::new();
        let len = 40;
        deq.reserve(len);
        copy_min(casa.as_bytes(), &mut deq, len);
        let casa = "ma non sai cosa ti aspetta";
        copy_min(casa.as_bytes(), &mut deq, len);

        let res = String::from_utf8(deq.into()).unwrap();

        assert!(res == " e vittoriano\nma non sai cosa ti aspetta");
    }
}
