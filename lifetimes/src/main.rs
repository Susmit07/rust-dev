fn next_language <'a>(languages: &'a[String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn last_language (languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn long_language <'l> (language1: &'l String, language2: &'l String) -> &'l String {
    if language1.len() > language2.len() {
        return language1;
    } else {
        return language2;
    }
}


fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_language(&languages, "go");

    println!("{}", result);
}
