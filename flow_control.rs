fn main() {
    demo_if();
    demo_match();
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

fn demo_match() {
    let num = 100;
    println!("\nUsing a match to test an expression against patterns");

    match num {
        100 => println!("A hundred"),
        200 => println!("Two hundred"),
        _ => print!("Something else"),
    }

    match num {
        25..=50 => println!("25 to 50"),
        51..=100 => println!("51 to 100"),
        _ => print!("Something else"),
    }
    match num {
        25 | 50 | 75 => println!("25, 50, or 7t"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Something else"),
    }

    match num {
        x if x < 50 => println!("less then 50"),
        x if x == 75 => println!("75"),
        _ => println!("Something else"),
    }
    let res = match num {
        x if x < 50 =>"less then 50",
       x if x == 75 => "75",
        _ => "Something else"
    };


    println!("Result of match expression {}", res)
}
