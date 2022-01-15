use crate::{error::Error as HttpError, request::attachment::AttachmentFile};
use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Form {
    boundary: [u8; 15],
    buffer: Vec<u8>,
}

impl Form {
    pub fn new(boundary: [u8; 15], buffer: Vec<u8>) -> Self {
        Self { boundary, buffer }
    }

    /// Get the form's appropriate content type for requests.
    pub fn content_type(&self) -> Vec<u8> {
        const NAME: &str = "multipart/form-data; boundary=";

        let mut content_type = Vec::with_capacity(NAME.len() + 15);
        content_type.extend(NAME.as_bytes());
        content_type.extend(&self.boundary);

        content_type
    }

    /// Consume the form, returning the buffer's contents.
    #[allow(clippy::missing_const_for_fn)]
    pub fn buffer(self) -> Vec<u8> {
        self.buffer
    }
}

pub struct FormBuilder<'a> {
    attachments: &'a [AttachmentFile<'a>],
    boundary: [u8; 15],
    payload_json: Cow<'a, [u8]>,
}

impl<'a> FormBuilder<'a> {
    const BOUNDARY_LEN: usize = 15;
    const BOUNDARY_TERMINATOR: &'static [u8; 2] = b"--";
    const NEWLINE: &'static [u8; 2] = b"\r\n";

    const CONTENT_DISPOSITION_PAYLOAD_JSON: &'static [u8; 51] =
        br#"Content-Disposition: form-data; name="payload_json""#;
    const CONTENT_TYPE_JSON: &'static [u8; 30] = br#"Content-Type: application/json"#;

    const CONTENT_DISPOSITION_IMAGE_PRE: &'static [u8; 44] =
        br#"Content-Disposition: form-data; name="files["#;
    const CONTENT_DISPOSITION_IMAGE_MID: &'static [u8; 14] = br#"]"; filename=""#;
    const CONTENT_DISPOSITION_IMAGE_POST: &'static [u8; 1] = br#"""#;

    pub fn from_fields(fields: &impl Serialize) -> Result<Self, HttpError> {
        crate::json::to_vec(fields)
            .map(Cow::Owned)
            .map(FormBuilder::from_payload_json)
            .map_err(HttpError::json)
    }

    pub fn from_payload_json(payload_json: Cow<'a, [u8]>) -> Self {
        Self {
            attachments: &[],
            boundary: random_boundary(),
            payload_json,
        }
    }

    pub const fn attachments(mut self, attachments: &'a [AttachmentFile<'a>]) -> Self {
        self.attachments = attachments;

        self
    }

    /// Precalculate the length of the buffer.
    pub fn count(&self) -> usize {
        let mut len = 0;

        // Length of the first boundary.
        len += Self::BOUNDARY_TERMINATOR.len();
        len += Self::BOUNDARY_LEN;
        len += Self::NEWLINE.len();

        // Length of the JSON payload.
        len += Self::CONTENT_DISPOSITION_PAYLOAD_JSON.len();
        len += Self::CONTENT_TYPE_JSON.len();
        len += Self::NEWLINE.len() * 4;
        len += self.payload_json.len();

        if self.attachments.is_empty() {
            // Length of the last boundary.
            len += Self::BOUNDARY_TERMINATOR.len() * 2;
            len += Self::BOUNDARY_LEN;
        } else {
            // Length of the boundary between JSON and the first attachment.
            len += Self::BOUNDARY_TERMINATOR.len();
            len += Self::BOUNDARY_LEN;

            for (index, attachment) in self.attachments.iter().enumerate() {
                len += Self::CONTENT_DISPOSITION_IMAGE_PRE.len();
                len += Self::CONTENT_DISPOSITION_IMAGE_MID.len();
                len += Self::CONTENT_DISPOSITION_IMAGE_POST.len();
                len += Self::NEWLINE.len() * 4;

                len += attachment.filename.len();
                len += attachment.file.len();

                // Add the length of the index.
                len += num_digits(index);

                len += Self::BOUNDARY_TERMINATOR.len();
                len += Self::BOUNDARY_LEN;
            }

            // Attachment loop has ended, add the length of a final terminator.
            len += Self::BOUNDARY_TERMINATOR.len();
        }

        len
    }

    pub fn build(self) -> Form {
        let mut buffer = Vec::with_capacity(self.count());

        let FormBuilder {
            attachments,
            boundary,
            payload_json,
        } = self;

        // Write the first boundary.
        //
        // # Example
        //
        // --abcdefghijklmno
        buffer.extend(Self::BOUNDARY_TERMINATOR);
        buffer.extend(&boundary);
        buffer.extend(Self::NEWLINE);

        // Write the JSON payload.
        //
        // # Example
        //
        // Content-Disposition: form-data; name="payload-json"
        // Content-Type: application/json
        //
        // {"content":"horse website"}
        buffer.extend(Self::CONTENT_DISPOSITION_PAYLOAD_JSON);
        buffer.extend(Self::NEWLINE);
        buffer.extend(Self::CONTENT_TYPE_JSON);
        buffer.extend(Self::NEWLINE);
        buffer.extend(Self::NEWLINE);
        buffer.extend(&*payload_json);
        buffer.extend(Self::NEWLINE);

        if attachments.is_empty() {
            // Write the last boundary.
            buffer.extend(Self::BOUNDARY_TERMINATOR);
            buffer.extend(&boundary);
            buffer.extend(Self::BOUNDARY_TERMINATOR);
        } else {
            // Write a boundary between the JSON and the attachments.
            buffer.extend(Self::BOUNDARY_TERMINATOR);
            buffer.extend(&boundary);

            // Write the image data.
            //
            // # Example
            //
            // Content-Disposition: form-data; name="files[0]"; filename="horse.jpg"
            //
            // [image bytes]
            // --abcdefghijklmno
            for (index, attachment) in attachments.iter().enumerate() {
                // Write a blank line after the last boundary.
                buffer.extend(Self::NEWLINE);

                // Write the Content-Disposition header, with name and filename.
                buffer.extend(Self::CONTENT_DISPOSITION_IMAGE_PRE);
                push_digits(index as u64, &mut buffer);
                buffer.extend(Self::CONTENT_DISPOSITION_IMAGE_MID);
                buffer.extend(attachment.filename.as_bytes());
                buffer.extend(Self::CONTENT_DISPOSITION_IMAGE_POST);
                buffer.extend(Self::NEWLINE);

                // Write a blank line between the headers and the data.
                buffer.extend(Self::NEWLINE);

                // Write the image data.
                buffer.extend(attachment.file);

                // Write a blank line between the image data and the next
                // boundary.
                buffer.extend(Self::NEWLINE);

                // Write a boundary between attachments, or part of the last
                // boundary.
                buffer.extend(Self::BOUNDARY_TERMINATOR);
                buffer.extend(&boundary);
            }

            // Since the attachments loop has ended and we have nothing left to
            // add to the form, write the final boundary marker.
            buffer.extend(Self::BOUNDARY_TERMINATOR);
        }

        // Return the completed form, and its boundary, to be used in requests.
        Form { boundary, buffer }
    }
}

/// Generate a random boundary that is 15 characters long.
pub fn random_boundary() -> [u8; 15] {
    let mut boundary = [0; 15];
    let mut rng = rand::thread_rng();

    for value in &mut boundary {
        *value = rng.sample(Alphanumeric);
    }

    boundary
}

/// Count the number of digits in a given number.
const fn num_digits(index: usize) -> usize {
    let mut index = index;
    let mut len = 0;

    if index == 0 {
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
/// `format!("files[{}]", id)`.
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

    buf.extend_from_slice(&inner_buf[..i])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::attachment::PartialAttachment;
    use serde::Serialize;
    use std::str;

    #[test]
    fn test_push_digits() {
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
    fn test_num_digits() {
        assert_eq!(1, num_digits(0));
        assert_eq!(1, num_digits(1));
        assert_eq!(2, num_digits(10));
    }

    #[test]
    fn test_form_builder() {
        #[derive(Serialize)]
        struct Fields<'a> {
            attachments: Vec<PartialAttachment<'a>>,
            content: String,
        }

        let attachment_file = AttachmentFile {
            description: Some("cool horse"),
            file: &[b'a'],
            filename: "applejack.jpg",
        };

        let partial_attachment = PartialAttachment {
            description: Some("cool horse"),
            filename: Some("applejack.jpg"),
            id: 0,
        };

        let fields = Fields {
            attachments: Vec::from([partial_attachment]),
            content: "horse picture".into(),
        };

        let payload_json = crate::json::to_vec(&fields).unwrap();
        let form_builder = FormBuilder::from_payload_json(Cow::Owned(payload_json));

        let attachments = Vec::from([attachment_file]);
        let form_builder = form_builder.attachments(attachments.as_ref());

        let actual_len = form_builder.count();

        let form = form_builder.build();

        let boundary_str = str::from_utf8(&form.boundary).unwrap();
        let expected = format!(
            "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"payload_json\"\r\n\
        Content-Type: application/json\r\n\
        \r\n\
        {{\"attachments\":[{{\"description\":\"cool horse\",\"filename\":\"applejack.jpg\",\"id\":0}}],\"content\":\"horse picture\"}}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"files[0]\"; filename=\"applejack.jpg\"\r\n\
        \r\n\
        a\r\n\
        --{boundary}--",
            boundary = boundary_str,
        );

        let buffer = form.buffer();

        assert_eq!(expected.as_bytes(), buffer);

        assert_eq!(expected.len(), actual_len);
    }
}
