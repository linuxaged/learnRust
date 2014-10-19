struct Point {
    x: f32,
    y: f32
}

struct Rect<'r> {
    verts: [&'r Point, ..2]
}

fn main() {
    let p0 = Point{x: 1.0, y: 2.0};
    let p1 = Point{x: 3.0, y: 4.0};
    let rect = Rect{verts:[&p0, &p1]};
    println!("rect {},{}",rect.verts[0].x, rect.verts[0].y);
}