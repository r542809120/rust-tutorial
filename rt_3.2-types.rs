fn main()
{
    // basic signed and unsigned integers:
    let _a: int  = 1i;
    let _b: uint = 2u;

    // 2 more ways to write values:
    let _c = 0xdeadbeef;
    let _d = 0x100010101;

    // more signed types:
    let _e: i8  = 1i8;
    let _f: i16 = 2i16;
    let _g: i32 = 3i32;
    let _h: i64 = 4i64;

    // more unsigned types:
    let _i: u8  = 1u8;
    let _j: u16 = 2u16;
    let _k: u32 = 3u32;
    let _l: u64 = 4u64;

    // floating-point types:
    let _m: float = 1.0f;
    let _n: f32   = 2.0f32;
    let _o: f64   = 3.0f64;

    // but you can also write floating-point numbers like this:
    let _p = 23e-2;

    let _q: bool = true; // or `false`
    let _r: char = 'x'; // or '\n', '\t'...

    // TODO: find a way to **declare** a nil variable
}
