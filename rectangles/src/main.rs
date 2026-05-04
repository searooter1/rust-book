struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rec = Rectangle {height: 3, width: 3};
    let area = rec.area();

    println!("Rectangle height: {}, width: {}, area: {}", rec.height, rec.width, area);
}
