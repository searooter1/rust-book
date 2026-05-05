enum Colours {
    Red,
    Green,
    EvilColour(String)
}

fn main() {
    let colour: Colours = Colours::EvilColour("Muhahah".to_string());

    match &colour {
        Colours::Red => println!("Red"),
        Colours::Green => println!("Green"),
        Colours::EvilColour(a) => println!("{}", a)
    }

    let evilness = check_evil(&colour);
    match evilness {
        Ok(s) => println!("{}", s),
        Err(_) => println!("Not Evil")
    }
}

fn check_evil(colour: &Colours) -> Result<String, String> {
    if let Colours::EvilColour(s) = colour {
        return Ok("Evil Colour Found!".to_string());
    }
    return Err(String::new())
}