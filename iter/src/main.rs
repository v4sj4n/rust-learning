fn print_elements(elements: &[String]) {
    // 1)
    // for element in elements {
    //     println!("{}", element);
    // }
    // 2)
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_string(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(1)); // iter_mut for modification
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<_>>()
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|char| char.to_string()).collect())
        .collect()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("violet"),
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];

    // print_elements(&colors[2..4]);
    // shorten_string(&mut colors);
    // println!("{:#?}", colors)

    // let uppercased_colors = to_uppercase(&colors);
    // println!("{:#?}", uppercased_colors)
    //
    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("{:#?}", destination);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded)

    let found_color = find_color_or(&colors, "re", "orange");
    println!("{:#?}", found_color)
}
