use rust_bootcamp_part1::*;

#[test]
fn furthest_generic() {
    let pts = [
        Point { x: 1.0, y: 1.0 },
        Point { x: -5.0, y: 0.0 },
        Point { x: 2.0, y: 2.0 },
    ];
    let res = furthest_from_origin(&pts).unwrap();
    assert_eq!(*res, Point { x: -5.0, y: 0.0 });

    let tuples = [(0.0, 1.0), (3.0, 4.0), (2.0, 0.0)];
    let res2 = furthest_from_origin(&tuples).unwrap();
    assert_eq!(*res2, (3.0, 4.0));
}

#[test]
fn min_by_key_works() {
    let nums = [10, 3, 7, 5];
    // find minimum by remainder when dividing by 4 -> minimal remainder is 0 at 4k (none), check normal min
    let res = min_by_key(&nums, |x| *x);
    assert_eq!(*res.unwrap(), 3);

    let pts = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: -2.0, y: 0.0 },
    ];
    let m = min_by_key(&pts, |p| (p.x * p.x + p.y * p.y) as i64);
    assert_eq!(*m.unwrap(), Point { x: 0.0, y: 0.0 });
}