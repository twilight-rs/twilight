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
