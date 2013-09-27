fn main()
{
    let n = 42;

    match n {
        // single values
        1       => println("one"),
        // multiple patterns
        2 | 3   => println("2 or 3"),
        // a range of numeric literal patterns
        4..10   => println("4 to 10"),
        // wildcard pattern
        _       => println("another value!")
    }

    // you can also use blocks for each case (and you won't need the commas anymore)
    match n {
        0   => { println("0"); }
        _   => { println("x"); }
    }

    // destructuring
    let v = (1, 2);
    let _w =
        match v {
            (1, y)              => y,
            // pattern guard
            (2, y) if y < 10    => y,
            // TODO: document _
            (3, _)              => 3,
            (x, y)              => x + y
        };
}
