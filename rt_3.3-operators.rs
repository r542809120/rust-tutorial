// TODO: replace with an array of computed values, and print in a loop

fn main()
{
    // arithmetic, print 4 every time!
    println(format!("{}", 2 * 2));
    println(format!("{}", 8 / 2));
    println(format!("{}", 34 % 5));
    println(format!("{}", 2 + 2));
    println(format!("{}", 6 - 2));
    println(format!("{}", -(-4)));
    println(format!("{}", 0b1000 >> 1));
    println(format!("{}", 0x01 << 2));
    println(format!("{}", 0b11101 & 0b110));
    println(format!("{}", 0x4 | 0));
    println(format!("{}", 0b11011 ^ 0b11111));
    println(format!("{}", !0b11111011u8));

    // too lazy for real example but you'll understand
    let (a, b) = (1, 2);
    if a == b {
        println("1 == 2, good!");
    } else if (a < b) && (a > b) {
        ;
    } else if (a <= b) || (a >= b) {
        ;
    } else if a != b {
        ;
    } else {
        ;
    }

    // type casting
    let x = 42.0;
    let y = x as u64;

    // this won't compile, please comment it
    assert!(x == 42);

    // but this will thanks to type casting!
    assert!(y == 42);
}
