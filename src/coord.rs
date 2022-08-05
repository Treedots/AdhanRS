#[derive(Debug)]
struct Coordinates {
    latitude: i32,
    longitude: i32
}
impl Coordinates{
    pub fn new(&self,lat:i32,long:i32){
        self.latitude = lat;
        self.longitude = long;
    }
}
