extern crate time;

use std::time;

pub mod log
{
    fn print(toPrint : String) -> ()
    {
        //let currentTime : u32 = time::now();
        //print!("{}", time::get_time().to_string());
        
        let now = time::get_time();
        println!("now:   {}", now);
    
        println!(" {}", toPrint);
    }
    
}