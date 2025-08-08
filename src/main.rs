fn main() {
    let colors = vec![
        String::from("Red"),
        String::from("Blue"),
        String::from("Green"),
        String::from("Orange"),
    ];
    let mut color_iter = colors.iter();
    println!("{:#?}",color_iter.next());
    println!("{:#?}",color_iter.next());
    println!("{:#?}",color_iter.next());
    println!("{:#?}",color_iter.next());
    println!("{:#?}",color_iter.next());
}
