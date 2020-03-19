// trait Error {}

#[derive(Debug)]
pub struct ReadError {
    pub message: String,
}

// impl Error for IoError {}

impl ReadError {
    pub fn new(message: String) -> Self {
        ReadError { message }
    }
}

pub struct WriteError {}

// impl From<None> for IoError {
//     fn from(_: None) -> Self {
//         IoError{}
//     }
// }

// impl Display for IoError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<> {
//         unimplemented!()
//     }
// }
