use crate::util::macros::value_enum;

value_enum!(
    /// Specifies the message type. Corresponds to the `QR` field (1 bit) in the header section.
    MessageType: u8 {
        /// Message is a query
        Query = 0,
        /// Message is a response
        Response = 1
    }
);

value_enum!(
    /// Specifies the kind of query in the message, and is copied from the request into the
    /// response. Corresponds to the `OPCODE` field (4 bits) in the header section.
    Opcode: u8 {
        /// Standard query (`QUERY`)
        Query = 0,
        /// Inverse query (`IQUERY`)
        InverseQuery = 1,
        /// Server status request (`STATUS`)
        ServerStatus = 2,
        [
            /// Reserved opcode for future use
            Reserved
        ]
    }
);

value_enum!(
    /// Used in responses. Corresponds to `RCODE` field (4 bits) in the header section.
    ResponseCode: u8 {
        /// No error condition
        NoErrorCondition = 0,
        /// Name server could not interpret the query
        FormatError = 1,
        /// Name server unauble to process query due to a server problem
        ServerFailure = 2,
        /// Domain referenced in query does not exist (only meaningful from an authoritative name
        /// server
        NameError = 3,
        /// Name server does not support the query
        NotImplemented = 4,
        /// Name server refused the operation
        Refused = 5,
        [
            /// Reserved response code for future use
            Reserved
        ]
    }
);
impl Default for ResponseCode {
    fn default() -> Self {
        Self::NoErrorCondition
    }
}

value_enum!(
    /// Resource record type. Referred to as `TYPE`.
    Type: u8 {
        /// Host address
        A = 1,
        /// Authoritative name server
        NS = 2,
        /// Mail destination (obsolete)
        MD = 3,
        /// Mail forwarder (obsolete)
        MF = 4,
        /// Canonical name for an alias
        CNAME = 5,
        /// Marks the start of a zone of authority
        SOA = 6,
        /// Mail box domain name (experimental)
        MB = 7,
        /// Mail box group member (experimental)
        MG = 8,
        /// Mail rename domain name (experimental)
        MR = 9,
        /// Null resoruce record
        NULL = 10,
        /// Well known service description
        WKS = 11,
        /// Domain name pointer
        PTR = 12,
        /// Host information
        HINFO = 13,
        /// Mailbox or mail list information
        MINFO = 14,
        /// Mail exchange
        MX = 15,
        /// Text strings
        TXT = 16
    }
);

value_enum!(
    /// Query type for a question. Is a superset of [TYPE]. Generally referred to as `QTYPE`.
    QueryType: u8 {
        /// Superset of [Type]
        ...Type,
        /// Request for a transfer of an entire zone
        AXFR = 252,
        /// REquest for mailbox-related records (MB, MG or MR)
        MAILB = 253,
        /// Request for mail agent resource records (obsolete)
        MAILA = 254,
        // Request for all records (aka `*`)
        ALL = 255
    }
);

value_enum!(
    /// Class for resource record.
    Class: u8 {
        /// The Internet
        IN = 1,
        /// CSNET class (obsolete)
        CS = 2,
        /// CHAOS class
        CH = 3,
        /// Hesoid
        HS = 4
    }
);

value_enum!(
    /// Query class for a question. Is a superset of `CLASS`. Generally referred to as `QCLASS`.
    QueryClass: u8 {
        // Superset of [Class]
        ...Class,
        /// Request for any class (aka `*`)
        ANY = 255
    }
);
