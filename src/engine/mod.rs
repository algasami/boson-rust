/**
 * Boson engine
 * This is intended as a singleton (although it's never forbidden to instantiate multiple instances)
 */
pub struct BosonEngine<const WIDTH: usize, const HEIGHT: usize> {
    ibuffer: [[f64; WIDTH]; HEIGHT],
}

impl<const W: usize, const H: usize> BosonEngine<W, H> {
    /**
     * Display assumes that **ibuffer is ready to go**, so bear this in mind.
     */
    pub fn display(&self) {
        for i in 0..H {
            for j in 0..W {
                print!("{}", get_luma(self.ibuffer[i][j]));
            }
            print!("\n");
        }
    }
}

impl<const W: usize, const H: usize> Default for BosonEngine<W, H> {
    fn default() -> BosonEngine<W, H> {
        BosonEngine::<W, H> {
            ibuffer: [[-0.1; W]; H],
        }
    }
}

fn get_luma(f: f64) -> char {
    if f < 0.0 {
        return ' ';
    }
    LUMA_INDEX.as_bytes()[((LUMA_INDEX.len() - 1) as f64 * f) as usize] as char
}

const LUMA_INDEX: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";
