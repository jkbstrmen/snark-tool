trait Error {}

#[derive(Debug)]
pub struct IoError {
    pub message: String,
}

impl Error for IoError {}

impl IoError {
    pub fn new(message: String) -> Self {
        IoError { message }
    }
}

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
