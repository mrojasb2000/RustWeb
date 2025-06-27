mod to_do;
use to_do::ItemTypes;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;

fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::DONE);
    match to_do_item {
        ItemTypes::Done(item) => {
            println!("{}", item.super_struct.title);
            println!("{}", item.super_struct.status.stringify());
        }
        ItemTypes::Pending(item) => {
            println!("{}", item.super_struct.title);
            println!("{}", item.super_struct.status.stringify());
        }
    }
}
