
use math;

pub fn describe_arc(x: f64, y: f64, radius: f64, start_angle: f64, end_angle:f64) -> str {
    let start: CarthesianCoord = polar_to_cartesian(x, y, radius, end_angle);
    let end: CarthesianCoord = polar_to_cartesian(x, y, radius, start_angle);

    let largeArcFlag = {
        if end_angle - start_angle <= 180 {
            return "0"
        }
        return "1"
    }
}