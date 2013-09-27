// more information about the format! macro can be found here: http://static.rust-lang.org/doc/master/std/fmt/index.html

fn main()
{
    // basic message
    println("Hello, world!");

    // same message with simple formatting
    println(format!("Hello, {:s}!", "World"));
}
