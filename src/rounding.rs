
pub enum Rounding{
    Nearest,
    Up,
    None
}
impl Rounding {
    pub fn as_string(&self)-> String{
        match self {
            Rounding::Nearest => String::from("Nearest"),
            Rounding::Up => String::from("Up"),
            _ => String::from("None")
        }
    }
} 
