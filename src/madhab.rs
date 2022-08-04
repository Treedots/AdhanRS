/// Setting for the Asr prayer time. 
/// For Hanafi madhab, the Asr is bit later 
/// than that of the Shafi madhab.

pub enum madhab{
    Shafi,
    Hanafi
}
impl madhab {
    // add code here
    pub fn shadow(&self)->i8{
        match self {
            madhab::Shafi => 1,
            madhab::Hanafi => 2,
            _ => panic!("Invalid Madhab")
        }
    }
}
