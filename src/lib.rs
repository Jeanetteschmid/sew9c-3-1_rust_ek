// src/lib.rs

use std::fmt;

// ---------- 1. BASICS ----------
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn flip(b: bool) -> bool {
    !b
}

// ---------- 2. OWNERSHIP & BORROWING ----------
pub fn take_ownership(s: String) -> usize {
    s.chars().count()
}

pub fn borrow_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn push_exclamation(s: &mut String) {
    s.push('!')
}

// ---------- 3. STRUCTS, ENUMS, METHODS ----------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Simple textual representation: (x, y)
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { center: Point, radius: f64 },
    Rect { top_left: Point, w: f64, h: f64 },
    Triangle { a: Point, b: Point, c: Point },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match *self {
            Shape::Circle { center: _, radius } => std::f64::consts::PI * radius * radius,
            Shape::Rect { top_left: _, w, h } => w * h,
            Shape::Triangle { a, b, c } => {
                let ax = a.x;
                let ay = a.y;
                let bx = b.x;
                let by = b.y;
                let cx = c.x;
                let cy = c.y;
                ((ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)).abs()) / 2.0
            }
        }
    }
}

// ---------- 4. TRAITS & GENERICS ----------
pub trait Plottable {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl Plottable for Point {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}

impl Plottable for (f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}

// Return a reference to the item farthest from the origin.
// Note the explicit lifetime tying the returned reference to the input slice.
pub fn furthest_from_origin<T: Plottable>(items: &[T]) -> Option<&T> {
    items.iter().max_by(|a, b| {
        let da = a.x() * a.x() + a.y() * a.y();
        let db = b.x() * b.x() + b.y() * b.y();
        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
    })
}

pub fn min_by_key<T, K: Ord, F: Fn(&T) -> K>(items: &[T], f: F) -> Option<&T> {
    items.iter().min_by_key(|x| f(x))
}

// ---------- 5. ERRORS & OPTION/RESULT ----------
pub fn parse_port(s: &str) -> Result<u16, String> {
    s.parse::<u16>()
        .map_err(|e| format!("failed to parse port: {}", e))
}

// ---------- 6. ITERATORS & CLOSURES ----------
pub fn even_squares(n: u32) -> Vec<u32> {
    (0..=n).filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

// ---------- 7. USING A CRATE (rand) ----------
pub fn roll_dice(sides: u8) -> u8 {
    use rand::Rng;
    rand::thread_rng().gen_range(1..=sides)
}


// Debug: ermoeglicht {:?}-Ausgabe zum Debuggen
// Clone: erlaubt explizites Kopieren (.clone()) eines Wertes
// Copy: erlaubt, dass der Typ bitweise kopiert werden kann (impliziert auch Clone)
// - Kopien passieren implizit bei Zuweisungen/Aufrufen. Nur fuer einfache,
// sicher kopierbare Typen
// PartialEq: erlaubt ==/!= Vergleiche zwischen Instanzen

// Ownership / Borrowing:
//  Falls eine Funktion nichts besitzen muss, nimm Referenzen (&T oder &str)
//  Falls du etwas mutieren willst, nimm &mut T
//  Rueckgabewerte, die laenger leben sollen als die Eingaben, muessen den Besitz uebertragen (z.B. String)