use chapter14::add_one;

fn main() {
    println!("Chapter 14: More About Cargo and Crates.io");
    println!("profiles: dev compiles quickly; release optimizes output.");
    println!("docs: cargo doc builds API documentation; cargo test --doc runs doctests.");
    println!("install: cargo install <crate> installs a published binary.");
    println!("workspaces: packages can share one repository and lockfile.");
    println!("custom commands: cargo-foo is invoked as cargo foo.");
    println!(
        "library re-export: chapter14::add_one(41) = {}",
        add_one(41)
    );
}
