fn main() {
    let total_num_of_particles = test_fluid::render::initialize_global();
    loop {
        test_fluid::render::step_global(total_num_of_particles);
        // don't peg the cpu, be merciful, pause a little.
        std::thread::sleep(std::time::Duration::from_micros(3000));
        // usleep(3000);
    }
}
