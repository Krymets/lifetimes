fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for language in languages {
        if found {
            return language;
        }
        if language == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("python"),
        String::from("sh"),
    ];

    let result = next_language(&languages, "python");
    println!("{:?}", result);
}
