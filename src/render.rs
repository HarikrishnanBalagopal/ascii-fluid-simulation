use std::io::Read;

use crate::constants::{PARTICLES, SCREEN_BUFFER, CHARACTERS, GRAVITY, PRESSURE, VISCOSITY, CONSOLE_WIDTH, CONSOLE_HEIGHT};

#[no_mangle]
pub extern fn initialize_global() -> usize {
    let mut total_of_particles: usize = 0;

    // terminal escape code to clear the screen
    print!("\x1b[2J");

    // read the input file to initialise the particles.
    // # stands for "wall", i.e. unmovable particles (very high density)
    // any other non-space character represents normal particles.
    let mut level_file_data = Vec::new();
    std::io::stdin()
        .read_to_end(&mut level_file_data)
        .expect("failed to read from stdin");
    let mut x_sandbox_area_scan: i32 = 0;
    let mut y_sandbox_area_scan: i32 = 0;

    let mut particles_counter: usize = 0;
    for x in level_file_data {
        if x == '\n' as u8 {
            // next row
            // rewind the x to -1 cause it's gonna be incremented at the
            // end of the while body
            y_sandbox_area_scan += 2;
            x_sandbox_area_scan = -1;
        } else if x == ' ' as u8 {
            // do nothing
        } else {
            if x == '#' as u8 {
                // The character # represents “wall particle” (a particle with fixed position),
                // and any other non-space characters represent free particles.
                // A wall sets the flag on 2 particles side by side.
                unsafe {
                    PARTICLES[particles_counter].wallflag = 1;
                    PARTICLES[particles_counter + 1].wallflag = 1;
                }
            }
            // Each non-empty character sets the position of two
            // particles one below the other (real part is rows)
            // i.e. each cell in the input file corresponds to 1x2 particle spaces,
            // and each character sets two particles
            // one on top of each other.
            // It's as if the input map maps to a space that has twice the height, as if the vertical
            // resolution was higher than the horizontal one.
            // This is corrected later, see "y scale correction" comment.
            // I think this is because of gravity simulation, the vertical resolution has to be
            // higher, or conversely you can get away with simulating a lot less of what goes on in the
            // horizontal axis.
            unsafe {
                PARTICLES[particles_counter].x_pos = x_sandbox_area_scan as f64;
                PARTICLES[particles_counter].y_pos = y_sandbox_area_scan as f64;

                PARTICLES[particles_counter + 1].x_pos = x_sandbox_area_scan as f64;
                PARTICLES[particles_counter + 1].y_pos = (y_sandbox_area_scan + 1) as f64;
            }

            // we just added two particles
            particles_counter += 2;
            total_of_particles = particles_counter;
        }
        // next column
        x_sandbox_area_scan += 1;
    }
    return total_of_particles;
}

#[no_mangle]
pub extern fn step_global(total_of_particles:usize) {
        // Iterate over every pair of particles to calculate the densities
        for particles_cursor in 0..total_of_particles {
            // density of "wall" particles is high, other particles will bounce off them.
            unsafe {
                PARTICLES[particles_cursor].density =
                    (PARTICLES[particles_cursor].wallflag * 9) as f64;
            }
    
            for particles_cursor2 in 0..total_of_particles {
                unsafe {
                    let x_particle_distance =
                        PARTICLES[particles_cursor].x_pos - PARTICLES[particles_cursor2].x_pos;
                    let y_particle_distance =
                        PARTICLES[particles_cursor].y_pos - PARTICLES[particles_cursor2].y_pos;
                    let particles_distance =
                        (x_particle_distance.powf(2.0) + y_particle_distance.powf(2.0)).sqrt();
                    let particles_interaction = particles_distance / 2.0 - 1.0;
    
                    // this line here with the alternative test
                    // works much better visually but breaks simmetry with the
                    // next block
                    // if particles_interaction.round() < 1.0 {
                    // density is updated only if particles are close enough
                    if (1.0 - particles_interaction).floor() > 0.0 {
                        PARTICLES[particles_cursor].density +=
                            particles_interaction * particles_interaction;
                    }
                }
            }
        }
    
        // Iterate over every pair of particles to calculate the forces
        for particles_cursor in 0..total_of_particles {
            unsafe {
                PARTICLES[particles_cursor].y_force = GRAVITY;
                PARTICLES[particles_cursor].x_force = 0.0;
            }
    
            for particles_cursor2 in 0..total_of_particles {
                unsafe {
                    let x_particle_distance =
                        PARTICLES[particles_cursor].x_pos - PARTICLES[particles_cursor2].x_pos;
                    let y_particle_distance =
                        PARTICLES[particles_cursor].y_pos - PARTICLES[particles_cursor2].y_pos;
                    let particles_distance =
                        (x_particle_distance.powf(2.0) + y_particle_distance.powf(2.0)).sqrt();
                    let particles_interaction = particles_distance / 2.0 - 1.0;
                    // force is updated only if particles are close enough
                    if (1.0 - particles_interaction).floor() > 0.0 {
                        PARTICLES[particles_cursor].x_force += particles_interaction
                            * (x_particle_distance
                                * (3.0
                                    - PARTICLES[particles_cursor].density
                                    - PARTICLES[particles_cursor2].density)
                                * PRESSURE
                                + PARTICLES[particles_cursor].x_velocity * VISCOSITY
                                - PARTICLES[particles_cursor2].x_velocity * VISCOSITY)
                            / PARTICLES[particles_cursor].density;
                        PARTICLES[particles_cursor].y_force += particles_interaction
                            * (y_particle_distance
                                * (3.0
                                    - PARTICLES[particles_cursor].density
                                    - PARTICLES[particles_cursor2].density)
                                * PRESSURE
                                + PARTICLES[particles_cursor].y_velocity * VISCOSITY
                                - PARTICLES[particles_cursor2].y_velocity * VISCOSITY)
                            / PARTICLES[particles_cursor].density;
                    }
                }
            }
        }
    
        // empty the buffer
        for screen_buffer_index in 0..(CONSOLE_WIDTH * CONSOLE_HEIGHT) {
            unsafe {
                SCREEN_BUFFER[screen_buffer_index] = 0;
            }
        }
    
        for particles_cursor in 0..total_of_particles {
            unsafe {
                if PARTICLES[particles_cursor].wallflag == 0 {
                    // This is the newtonian mechanics part: knowing the force vector acting on each
                    // particle, we accelerate the particle (see the change in velocity).
                    // In turn, velocity changes the position at each tick.
                    // Position is the integral of velocity, velocity is the integral of acceleration and
                    // acceleration is proportional to the force.
    
                    // force affects velocity
                    if (PARTICLES[particles_cursor].x_force.powf(2.0)
                        + PARTICLES[particles_cursor].y_force.powf(2.0))
                    .sqrt()
                        < 4.2
                    {
                        PARTICLES[particles_cursor].x_velocity +=
                            PARTICLES[particles_cursor].x_force / 10.0;
                        PARTICLES[particles_cursor].y_velocity +=
                            PARTICLES[particles_cursor].y_force / 10.0;
                    } else {
                        PARTICLES[particles_cursor].x_velocity +=
                            PARTICLES[particles_cursor].x_force / 11.0;
                        PARTICLES[particles_cursor].y_velocity +=
                            PARTICLES[particles_cursor].y_force / 11.0;
                    }
    
                    // velocity affects position
                    PARTICLES[particles_cursor].x_pos += PARTICLES[particles_cursor].x_velocity;
                    PARTICLES[particles_cursor].y_pos += PARTICLES[particles_cursor].y_velocity;
                }
            }
            // given the position of the particle, determine the screen buffer
            // position that it's going to be in.
            unsafe {
                let x = PARTICLES[particles_cursor].x_pos.floor() as i32;
                // y scale correction, since each cell of the input map has
                // "2" rows in the particle space.
                let y = (PARTICLES[particles_cursor].y_pos / 2.0).floor() as i32;
                let screen_buffer_index: i32 = x + (CONSOLE_WIDTH as i32) * y;
                // if the particle is on screen, update
                // four buffer cells around it
                // in a manner of a "gradient",
                // the representation of 1 particle will be like this:
                //
                //      8 4
                //      2 1
                //
                // which after the lookup that puts chars on the
                // screen will look like:
                //
                //      ,.
                //      `'
                //
                // With this mechanism, each particle creates
                // a gradient over a small area (four screen locations).
                // As the gradients of several particles "mix",
                // (because the bits are flipped
                // independently),
                // a character will be chosen such that
                // it gives an idea of what's going on under it.
                // You can see how corners can only have values of 8,4,2,1
                // which will have suitably "pointy" characters.
                // A "long vertical edge" (imagine two particles above another)
                // would be like this:
                //
                //      8  4
                //      10 5
                //      2  1
                //
                // and hence 5 and 10 are both vertical bars.
                // Same for horizontal edges (two particles aside each other)
                //
                //      8  12 4
                //      2  3  1
                //
                // and hence 3 and 12 are both horizontal dashes.
                // ... and so on for the other combinations such as
                // particles placed diagonally, where the diagonal bars
                // are used, and places where four particles are present,
                // in which case the highest number is reached, 15, which
                // maps into the blackest character of the sequence, '#'
    
                if y >= 0
                    && y < (CONSOLE_HEIGHT - 1) as i32
                    && x >= 0
                    && x < (CONSOLE_WIDTH - 1) as i32
                {
                    SCREEN_BUFFER[screen_buffer_index as usize] |= 8; // set 4th bit to 1
                    SCREEN_BUFFER[screen_buffer_index as usize + 1] |= 4; // set 3rd bit to 1
                                                                          // now the cell in row below
                    SCREEN_BUFFER[screen_buffer_index as usize + CONSOLE_WIDTH] |= 2; // set 2nd bit to 1
                    SCREEN_BUFFER[screen_buffer_index as usize + CONSOLE_WIDTH + 1] |= 1;
                    // set 1st bit to 1
                }
            }
        }
    
        // Update the screen buffer
        for screen_buffer_index in 0..(CONSOLE_WIDTH * CONSOLE_HEIGHT) {
            if screen_buffer_index % CONSOLE_WIDTH == CONSOLE_WIDTH - 1 {
                unsafe {
                    SCREEN_BUFFER[screen_buffer_index] = '\n' as u8;
                }
            } else {
                // the string below contains 16 characters, which is for all
                // the possible combinations of values in the screenbuffer since
                // it can be subject to flipping of the first 4 bits
                unsafe {
                    SCREEN_BUFFER[screen_buffer_index] =
                        CHARACTERS[SCREEN_BUFFER[screen_buffer_index] as usize] as u8;
                }
                // ---------------------- the mappings --------------
                // 0  maps into space
                // 1  maps into '    2  maps into `    3  maps into -
                // 4  maps into .    5  maps into |    6  maps into /
                // 7  maps into /    8  maps into ,    9  maps into \
                // 10 maps into |    11 maps into \    12 maps into _
                // 13 maps into \    14 maps into /    15 maps into #
            }
        }
    
        // terminal escape code to put cursor back to the top left of the screen
        print!("\x1b[1;1H");
        // finally blit the screen buffer to screen
        unsafe {
            print!("{}", std::str::from_utf8_unchecked(&SCREEN_BUFFER));
        }
}
