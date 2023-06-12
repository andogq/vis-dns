use enums::{QueryClass, QueryType, Type};
use message::Question;

pub mod enums;
pub mod message;
pub mod primatives;
pub mod util;

pub fn test() {
    message::MessageBuilder::query()
        .set_id(10)
        .question(Question::new("ando.gq", QueryType::Type(Type::A), QueryClass::ANY).unwrap())
        .build()
        .unwrap();
}
