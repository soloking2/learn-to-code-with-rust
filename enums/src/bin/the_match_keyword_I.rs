#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
}
fn main() {
    let windows = years_since_release(OperatingSystem::Windows);
    let macos = years_since_release(OperatingSystem::MacOs);
    let linux = years_since_release(OperatingSystem::Linux);
    println!("My computer's operating system is {}\n", windows);
    println!("My computer's operating system is {}\n", macos);
    println!("My computer's operating system is {}\n", linux);
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => 41,
        OperatingSystem::MacOs => 25,
        OperatingSystem::Linux => 36
    }
}


