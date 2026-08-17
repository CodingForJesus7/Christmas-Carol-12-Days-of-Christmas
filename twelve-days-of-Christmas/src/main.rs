use std::io;
fn main() {
    let mut counter = 0;

    //List of 12 days
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

    //Beginning of each day
    let begin_fst = "On the";
    let begin_lst = "day of Christmas my true love gave to me";

    //Day number in word form
    let first = "first";
    let second = "second";
    let third = "third";
    let fourth = "fourth";
    let fifth = "fifth";
    let sixth = "sixth";
    let seventh = "seventh";
    let eighth = "eighth";
    let nineth = "nineth";
    let tenth = "tenth";
    let eleventh = "eleventh";
    let twelfth = "twelfth";

    //Introduction loop
    loop {
        if counter == 0 {
            println!("Welcome to The 12 Days of Christmas!");
            println!("To begin, press 'Enter'");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            let input = input.trim();

            if input.is_empty() {
                let one = if counter == 11 {
                    "And a partridge in a pear tree!"
                } else if counter > 0 {
                    "And a partridge in a pear tree."
                } else {
                    "A partridge in a pear tree."
                };

                println!(
                    "{}",
                    format!("{} {} {}\n{}", begin_fst, first, begin_lst, one)
                );

                counter += 1;

                continue;
            };
        } else {
        }
    }
}

//Link to Claude chat:
//https://claude.ai/share/aad00822-3c65-4f66-8f9b-17a4ca99970e
