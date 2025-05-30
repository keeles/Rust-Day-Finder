use std::io;

fn main() {
    const MONTH_NAMES: [&'static str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Find out the day of the week for any date!");
    println!("Date must be in the format: dd/mm/yyyy");

    loop {
        println!("Enter your date now: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the link");

        let (valid, day, month, year) = verify_input(input);

        if !valid {
            println!("Error, please ensure date format is dd/mm/yyyy");
            continue;
        }

        let month_index: usize = month as usize - 1;
        let month_name = MONTH_NAMES[month_index];

        let day_of_week = get_day_of_week(day, month, year);

        if day_of_week == "error" {
            println!("There was an issue finding the day of week, please try again.");
            continue;
        }

        println!("{month_name} {day}, {year} was a {day_of_week}");
        break;
    }
}

fn verify_input(input: String) -> (bool, usize, usize, usize) {
    const MONTHS: std::ops::Range<usize> = 1..13;
    const DAYS: std::ops::Range<usize> = 1..32;
    const YEARS: std::ops::Range<usize> = 1753..2025;

    let array: Vec<&str> = input.split("/").collect();

    if array.len() != 3 {
        return (false, 0, 0, 0);
    }
    let d = String::from(array[0]);
    let day: usize = match d.trim().parse() {
        Ok(num) => num,
        Err(_) => return (false, 0, 0, 0),
    };
    let m = array[1];
    let month: usize = match m.trim().parse() {
        Ok(num) => num,
        Err(_) => return (false, 0, 0, 0),
    };
    let y = array[2];
    let year: usize = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => return (false, 0, 0, 0),
    };

    if !MONTHS.contains(&month) {
        println!("Error: Month must be between 1 and 12.");
        return (false, 0, 0, 0);
    }

    if !DAYS.contains(&day) {
        println!("Error: Day must be between 1 and 31.");
        return (false, 0, 0, 0);
    }

    if !YEARS.contains(&year) {
        println!("Error: Year must be between 1 and 2023.");
        return (false, 0, 0, 0);
    }

    return (true, day, month, year);
}

fn get_day_of_week(day: usize, month: usize, year: usize) -> &'static str {
    const DAYS: [&'static str; 7] = [
        "Saturday",
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
    ];

    let year_string = year.to_string();

    if year_string.len() != 4 {
        return "error";
    }

    let last_two_digits = {
        let split_pos: usize = year_string.char_indices().nth_back(1).unwrap().0;
        &year_string[split_pos..]
    };

    let last_digits: usize = match last_two_digits.trim().parse() {
        Ok(num) => num,
        Err(_) => return "error",
    };

    let first_two_digits = &year_string[0..2];

    let first_digits: usize = match first_two_digits.trim().parse() {
        Ok(num) => num,
        Err(_) => return "error",
    };

    let month_value = month_value(month);

    let plus_one_quarter = last_digits + (last_digits / 4);
    let mut plus_day_month = plus_one_quarter + day + month_value;

    if first_digits == 17 {
        plus_day_month += 4;
    }

    if first_digits == 18 {
        plus_day_month += 2;
    }

    if year % 4 == 0 {
        plus_day_month -= 1;
    }

    let day_index: usize = plus_day_month % 7;
    return DAYS[day_index];
}

fn month_value(month: usize) -> usize {
    match month {
        1 => 1,
        2 => 4,
        3 => 4,
        4 => 0,
        5 => 2,
        6 => 5,
        7 => 0,
        8 => 3,
        9 => 6,
        10 => 1,
        11 => 4,
        12 => 6,
        _ => panic!("Invalid month value!"),
    }
}
