use nalgebra::Point2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex(pub Point2<f32>);

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Point2::new(x, y))
    }

    pub fn is_upper(&self, other: &Vertex) -> bool {
        // Higher-ness is primarily determined by y coordinate.
        if self.0.y > other.0.y {
            return true;
        }
        if self.0.y < other.0.y {
            return false;
        }

        // If y coordinates are equal, the left vertex is higher.
        if self.0.y == other.0.y {
            if self.0.x < other.0.x {
                return true;
            }
            if self.0.x > other.0.x {
                return false;
            }
        }

        // If we land here, the vertices are either equal, or we have NaN's or
        // some other weirdness.
        false
    }

    pub fn is_lower(&self, other: &Vertex) -> bool {
        other.is_upper(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Vertex;

    #[test]
    fn vertex_with_high_y_should_be_higher_than_vertex_with_low_y() {
        let upper = Vertex::new(0.0, 1.0);
        let lower = Vertex::new(0.0, 0.0);

        assert!(upper.is_upper(&lower));
        assert!(lower.is_lower(&upper));
    }

    #[test]
    fn vertex_with_equal_y_but_left_x_should_be_higher_than_right_x() {
        let upper = Vertex::new(0.0, 0.0);
        let lower = Vertex::new(1.0, 0.0);

        assert!(upper.is_upper(&lower));
        assert!(lower.is_lower(&upper));
    }

    #[test]
    fn vertex_should_not_be_higher_or_lower_than_equal_vertex() {
        let vertex = Vertex::new(0.0, 0.0);

        assert_eq!(vertex.is_upper(&vertex), false);
        assert_eq!(vertex.is_lower(&vertex), false);
    }
}
