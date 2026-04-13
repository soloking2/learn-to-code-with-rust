#[derive(Debug)]
struct File {
    name: String
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>
}

impl Folder {
    fn new(name: String) -> Self {
        Folder {
            name,
            contents: Vec::new()
        }
    }

    fn create_file(&mut self, name: String) {
        self.contents.push(File { name: name });
    }

    fn delete_file(&mut self, index: usize) -> File{
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File>{
        self.contents.get(index)
    }
}

fn main() {
    let mut languages_folder = Folder::new("Languages".to_string());
    languages_folder.create_file(String::from("Rust"));
    languages_folder.create_file(String::from("JavaScript"));
    println!("{:?}", languages_folder);
    println!("{:?}", languages_folder.delete_file(0));

    let file = languages_folder.get_file(0);

    match file {
        Option::Some(file) => println!("{file:?}"),
        Option::None => println!("There was no file")
    }
}
