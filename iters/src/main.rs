fn main() {
    let colors = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

    print_elements(&colors);

    print_elements(&colors[1..3]);

    //let mut colors_iter: std::slice::Iter<String> = colors.iter();
}

// cextor slice.
fn print_elements(elements: &[String]) {
    for element in elements {
        println!("{}", element)
    }

    elements
        .iter()
        .map(|el|  format!("{} => {}", el, el.to_uppercase()))
        .for_each(|el| println!("{}", el))

}