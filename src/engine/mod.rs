/**
 * Boson engine
 * This is intended as a singleton (although it's never forbidden to instantiate multiple instances)
 */
pub struct BosonEngine<const WIDTH: usize, const HEIGHT: usize> {
    pub config: BosonConfig,

    ibuffer: [[f64; WIDTH]; HEIGHT],
}
pub struct BosonConfig {}

impl<const W: usize, const H: usize> BosonEngine<W, H> {
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
            config: BosonConfig {},
            ibuffer: [[0.0; W]; H],
        }
    }
}

fn get_luma(f: f64) -> char {
    return LUMA_INDEX.as_bytes()[((LUMA_INDEX.len() - 1) as f64 * f) as usize] as char;
}

const LUMA_INDEX: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";