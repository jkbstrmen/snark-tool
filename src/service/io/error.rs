trait Error {}

#[derive(Debug)]
pub struct IoError {}

impl Error for IoError {}

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
