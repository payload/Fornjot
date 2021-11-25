use crate::{
    geometry::{
        bounding_volume::Aabb,
        edges::{Edge, Edges},
        faces::Faces,
        Shape,
    },
    math::Point,
};

impl Shape for fj::Rectangle {
    fn bounding_volume(&self) -> Aabb {
        Aabb::from_vertices(self.vertices())
    }

    fn faces(&self, _: f64) -> Faces {
        let mut triangles = Vec::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]].into());
        triangles.push([v[0], v[2], v[3]].into());

        Faces(triangles)
    }

    fn edges(&self) -> Edges {
        let v = self.vertices();

        let mut edges = Edges::new();
        edges.0.push(Edge::line_segment(v[0], v[1]));
        edges.0.push(Edge::line_segment(v[1], v[2]));
        edges.0.push(Edge::line_segment(v[2], v[3]));
        edges.0.push(Edge::line_segment(v[3], v[0]));

        edges
    }

    fn vertices(&self) -> Vec<Point> {
        let x = self.x / 2.;
        let y = self.y / 2.;

        #[rustfmt::skip]
        let v = [
            [-x, -y, 0.0],
            [ x, -y, 0.0],
            [ x,  y, 0.0],
            [-x,  y, 0.0],
        ];

        v.map(|coord| coord.into()).to_vec()
    }
}
