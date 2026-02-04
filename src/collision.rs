use macroquad::prelude::*;
use macroquad::math::Circle;

// Debug draw helpers
pub fn draw_hitbox(hitbox: &Hitbox, color: Color) {
    match hitbox {
        Hitbox::OBB(obb) => draw_obb(obb, color),
        Hitbox::Circle(c) => draw_circle_hitbox(c, color),
    }
}

fn draw_obb(obb: &OBB, color: Color) {
    let corners = obb.corners();
    let thickness = 2.0;
    for i in 0..4 {
        let a = corners[i];
        let b = corners[(i + 1) % 4];
        draw_line(a.x, a.y, b.x, b.y, thickness, color);
    }
}

fn draw_circle_hitbox(c: &Circle, color: Color) {
    let thickness = 2.0;
    draw_circle_lines(c.x, c.y, c.r, thickness, color);
}

pub enum Hitbox {
    OBB(OBB),
    Circle(Circle),
}

pub fn hitbox_intersects(a: &Hitbox, b: &Hitbox) -> bool {
    match (a, b) {
        (Hitbox::OBB(a), Hitbox::OBB(b)) => obb_intersects_obb(a, b),
        (Hitbox::OBB(a), Hitbox::Circle(b)) => obb_intersects_circle(a, b),
        (Hitbox::Circle(a), Hitbox::OBB(b)) => obb_intersects_circle(b, a),
        (Hitbox::Circle(a), Hitbox::Circle(b)) => circle_intersects_circle(a, b),
    }
}

fn circle_intersects_circle(a: &Circle, b: &Circle) -> bool {
    let pos_a = Vec2{x: a.x, y: a.y};
    let pos_b = Vec2{x: b.x, y: b.y};
    if (pos_a - pos_b).length() < a.r {
        return false;
    }
    true
}

fn obb_intersects_obb(a: &OBB, b: &OBB) -> bool {
    let ca = a.corners();
    let cb = b.corners();
    
    let axes_a = obb_axes(&ca);
    let axes_b = obb_axes(&cb);
    
    for axis in axes_a.iter().chain(axes_b.iter()) {
        let pa = project_points_on_axis(&ca, *axis);
        let pb = project_points_on_axis(&cb, *axis);
        
        if !intervals_overlap(pa, pb) {
            return false;
        }
    }
    true
}

fn obb_intersects_circle(obb: &OBB, circle: &Circle) -> bool {
    let corners = obb.corners();
    let axes = obb_axes(&corners);
    
    // 1) Axes de l'OBB
    for axis in axes {
        if !obb_circle_overlap_on_axis(&corners, circle, axis) {
            return false;
        }
    }
    
    let center = Vec2 { x: circle.x, y: circle.y };
    // 2) Axe supplémentaire : du point le plus proche vers le centre du cercle
    let closest = closest_point_on_obb(obb, center);
    let dir = center - closest;
    
    // Si le centre est exactement sur l'OBB
    if dir.length_squared() > 1e-6 {
        let axis = dir.normalize();
        if !obb_circle_overlap_on_axis(&corners, circle, axis) {
            return false;
        }
    }
    true
}

fn obb_circle_overlap_on_axis(corners: &[Vec2; 4], circle: &Circle, axis: Vec2) -> bool {
    let axis = axis.normalize();
    
    let (min_r, max_r) = project_points_on_axis(corners, axis);
    
    let center = Vec2{x: circle.x, y: circle.y};
    
    let c = center.dot(axis);
    let min_c = c - circle.r;
    let max_c = c + circle.r;
    
    intervals_overlap((min_r, max_r), (min_c, max_c))
}

fn closest_point_on_obb(obb: &OBB, point: Vec2) -> Vec2 {
    let rot = Mat2::from_angle(obb.rotation);
    let inv = rot.transpose(); // inverse pour une rotation

    // point dans l'espace local de l'OBB
    let local = inv * (point - obb.center);

    let clamped = vec2(
        local.x.clamp(-obb.half.x, obb.half.x),
        local.y.clamp(-obb.half.y, obb.half.y),
    );

    // retour en espace monde
    obb.center + rot * clamped
}

// Oriented Bounding Box
pub struct OBB {
    pub center: Vec2,
    pub half: Vec2,
    pub rotation: f32,
}

impl OBB {
    pub fn corners(&self) -> [Vec2; 4] {
        let hx = self.half.x;
        let hy = self.half.y;
        
        let local = [
        vec2(-hx, -hy),
        vec2( hx, -hy),
        vec2( hx,  hy),
        vec2(-hx,  hy),
        ];
        
        let rot = Mat2::from_angle(self.rotation);
        
        [
        self.center + rot * local[0],
        self.center + rot * local[1],
        self.center + rot * local[2],
        self.center + rot * local[3],
        ]
    }
}

fn project_points_on_axis(points: &[Vec2; 4], axis: Vec2) -> (f32, f32) {
    let mut min = points[0].dot(axis);
    let mut max = min;
    
    for p in &points[1..] {
        let d = p.dot(axis);
        min = min.min(d);
        max = max.max(d);
    }
    
    (min, max)
}

fn intervals_overlap(a: (f32, f32), b: (f32, f32)) -> bool {
    a.1 >= b.0 && b.1 >= a.0
}

fn obb_axes(corners: &[Vec2; 4]) -> [Vec2; 2] {
    let e0 = corners[1] - corners[0];
    let e1 = corners[3] - corners[0];
    
    [
    vec2(-e0.y, e0.x).normalize(),
    vec2(-e1.y, e1.x).normalize(),
    ]
}
