use crate::backend::arch::ultrascale::Ultrascale;
use crate::util::file::read_to_string;

impl Default for Ultrascale {
    fn default() -> Self {
        Ultrascale {
            spec: read_to_string("src/backend/arch/ultrascale/spec.json"),
        }
    }
}
