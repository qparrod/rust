extern crate time;

pub fn verbose(to_print : &str) -> ()
{
    let current_time = time::now();
    
    println!("[{}] {} ", current_time.rfc822(), to_print);
}