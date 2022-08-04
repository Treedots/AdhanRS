use std::f64::consts::PI;
pub fn degrees_radians(degrees: f64) -> f64 {
  (degrees * PI) / 180.0
}

pub fn radians_degrees(radians: f64) -> f64 {
  (radians * 180.0) / PI
}

pub fn normalize_scale(num: f32, max: f32) -> f32 {
  num - max * (num / max).floor()
}

pub fn unwind_angle(angle: f32) -> f32{
  normalize_scale(angle, 360.0)
}

pub fn quadrant_shift_angle(angle: f32) -> f32{
  if angle >= -180.0 && angle <= 180.0 {
    angle
  }
  else{
    angle - 360.0 * (angle / 360.0).round()
  }
}
