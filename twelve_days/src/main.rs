fn get_day(day: u32) -> &'static str {
    match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "invalid",
    }
}

fn get_gift(day: u32) -> String {
    match day {
        1 => "a partridge in a pear tree".to_string(),
        2 => format!("two turtle doves, and {}", get_gift(1)),
        3 => format!("three French hens, {}", get_gift(2)),
        4 => format!("four calling birds, {}", get_gift(3)),
        5 => format!("five gold rings, {}", get_gift(4)),
        6 => format!("six geese a-laying, {}", get_gift(5)),
        7 => format!("seven swans a-swimming, {}", get_gift(6)),
        8 => format!("eight maids a-milking, {}", get_gift(7)),
        9 => format!("nine ladies dancing, {}", get_gift(8)),
        10 => format!("ten lords a-leaping, {}", get_gift(9)),
        11 => format!("eleven pipers piping, {}", get_gift(10)),
        12 => format!("twelve drummers drumming, {}", get_gift(11)),
        _ => "Invalid".to_string(),
    }
}

fn main() {
    for day in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love gave to me {}.",
            get_day(day),
            get_gift(day)
        );
    }
}
