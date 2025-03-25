// La representacion interna de la memoria del z80.
// Un arreglo grande.
pub const MEMORY_SIZE: usize = 64 * 1024; // 64 kibibytes

pub struct Memoria {
    pub mem: [u8; MEMORY_SIZE],
}

impl Default for Memoria {
    fn default() -> Self {
        Memoria {
            mem: [0; MEMORY_SIZE],
        }
    }
}
