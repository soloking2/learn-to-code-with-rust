#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn main() {
    let my_computer = OperatingSystem::MacOS;
    let age = years_since_release(my_computer);
    println!("My computer's operating system is {age} years old");

    let spouse_computer = OperatingSystem::Linux;
    let age = years_since_release(spouse_computer);
    println!("My spouse's computer is {age} years old");

    let old_computer = years_since_release(OperatingSystem::Windows);
    println!("My old computer is {old_computer} years old");
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system!");
            39
        }
        OperatingSystem::MacOS => 20,
        OperatingSystem::Linux => 25,
    }
}
