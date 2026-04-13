fn main() {
    let ok: Result<i8, &str> = Ok(5);
    let resp: Result<(i32, String), &str> = Ok((32, String::from("Google")));
    println!("{:?}", ok);
    let disaster: Result<i32, &str> = Err("Something went wrong");
    println!("{:?}", disaster);
    println!("{:?}", resp.unwrap())
}
