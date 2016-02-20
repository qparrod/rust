fn main()
{
    let x = 13;
    let y = test(6);
    println!("hello world {}", y);
}

fn test(x: i32) -> i32 {
    return x + 1;
}