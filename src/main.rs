
fn print_elements(elements: &Vec<String>){
    // for element in elements {
    //     println!("{}", element);
    // }

    elements.iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|value| println!("{}", value));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &mut [String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el))
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements.iter().map(|el| el.chars().map(|ch| ch.to_string()).collect()).collect()
}

fn main() {
    let colors = vec![
        String::from("Red"),
        String::from("Blue"),
        String::from("Green"),
        String::from("Orange"),
    ];

    // print_elements(&colors);

    // let mut color_iter = colors.iter();
    // println!("{:#?}",color_iter.next());
    // println!("{:#?}",color_iter.next());
    // println!("{:#?}",color_iter.next());
    // println!("{:#?}",color_iter.next());
    // println!("{:#?}",color_iter.next());

    // shorten_strings(&mut colors[1..3]);
    // println!("{:#?}", colors)

    // let uppercases = to_uppercase(&mut colors[1..3]);
    // println!("{:#?}", uppercases)

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("Destination: {:#?}", destination);

    let exploded = explode(&colors);
    println!("{:#?}", exploded);
}
