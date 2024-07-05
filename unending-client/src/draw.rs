use macroquad::prelude::*;

pub fn ground() {
    draw_plane(vec3(0., 0., 0.), vec2(10., 10.), None, BROWN);
}

pub fn person(bottom_center: Vec3, color: Color) {
    let parts: Vec<(Vec3, Vec3)> = vec![
        (vec3(-1., 0., 0.), vec3(1., 2., 1.)), // left leg
        (vec3(1., 0., 0.), vec3(1., 2., 1.)), // right leg
        (vec3(0., 2., 0.), vec3(1., 2., 1.)), // body
        (vec3(-1., 2., 0.), vec3(2., 1., 1.)), // left arm
        (vec3(1., 2., 0.), vec3(2., 1., 1.)), // right arm
        (vec3(0., 4., 0.), vec3(2., 2., 1.)), // head
    ];
    for part in parts {
        person_part(part.0, part.1, bottom_center, color);
    }
}

fn person_part(pos: Vec3, size: Vec3, bottom_center: Vec3, color: Color) {
    draw_cube(
        bottom_center + pos,
        size,
        None,
        color,
    );
}

pub fn exclamation_point(bottom_center: Vec3) {
    draw_sphere(
        bottom_center + vec3(0., 1., 0.),
        0.5,
        None,
        YELLOW,
    );
    draw_cube(
        bottom_center + vec3(0., 3.5, 0.),
        vec3(0.75, 2., 0.75),
        None,
        YELLOW,
    )
}

pub fn quest_giver(color: Color) {
    person(vec3(0., 0., 0.), color);
    exclamation_point(vec3(0., 6., 0.));
}