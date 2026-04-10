fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("{seconds}")
    } else {
        println!("{seconds} seconds to blastoff...");
        countdown(seconds - 1);
    }
}

fn main() {
    countdown(10);
}
