use std::f64::consts::PI;

pub fn degrees_to_radians(degrees:f32)-> f32{
    (degrees * PI)/180.0
}

pub fn radians_to_degrees(radian:f32)->f32{
    (radians * 180.0) / PI
}

pub fn normalize_to_scale(num:f32,max:f32){
    num - max * (num/max).floor()
}
 
