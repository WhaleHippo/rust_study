use day002_project_units::{ProjectUnit, unit_label};

fn main() {
    println!("package: {}", unit_label(ProjectUnit::Package));
    println!("crate: {}", unit_label(ProjectUnit::Crate));
    println!("module: {}", unit_label(ProjectUnit::Module));
}
