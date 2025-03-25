//! Methods associated with the IN and OUT instructions of the z80

use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use super::Z80;

// Un InputDevice puede leer un byte cada vez
pub trait InputDevice {
    // Lee in simple byte
    fn input(&self) -> u8;
}

// An OutputDevice puede escribir un byte cada vez
pub trait OutputDevice: Debug {
    // Escribe un simple byte
    //fn output(&self, val: u8);
    fn write(&self, data: &[u8]);
    fn read(&self) -> Vec<u8>;
    fn get_buffer(&self) -> Rc<RefCell<Vec<u8>>>;
    fn as_any(&self) -> &dyn Any;
}

impl Z80 {
    // Instala un input device con el index dado. Por ejemplo:

    // let mut z80 = Z80::default();
    // let inp = z80::io::BufInput::new(vec!(b'Z'));
    // z80.install_input(0, Box::new(inp.clone()));
    // Ahora se podra usar `IN (0), <registro>`.
    pub fn install_input(&mut self, index: u8, device: Box<dyn InputDevice>) {
        self.input_devices.insert(index, device);
    }

    // Instala un output device en el index determinado. Por ejemplo:

    // let mut z80 = Z80::default();
    // let out = z80::io::BufOutput::default();
    // z80.install_output(0, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0), <registro>`.
    pub fn install_output(&mut self, index: u8, device: Box<dyn OutputDevice>) {
        self.output_devices.insert(index, device);
    }
}

// BufInput es un simple InputDevice que produce entradas cuando es requerido, desde back a front.
// Usado en tests.
#[derive(Default, PartialEq, Clone)]
pub struct BufInput {
    pub input: Rc<RefCell<Vec<u8>>>,
}

impl InputDevice for BufInput {
    // Lee el right-most byte del buffer interno
    fn input(&self) -> u8 {
        self.input.borrow_mut().pop().unwrap()
    }
}

impl BufInput {
    pub fn new(v: Vec<u8>) -> Self {
        Self {
            input: Rc::new(RefCell::new(v)),
        }
    }
}

// BufOutput es un Output device que recibe salida y la añade a un vector interno.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BufOutput {
    output: Rc<RefCell<Vec<u8>>>,
}

impl BufOutput {
    // Todas las salidas recibidas del procesador, el mas reciente el ultimo.
    pub fn result(&self) -> Vec<u8> {
        self.output.borrow_mut().to_vec()
    }
}

impl OutputDevice for BufOutput {
    // Escribe un byte al final del buffer interno
    // fn output(&self, val: u8) {
    //     self.output.borrow_mut().push(val)
    // }
    fn write(&self, data: &[u8]) {
        // Escribe todos los bytes del slice, no solo el primero
        self.output.borrow_mut().extend_from_slice(data);
    }
    fn read(&self) -> Vec<u8> {
        // Clona el contenido del Vec<u8> en lugar del Rc
        self.output.borrow().clone()
    }
    fn get_buffer(&self) -> Rc<RefCell<Vec<u8>>> {
        self.output.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
