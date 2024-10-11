fn get_element(year: i32) -> &'static str {
    let element = ((year - 4) % 10 / 2) as usize;
    return ELEMENTS[element];
}

fn get_animal(year: i32) -> &'static str {
    return ANIMALS[(year - 4) % 12];
}

fn get_yy(year: i32) -> &'static str {
    if year % 2 == 0 {
        return "yang";
    } else {
        return "yin";
    }
}

fn main() {
    const ANIMALS: [&str; 12] = ["Rat", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey", "Rooster", "Dog", "Pig"];
    const ELEMENTS: [&str; 5] = ["Wood", "Fire", "Earth", "Metal", "Water"];

    let years = [1935, 1938, 1968, 1972, 1976, 2017];

    for &year in years.iter() {
        println!("{} is the year of the {} {} ({}).", year, get_element(year), get_animal(year), get_yy(year));
    }
}