fn main() {
    demo_simple_iteration();
    demo_unused_closure_variable();
    demo_filtering_mapping();
    demo_collecting_results();
}

fn demo_simple_iteration() {
    let v = vec!["donald", "huey", "louie", "dewey"];
    println!("All ducks:");
    v.iter().for_each(|d| println!("  {}", d));
}

fn demo_unused_closure_variable() {
    let v = vec!["donald", "huey", "louie", "dewey"];
    println!("All ducks:");
    v.iter().for_each(|_| println!(" xxx"));
}

fn demo_filtering_mapping() {
    let v = vec!["donald", "huey", "louie", "dewey"];
    v.iter()
        .filter(|e| e.starts_with("d"))
        .map(|e| e.to_uppercase())
        .for_each(|e| println!("  {}", e));
}

fn demo_collecting_results() {
    let v = vec!["donald", "huey", "louie", "dewey"];
    v.iter()
        .filter(|e| e.starts_with("d"))
        .map(|e| e.to_uppercase())
        .for_each(|e| println!("  {}", e));
}
