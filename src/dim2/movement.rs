use super::vec;

/// calculates the arc movement destination point from a given starting point
/// and the distance traveled around the arc.  
/// positive arc_distance moves counter clockwise, negative arc_distance moves clockwise.
/// # Examples
/// ```
/// // calculate the next position of a moving object 'obj' around a point at x = 100, y = 100
/// let arc_distance = obj.speed * time.delta;
/// (obj.x, obj.y) = around_arc(obj.x, obj.y, 100., 100., arc_distance);
/// ```
pub fn around_arc(start_x: f32, start_y: f32, arc_center_x: f32, arc_center_y: f32, arc_distance: f32) -> (f32, f32) {
    // atan2 is not defined for this case and even if the program is allowed
    // to continue here, the outcome would be at best non-sensical
    if start_x == arc_center_x && start_y == arc_center_y {
        return (start_x, start_y);
    }

    let distance_x = start_x - arc_center_x;
    let distance_y = start_y - arc_center_y;

    let radius = vec::len(distance_x, distance_y);

    let starting_angle = f32::atan2(distance_y, distance_x);

    let angle_delta = arc_distance / radius;
    let destination_angle = starting_angle + angle_delta;

    let destination_x = arc_center_x + radius * f32::cos(destination_angle);
    let destination_y = arc_center_y + radius * f32::sin(destination_angle);
    
    (destination_x, destination_y)
}
