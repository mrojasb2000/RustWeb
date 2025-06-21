#[derive(Debug)]
enum Friend {
    HUMAN(Box<Human>),
    NIL,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Human {
    name: String,
    age: i8,
    current_thought: Option<String>,
    friend: Friend,
}

impl Human {
    fn new(name: &str, age: i8) -> Human {
        return Human {
            name: name.to_string(),
            age,
            current_thought: None,
            friend: Friend::NIL,
        };
    }
    fn with_thought(mut self, thought: &str) -> Human {
        self.current_thought = Some(thought.to_string());
        return self;
    }
    fn with_friend(mut self, friend: Box<Human>) -> Human {
        self.friend = Friend::HUMAN(friend);
        return self;
    }
}

fn main() {
    // let another_developer = Human::new("John Doe", 25);

    let developer = Human::new("Maxwell Flitton", 32)
        .with_friend(Box::new(Human::new("John Doe", 25)))
        .with_thought("I love Rust!");

    // println!("{Name:}", developer.name);
    // println!("{Age:}", developer.age);
    // println!(
    //     "{Thought:}",
    //     match developer.current_thought {
    //         Some(thought) => thought,
    //         None => "No thought".to_string(),
    //     }
    // );
    println!("{:?}", developer);
    match &developer.friend {
        Friend::HUMAN(data) => {
            println!("{:?}", data);
        }
        Friend::NIL => {}
    }
}
