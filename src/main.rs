use gfx_demo;
use rand::Rng;

const TITLE: &'static str = "Conway";
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;
const GRID_WIDTH: usize = WINDOW_WIDTH / 4;
const GRID_HEIGHT: usize = WINDOW_HEIGHT / 4;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const COLOUR_ALIVE: u32 = 0xff00ff00;
const COLOUR_DEAD: u32 = 0xff000000;
const INV_DENSITY: u32 = 5;

/// Returns `1` if the pixel at `index` is alive or `0` otherwise.
fn count_index(pixels: &[u32], index: isize) -> usize {
    if pixels[index as usize] == COLOUR_ALIVE { 1 } else { 0 }
}

/// Counts the number of alive neighbours for the pixel at `index` in `pixels`.
fn neighbours(pixels: &[u32], index: usize) -> usize {
    let mut count = 0;
    let iwidth = GRID_WIDTH as isize;
    let iheight = GRID_HEIGHT as isize;
    let imax = iwidth * iheight;
    let iindex = index as isize;
    let mut test_idx: isize;

    if index >= GRID_WIDTH {
        // NW:
        test_idx = iindex - iwidth - 1;
        if test_idx >= 0 {
            count += count_index(pixels, test_idx);
        }

        // N:
        test_idx = iindex - iwidth;
        if test_idx >= 0 {
            count += count_index(pixels, test_idx);
        }

        // NE:
        test_idx = iindex - iwidth + 1;
        if test_idx >= 0 && (test_idx % iwidth) != 0 {
            count += count_index(pixels, test_idx);
        }
    }

    // W:
    test_idx = iindex - 1;
    if test_idx >= 0 && (iindex % iwidth) != 0 {
        count += count_index(pixels, test_idx);
    }

    // E:
    test_idx = iindex + 1;
    if test_idx < imax && (test_idx % iwidth) != 0 {
        count += count_index(pixels, test_idx);
    }

    if index < GRID_SIZE - GRID_WIDTH {
        // SW:
        test_idx = iindex + iwidth - 1;
        if test_idx < imax && (iindex % iwidth) != 0 {
            count += count_index(pixels, test_idx);
        }

        // S:
        test_idx = iindex + iwidth;
        if test_idx < imax {
            count += count_index(pixels, test_idx);
        }

        // SE:
        test_idx = iindex + iwidth + 1;
        if test_idx < imax && (test_idx % iwidth) != 0 {
            count += count_index(pixels, test_idx);
        }
    }

    count
}

fn main() {
    let mut new_pixels = vec![COLOUR_DEAD; (GRID_WIDTH * GRID_HEIGHT) as usize];
    let mut init = true;

    gfx_demo::gfx_demo(
        TITLE,
        WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32,
        GRID_WIDTH as u32, GRID_HEIGHT as u32,
        50,
        |pixels| {
            if init {
                init = false;

                // Glider:
                // new_pixels[0] = COLOUR_DEAD;
                // new_pixels[1] = COLOUR_ALIVE;
                // new_pixels[2] = COLOUR_DEAD;

                // new_pixels[GRID_WIDTH + 0] = COLOUR_DEAD;
                // new_pixels[GRID_WIDTH + 1] = COLOUR_DEAD;
                // new_pixels[GRID_WIDTH + 2] = COLOUR_ALIVE;

                // new_pixels[GRID_WIDTH + GRID_WIDTH + 0] = COLOUR_ALIVE;
                // new_pixels[GRID_WIDTH + GRID_WIDTH + 1] = COLOUR_ALIVE;
                // new_pixels[GRID_WIDTH + GRID_WIDTH + 2] = COLOUR_ALIVE;

                let mut rng = rand::thread_rng();
                for idx in 0..(GRID_WIDTH * GRID_HEIGHT) {
                    if rng.gen_range(0..INV_DENSITY) == 0 {
                        new_pixels[idx] = COLOUR_ALIVE;
                    } else {
                        new_pixels[idx] = COLOUR_DEAD;
                    }
                }
            } else {
                for y in 0..GRID_HEIGHT {
                    for x in 0..GRID_WIDTH {
                        let idx = (y * GRID_WIDTH) + x;
                        let n = neighbours(pixels, idx);

                        if pixels[idx] == COLOUR_ALIVE {
                            new_pixels[idx] = if n == 2 || n == 3 {
                                COLOUR_ALIVE
                            } else {
                                COLOUR_DEAD
                            };
                        } else {
                            new_pixels[idx] = if n == 3 {
                                COLOUR_ALIVE
                            } else {
                                COLOUR_DEAD
                            }
                        }
                    }
                }
            }

            pixels.copy_from_slice(&new_pixels);
        }
    ).unwrap();
}
