use crate::request::AttachmentFile;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug)]
pub struct Form {
    boundary: [u8; 15],
    buffer: Vec<u8>,
}

impl Form {
    /// Get the form's appropriate content type for requests.
    pub fn content_type(&self) -> Vec<u8> {
        const NAME: &str = "multipart/form-data; boundary=";

        let mut content_type = Vec::with_capacity(NAME.len() + 15);
        content_type.extend(NAME.as_bytes());
        content_type.extend(&self.boundary);

        content_type
    }

    /// Return the form's contents.
    pub fn buffer(mut self) -> Vec<u8> {
        self.buffer
    }
}

pub struct FormBuilder<'a> {
    attachments: &'a [AttachmentFile<'a>],
    boundary: [u8; 15],
    payload_json: &'a [u8],
}

impl<'a> FormBuilder<'a> {
    pub fn new(payload_json: &'a [u8]) -> Self {
        let mut boundary = [0; 15];
        let mut rng = rand::thread_rng();

        for value in &mut boundary {
            *value = rng.sample(Alphanumeric);
        }

        Self {
            attachments: &[],
            boundary,
            payload_json,
        }
    }

    pub const fn attachments(mut self, attachments: &'a [AttachmentFile<'a>]) -> Self {
        self.attachments = attachments;

        self
    }

    pub fn build(self) -> Form {
        let FormBuilder {
            attachments,
            boundary,
            payload_json,
        } = self;

        let mut buffer = Vec::new();

        // Write the first boundary.
        buffer.extend(br#"--"#);
        buffer.extend(&boundary);
        buffer.extend(br#"\r\n"#);

        // Write the JSON payload.
        buffer.extend(br#"Content-Disposition: form-data; name="payload_json"\r\n"#);
        buffer.extend(br#"Content-Type: application/json\r\n"#);
        buffer.extend(br#"\r\n"#);
        buffer.extend(payload_json);

        if attachments.is_empty() {
            // Write the last boundary.
            buffer.extend(br#"--"#);
            buffer.extend(&boundary);
            buffer.extend(br#"--"#);
        } else {
            // Write a boundary between the JSON and the attachments.
            buffer.extend(br#"--"#);
            buffer.extend(&boundary);
            buffer.extend(br#"\r\n"#);

            for (index, attachment) in attachments.iter().enumerate() {
                // Write the Content Disposition, name, and filename.
                //
                // Example:
                // `Content-Disposition: form-data; name="files[0]"; filename="horse.jpg"`
                buffer.extend(br#"Content-Disposition: form-data; name="files["#);
                push_digits(index as u64, &mut buffer);
                buffer.extend(br#"]; filename=""#);
                buffer.extend(attachment.filename.as_bytes());
                buffer.extend(br#""\r\n"#);

                // Write a blank line between the headers and the data.
                buffer.extend(b"\r\n");

                // Write the image data.
                buffer.extend(attachment.file);

                // Write a boundary between attachments, or part of the last
                // boundary.
                buffer.extend(br#"--"#);
                buffer.extend(&boundary);
            }

            // Since the attachments loop has ended and we have nothing left to
            // add to the form, write the final boundary marker.
            buffer.extend(b"--");
        }

        // Return the completed form, and its boundary, to be used in requests.
        Form { boundary, buffer }
    }
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
    use super::push_digits;

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
}
