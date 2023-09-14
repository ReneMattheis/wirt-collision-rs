#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle { radius: f64 },
    Square { edge_length: f64 },
}
