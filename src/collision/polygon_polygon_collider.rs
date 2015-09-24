use super::super::shape::shape::Shape::PolygonShape;
use super::super::body::Body;
use super::super::math::Vec2;
use super::super::manifold::Manifold;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct PolygonPolygonCollider {
    pair: (Body, Body)
}

impl Collider for PolygonPolygonCollider {
    fn new(pair: (Body, Body)) -> PolygonPolygonCollider {
        return PolygonPolygonCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let polygon_a_shape = self.pair().0.shape;
        let polygon_b_shape = self.pair().1.shape;

        match (polygon_a_shape, polygon_b_shape) {
            (PolygonShape{points: points_a}, PolygonShape{points: points_b}) => {

                let mut axes_one: Vec<Vec2> = Vec::new();
                let mut axes_two: Vec<Vec2> = Vec::new();

                for (p1, p2) in get_lines(points_a.clone(), self.pair().0.position) {
                    let edge: Vec2 = p2 - p1;
                    let normal: Vec2 = Vec2::new((-1.0 * edge.y), edge.x);
                    axes_one.push(normal.normal());
                }

                for (p1, p2) in get_lines(points_b.clone(), self.pair().1.position) {
                    let edge: Vec2 = p2 - p1;
                    let normal: Vec2 = Vec2::new((-1.0 * edge.y), edge.x);
                    axes_two.push(normal.normal());
                }

                let mut best_overlap: f32 = 9999999.0;
        		let mut mtv: Vec2 = Vec2::new(0.0,0.0);

                for axis in axes_one {
        			let min1: f32 = get_min(&points_a, axis, self.pair().0.position);
        			let max1: f32 = get_max(&points_a, axis, self.pair().0.position);
        			let min2: f32 = get_min(&points_b, axis, self.pair().1.position);
        			let max2: f32 = get_max(&points_b, axis, self.pair().1.position);
        			if min2 > min1 && min2 < max1 {
        				let mut overlap: f32 = min2 - min1;
        				if max1 - min1 < overlap {
        					overlap = max1 - min2;
        				}
        				if overlap < best_overlap {
        					best_overlap = overlap;
        					mtv = axis;
        				}
        			} else if max2 > min1 && max2 < max1 {
        				let mut overlap: f32 = max2 - min1;
        				if max1 - min2 < overlap {
        					overlap = max1 - min2;
        				}
        				if overlap < best_overlap  {
        					best_overlap = overlap;
        					mtv = axis;
        				}
        			} else {
        				return ColliderResult::new_empty_false();
        			}
        		}

                for axis in axes_two {
        			let min1: f32 = get_min(&points_a, axis, self.pair().0.position);
        			let max1: f32 = get_max(&points_a, axis, self.pair().0.position);
        			let min2: f32 = get_min(&points_b, axis, self.pair().1.position);
        			let max2: f32 = get_max(&points_b, axis, self.pair().1.position);

        			if min2 > min1 && min2 < max1 {
        			    let mut overlap: f32 = min2 - min1;
        				if max1 - min1 < overlap {
        					overlap = max1 - min2;
        				}
        				if overlap < best_overlap {
        					best_overlap = overlap;
        					mtv = axis;
        				}
        			} else if max2 > min1 && max2 < max1 {
        				let mut overlap: f32 = max2 - min1;
        				if max1 - min2 < overlap {
        					overlap = max1 - min2;
        				}
        				if overlap < best_overlap {
        					best_overlap = overlap;
        					mtv = axis;
        				}
        			}
        			else {
        				return ColliderResult::new_empty_false();
        			}
        		}

                let manifold = Manifold{body_a: self.pair().0, body_b: self.pair().1, normal: mtv, penetration:best_overlap};
                return ColliderResult::new(Some(manifold), true);

            },
            _ => {
                panic!("Something happened. Cannot test polygon to polygon collision without two polygons!!!");
            }
        }
    }
}

fn get_min(points: &Vec<Vec2>, axis: Vec2, position: Vec2) -> f32 {
    let mut min: f32 = (points[0] + position).dot(axis);
    for point in points.iter() {
        let mut new_point = point.clone() + position;
        if new_point.dot(axis) < min {
            min = new_point.dot(axis);
        }
    }
    return min;
}

fn get_max(points: &Vec<Vec2>, axis: Vec2, position: Vec2) -> f32 {
    let mut max: f32 = (points[0] + position).dot(axis);
    for point in points.iter() {
        let mut new_point = point.clone() + position;
        if new_point.dot(axis) > max {
            max = new_point.dot(axis);
        }
    }
    return max;
}

fn get_lines(points: Vec<Vec2>, position: Vec2) -> Vec<(Vec2, Vec2)> {
    let mut lines: Vec<(Vec2, Vec2)> = Vec::new();
    for index in 0..(points.len() - 1) {
	    let p1: Vec2 = points[index] + position;
	    let p2: Vec2 = points[index + 1] + position;
	    lines.push( (p1, p2) );
	    if index == points.len() - 2 {
		    let point1: Vec2 = points[index + 1] + position;
		    let point2: Vec2 = points[0] + position;
		    lines.push( (point1,point2) );
	    }
     }
     return lines;
}
