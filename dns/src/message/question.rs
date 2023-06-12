use crate::enums::*;
use crate::primatives::*;

/// Question section of a mesasge.
pub struct Question {
    /// A domain name for the question (`QNAME`)
    domain_name: LabelSequence,
    /// Type of query to run (`QTYPE`)
    r#type: QueryType,
    /// Class of the query (`QCLASS`)
    class: QueryClass,
}
impl Question {
    pub fn new(
        domain_name: &str,
        r#type: QueryType,
        class: QueryClass,
    ) -> Result<Self, LabelError> {
        Ok(Question {
            domain_name: LabelSequence::try_from(domain_name.as_bytes())?,
            r#type,
            class,
        })
    }
}
