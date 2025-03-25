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

// Instalo periferico de salida en 0xFE

//render_registers(&mut canvas, &font, &texture_creator, &z80, &mut texture_cache);

use crate::z80::{io, Z80};

// pub struct Ula {
//     bytes: Vec<u8>,
//
// }
// impl Ula {
//     pub fn new() -> Ula {
//         Ula {
//             bytes: Vec::new()
//         }
//     }
// }

impl Z80 {
    pub fn instala_devices(&mut self) {
        // Instalo periferico de salida en 0xFE
        let out = io::BufOutput::default();
        self.install_output(0xFE, Box::new(out.clone()));

        // Instalo periferico de entrada en 0xFE
        let inp = io::BufInput::new(vec!(0x00, 0x00, 0x00));
        self.install_input(0xFE, Box::new(inp.clone()));
    }

    // pub fn imprime(&mut self) {
    //     // for (clave, valor) in &z80.output_devices {
    //     //     //println!("clave->{:2X}  valor->{:2X}", clave, valor);
    //     //     println!("clave--->{:?} valor--->{:?}", clave, valor);
    //     // }
    //     for (port, output_device) in &self.output_devices {
    //         // Primero obtenemos el Rc y luego el borrow
    //         let buffer_rc = output_device.get_buffer();
    //         let buffer = buffer_rc.borrow();
    //
    //         //dbg_hex!(&buffer);
    // 
    //         for n in 0..buffer.len() {
    //             println!("Puerto 0x{:02X}: {:02X}",
    //                      port,
    //                      buffer[n],
    //             );
    //         }
    //     }
    // }
}