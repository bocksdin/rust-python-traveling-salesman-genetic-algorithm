#[derive(Debug, Copy, Clone)]
pub struct Path {
    pub origin: i8,
    pub destination: i8,
    pub distance: i32,
    pub avg_speed: i8,
    pub travel_time: f32
}