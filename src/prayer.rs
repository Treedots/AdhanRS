
#[allow(dead_code)]
pub enum Prayer {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Magrib,
    Isha,
    None
}

pub fn prayer_string(p:Prayer)-> String{
    match p {
        Prayer::Fajr =>  String::from("Fajr"),
        Prayer::Sunrise =>  String::from("Sunrise"),
        Prayer::Dhuhr =>  String::from("Dhuhr"),
        Prayer::Asr =>  String::from("Asr"),
        Prayer::Magrib =>  String::from("Margib"),
        Prayer::Isha =>  String::from("Isha"),
        _ =>  String::from("None"),
    }
}


