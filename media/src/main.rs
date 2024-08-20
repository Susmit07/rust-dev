mod content;

use content::catalog::Catalog;
use content::media::Media;


pub fn print_media(media: &Media) {
    println!("{:#?}", media);
}


fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook")
    };

    let movie = Media::Movie { 
        title: ("3 Idiots".to_string()), director: ("R Hirani".to_string())
    };

    let podcast = Media::Podcast(125);

    let audiobook_ref = &audiobook;
    let movie_ref = &movie;

    print_media(audiobook_ref);
    print_media(movie_ref);

    let description = Media::description(audiobook_ref);
    println!("Description {}", description);

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(podcast);

    // optionals.
    match catalog.get_by_index(100) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at that index")
        }
    }

}
