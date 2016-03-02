extern crate time;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::Display;

static path : Path = Path::new("log/test.log");
static display : Display = path.display();
const file : File = 0 as File;

pub fn init() -> ()
{
    file = match File::open(&path)
    {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
}

pub fn verbose(to_print : &str) -> ()
{
    let line = "[" + time::now().rfc822() + "] " + to_print;
    
    //println!("[{}] {} ", current_time.rfc822(), to_print);// redirect into a file
    
    match file.write_all(line.as_bytes())
    {
        Err(why) => { panic!("couldn't write to {}: {}", display, Error::description(&why)) },
        Ok(_)    => println!("successfully wrote to {}", display),
    }
}