#[derive(Clone, Debug)]
#[must_use = "has no effect if not built into a Form"]
pub struct Form {
    boundary: [u8; 15],
    buffer: Vec<u8>,
}

impl Form {
    const APPLICATION_JSON: &'static [u8; 16] = b"application/json";

    const BOUNDARY_TERMINATOR: &'static [u8; 2] = b"--";
    const CONTENT_DISPOSITION_1: &'static [u8; 38] = b"Content-Disposition: form-data; name=\"";
    const CONTENT_DISPOSITION_2: &'static [u8; 13] = b"\"; filename=\"";
    const CONTENT_DISPOSITION_3: &'static [u8; 1] = b"\"";
    const CONTENT_TYPE: &'static [u8; 14] = b"Content-Type: ";
    const NEWLINE: &'static [u8; 2] = b"\r\n";

    pub fn new() -> Self {
        Self::default()
    }

    /// Consume the form, returning the buffer's contents.
    pub fn build(mut self) -> Vec<u8> {
        self.buffer.extend(Self::BOUNDARY_TERMINATOR);

        self.buffer
    }

    /// Get the form's appropriate content type for requests.
    pub fn content_type(&self) -> Vec<u8> {
        const NAME: &str = "multipart/form-data; boundary=";

        let mut content_type = Vec::with_capacity(NAME.len() + self.boundary.len());
        content_type.extend(NAME.as_bytes());
        content_type.extend(self.boundary);

        content_type
    }

    pub fn part(mut self, name: &[u8], value: &[u8]) -> Self {
        self.reserve_part(
            Self::CONTENT_DISPOSITION_1.len() + name.len() + Self::CONTENT_DISPOSITION_3.len(),
            value.len(),
        );

        // Write the Content-Disposition header.
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(Self::CONTENT_DISPOSITION_1);
        self.buffer.extend(name);
        self.buffer.extend(Self::CONTENT_DISPOSITION_3);
        self.buffer.extend(Self::NEWLINE);

        // Write a newline between the headers and the value, the value
        // itself, a newline, and finally the boundary.
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(value);
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(Self::BOUNDARY_TERMINATOR);
        self.buffer.extend(self.boundary);

        self
    }

    pub fn file_part(mut self, name: &[u8], filename: &[u8], value: &[u8]) -> Self {
        self.reserve_part(
            Self::CONTENT_DISPOSITION_1.len()
                + name.len()
                + Self::CONTENT_DISPOSITION_2.len()
                + filename.len()
                + Self::CONTENT_DISPOSITION_3.len(),
            value.len(),
        );

        // Write the Content-Disposition header.
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(Self::CONTENT_DISPOSITION_1);
        self.buffer.extend(name);
        self.buffer.extend(Self::CONTENT_DISPOSITION_2);
        self.buffer.extend(filename);
        self.buffer.extend(Self::CONTENT_DISPOSITION_3);
        self.buffer.extend(Self::NEWLINE);

        // Write a newline between the headers and the value, the value
        // itself, a newline, and finally the boundary.
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(value);
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(Self::BOUNDARY_TERMINATOR);
        self.buffer.extend(self.boundary);

        self
    }

    /// Preview the built buffer's length without consuming the form.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.buffer.len() + Self::BOUNDARY_TERMINATOR.len()
    }

    pub fn json_part(mut self, name: &[u8], value: &[u8]) -> Self {
        self.reserve_part(
            Self::CONTENT_DISPOSITION_1.len()
                + name.len()
                + Self::CONTENT_DISPOSITION_3.len()
                + Self::CONTENT_TYPE.len()
                + Self::APPLICATION_JSON.len()
                + Self::NEWLINE.len(),
            value.len(),
        );

        // Write the Content-Disposition header.
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(Self::CONTENT_DISPOSITION_1);
        self.buffer.extend(name);
        self.buffer.extend(Self::CONTENT_DISPOSITION_3);
        self.buffer.extend(Self::NEWLINE);

        // If there is a Content-Type, write its key, itself, and a newline.
        self.buffer.extend(Self::CONTENT_TYPE);
        self.buffer.extend(Self::APPLICATION_JSON);
        self.buffer.extend(Self::NEWLINE);

        // Write a newline between the headers and the value, the value
        // itself, a newline, and finally the boundary.
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(value);
        self.buffer.extend(Self::NEWLINE);
        self.buffer.extend(Self::BOUNDARY_TERMINATOR);
        self.buffer.extend(self.boundary);

        self
    }
}

impl Form {
    /// Reserve exactly enough for one part plus the closing terminator
    /// `build` appends, so a large file part is written into a single
    /// allocation of its own size instead of doubling the buffer's capacity
    /// (and copying it) on the way.
    fn reserve_part(&mut self, header: usize, value: usize) {
        self.buffer.reserve_exact(
            Self::NEWLINE.len() * 4
                + header
                + value
                + Self::BOUNDARY_TERMINATOR.len() * 2
                + self.boundary.len(),
        );
    }
}

impl Default for Form {
    fn default() -> Self {
        let mut form = Self {
            boundary: random_boundary(),
            buffer: Vec::new(),
        };

        // Write the first boundary.
        form.buffer.extend(Self::BOUNDARY_TERMINATOR);
        form.buffer.extend(form.boundary);

        form
    }
}

/// Generate a random boundary that is 15 characters long.
pub fn random_boundary() -> [u8; 15] {
    let mut boundary = [0; 15];

    for value in &mut boundary {
        *value = fastrand::alphanumeric() as u8;
    }

    boundary
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn form_builder() {
        let form = Form::new()
            .json_part(b"payload_json", b"json_value")
            .file_part(b"files[0]", b"filename.jpg", b"file_value");

        let boundary = str::from_utf8(&form.boundary).unwrap();
        let expected = format!(
            "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"payload_json\"\r\n\
        Content-Type: application/json\r\n\
        \r\n\
        json_value\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"files[0]\"; filename=\"filename.jpg\"\r\n\
        \r\n\
        file_value\r\n\
        --{boundary}--",
        );

        let buffer_len = form.len();
        let buffer = form.build();

        assert_eq!(expected.as_bytes(), buffer);
        assert_eq!(buffer_len, buffer.len());
    }

    #[test]
    fn file_part_is_one_exact_allocation() {
        let file = vec![7u8; 9 * 1024 * 1024];
        let form = Form::new()
            .json_part(b"payload_json", b"{}")
            .file_part(b"files[0]", b"chunk.bin", &file);

        let buffer = form.build();

        assert_eq!(buffer.capacity(), buffer.len());
    }
}
