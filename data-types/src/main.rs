fn main() {
    //Strings and and raw string
    let file_path = r"C:\MY Documents\new\videos";

    //Methods
    let val: i32 = -15;
    let empty_space: &str = "      my comment           ";
    println!("{} empty spaces are {}", val.abs(), empty_space.trim());
    println!("{file_path}");

    //The Array type
    let seasons = ["Spring", "Summer", "Fall", "Winter"];
    println!("The season is {}", seasons[1]);

    //The dbg! macro
    dbg!(seasons);

    //The tuple type
    let employee = ("Molly", 32, "HR");
    let (name, age, department) = employee;
    println!("Name: {name}, age: {age}, department: {department}");

    //The debug trait
    println!("{employee:?}");

    //Range and range iteration
    // The range from a and include z
    let letters = 'a'..='c';
    #[allow(unused_variables)]
    let month_days: std::ops::Range<i32> = 1..31;

    for alpha in letters {
        println!("{alpha}");
    }

    println!("We are in {}", match_seasons("Fall"))
}

fn match_seasons(seasons: &str) -> &str {
    let accepted;
     match seasons {
        "Spring" => {
            accepted = "Spring";
        }
        "Summer" => {
            accepted = "Summer";
        }
        "Fall" => {
            accepted = "Fall"
        }
        "Winter" => {
            accepted = "Winter"
        }
        _ => {
            accepted = "Not found"
        }
    };
    return accepted;
}
