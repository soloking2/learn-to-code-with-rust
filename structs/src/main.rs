#[derive(Debug)]

struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {

    //Construct - Associate Function
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self { title, release_year, duration_secs }
    }
    fn display_song_info(self) {
        //Immutable struct value (self parameter takes ownership)
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} in seconds\n", self.duration_secs);

    }

    fn double_length(mut self: Self) {
        //Mutable struct value (self parameter takes ownership, has permission to mutate)
        self.duration_secs *= 2;
        //Self doesn't implement the Display trait
        println!("{:#?}\n", self);
    }

    //&self is same as => self: &Self and self: &TaylorSwiftSong
    fn display_and_change_song_info(&self) {
        //Immutable reference to the struct instance (no ownership moved)
        println!("Title: {}", self.title);
        println!("Years since released: {}", self.years_since_release());
        println!("Duration: {} in seconds\n", self.duration_secs);
    }

    //self: &mut Self is the same as => self: &mut TaylorSwiftSong and &mut self;
    fn double_length_value(self: &mut Self) {
        //Mutable reference to the struct instance (self parameter takes ownership, has permission to mutate)
        self.duration_secs *= 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self)  -> u32{
        2026 - self.release_year
    }


}
fn main() {
    let taylor_swift_song = TaylorSwiftSong::new(String::from("Taylor"), 2020, 90);

    println!("=====================================================\n");

    let mut prince_emmanuel_song = TaylorSwiftSong {
        title: String::from("Ifunanya"),
        duration_secs: 240,
        release_year: 2023
    };

    prince_emmanuel_song.display_and_change_song_info();
    prince_emmanuel_song.double_length_value();
    prince_emmanuel_song.display_and_change_song_info();

    if prince_emmanuel_song.is_longer_than(&taylor_swift_song) {
        println!("{} is longer than {}", prince_emmanuel_song.title, taylor_swift_song.title)

    }
}
