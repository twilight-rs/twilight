use crate::request::Form;
use serde::{Deserialize, Serialize};
use twilight_model::{
    http::attachment::Attachment,
    id::{marker::AttachmentMarker, Id},
};

pub struct AttachmentManager<'a> {
    files: Vec<&'a Attachment>,
    ids: Vec<Id<AttachmentMarker>>,
}

impl<'a> AttachmentManager<'a> {
    pub const fn new() -> Self {
        Self {
            files: Vec::new(),
            ids: Vec::new(),
        }
    }

    pub fn build_form(&self, fields: &'a [u8]) -> Form {
        let mut form = Form::new().json_part(b"payload_json", fields);

        for file in &self.files {
            let mut name = Vec::with_capacity(7 + num_digits(file.id));
            name.extend(b"files[");
            push_digits(file.id, &mut name);
            name.extend(b"]");

            form = form.file_part(name.as_ref(), file.filename.as_bytes(), file.file.as_ref());
        }

        form
    }

    pub fn get_partial_attachments(&self) -> Vec<PartialAttachment<'a>> {
        self.files
            .iter()
            .map(|attachment| PartialAttachment {
                description: attachment.description.as_deref(),
                filename: Some(attachment.filename.as_ref()),
                id: attachment.id,
            })
            .chain(self.ids.iter().map(|id| PartialAttachment {
                description: None,
                filename: None,
                id: id.get(),
            }))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty() && self.ids.is_empty()
    }

    #[must_use = "has no effect if not built into a Form"]
    pub fn set_files(mut self, files: Vec<&'a Attachment>) -> Self {
        self.files = files;

        self
    }

    #[must_use = "has no effect if not built into a Form"]
    pub fn set_ids(mut self, ids: Vec<Id<AttachmentMarker>>) -> Self {
        self.ids = ids;

        self
    }
}

impl Default for AttachmentManager<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialAttachment<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<&'a str>,
    pub id: u64,
}

/// Count the number of digits in a given number.
const fn num_digits(index: u64) -> usize {
    let mut index = index;
    let mut len = 0;

    if index < 10 {
        return 1;
    }

    while index > 0 {
        index /= 10;
        len += 1;
    }

    len
}

/// Value of '0' in ascii
const ASCII_NUMBER: u8 = 0x30;

/// Extend the buffer with the digits of the integer `id`.
///
/// The reason for this is to get around a allocation by for example using
/// `format!("files[{id}]")`.
fn push_digits(mut id: u64, buf: &mut Vec<u8>) {
    // The largest 64 bit integer is 20 digits.
    let mut inner_buf = [0_u8; 20];
    // Amount of digits written to the inner buffer.
    let mut i = 0;

    // While the number have more than one digit we print the last digit by
    // taking the rest after modulo 10. We then divide with 10 to truncate the
    // number from the right and then loop
    while id >= 10 {
        // To go from the integer to the ascii value we add the ascii value of
        // '0'.
        //
        // (id % 10) will always be less than 10 so truncation cannot happen.
        #[allow(clippy::cast_possible_truncation)]
        let ascii = (id % 10) as u8 + ASCII_NUMBER;
        inner_buf[i] = ascii;
        id /= 10;
        i += 1;
    }
    // (id % 10) will always be less than 10 so truncation cannot happen.
    #[allow(clippy::cast_possible_truncation)]
    let ascii = (id % 10) as u8 + ASCII_NUMBER;
    inner_buf[i] = ascii;
    i += 1;

    // As we have written the digits in reverse we reverse the area of the array
    // we have been using to get the characters in the correct order.
    inner_buf[..i].reverse();

    buf.extend_from_slice(&inner_buf[..i]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_digits_limits() {
        let min_d = b"0";
        let max_d = b"18446744073709551615";

        let mut min_v = Vec::new();
        let mut max_v = Vec::new();

        push_digits(u64::MIN, &mut min_v);
        push_digits(u64::MAX, &mut max_v);

        assert_eq!(min_d[..], min_v[..]);
        assert_eq!(max_d[..], max_v[..]);
    }

    #[test]
    fn num_digits_count() {
        assert_eq!(1, num_digits(0));
        assert_eq!(1, num_digits(1));
        assert_eq!(2, num_digits(10));
    }
}
