use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::any::Any;
use std::cell::RefCell;
//use std::io::Write;
use std::rc::Rc;
use z80::io;
use zilog_z80::cpu::CPU;

mod cpu;
mod z80;
mod ops;
mod assert;
mod ula;
mod sonido;

#[macro_export]
macro_rules! println_tee {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        // Imprime en consola
        println!("{}", message);
        // Escribe en el archivo (manejo básico de errores)
        let _ = OpenOptions::new()
            .append(true)
            .create(true)
            .open("salida.txt")
            .and_then(|mut file| writeln!(file, "{}", message));
    }};
}

#[derive(Debug, Clone, Default)]
pub struct StdoutOutput {
    output: Rc<RefCell<Vec<u8>>>,
}

impl io::OutputDevice for StdoutOutput {
    // fn output(&self, byte: u8) {
    //     let _ = stdout().write(&[byte]);
    // }

    fn write(&self, data: &[u8]) {
        print!("{}", String::from_utf8_lossy(data));
        self.output.borrow_mut().extend_from_slice(data);
    }
    fn read(&self) -> Vec<u8> {
        //stdout().borrow().clone()
        //dbg!(std::io::stdout().flush());
        vec![]
    }
    fn get_buffer(&self) -> Rc<RefCell<Vec<u8>>> {
        self.output.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// // Lo pongo yo
// impl fmt::Display for StdoutOutput {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "StdoutOutput") // Personaliza la salida aquí
//     }
// }

static DEBUG: bool = true;
//static VALOR_BREAKPOINT: u16 = 0x11EF;
static VALOR_BREAKPOINT: u16 = 0x0000;

fn main() -> Result<(), String> {
    // creo instancia zilog_z80
    let mut c = CPU::new(0xFFFF);

    // Inicializar SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Inicializar el subsistema TTF
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // La pantalla de spectrum es de 256x192
    // Añadiendo por debajo 192 pixels para poner los registros
    let window = video_subsystem
        .window("ZX Spectrum Emulator", 1600, 900) // Escalado x2 para mejor visualización
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_scale(1.0, 1.0)?; // Escalar la pantalla x2

    let path = "ROMS/ZXSpectrum48.rom";
    //let path = "ROMS/pbas.rom";
    //let path = "PROGRAMAS/pba00.z80.bin";
    let rom: Vec<u8> = std::fs::read(path).expect("No se pudo cargar la ROM");
    //let rom: Vec<u8> = std::fs::read("ROMS/pbas.rom").expect("No se pudo cargar la ROM");
    //let rom: Vec<u8> = std::fs::read("PROGRAMAS/pba00.z80.bin").expect("No se pudo cargar la ROM");

    if rom.len() != 0x4000 {
        panic!("El tamaño de la ROM no es válido (debe ser 16KB)");
    }

    // Creamos un microP por defecto, debug y breakpoint
    let mut z80 = z80::Z80::default();

    // Copiar la ROM de 16K a la memoria (dirección 0x0000 a 0x3FFF)
    z80.load(rom.as_slice());
    c.bus.load_bin(path, 0).unwrap();

    z80.instala_devices();

    // Cargar una fuente TTF
    let font = ttf_context.load_font("FONTS/DejaVuSansMono.ttf", 16)?;

    // Crear una textura para los registros
    let texture_creator = canvas.texture_creator();
    //let mut texture_cache: HashMap<String, Texture> = HashMap::new();

    // Bucle principal de ejecución
    let mut event_pump = sdl_context.event_pump()?;
    //let mut tsc = host::TsCounter::default(); // Contador de T-states
    //let mut last_registers: [String; 7] = Default::default(); // Almacenar los últimos valores de los registros

    //let mut cpu_debug = CpuDebug::default();
    z80.es_halted = false;
    'running: loop {
        // Manejar eventos de SDL2 (cierre de ventana, teclado, etc.)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    //dbg!("Presionada tecla S");

                    z80.es_halted = false;
                }
                _ => {}
            }
        }

        // Crea un dispositivo de salida en 0xFE (Usar la ULA), los bits tienen este significado:
        //             Bit   7   6   5   4   3   2   1   0
        //                 +-------------------------------+
        //                 |   |   |   | E | M |   Border  |
        //                 +-------------------------------+
        // Los tres bits más bajos especifican el color del borde; un cero en el bit 3 activa la salida MIC,
        // mientras que un uno en el bit 4 activa la salida EAR y el altavoz interno.
        // Sin embargo, los conectores EAR y MIC están conectados solo por resistencias, por lo que al activar
        // uno se activa el otro; el EAR se utiliza generalmente para la salida, ya que produce un sonido más fuerte.
        // Los dos bits superiores no se utilizan.
        //
        // Si se lee desde el puerto 0xfe, las ocho líneas de dirección más altas también son importantes.
        // Un cero en una de estas líneas selecciona una media fila particular de cinco teclas:
        //
        // IN: Lee las teclas (bit 0 a bit 4 inclusive)
        //
        // 0xfefe SHIFT, Z, X, C, V    0xeffe 0,            9, 8, 7, 6
        // 0xfdfe A,     S, D, F, G    0xdffe P,            O, I, U, Y
        // 0xfbfe Q,     W, E, R, T    0xbffe ENTER,        L, K, J, H
        // 0xf7fe 1,     2, 3, 4, 5    0x7ffe SPACE, SYM SHFT, M, N, B
        //
        // Un cero en uno de los cinco bits más bajos significa que se presionó la tecla correspondiente.
        // Si más de una línea de dirección se convierte en baja, el resultado es el AND lógico de todas
        // las entradas individuales, por lo que un cero en un bit significa que se presionó al menos
        // una de las teclas apropiadas. Por ejemplo, sólo si cada uno de los cinco bits más bajos del resultado
        // de la lectura desde el puerto 00FE (por ejemplo, mediante XOR A/IN A,(FE)) es uno, no se pulsa ninguna tecla.
        // El teclado está conectado de forma matricial, con 8 filas de 5 columnas. Cualquier par de teclas pulsadas
        // simultáneamente se pueden decodificar de forma única mediante la lectura desde los puertos IN.
        // Sin embargo, si se pulsan más de dos teclas, es posible que la decodificación no sea posible de forma única.
        // Por ejemplo, si se pulsa CAPS, B y V, Spectrum pensará que también se ha pulsado la tecla Espacio y
        // reaccionará dando el informe "Interrumpir en el programa".
        // Sin este comportamiento matricial, Zynaps, por ejemplo, no se detendrá cuando pulse
        // 5, 6, 7, 8 y 0 simultáneamente.
        //
        // El bit 6 del puerto IN 0xfe es el bit de entrada EAR. El valor leído desde este puerto no es trivial,
        // como se puede ver en el siguiente programa:
        //
        // 10 OUT 254,BIN 11101111
        // 20 PRINT IN 254
        // 30 OUT 254,BIN 11111111
        // 40 PRINT IN 254
        // 50 GOTO 10
        //
        // Para una prueba correcta no presione ninguna tecla mientras se ejecuta y no tenga ninguna entrada EAR.
        //
        // Si la salida es 191,255,191,255, etc., estás en la versión 3 de Spectrum.
        // Si la salida es siempre 191 o siempre 255, cambia el valor en la línea 10 a BIN 11100111.
        // Si la salida es entonces 191,255,191,255, etc., estás en la versión 2 de Spectrum.
        // Si la salida sigue siendo siempre 191 o siempre 255, estás en el emulador de Spectrum.
        //
        // El chip ULA usa el mismo pin (28) para todos los conectores MIC, EAR y el altavoz interno,
        // por lo que los bits 3 y 4 de una OUT al puerto 0xfe afectarán al bit 6 leído por una IN del puerto 0xfe.
        // La diferencia entre las máquinas de la versión 2 y 3 es:

        //z80.install_output(0xFE, Box::new(StdoutOutput {}));

        /* // Instalo periferico de salida en 0xFE
         let out = io::BufOutput::default();
         z80.install_output(0xFE, Box::new(out.clone()));

         // Instalo periferico de entrada en 0xFE
         let inp = io::BufInput::new(vec!(0x00, 0x00, 0x00));
         z80.install_input(0xFE, Box::new(inp.clone()));*/

        //render_registers(&mut canvas, &font, &texture_creator, &z80, &mut texture_cache);

        // Experimento zilog
        c.reg.pc = 0;

        // Ejecutar una instrucción de la CPU Z80 y presentar los cambios

        z80.run(&mut canvas, &font, &texture_creator, &mut c);

        //z80.es_halted = true; // Para poder hacer paso a paso
        //ula.imprime(&mut z80);
        // Renderizar los registros después de ejecutar z80.run()
        //render_registers(&mut canvas, &font, &texture_creator, &z80, &mut texture_cache)?;

    }
    Ok(())
}

