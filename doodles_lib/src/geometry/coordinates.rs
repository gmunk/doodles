use nannou::geom::{pt2, Point2, Rect};
use nannou::math::map_range;

pub fn convert_to_upper_left_origin(point: &Point2, plane: &Rect) -> Point2 {
    pt2(
        map_range(point.x, plane.left(), plane.right(), 0.0, plane.w()),
        map_range(point.y, plane.bottom(), plane.top(), 0.0, plane.h()),
    )
}
