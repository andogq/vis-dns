use crate::enums::*;
use bitvec::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("Header size must be 12 bytes, however was only {0} bytes")]
    InvalidHeaderSize(usize),
}

/// Header section of a message. Always present for every type of message.
pub struct Header {
    /// Assigned by the program that generates a query, and returned by the server (`ID`)
    pub(crate) id: u16,
    /// Specifies message type (`QR`)
    pub(crate) r#type: MessageType,
    /// Specifies the kind of query for this message (`OPCODE`)
    pub(crate) opcode: Opcode,
    /// Indicates the responding server is an authority for the domain (`AA`)
    pub(crate) authoritative_answer: bool,
    /// Indicates that the message was truncated (`TC`)
    pub(crate) truncation: bool,
    /// Set in the query and copied into the response to indicate that the client wants the name
    /// server to query recursively (`RD`)
    pub(crate) recursion_desired: bool,
    /// Set in the response, to indicate that the name server supports recursive querying (`RA`)
    pub(crate) recursion_available: bool,
    /// Set in the response to a query (`RCODE`)
    pub(crate) response_code: ResponseCode,
    /// Number of question entries (`QDCOUNT`)
    pub(crate) question_count: u16,
    /// Number of resource records in the answer section (`ANCOUNT`)
    pub(crate) resource_records_count: u16,
    /// Number of name server records in the authority section (`NSCOUNT`)
    pub(crate) name_server_records_count: u16,
    /// Number of resource records in the additional records section (`ARCOUNT`)
    pub(crate) additional_records_count: u16,
}

impl From<Header> for Vec<u8> {
    fn from(header: Header) -> Self {
        [
            // 2 bytes
            header.id.to_be_bytes().as_slice(),
            &[
                // 1 byte packed
                ((u8::from(header.r#type) << 7) & 0b10000000)
                    | ((u8::from(header.opcode) << 3) & 0b01111000)
                    | ((u8::from(header.authoritative_answer) << 2) & 0b00000100)
                    | ((u8::from(header.truncation) << 1) & 0b00000010)
                    | ((u8::from(header.recursion_desired) << 0) & 0b00000001),
                // 1 byte packed
                ((u8::from(header.recursion_available) << 7) & 0b10000000)
                    | ((0b000 << 4) & 0b01110000) // Z
                    | ((u8::from(header.response_code) << 0) & 0b00001111),
            ],
            header.question_count.to_be_bytes().as_slice(),
            header.resource_records_count.to_be_bytes().as_slice(),
            header.name_server_records_count.to_be_bytes().as_slice(),
            header.additional_records_count.to_be_bytes().as_slice(),
        ]
        .concat()
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = HeaderError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 12 {
            return Err(Self::Error::InvalidHeaderSize(bytes.len()));
        }

        let mut bytes = bytes.iter();

        Ok(Header {
            id: u16::from_be_bytes([*bytes.next().unwrap(), *bytes.next().unwrap()]),
            r#type: MessageType::try_from(bytes.next().unwrap())?,
            opcode: todo!(),
            authoritative_answer: todo!(),
            truncation: todo!(),
            recursion_desired: todo!(),
            recursion_available: todo!(),
            response_code: todo!(),
            question_count: todo!(),
            resource_records_count: todo!(),
            name_server_records_count: todo!(),
            additional_records_count: todo!(),
        })
    }
}
