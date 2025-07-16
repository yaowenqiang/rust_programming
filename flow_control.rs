fn main() {
    demo_if();
}

fn demo_if() {
    let age = 50;
    if age > 50 {
        print!("You are old!");
    } else {
        print!("You are young ");
    }

    let height = 1.67;
    if height > 1.8 {
        println!("You are tall");
    } else {
        println!("You are not so tall");
    }

    let swans_games = 500;
    if swans_games > 300 {
        println!("You are a very loyal fan, we appreciate it dude");
    } else if swans_games > 100 {
        println!("You are a discerning fan");
    } else {
        println!("You are quite a new fan, welcome buddy");
    }

    let msg = if age > 50 { "old" } else { "young" };
    print!("You are {}", msg);
}
