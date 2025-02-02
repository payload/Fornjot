use fj_math::{Line, Point, Scalar, Vector};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Surface,
        SurfaceVertex, Vertex,
    },
    path::SurfacePath,
};

use super::Sweep;

impl Sweep for (Vertex, Surface) {
    type Swept = HalfEdge;

    fn sweep(self, path: impl Into<Vector<3>>) -> Self::Swept {
        let (vertex, surface) = self;
        let path = path.into();

        // The result of sweeping a `Vertex` is an `Edge`. Seems
        // straight-forward at first, but there are some subtleties we need to
        // understand:
        //
        // 1. To create an `Edge`, we need the `Curve` that defines it. A
        //    `Curve` is defined in a `Surface`, and we're going to need that to
        //    create the `Curve`. Which is why this `Sweep` implementation is
        //    for `(Vertex, Surface)`, and not just for `Vertex`.
        // 2. Please note that, while the output `Edge` has two vertices, our
        //    input `Vertex` is not one of them! It can't be, unless the `Curve`
        //    of the output `Edge` happens to be the same `Curve` that the input
        //    `Vertex` is defined on. That would be an edge case that probably
        //    can't result in anything valid, and we're going to ignore it for
        //    now.
        // 3. This means, we have to compute everything that defines the
        //    output `Edge`: The `Curve`, the vertices, and the `GlobalCurve`.
        //
        // Before we get to that though, let's make sure that whoever called
        // this didn't give us bad input.

        // So, we're supposed to create the `Edge` by sweeping a `Vertex` using
        // `path`. Unless `path` is identical to the path that created the
        // `Surface`, this doesn't make any sense.
        //
        // Further, the `Curve` that was swept to create the `Surface` needs to
        // be the same `Curve` that the input `Vertex` is defined on. If it's
        // not, we have no way of knowing the surface coordinates of the input
        // `Vertex` on the `Surface`, and we're going to need to do that further
        // down.
        //
        // Let's make sure that these requirements are met.
        {
            assert_eq!(vertex.curve().global_form().path(), surface.u());
            assert_eq!(path, surface.v());
        }

        // With that out of the way, let's start by creating the `GlobalEdge`,
        // as that is the most straight-forward part of this operations, and
        // we're going to need it soon anyway.
        let edge_global = vertex.global_form().sweep(path);

        // Next, let's compute the surface coordinates of the two vertices of
        // the output `Edge`, as we're going to need these for the rest of this
        // operation.
        //
        // They both share a u-coordinate, which is the t-coordinate of our
        // input `Vertex`. Remember, we validated above, that the `Curve` of the
        // `Surface` and the curve of the input `Vertex` are the same, so we can
        // do that.
        //
        // Now remember what we also validated above: That `path`, which we're
        // using to create the output `Edge`, also created the `Surface`, and
        // thereby defined its coordinate system. That makes the v-coordinates
        // straight-forward: The start of the edge is at zero, the end is at
        // one.
        let points_surface = [
            Point::from([vertex.position().t, Scalar::ZERO]),
            Point::from([vertex.position().t, Scalar::ONE]),
        ];

        // Armed with those coordinates, creating the `Curve` of the output
        // `Edge` is straight-forward.
        let curve = {
            let line = Line::from_points(points_surface);
            Curve::new(surface, SurfacePath::Line(line), *edge_global.curve())
        };

        // And now the vertices. Again, nothing wild here.
        let vertices = {
            let vertices_global = edge_global.vertices();

            // Can be cleaned up, once `zip` is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.zip
            let [a_surface, b_surface] = points_surface;
            let [a_global, b_global] = vertices_global;
            let vertices_surface =
                [(a_surface, a_global), (b_surface, b_global)].map(
                    |(point_surface, &vertex_global)| {
                        SurfaceVertex::new(
                            point_surface,
                            surface,
                            vertex_global,
                        )
                    },
                );

            // Can be cleaned up, once `zip` is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.zip
            let [a_surface, b_surface] = vertices_surface;
            let [a_global, b_global] = vertices_global;
            let vertices = [(a_surface, a_global), (b_surface, b_global)];

            vertices.map(|(vertex_surface, &vertex_global)| {
                Vertex::new(
                    [vertex_surface.position().v],
                    curve,
                    vertex_surface,
                    vertex_global,
                )
            })
        };

        // And finally, creating the output `Edge` is just a matter of
        // assembling the pieces we've already created.
        HalfEdge::new(curve, vertices, edge_global)
    }
}

impl Sweep for GlobalVertex {
    type Swept = GlobalEdge;

    fn sweep(self, path: impl Into<Vector<3>>) -> Self::Swept {
        let a = self;
        let b = GlobalVertex::from_position(self.position() + path.into());

        let curve =
            GlobalCurve::build().line_from_points([a.position(), b.position()]);

        GlobalEdge::new(curve, [a, b])
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::sweep::Sweep,
        objects::{
            Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Surface,
            Vertex,
        },
    };

    #[test]
    fn vertex_surface() {
        let surface = Surface::xz_plane();
        let curve = Curve::build(surface).u_axis();
        let vertex = Vertex::build(curve).from_point([0.]);

        let half_edge = (vertex, surface).sweep([0., 0., 1.]);

        let expected_half_edge = HalfEdge::build(surface)
            .line_segment_from_points([[0., 0.], [0., 1.]]);
        assert_eq!(half_edge, expected_half_edge);
    }

    #[test]
    fn global_vertex() {
        let edge =
            GlobalVertex::from_position([0., 0., 0.]).sweep([0., 0., 1.]);

        let expected_edge = GlobalEdge::new(
            GlobalCurve::build().z_axis(),
            [[0., 0., 0.], [0., 0., 1.]].map(GlobalVertex::from_position),
        );
        assert_eq!(edge, expected_edge);
    }
}
