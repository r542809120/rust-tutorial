fn recalibrate_universe(n: int) -> bool
{
    println(format!("Recalibrating the universe: {:i}...", n));

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
        println(format!("While looping with value {:i}!", n));
    }
}