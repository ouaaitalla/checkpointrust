#[derive(Eq, PartialEq, Debug)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}
#[derive(Eq , PartialEq, Debug)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    let mut outfit = Outfit {
        jacket: Jacket::Black,
        hat: Hat::Snapback,
    };
    if formality_level == None {
        outfit.jacket = Jacket::Flowers;
    }
    if formality_level > Some(0) {
        outfit.jacket = Jacket::White;
    }else {
        outfit.jacket = Jacket::Black;
    }
    if invitation_message.is_ok() {
        outfit.hat = Hat::Fedora;
    }else {
        outfit.hat = Hat::Snapback;
    }
    if formality_level == None && invitation_message.is_err() {
        return Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        };
    }
    outfit
}

fn main() {
    println!(
        "My outfit will be: {:?}",
        choose_outfit(None, Err("Dear friend, ..."))
    );
}
