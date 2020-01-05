use std::fmt::{Display, Formatter, Error};

pub trait TeamSpeakClient {
    fn connect(&mut self,server: &str,server_pw: &str,channel: &str,channel_pw: &str) -> Result<(),ClientError>;
    fn disconnect(&mut self) -> Result<(),ClientError>;
}


#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Debug,Hash)]
pub enum ClientError {

}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f,"{}",self)
    }
}
impl std::error::Error for ClientError {}