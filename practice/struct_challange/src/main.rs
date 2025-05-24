// Rectangle Analyzer :

// Create a program that:
// Takes a list of rectangles (Vec<Rectangle>)
// Prints each one using dbg!()
// Calculates and prints:
// Which one has the largest area
// How many are squares
// Use all struct and method concepts properly.

#[derive(Debug, Default)]
struct Rectangle {
    length: f32,
    breadth: f32,
}
impl Rectangle {
    fn area(&self) -> f32 {
        self.length * self.breadth
    }
    fn is_square(&self) -> bool {
        self.length == self.breadth
    }
}
// main function
fn main() {
    let rectangles = vec![
        Rectangle {
            length: 10.0,
            breadth: 20.0,
        },
        Rectangle {
            length: 15.0,
            breadth: 15.0,
        },
        Rectangle {
            length: 12.0,
            breadth: 10.0,
        },
        Rectangle {
            length: 5.0,
            breadth: 5.0,
        },
    ];
    // println!("{:#?}", rectangles);
    // Print each rectangle using dbg!
    for (i, rect) in rectangles.iter().enumerate() {
        dbg!((i, rect));
    }

    // Find the rectangle with the largest area
    // let largest = rectangles.iter().max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap()).unwrap();
    let largest = rectangles.iter().max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap()).unwrap();

    println!("Rectangle with largest area: {:?} with area: {}",largest,largest.area());

    // Count how many are squares
    let square_count = rectangles.iter().filter(|r| r.is_square()).count();
    println!("Number of squares: {}", square_count);
}
