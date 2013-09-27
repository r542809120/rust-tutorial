struct Point
{
    x: int,
    y: int
}

fn main()
{
    let p = Point { x: 1, y: 2 };
    let mut mp = Point { x: 3, y: 4 };

    mp.x -= 1;
    mp.y -= 1;

    match p {
        Point { x: 1, y: 2 }    => println("(1, 2)"),
        Point { x, _ }          => println(format!("({}, ...", x))
    };
}
