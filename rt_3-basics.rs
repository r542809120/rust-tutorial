fn recalibrate_universe(n: int) -> bool
{
    println(format!("Recalibrating the universe: {}...", n));

    // return true if n == 10, no semicolon here:
    n == 10
}

fn main()
{
    // a simple loop
    let mut n = 0;
    loop
    {
        n += 1;

        if recalibrate_universe(n) {
            break;
        }
    }

    // while loop
    n = 0;
    while n <= 10
    {
        n += 2;
        println(format!("While looping with value {}!", n));
    }

    // static items MUST have the type specified:
    static MAGIC_VALUE: int = 42;

    // compiler warning since this variable is not used:
    let i: int = MAGIC_VALUE;

    // prepend the name with an underscore to prevent warnings:
    let _size = 123;
}
