

enum Shafaq{ 
    General,
    Ahmer,
    Abyad
}

impl Shafaq {
    // add code here
    fn as_string(&self)-> String{
        match self {
            Shafaq::General => String::from("General"),
            Shafaq::Ahmer => String::from("Ahmer"),
            Shafaq::Abyad => String::from("Abyad"),
            _ => panic!("Invalid Shafaq")
        }
    }
}
