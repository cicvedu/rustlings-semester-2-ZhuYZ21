struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    if let Some(ref p) = y {
        println!("Co-ordinates are {},{} ", p.x, p.y);
    } else {
        panic!("no match!");
    }
    y; // Fix without deleting this line.
}
