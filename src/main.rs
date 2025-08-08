
fn print_elements(elements: &Vec<String>){
    // for element in elements {
    //     println!("{}", element);
    // }

    elements.iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|value| println!("{}", value));
}

fn main() {
    let colors = vec![
        String::from("Red"),
        String::from("Blue"),
        String::from("Green"),
        String::from("Orange"),
    ];

    print_elements(&colors);

    let mut color_iter = colors.iter();
    println!("{:#?}",color_iter.next());
    println!("{:#?}",color_iter.next());
    println!("{:#?}",color_iter.next());
    println!("{:#?}",color_iter.next());
    println!("{:#?}",color_iter.next());
}
