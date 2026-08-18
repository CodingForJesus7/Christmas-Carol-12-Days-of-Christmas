use std::io;
fn main() {
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

    'restart: loop {
        let mut index = 0;
        let mut counter = 0;

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

                    println!("Enter 's' to sleep into the next day,");
                    println!("or enter 'q' to quit.");
                    println!("You can enter 'q' to quit at anytime.");

                    counter += 1;

                    continue;
                };
            } else {
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line.");

                let input = input.trim();

                match input {
                    "s" => {
                        println!("");
                        index += 1;
                    }
                    "q" => break,
                    _ => {
                        println!(
                            "Please enter 's' to sleep into the next day or enter 'q' to quit."
                        );
                        continue;
                    }
                }

                let one = if index == 11 {
                    "And a partridge in a pear tree!"
                } else if counter > 0 {
                    "And a partridge in a pear tree."
                } else {
                    "A partridge in a pear tree."
                };

                let output = match index {
                    0 => format!("{} {} {}\n{}", begin_fst, first, begin_lst, one),
                    1 => format!("{} {} {}\n{}\n{}", begin_fst, second, begin_lst, two, one),
                    2 => format!(
                        "{} {} {}\n{}\n{}\n{}",
                        begin_fst, third, begin_lst, three, two, one
                    ),
                    3 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}",
                        begin_fst, fourth, begin_lst, four, three, two, one
                    ),
                    4 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}\n{}",
                        begin_fst, fifth, begin_lst, five, four, three, two, one
                    ),
                    5 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}\n{}\n{}",
                        begin_fst, sixth, begin_lst, six, five, four, three, two, one
                    ),
                    6 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                        begin_fst, seventh, begin_lst, seven, six, five, four, three, two, one
                    ),
                    7 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                        begin_fst,
                        eighth,
                        begin_lst,
                        eight,
                        seven,
                        six,
                        five,
                        four,
                        three,
                        two,
                        one
                    ),
                    8 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                        begin_fst,
                        nineth,
                        begin_lst,
                        nine,
                        eight,
                        seven,
                        six,
                        five,
                        four,
                        three,
                        two,
                        one
                    ),
                    9 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                        begin_fst,
                        tenth,
                        begin_lst,
                        ten,
                        nine,
                        eight,
                        seven,
                        six,
                        five,
                        four,
                        three,
                        two,
                        one
                    ),
                    10 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                        begin_fst,
                        eleventh,
                        begin_lst,
                        eleven,
                        ten,
                        nine,
                        eight,
                        seven,
                        six,
                        five,
                        four,
                        three,
                        two,
                        one
                    ),
                    11 => format!(
                        "{} {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                        begin_fst,
                        twelfth,
                        begin_lst,
                        twelve,
                        eleven,
                        ten,
                        nine,
                        eight,
                        seven,
                        six,
                        five,
                        four,
                        three,
                        two,
                        one
                    ),

                    _ => format!("e"),
                };

                println!("{}", output);

                if index < 11 {
                    println!("Enter 's' to sleep into the next day");
                }

                if index == 12 {
                    println!("You made it through all the days!");
                    println!("You can enter 'q' to quit,");
                    println!("or you can restart by entering 'r'.");

                    loop {
                        let mut input = String::new();

                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line.");

                        let input = input.trim();

                        match input {
                            "r" => continue 'restart,
                            "q" => break,
                            _ => {
                                println!("Invalid input.");
                                println!("Enter 'q' to quit,");
                                println!("or enter 'r' to restart");
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}

//Link to Claude chat:
//https://claude.ai/share/aad00822-3c65-4f66-8f9b-17a4ca99970e

// Issue: The 12th day still expects you to enter 's' to proceed
