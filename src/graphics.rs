use super::*;

use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle, Triangle};

pub struct ProjectionData {
    pub fov_rad: f32,
    pub near: f32,
    pub far: f32,
}

pub struct RenderPass<'triangles> {
    pub camera_front: Vec3,
    pub camera_position: Vec3,
    pub triangles: &'triangles [f32],
    pub model: Mat4,
    pub color: Color,
    pub border_color: Color,
    pub invert_culling: bool,
    pub enable_depth: bool,
    pub projection: Option<ProjectionData>,
}

pub struct Framebuffer {
    width: u16,
    height: u16,
}

impl Framebuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn clear_color<T, E>(&self, display: &mut T, color: Color)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(color.into())
            .build();
        let Ok(_) = Rectangle::new(
            Point { x: 0, y: 0 },
            Size {
                width: self.width as u32,
                height: self.height as u32,
            },
        )
        .into_styled(style)
        .draw(display) else {
            panic!("Failed to draw.");
        };
    }

    pub fn clear_depth(&mut self, value: f32) {
        // self.fb_depth.iter_mut().for_each(|v| *v = value)
    }

    pub fn render_pass<T, E>(&self, display: &mut T, pass: &RenderPass)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        pass.triangles.windows(9).for_each(|triangles| {
            let mut vertices = triangles.chunks(3);
            let vertex_a: Vec3 = vertices.next().unwrap().try_into().unwrap();
            let vertex_b: Vec3 = vertices.next().unwrap().try_into().unwrap();
            let vertex_c: Vec3 = vertices.next().unwrap().try_into().unwrap();
            let world_vertex_a =
                vec4_into_vec3(mat4_mul_vec4(pass.model, vec3_into_vec4(vertex_a)));
            let world_vertex_b =
                vec4_into_vec3(mat4_mul_vec4(pass.model, vec3_into_vec4(vertex_b)));
            let world_vertex_c =
                vec4_into_vec3(mat4_mul_vec4(pass.model, vec3_into_vec4(vertex_c)));
            let normal = vec_normalize(vec3_cross_product(
                vec_sub_vec(world_vertex_b, world_vertex_a),
                vec_sub_vec(world_vertex_c, world_vertex_a),
            ));

            if vec_dot(normal, vec_sub_vec(world_vertex_a, pass.camera_position))
                * if pass.invert_culling { -1f32 } else { 1f32 }
                < 0.0
            {
                let view = mat4_get_look_at(
                    pass.camera_position,
                    vec_add_vec(pass.camera_position, pass.camera_front),
                    [0.0, 1.0, 0.0],
                );
                let view_vertex_a =
                    vec4_into_vec3(mat4_mul_vec4(view, vec3_into_vec4(world_vertex_a)));
                let view_vertex_b =
                    vec4_into_vec3(mat4_mul_vec4(view, vec3_into_vec4(world_vertex_b)));
                let view_vertex_c =
                    vec4_into_vec3(mat4_mul_vec4(view, vec3_into_vec4(world_vertex_c)));

                let clipped_triangles = if let Some(projection) = &pass.projection {
                    triangle_clip_plane(
                        [0.0, 0.0, projection.near],
                        [0.0, 0.0, 1.0],
                        (view_vertex_a, view_vertex_b, view_vertex_c),
                    )
                } else {
                    smallvec![(view_vertex_a, view_vertex_b, view_vertex_c)]
                };

                clipped_triangles.iter().for_each(|triangle| {
                    let (vertex_a, vertex_b, vertex_c) = if let Some(projection) = &pass.projection
                    {
                        let projection = mat4_get_projection(
                            1.0,
                            projection.fov_rad,
                            projection.near,
                            projection.far,
                        );
                        let projected_vertex_a =
                            mat4_mul_vec4(projection, vec3_into_vec4(triangle.0));
                        let projected_vertex_b =
                            mat4_mul_vec4(projection, vec3_into_vec4(triangle.1));
                        let projected_vertex_c =
                            mat4_mul_vec4(projection, vec3_into_vec4(triangle.2));

                        (
                            vec4_into_vec3(vec4_scale_with_w(projected_vertex_a)),
                            vec4_into_vec3(vec4_scale_with_w(projected_vertex_b)),
                            vec4_into_vec3(vec4_scale_with_w(projected_vertex_c)),
                        )
                    } else {
                        (vertex_a, vertex_b, vertex_c)
                    };

                    let mut vertex_a = vec_add_scalar(vertex_a, 1.0);
                    let mut vertex_b = vec_add_scalar(vertex_b, 1.0);
                    let mut vertex_c = vec_add_scalar(vertex_c, 1.0);
                    vertex_a[0] *= self.width as f32 * 0.5;
                    vertex_b[0] *= self.width as f32 * 0.5;
                    vertex_c[0] *= self.width as f32 * 0.5;

                    vertex_a[1] *= self.height as f32 * 0.5;
                    vertex_b[1] *= self.height as f32 * 0.5;
                    vertex_c[1] *= self.height as f32 * 0.5;

                    let test_planes = [
                        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
                        ([0.0, self.height as f32 - 1.0, 0.0], [0.0, -1.0, 0.0]),
                        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
                        ([self.width as f32 - 1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]),
                    ];
                    let mut final_triangles: SmallVec<[(Vec3, Vec3, Vec3); 2]> =
                        smallvec![(vertex_a, vertex_b, vertex_c)];

                    for plane in &test_planes {
                        let mut passed: SmallVec<[(Vec3, Vec3, Vec3); 2]> = smallvec![];
                        for t in final_triangles {
                            passed.append(&mut triangle_clip_plane(plane.0, plane.1, t));
                        }
                        final_triangles = passed;
                    }

                    final_triangles.iter().for_each(|(a, b, c)| {
                        let style = PrimitiveStyleBuilder::new()
                            .stroke_color(Rgb565::from(pass.border_color))
                            .stroke_width(1)
                            .fill_color(Rgb565::from(pass.color))
                            .build();
                        let Ok(_) = Triangle::new(
                            Point {
                                x: a[0] as i32,
                                y: a[1] as i32,
                            },
                            Point {
                                x: b[0] as i32,
                                y: b[1] as i32,
                            },
                            Point {
                                x: c[0] as i32,
                                y: c[1] as i32,
                            },
                        )
                        .into_styled(style)
                        .draw(display) else {
                            panic!("Failed to draw.");
                        };
                    });
                });
            }
        });
    }
}
