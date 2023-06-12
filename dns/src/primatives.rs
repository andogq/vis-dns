use std::ops::Deref;

use thiserror::Error;

/// Represents a portion of a domain name. First octet contains the number of octets in the label
/// (two MSBs are `0`, so can only contain 63 octets), followed by the content of the label.
///
/// # Examples
///
/// ```
/// use dns::Label;
///
/// // Parse a slice of bytes into a Label
/// let bytes: &[u8] = &[0x05, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e];
/// let label = Label::try_from(bytes).unwrap();
///
/// // Convert Label to bytes
/// let to_bytes: Vec<u8> = label.into();
///
/// assert_eq!(bytes, to_bytes);
/// ```
pub struct Label {
    size: u8,
    content: Vec<u8>,
}
impl From<Label> for Vec<u8> {
    fn from(label: Label) -> Self {
        [&[label.size], label.content.as_slice()].concat()
    }
}
impl TryFrom<&[u8]> for Label {
    type Error = LabelError;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.is_empty() || bytes.len() > 63 {
            return Err(Self::Error::InvalidSize(bytes.len()));
        }

        let size_octet = bytes[0];
        let content = &bytes[1..];

        if size_octet > 63 {
            return Err(Self::Error::InvalidSizeOctet(size_octet));
        }

        if content.len() != size_octet as usize {
            return Err(Self::Error::SizeOctetMismatch {
                size_octet,
                content_size: content.len(),
            });
        }

        Ok(Self {
            size: size_octet,
            content: content.to_vec(),
        })
    }
}

#[derive(Debug, Error)]
pub enum LabelError {
    #[error("content must be greater than 0 bytes, and cannot be longer than 63 bytes, however content is {0}")]
    InvalidSize(usize),
    #[error("size octet cannot be greater than 63, however it is {0}")]
    InvalidSizeOctet(u8),
    #[error("size octet ({size_octet}) does not match content size ({content_size})")]
    SizeOctetMismatch { size_octet: u8, content_size: usize },
}

/// A sequence of labels that are combined and treated as one.
pub struct LabelSequence(Vec<Label>);
impl Deref for LabelSequence {
    type Target = [Label];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<LabelSequence> for Vec<u8> {
    fn from(sequence: LabelSequence) -> Self {
        sequence.0.into_iter().flat_map(Vec::<u8>::from).collect()
    }
}
impl TryFrom<&[u8]> for LabelSequence {
    type Error = LabelError;

    /// Converts a slice of bytes into a `LabelSequence`.
    ///
    /// **Note**: This implementation assumes that every label is the full 64 bytes (size + 63
    /// bytes of content), aside from the last label in the sequence.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let full_labels = bytes.len() / 64;
        let partial_label = bytes.len() % 64;

        let number_labels = full_labels + partial_label.max(1);
        let mut labels = Vec::with_capacity(number_labels);

        for i in 0..full_labels {
            labels.push(Label::try_from(&bytes[i..i + 64])?);
        }

        if partial_label > 0 {
            labels.push(Label::try_from(&bytes[bytes.len() - partial_label..])?);
        }

        Ok(Self(labels))
    }
}
