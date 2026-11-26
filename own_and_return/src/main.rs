pub struct Film {
    pub name: String,
}

pub fn read_film_name(film: &Film) -> String {
    film.name.clone()
}

pub fn take_film_name(film: Film) -> String {
    film.name
}


fn main() {
    let mut my_film = Film {
        name: "Terminator".to_owned(),
    };

    // println!("{}", take_film_name(/* to be implemented */));

    println!("{}", read_film_name(&my_film));
    println!("{}", read_film_name(&my_film));
    println!("{}", read_film_name(&my_film));
    println!("{}", read_film_name(&my_film));
    println!("{}", take_film_name(my_film));
    // println!("{}", read_film_name(&my_film));

    // the order of the print statements is intentional.
    // you can test this exercise properly by uncommenting out the first print statement,
    // you should get a compilation error then if your implementation is correct.
}
