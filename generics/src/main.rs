#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage <T>{
    content: T,
    time: String
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String{
        self.time.clone()
    }
}

fn main() {
    let chat_message = ChatMessage {
        content: "Hello from us",
        time: String::from("Germany")
    };

    let second_chat_message = ChatMessage {
        content: String::from("Hey, I am good"),
        time: String::from("Brazil")
    };

    let third_chat_message = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("Japan")
    };
    third_chat_message.consume_entertainment();
    chat_message.retrieve_time();
    println!("{:?}", chat_message);
    second_chat_message.retrieve_time();
    println!("{:?}", second_chat_message);
    third_chat_message.retrieve_time();
    println!("{:?}", third_chat_message);
}
