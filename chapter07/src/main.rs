use chapter07::hosting as host;

fn main() {
    println!("Chapter 07: Managing Growing Projects with Packages, Crates, and Modules");
    println!("{}", host::greeting());
    println!("{}", chapter07::breakfast_toast());
    println!("{}", chapter07::menu_summary());
}
