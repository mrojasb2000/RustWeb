fn main() {
    enum SomeValue {
        StringValue(String),
        IntValue(i32),
    }

    let multi_array: [SomeValue; 4] = [
        SomeValue::StringValue(String::from("one")),
        SomeValue::IntValue(2),
        SomeValue::StringValue(String::from("threee")),
        SomeValue::IntValue(4),
    ];

    for value in multi_array {
        match value {
            SomeValue::StringValue(s) => println!("String: {}", s),
            SomeValue::IntValue(i) => println!("Integer: {}", i),
        }
    }
}
