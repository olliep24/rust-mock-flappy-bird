use crate::game::vector2::Vector2;

pub struct CollisionBox {
    pub min: Vector2,
    pub max: Vector2,
}

impl CollisionBox {
    pub fn new(min: Vector2, max: Vector2) -> CollisionBox {
        CollisionBox { min, max }
    }

    pub fn collides_with(&self, other: &CollisionBox) -> bool {
        self.max.x > other.min.x &&
        self.min.x < other.max.x &&
        self.max.y > other.min.y &&
        self.min.y < other.max.y
    }
}