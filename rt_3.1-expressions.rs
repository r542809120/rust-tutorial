fn main()
{
    // TODO: check why we can't use ~str or something else, the syntax is strange
    static ITEM_SALAD: &'static str = &"salad";

    let item = "salad";
    let price =
        if item == ITEM_SALAD {
            12
        } else {
            42
        };

    println(format!("The price of the {} is ${}", item, price));
}
