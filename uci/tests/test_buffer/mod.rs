use std::io;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct TestBuffer {
    buf: Arc<Mutex<Vec<u8>>>,
}

impl TestBuffer {
    pub fn new() -> Self {
        Self {
            buf: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> Vec<u8> {
        Arc::try_unwrap(self.buf)
            .expect("More than one Arc refers to the inner Vec")
            .into_inner()
            .expect("Error accessing inner value of mutex")
    }

    #[allow(dead_code)]
    pub fn into_string(self) -> String {
        String::from_utf8(self.into_inner()).expect("Error converting Vec<u8> to String")
    }

    pub fn split_off(&mut self, at: usize) -> Vec<u8> {
        self.buf.lock().expect("Error locking mutex").split_off(at)
    }
}

impl io::Write for TestBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.lock().expect("Error locking mutex").write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.lock().expect("Error locking mutex").flush()
    }
}
