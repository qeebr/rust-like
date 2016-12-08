pub struct Log {
    pub messages: Vec<String>,
}

impl Log {
    pub fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
    }

    pub fn get_message(&mut self) -> Option<String> {
        if self.messages.len() > 0 {
            let copy = self.messages[0].clone();
            self.messages.remove(0);

            Option::Some(copy)
        } else {
            Option::None
        }
    }
}

#[test]
fn test_log() {
    let mut log = Log {messages: Vec::new()};

    let foo = "foo".to_string();
    let bar = "bar".to_string();
    log.add_message(foo);
    log.add_message(bar);

    match log.get_message() {
        Option::Some(val) => assert_eq!("foo", val),
        Option::None => panic!("Log should not be empty."),
    }

    match log.get_message() {
        Option::Some(val) => assert_eq!("bar", val),
        Option::None => panic!("Log should not be empty."),
    }

    match log.get_message() {
        Option::Some(_) => panic!("Log should not contain messages!"),
        Option::None => (),
    }
}