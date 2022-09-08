use fj_interop::mesh::Color;
use fj_math::{Line, Scalar};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    objects::{
        Curve, CurveKind, Cycle, Edge, Face, GlobalEdge, SurfaceVertex, Vertex,
    },
};

use super::{Path, Sweep};

impl Sweep for (Edge, Color) {
    type Swept = Face;

    fn sweep(self, path: impl Into<Path>) -> Self::Swept {
        let (edge, color) = self;
        let path = path.into();

        let edge = if path.is_negative_direction() {
            edge.reverse_including_curve()
        } else {
            edge
        };

        let surface = edge.curve().sweep(path);

        // We can't use the edge we're sweeping from as the bottom edge, as that
        // is not defined in the right surface. Let's create a new bottom edge,
        // by swapping the surface of the original.
        let bottom_edge = {
            let vertices = edge.vertices();

            let points_curve_and_surface = vertices.map(|vertex| {
                (vertex.position(), [vertex.position().t, Scalar::ZERO])
            });

            let curve = {
                // Please note that creating a line here is correct, even if the
                // global curve is a circle. Projected into the side surface, it is
                // going to be a line either way.
                let kind = CurveKind::Line(Line::from_points_with_line_coords(
                    points_curve_and_surface,
                ));

                Curve::new(surface, kind, *edge.curve().global_form())
            };

            let vertices = {
                let points_surface = points_curve_and_surface
                    .map(|(_, point_surface)| point_surface);

                // Can be cleaned up, once `zip` is stable:
                // https://doc.rust-lang.org/std/primitive.array.html#method.zip
                let [a_vertex, b_vertex] = vertices;
                let [a_surface, b_surface] = points_surface;
                let vertices_with_surface_points =
                    [(a_vertex, a_surface), (b_vertex, b_surface)];

                vertices_with_surface_points.map(|(vertex, point_surface)| {
                    let surface_vertex = SurfaceVertex::new(
                        point_surface,
                        surface,
                        *vertex.global_form(),
                    );

                    Vertex::new(
                        vertex.position(),
                        curve,
                        surface_vertex,
                        *vertex.global_form(),
                    )
                })
            };

            Edge::new(curve, vertices, *edge.global_form())
        };

        let side_edges = bottom_edge
            .vertices()
            .map(|vertex| (vertex, surface).sweep(path));

        let top_edge = {
            let bottom_vertices = bottom_edge.vertices();

            let global_vertices = side_edges.map(|edge| {
                let [_, vertex] = edge.vertices();
                *vertex.global_form()
            });

            let points_curve_and_surface = bottom_vertices.map(|vertex| {
                (vertex.position(), [vertex.position().t, Scalar::ONE])
            });

            let curve = {
                let global =
                    bottom_edge.curve().global_form().translate(path.inner());

                // Please note that creating a line here is correct, even if the
                // global curve is a circle. Projected into the side surface, it
                // is going to be a line either way.
                let kind = CurveKind::Line(Line::from_points_with_line_coords(
                    points_curve_and_surface,
                ));

                Curve::new(surface, kind, global)
            };

            let global = GlobalEdge::new(*curve.global_form(), global_vertices);

            let vertices = {
                let surface_points = points_curve_and_surface
                    .map(|(_, point_surface)| point_surface);

                // Can be cleaned up, once `zip` is stable:
                // https://doc.rust-lang.org/std/primitive.array.html#method.zip
                let [a_vertex, b_vertex] = bottom_vertices;
                let [a_surface, b_surface] = surface_points;
                let [a_global, b_global] = global_vertices;
                let vertices = [
                    (a_vertex, a_surface, a_global),
                    (b_vertex, b_surface, b_global),
                ];

                vertices.map(|(vertex, point_surface, vertex_global)| {
                    let vertex_surface = SurfaceVertex::new(
                        point_surface,
                        surface,
                        vertex_global,
                    );
                    Vertex::new(
                        vertex.position(),
                        curve,
                        vertex_surface,
                        vertex_global,
                    )
                })
            };

            Edge::new(curve, vertices, global)
        };

        let cycle = {
            let a = bottom_edge;
            let [d, b] = side_edges;
            let c = top_edge;

            let mut edges = [a, b, c, d];

            // Make sure that edges are oriented correctly.
            let mut i = 0;
            while i < edges.len() {
                let j = (i + 1) % edges.len();

                let [_, prev_last] = edges[i].vertices();
                let [next_first, _] = edges[j].vertices();

                // Need to compare surface forms here, as the global forms might
                // be coincident when sweeping circles, despite the vertices
                // being different!
                if prev_last.surface_form() != next_first.surface_form() {
                    edges[j] = edges[j].reverse();
                }

                i += 1;
            }

            Cycle::new(surface, edges)
        };

        Face::new(surface, cycle).with_color(color)
    }
}
