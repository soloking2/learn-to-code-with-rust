fn main() {
    let mut seconds = 10.0;

    loop {
        if seconds == 0.0 {
            println!("Blastoff! 🚀");
            break;
        }
        println!("{seconds} seconds to blastoff...");
        seconds -= 0.5;
    }
}
