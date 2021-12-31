fn main() {
    sing_12_days_of_christmas();
}

fn sing_12_days_of_christmas() {
    let mut things_on_days: [&str; 12] = [
        "12 drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtle-doves",
        "and a partridge in a pear tree",
    ];

    things_on_days.reverse();

    let days = [
        "1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th",
    ];

    let mut song: Vec<String> = Vec::new();

    for i in 1..=12 {
        let mut things_on_that_day_and_before = things_on_days.clone()[0..i]
            .iter_mut()
            .enumerate()
            .map(|(index, str)| {
                if index == 0 {
                    return str.replace("and ", "");
                }
                
                str.to_string()
            })
            .collect::<Vec<String>>();

        things_on_that_day_and_before.reverse();

        song = [
            song,
            Vec::from([
                format!("On the {} day of Christmas", days[i - 1]),
                String::from("My true love sent to me"),
            ]),
            things_on_that_day_and_before,
        ]
        .concat();

        if i == 12 {
            let last_line_repeat = &things_on_days[0];

            song.push(last_line_repeat.to_string());
        }
    }

    for line in song {
        println!("{}", line);
    }
}
