use std::io::Read;

fn main() {
    let mut level_file_data = Vec::new();
    std::io::stdin().read_to_end(&mut level_file_data).expect("failed to read from stdin");

    let total_num_of_particles = test_fluid::render::initialize(level_file_data);
    // terminal escape code to clear the screen
    print!("\x1b[2J");
    loop {
        test_fluid::render::step_global(total_num_of_particles);
        test_fluid::render::draw();
        // don't peg the cpu, be merciful, pause a little.
        std::thread::sleep(std::time::Duration::from_micros(3000));
        // usleep(3000);
    }
}
