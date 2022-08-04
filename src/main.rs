mod prayer;
mod math_utils;
use chrono::prelude::*;
use prayer::*;

use math_utils::*;

//Todo: SolarTime
struct PrayerTimes{
    fajr: DateTime<Utc>,
    sunrise: DateTime<Utc>,
    dhuhr: DateTime<Utc>,
    asr: DateTime<Utc>,
    magrib: DateTime<Utc>,
    isha: DateTime<Utc>
}
impl PrayerTimes {

    // add code here
}
fn main() {
    println!("Hello, world!");
    println!("{}",normalize_scale(100.0, 360.0)); 
    println!("{}",prayer_string(Prayer::Magrib));
}
