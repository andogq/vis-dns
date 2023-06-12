use thiserror::Error;

use super::*;
use crate::enums::*;

#[derive(Debug, Error)]
pub enum MessageError {

}

pub struct Message {
    header: Header,
    question: Vec<Question>,
    answer: Vec<Resource>,
    authority: Vec<Resource>,
    additional: Vec<Resource>,
}

impl TryFrom<Message> for &[u8] {
    type Error = MessageError;
    fn try_from(message: Message) -> Result<Self, Self::Error> {

    }
}

#[derive(Debug, Error)]
pub enum MessageBuilderError {
    #[error("ID has not been provided")]
    MissingId,
    #[error("Message type has not been provided")]
    MissingMessageType,
    #[error("Opcode has not been provided")]
    MissingOpcode,
    #[error("Response code has not been provided")]
    MissingResponseCode,
}

#[derive(Default)]
pub struct MessageBuilder {
    id: Option<u16>,
    message_type: Option<MessageType>,
    opcode: Option<Opcode>,
    authoritative_answer: bool,
    truncation: bool,
    recursion_desired: bool,
    recursion_available: bool,
    response_code: ResponseCode,

    questions: Vec<Question>,
    answers: Vec<Resource>,
    authorities: Vec<Resource>,
    additionals: Vec<Resource>,
}
impl MessageBuilder {
    pub fn query() -> Self {
        Self::default()
            .set_message_type(MessageType::Query)
            .set_opcode(Opcode::Query)
    }

    pub fn set_id(mut self, id: u16) -> Self {
        self.id = Some(id);
        self
    }

    pub fn set_message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = Some(message_type);
        self
    }

    pub fn set_opcode(mut self, opcode: Opcode) -> Self {
        self.opcode = Some(opcode);
        self
    }

    pub fn set_authoritative_answer(mut self, authoritative_answer: bool) -> Self {
        self.authoritative_answer = authoritative_answer;
        self
    }

    pub fn set_truncation(mut self, truncation: bool) -> Self {
        self.truncation = truncation;
        self
    }

    pub fn set_recursion_desired(mut self, recursion_desired: bool) -> Self {
        self.recursion_desired = recursion_desired;
        self
    }

    pub fn set_recursion_available(mut self, recursion_available: bool) -> Self {
        self.recursion_available = recursion_available;
        self
    }

    pub fn set_response_code(mut self, response_code: ResponseCode) -> Self {
        self.response_code = response_code;
        self
    }

    pub fn question(mut self, question: Question) -> Self {
        self.questions.push(question);
        self
    }

    pub fn answer(mut self, answer: Resource) -> Self {
        self.answers.push(answer);
        self
    }

    pub fn authority(mut self, authority: Resource) -> Self {
        self.authorities.push(authority);
        self
    }

    pub fn additional(mut self, additional: Resource) -> Self {
        self.additionals.push(additional);
        self
    }

    pub fn build(self) -> Result<Message, MessageBuilderError> {
        let header = Header {
            id: self.id.ok_or(MessageBuilderError::MissingId)?,
            r#type: self
                .message_type
                .ok_or(MessageBuilderError::MissingMessageType)?,
            opcode: self.opcode.ok_or(MessageBuilderError::MissingOpcode)?,
            authoritative_answer: self.authoritative_answer,
            truncation: self.truncation,
            recursion_desired: self.recursion_desired,
            recursion_available: self.recursion_available,
            response_code: self.response_code,
            question_count: self.questions.len() as u16,
            resource_records_count: self.answers.len() as u16,
            name_server_records_count: self.authorities.len() as u16,
            additional_records_count: self.additionals.len() as u16,
        };

        Ok(Message {
            header,
            question: self.questions,
            answer: self.answers,
            authority: self.authorities,
            additional: self.additionals,
        })
    }
}
