use rust_bootcamp_part1::*;

#[test]
fn structs_enums_methods() {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 3.0, y: 4.0 };
    assert!((a.distance_to(&b) - 5.0).abs() < 1e-9);

    let c = Shape::Circle {
        center: Point::origin(),
        radius: 2.0,
    };
    let r = Shape::Rect {
        top_left: Point::origin(),
        w: 3.0,
        h: 4.0,
    };

    let pi = std::f64::consts::PI;
    assert!((c.area() - (pi * 4.0)).abs() < 1e-9);
    assert!((r.area() - 12.0).abs() < 1e-9);

    //Triangle
    let t = Shape::Triangle {
        a: Point { x: 0.0, y: 0.0 },
        b: Point { x: 3.0, y: 0.0 },
        c: Point { x: 0.0, y: 4.0 },
    };
    assert!((t.area() - 6.0).abs() < 1e-9);

    //Display
    let p = Point { x: 1.5, y: -2.0 };
    let s = format!("{}", p);
    assert!(s.contains("1.5") && s.contains("-2"));
}
