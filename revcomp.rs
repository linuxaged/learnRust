use std::io;
use std::string::String;

static key: &'static str = "AaCcGgTtUuMmRrWwSsYyKkVvHhDdBbNn";
static cmp: &'static str = "TTGGCCAAAAKKYYWWSSRRMMBBDDHHVVNN";

fn read_all(buff: &mut String) {
    for line in io::stdin().lines() {
        let current = line.unwrap();
        if current.as_slice().char_at(0) == '>' {
            print_buffer(buff.as_slice());
            *buff = "".to_string();
            print!("{}", current);
        }
        else {
            for c in current.as_slice().chars() {
              let outc = key.find(c);
              match outc {
                Some(i) => {
                    buff.push_str(String::from_char(1, cmp.char_at(i)).as_slice())
                },
                None => ()
              };
            }
       }
    }
}

fn print_buffer(temp: &str)
{
    let mut counter: uint = 0;
    for c in temp.chars().rev() {
        print!("{}", c);
        counter += 1;
        if counter >= 60 {
            counter = 0;
            println!("");
        }
    }
    if counter != 0 {
        println!("");
    }
}

fn main() {
    let mut buff = "".to_string();
    read_all(&mut buff);
    print_buffer(buff.as_slice());
}
