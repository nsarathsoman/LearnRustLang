//enums are types that have a few definite values

enum Movement {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

fn move_avatar(m: Movement) {
    match m {
        Movement::UP => println!("Avatar moving UP"),
        Movement::DOWN => println!("Avatar moving DOWN"),
        Movement::LEFT => println!("Avatar moving LEFT"),
        Movement::RIGHT => println!("Avatar moving RIGHT")
    }
}

pub fn run() {
    let avatar1 = Movement::LEFT;
    move_avatar(avatar1);
}