//! Procedural graphics generator for Nanite Swarm (2D tiles)

mod graphics_gen;

fn main() {
    println!("=== Nanite Swarm Graphics Generator ===\n");

    create_asset_directories();
    graphics_gen::generate_all_graphics();

    println!("\n=== All graphics generated successfully! ===");
}

fn create_asset_directories() {
    let directories = ["assets/tiles", "assets/tiles/buildings", "assets/ui/buildings"];

    for dir in directories {
        std::fs::create_dir_all(dir).unwrap();
        println!("Created directory: {}", dir);
    }
    println!();
}
