fn main() {
    //List of 12 days
    let mut one = "a partridge in a pear tree.";
    let two = "Two turtle doves,";
    let three = "Three French hens,";
    let four = "Four calling birds,";
    let five = "Five golden rings,";
    let six = "Six geese a-laying,";
    let seven = "Seven swans a-swimming,";
    let eight = "Eight maids a-milking,";
    let nine = "Nine ladies dancing,";
    let ten = "Ten lords a-leaping,";
    let eleven = "Eleven pipers piping,";
    let twelve = "Twelve drummers drumming,";

    //"index" is the index number for "day"
    let mut index: u32 = 0;
    loop {
        index += 1;

        if index > 1 {
            let one = "And a partridge in a pear tree.";
        }

        if index == 12 {
            let one = "And a partridge in a pear tree!";
        }

        println!("")
    }
}

//note to self: learn how to add "new line" for each sentence (e.g. one{new_line}two{new_line}three
//and so on)
