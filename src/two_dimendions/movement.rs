use super::vec;

/// positive speed moves counter clockwise, negative speed clockwise
pub fn around_arc(pos_x: f32, pos_y: f32, arc_center_x: f32, arc_center_y: f32, speed: f32) -> (f32, f32) {
    // atan2 is not defined for this case and even if the program is allowed
    // to continue here, the outcome would be at best non-sensical
    if pos_x == arc_center_x && pos_y == arc_center_y {
        return (pos_x, pos_y);
    }

    let distance_x = pos_x - arc_center_x;
    let distance_y = pos_y - arc_center_y;

    let radius = vec::len(distance_x, distance_y);

    let current_angle = f32::atan2(distance_y, distance_x);

    let angle_delta = speed / radius;
    let angle = current_angle + angle_delta;

    (arc_center_x + radius * f32::cos(angle), arc_center_y + radius * f32::sin(angle))
}
