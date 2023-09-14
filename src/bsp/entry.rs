use super::aabb::Aabb;

#[derive(Clone, Copy, Default)]
pub struct Entry {
    pub id: usize,
    pub aabb: Aabb,
}