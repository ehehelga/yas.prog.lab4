use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Point {
    x: f64,
    y: f64,
}

struct Set {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}


fn in_segment(a: Point, b: Point, c: Point) -> bool {
    let case1 = (c.x <= a.x) & (c.y <= a.y) & (c.x >= b.x) & (c.y >= b.y);
    let case2 = (c.x >= a.x) & (c.y >= a.y) & (c.x <= b.x) & (c.y <= b.y);
    if case1 | case2 {
        return true;
    };
    return false;
}

fn between_points(a: Point, b: Point) -> f64{
    let exp: f64 = (a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0);
    return exp.sqrt();
}

fn get_distance(ray: &Set, set: &Set) -> f64{
    let x0 = ray.x1; 
    let y0 = ray.y1;
    let v = ray.x2 - x0;
    let w = ray.y2 - y0;

    let a = set.y2 - set.y1;
    let b = set.x1 - set.x2;
    let c = -1.0 * set.x1 * set.y2 + set.y1 * set.x2;

    if (a * v + b * w) == 0.0 {
        return -1.0;
    } else {
        let t = (-1.0 * a * x0 - b * y0 - c) / (a * v + b * w);
        if t < 0.0 {
            return -2.0;
        }
        let x = x0 + v * t;
        let y = y0 + w * t;
        if in_segment(Point{x: set.x1, y: set.y1}, Point{x: set.x2, y: set.y2}, Point{x, y}) {
            return between_points(Point{x: x0, y: y0}, Point{x, y});
        }
        return 1.0;
    }
}


fn get_set(line: String) -> Set{
    let dots: Vec<&str> = line.split(' ').collect();
    let coordinates1: Vec<&str> = dots[0].split(',').collect();
    let coordinates2: Vec<&str> = dots[1].split(',').collect();

    let set = Set{
        x1: coordinates1[0].parse().unwrap(),
        y1: coordinates1[1].parse().unwrap(),
        x2: coordinates2[0].parse().unwrap(),
        y2: coordinates2[1].parse().unwrap(),
    };
    set
}

fn print_set(set: &Set) {
    println!("{}, {}, {}, {}", set.x1, set.y1, set.x2, set.y2);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Couldn't parse the line!"))
        .collect()
}

fn main() {
    let lines = lines_from_file("./InputFile");
    let mut vec_of_sets= Vec::new();
    for line in lines {
        vec_of_sets.push(get_set(line));
    };

    let ray = &vec_of_sets[0];

    let mut best_set: &Set = ray;
    let mut best_dist: f64 = -1.0;

    let v = &vec_of_sets;
    let start_index = 1;
    for e in v[start_index..].iter() {
        let distance = get_distance(ray, e);
        if (distance >= 0.0) & ( (best_dist < 0.0) | (distance < best_dist)){
            best_dist = distance;
            best_set = e;
        };
        // println!("{}", get_distance(ray, e));
    }
    if best_dist < 0.0 {
        println!("");
    } else {
        print_set(best_set);
    }
}
