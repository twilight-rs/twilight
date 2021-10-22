use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug)]
pub struct Form {
    boundary: [u8; 15],
    buffer: Vec<u8>,
}

impl Form {
    pub fn new() -> Self {
        Self::default()
    }

    /// Write the data needed to attach a multipart file to the buffer.
    pub fn attach(&mut self, id: u64, filename: &[u8], data: &[u8]) -> &mut Self {
        self.start();
        self.name_id(id);
        self.filename(filename);
        self.data(data);

        self
    }

    pub fn build(mut self) -> Vec<u8> {
        self.buffer.extend(b"\r\n");
        self.boundary();
        self.buffer.extend(b"--");

        self.buffer
    }

    pub fn content_type(&self) -> Vec<u8> {
        const NAME: &str = "multipart/form-data; boundary=";

        let mut content_type = Vec::with_capacity(NAME.len() + 15);
        content_type.extend(NAME.as_bytes());
        content_type.extend(&self.boundary);

        content_type
    }

    #[deprecated(since = "0.7.1", note = "use attach instead")]
    pub fn file(&mut self, name: &[u8], filename: &[u8], data: &[u8]) -> &mut Self {
        self.start();
        self.name(name);
        self.filename(filename);
        self.data(data);

        self
    }

    pub fn part(&mut self, name: &[u8], data: &[u8]) -> &mut Self {
        self.start();
        self.name(name);
        self.data(data);

        self
    }

    pub fn payload_json(&mut self, json: &[u8]) -> &mut Self {
        self.start();
        self.name(b"payload_json");
        self.buffer.extend(b"\r\nContent-Type: application/json");
        self.data(json);

        self
    }

    fn start(&mut self) {
        self.buffer.extend(b"\r\n");
        self.boundary();
        self.buffer.extend(b"\r\nContent-Disposition: form-data");
    }

    fn boundary(&mut self) {
        self.buffer.extend(b"--");
        self.buffer.extend(&self.boundary);
    }

    fn filename(&mut self, filename: &[u8]) {
        self.buffer.extend(br#"; filename=""#);
        self.buffer.extend(filename);
        self.buffer.push(b'"');
    }

    fn name(&mut self, name: &[u8]) {
        self.buffer.extend(br#"; name=""#);
        self.buffer.extend(name);
        self.buffer.push(b'"');
    }

    fn name_id(&mut self, id: u64) {
        self.buffer.extend(br#"; name="files["#);
        push_digits(id, &mut self.buffer);
        self.buffer.extend(br#"]""#);
    }

    fn data(&mut self, data: &[u8]) {
        self.buffer.extend(b"\r\n\r\n");
        self.buffer.extend(data);
    }
}

impl Default for Form {
    fn default() -> Self {
        let mut boundary = [0; 15];
        let mut rng = rand::thread_rng();

        for value in &mut boundary {
            *value = rng.sample(Alphanumeric);
        }

        Self {
            boundary,
            buffer: Vec::new(),
        }
    }
}

/// Value of '0' in ascii
const ASCII_NUMBER: u8 = 0x30;

/// Extend the buffer with the digits of the integer `id`, the reason
/// for this is to get around a allocation by for example using
/// `format!("files[{}]", id)`.
fn push_digits(mut id: u64, buf: &mut Vec<u8>) {
    // The largest 64 bit integer is 20 digits.
    let mut inner_buf = [0_u8; 20];
    // Amount of digits written to the inner buffer.
    let mut i = 0;

    // While the number have more than one digit we print the last
    // digit by taking the rest after modulo 10. We then divide with
    // 10 to truncate the number from the right and then loop
    while id >= 10 {
        // To go from the interger to the ascii value we add the
        // ascii value of '0'.
        //
        // (id % 10) will always be less than 10 so trunccation cannot
        // happen.
        #[allow(clippy::cast_possible_truncation)]
        let ascii = (id % 10) as u8 + ASCII_NUMBER;
        inner_buf[i] = ascii;
        id /= 10;
        i += 1;
    }
    // (id % 10) will always be less than 10 so trunccation cannot
    // happen.
    #[allow(clippy::cast_possible_truncation)]
    let ascii = (id % 10) as u8 + ASCII_NUMBER;
    inner_buf[i] = ascii;
    i += 1;

    // As we have written the digits in reverse we reverse the area of
    // the array we have been using to get the characters in the
    // correct order.
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
