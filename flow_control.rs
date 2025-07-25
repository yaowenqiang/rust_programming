fn main() {
    demo_if();
    demo_match();
    demo_loops();
    demo_break_continue();
}

fn demo_if() {
    let age = 50;
    if age > 50 {
        println!("You are old!");
    } else {
        println!("You are young ");
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
    println!("You are {}", msg);
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
        25 | 50 | 75 => println!("25, 50, or 75"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Something else"),
    }

    match num {
        x if x < 50 => println!("less than 50"),
        x if x == 75 => println!("75"),
        _ => println!("Something else"),
    }
    let res = match num {
        x if x < 50 =>"less than 50",
       x if x == 75 => "75",
        _ => "Something else"
    };


    println!("Result of match expression {}", res)
}

fn demo_loops() {
    /*println!("\nUsing a infinite loop");
    loop {
        println!("This loop will go on forever, hit Ctrl + C to stop me!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    */

    println!("\nUsing a while loop");

    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    println!("\nUsing a for loop over an array");
    let arr = [99,15, 95, 100,82];
    for elem in &arr {
        println!("{}", elem);
    }

    println!("\nUsing a for loop over a range(exclusive upper limit)");
    for i in 0..10 {
        println!("{}", i);
    }

    println!("\nUsng ia for loop over a range (inclusive upper limit)");
    for i in 0 ..= 10 {
        println!("{}", i);
    }
}

fn demo_break_continue() {
    println!("\nDemo using break and continue.");
    let arr = [1,2,3,4,5];
    for elem in arr {
        if elem == 5 {
            println!("Found 5, so break out of the loop compeletely");
            break;
        }
        println!("{}", elem);
    }

    for elem in arr {
        if elem < 5 {
            println!("\nFound value less than 5, continue to next iteration");
            continue;
        }
        println!("{}", elem);
    }
    
    'outer: loop {
        println!("Entered the outer loop.");
        loop {
            println!("Entered the inner loop");
            break 'outer;
       }
    }
    println!("Exited the outer loop");
    println!("The end.");
}
