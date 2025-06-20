use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum CharacterValue {
        Name(String),
        Age(i32),
        Items(Vec<String>),
    }

    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

    profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
    profile.insert("age", CharacterValue::Age(32));
    profile.insert(
        "items",
        CharacterValue::Items(vec![
            "laptop".to_string(),
            String::from("book"),
            "coat".to_string(),
        ]),
    );

    println!("{:?}", profile);
}
