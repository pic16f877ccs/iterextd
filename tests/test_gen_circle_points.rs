use approx::assert_relative_eq;
use iterextd::GenCirclePoints;

#[test]
fn test_gen_circle_points_debug() {
    let iter = GenCirclePoints::new(1.0, 0);
    assert_eq!(
        format!("{:?}", iter),
        "GenCirclePoints { radius: 1.0, num_points: 0, \
            base_cos: 1.0, base_sin: 0.0, incr_cos: NaN, incr_sin: NaN }"
    );
}

#[test]
fn test_gen_circle_points_len() {
    let mut iter = GenCirclePoints::new(1.0, 6);
    assert_eq!(iter.size_hint(), (6, Some(6)));
    assert_eq!(iter.len(), 6);
    iter.next();
    assert_eq!(iter.size_hint(), (5, Some(5)));
    assert_eq!(iter.len(), 5);
    iter.nth(4);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.len(), 0);
    iter.next();
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.len(), 0);
}

#[test]
fn test_gen_circle_points_point() {
    let iter = GenCirclePoints::new(1.0, 1);
    let iter_clone = iter.clone();
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.len(), 1);
    let vec = iter.collect::<Vec<_>>();
    assert_relative_eq!(vec[0].0, 1.0e0, epsilon = 1.0e-16);
    assert_relative_eq!(vec[0].1, 0.0e0, epsilon = 1.0e-16);
    let vec_clone = iter_clone.collect::<Vec<_>>();
    assert_relative_eq!(vec_clone[0].0, 1.0e0, epsilon = 1.0e-16);
    assert_relative_eq!(vec_clone[0].1, 0.0e0, epsilon = 1.0e-16);
}

#[test]
fn test_gen_circle_points_three_points() {
    let iter = GenCirclePoints::new(1.0, 3);
    let vec = iter.collect::<Vec<_>>();
    let cos_0_deg = 1.0e0_f64;
    let sin_0_deg = 0.0e0_f64;
    let cos_120_deg = -0.5e0_f64;
    let sin_120_deg = 0.8660254037844386_f64;
    let cos_240_deg = -0.5e0_f64;
    let sin_240_deg = -0.8660254037844386_f64;
    assert_relative_eq!(vec[0].0, cos_0_deg, epsilon = 1.0e-16);
    assert_relative_eq!(vec[0].1, sin_0_deg, epsilon = 1.0e-16);
    assert_relative_eq!(vec[1].0, cos_120_deg, epsilon = 4.0e-16);
    assert_relative_eq!(vec[1].1, sin_120_deg, epsilon = 4.0e-16);
    assert_relative_eq!(vec[2].0, cos_240_deg, epsilon = 4.0e-16);
    assert_relative_eq!(vec[2].1, sin_240_deg, epsilon = 4.0e-16);
}

#[test]
fn test_gen_circle_points_points() {
    let num_points = 739;
    let mut iter = GenCirclePoints::new(1.0, num_points);
    let (cos, sin) = iter.nth(num_points - 1).unwrap();
    let rhs_cos = 0.9999638558306818_f64;
    assert_relative_eq!(cos, rhs_cos, epsilon = 9.0e-16);
    let rhs_sin = -0.00850217808772121_f64;
    assert_relative_eq!(sin, rhs_sin, epsilon = 9.0e-16);
}
