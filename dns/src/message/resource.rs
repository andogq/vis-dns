use crate::enums::*;
use crate::primatives::*;
use std::time::Duration;

/// Resource record section of a message. Used for the answer, authority, and additional sections.
pub struct Resource {
    /// Domain name for the resource record
    name: LabelSequence,
    /// Type for the recource record
    r#type: Type,
    /// Class of the resource record
    class: Class,
    /// Time to live of the resource record
    ttl: Duration,
    /// Size of the data section, in bytes
    data_length: u16,
    // Data section
    data: Vec<u8>,
}
