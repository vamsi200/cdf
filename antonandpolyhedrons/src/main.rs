use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_parsed: usize = input.trim().parse().unwrap();

    let face_counts: HashMap<&str, i32> = [
        ("Tetrahedron", 4),
        ("Cube", 6),
        ("Octahedron", 8),
        ("Dodecahedron", 12),
        ("Icosahedron", 20),
    ]
    .iter()
    .cloned()
    .collect();

    let mut total_faces = 0;

    for _ in 0..input_parsed {
        let mut shape = String::new();
        io::stdin().read_line(&mut shape).unwrap();
        let shape = shape.trim();

        if let Some(&faces) = face_counts.get(shape) {
            total_faces += faces;
        }
    }

    println!("{total_faces}");
}
