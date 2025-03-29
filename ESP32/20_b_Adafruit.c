//#include <DFRobot_MCP23017.h>
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <Adafruit_MCP23X17.h>

const int I2C_SDA = 8;
const int I2C_SCL = 9;

const int numBytes = 65536;
const int dly = 1;

// Actualizar OLED solo cada N ciclos
static int contador = 0;
static uint16_t direccion = 0;



// parametros OLED
#define SCREEN_WIDTH 128  // ancho en pixels del display OLED
#define SCREEN_HEIGHT 64  // alto en pixels del display OLED
#define OLED_RESET -1     // pin de reset (o -1 si se comparte pin de Arduino)

Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);
Adafruit_MCP23X17 mcp0;  // Bus de direcciones (A0-A15)
Adafruit_MCP23X17 mcp1;  // Bus de datos (D0-D7)
Adafruit_MCP23X17 mcp2;  // Señales de control

enum State {
  STATE_WAIT,
  STATE_LOAD,
  STATE_CONTROL_Z80,
  STATE_Z80_RUNNING,
  STATE_RESPONSE
};

State currentState = STATE_WAIT;
byte z80_memoria[numBytes];
byte z80_registers[10];

int ciclos = 0;


void inicializa_oled() {
  if (!display.begin(SSD1306_SWITCHCAPVCC, 0x3C)) {
    Serial.println("Error en pantalla OLED!");
    while (true)
      ;
  }
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(0, 0);
  display.display();
}

void imprime(const char* txt) {
  Serial.println(txt);
  //display.clearDisplay();
  //display.setCursor(0,0);
  display.println(txt);
  display.display();
}
void resetRam() {
  // int x = 0;

  // while (x < numBytes) {
  //   z80_memoria[x] = 0;
  //   x++;
  // }
  memset(z80_memoria, 0, numBytes);
}

void progEjemplo() {
  // Programa de prueba (Z80 NOP loop)
  z80_memoria[0x0000] = 0x3E;  // LD A,0x01
  z80_memoria[0x0001] = 0x01;
  z80_memoria[0x0002] = 0x32;  // LD (0x4000),A
  z80_memoria[0x0003] = 0x00;
  z80_memoria[0x0004] = 0x40;
  z80_memoria[0x0005] = 0xC3;  // JP 0x0000
  z80_memoria[0x0006] = 0x00;
  z80_memoria[0x0007] = 0x00;
}
void inicializa_mcps() {
  // Inicializar MCPs
  if (!mcp0.begin_I2C(0x20)) {
    imprime("Error MCP0!");
    while (1)
      ;
  }
  if (!mcp1.begin_I2C(0x21)) {
    imprime("Error MCP1!");
    while (1)
      ;
  }
  if (!mcp2.begin_I2C(0x22)) {
    imprime("Error MCP2!");
    while (1)
      ;
  }

  // Configurar MCP0 (bus de direcciones) como ENTRADAS
  for (int pin = 0; pin < 16; pin++) {
    mcp0.pinMode(pin, INPUT);
    //mcp0.pinMode(pin, INPUT_PULLUP);  // <--- Activar pull-up interno
  }

  // Configurar MCP1 (Bus de datos)
  for (int pin = 0; pin < 8; pin++) {
    mcp1.pinMode(pin, OUTPUT);
  }

  // Configurar MCP2 (Control)
  mcp2.pinMode(8, OUTPUT);   // RESET (GPB0)
  mcp2.pinMode(9, OUTPUT);   // CLOCK (GPB1)
  //mcp2.pinMode(10, OUTPUT);  // WAIT (GPB2)
  mcp2.pinMode(0, INPUT);    // MREQ (GPA0)
  mcp2.pinMode(1, INPUT);    // IORQ (GPA1)
  mcp2.pinMode(2, INPUT);    // RD (GPA2)
  mcp2.pinMode(3, INPUT);    // WR (GPA3)
}

uint16_t leeBusDirecciones() {


  //delayMicroseconds(200);  // Espera para estabilizar
  uint16_t valor = mcp0.readGPIOAB();
  // Swap de bytes: (GPIOB << 8) | GPIOA -> (GPIOA << 8) | GPIOB


  return (valor << 8) | (valor >> 8);
}

void escribeBusDatos(uint8_t data) {
  mcp1.writeGPIOB(data);
}
// void escaneaI2C() {
//   byte direccion = 0;
//   byte count = 0;
//   // Solo escaneo direcciones de expansores en I2C
//   for (byte address = 0x20; address < 0x28; address++) {
//     Wire.beginTransmission(address);
//     byte error = Wire.endTransmission();

//     if (error == 0) {
//       char msg[50];
//       snprintf(msg, sizeof(msg), "Expander en 0x%02X", address);
//       imprime(msg);
//       count++;
//     }
//   }

//   if (count == 0) {
//     imprime("No se encontraron dispositivos");
//   }

//   imprime("Escaneo completado");
// }
void setup() {
  Serial.begin(115200);
  Wire.begin(I2C_SDA, I2C_SCL);

  resetRam();
  // si hay un fichero de entrada ponerlo aqui en la memoria
  // por ahora pongo un programa basico
  progEjemplo();

  // Serial.begin(115200);
  // while (!Serial)
  //   ;  // Esperar a que se inicialice el puerto
  // Serial.setTimeout(1000);

  inicializa_oled();

  //Serial.println("ESP32 - Controlador Z80 \nIniciando escaneo I2C...");
  //display.println("Escaneo I2C:");
  //display.display();
  //imprime("Escaneo I2C");

  // Bus de direcciones solo entrada
  //mcp0.pinMode(mcp0.eGPA, INPUT);
  //mcp0.pinMode(mcp0.eGPB, INPUT);
  // Bus de datos bidireccional
  //mcp1.pinMode(mcp1.eGPB, INPUT_PULLUP);
  //mcp1.pinMode(mcp1.eGPB, OUTPUT);
  // bus de control parte de entrada
  //mcp2.pinMode(mcp2.eGPA, INPUT);
  // bus de control parte de salida
  //mcp2.pinMode(mcp2.eGPB, OUTPUT);

  inicializa_mcps();

  // byte direccion = 0;
  // byte count = 0;
  // // Solo escaneo direcciones de expansores en I2C
  // for (byte address = 0x20; address < 0x28; address++) {
  //   Wire.beginTransmission(address);
  //   byte error = Wire.endTransmission();

  //   if (error == 0) {
  //     char msg[50];
  //     snprintf(msg, sizeof(msg), "Expander en 0x%02X", address);
  //     //expanders[0] = address;
  //     imprime(msg);
  //     count++;
  //   }
  // }

  // if (count == 0) {
  //   imprime("No se encontraron dispositivos");
  // }

  // imprime("Escaneo completado");

  //Wire.begin(I2C_SDA, I2C_SCL);
  //escaneaI2C();
  // Envia un reset (primero un clock)
  //int r;
  //mcp2.digitalWrite(mcp2.eGPB1, HIGH);  // CLOCK
  //delay(dly);
  //for (r = 0; r < 5; r++) {
  //mcp2.digitalWrite(mcp2.eGPB0, LOW);  // RESET
  //}
  //delay(dly);
  //mcp2.digitalWrite(mcp2.eGPB0, HIGH);  // RESET

  //mcp2.digitalWrite(mcp2.eGPB1, LOW);  // CLOCK
  //delay(dly);

  // Secuencia de reset CORREGIDA (usando pin 8)
  mcp2.digitalWrite(8, LOW);  // RESET LOW
  delay(100);
  mcp2.digitalWrite(8, HIGH);  // RESET HIGH
  delay(100);

  imprime("Sistema listo");
}

//void loop() {

// if (Serial.available() > 0) {
//   String command = Serial.readStringUntil('\n');
//   command.trim();

//   if (command.startsWith("LOAD_FILE")) {
//     handleLoadCommand();
//   } else if (command == "RUN_Z80") {
//     handleRunCommand();
//   }
// }
// mcp0.digitalWrite(mcp0.eGPB3, HIGH);
// mcp1.digitalWrite(mcp1.eGPB3, HIGH);
// mcp2.digitalWrite(mcp2.eGPA1, HIGH);

// delay(1000);
// mcp0.digitalWrite(mcp0.eGPB3, LOW);
// mcp1.digitalWrite(mcp1.eGPB3, LOW);
// mcp2.digitalWrite(mcp2.eGPA1, LOW);

// poneDato();
// delay(dly);
// // Envia 4 clocks
// for (int ciclo = 0; ciclo < 1000; ciclo++) {
//   mcp2.digitalWrite(mcp2.eGPB1, HIGH);
//   //delay(dly);
//   mcp2.digitalWrite(mcp2.eGPB1, LOW);
//   //delay(dly);
// }
// char msg0[50];
// snprintf(msg0, sizeof(msg0), "Pasado el bucle %d", relojadas);
// imprime(msg0);

// char msg1[50];
// snprintf(msg1, sizeof(msg1), "B Direcc = 0x%04X", leeBusDirecciones());
// imprime(msg1);

// relojadas++;
// display.clearDisplay();
// display.setCursor(0, 0);
//}
void loop() {
  // Un NOP tiene 4 ciclos
  for (int n = 0; n < 3; n++) {
    mcp2.digitalWrite(9, HIGH);
    delayMicroseconds(1);
    mcp2.digitalWrite(9, LOW);
    delayMicroseconds(1);
  }

  // Generar pulso de reloj en pin 9 (GPB1)
  mcp2.digitalWrite(9, HIGH);
  delayMicroseconds(1);
  //if (mcp2.digitalRead(2)) {
  // Leer dirección en el flanco de subida (bus estable)
  //uint16_t direccion = leeBusDirecciones();
  direccion = leeBusDirecciones();
  //}
  mcp2.digitalWrite(9, LOW);  // Flanco de bajada
  delayMicroseconds(1);

  // // Actualizar OLED solo cada N ciclos
  // static int contador = 0;


  // Leer bus de direcciones cada 100 ciclos
  if (contador++ >= 100) {
    contador = 0;

    char msg1[40];
    snprintf(msg1, sizeof(msg1), "Addr: 0x%04X", direccion);
    imprime(msg1);
    char msg2[40];
    snprintf(msg2, sizeof(msg2), "Ciclos: %d", ciclos);
    imprime(msg2);


    display.display();
  }

  delayMicroseconds(10);  // Frecuencia ~50 kHz
  ciclos++;
  display.clearDisplay();
  display.setCursor(0, 0);
}
// void poneDato() {
//   mcp1.digitalWrite(mcp1.eGPB0, LOW);
//   mcp1.digitalWrite(mcp1.eGPB1, LOW);
//   mcp1.digitalWrite(mcp1.eGPB2, LOW);
//   mcp1.digitalWrite(mcp1.eGPB3, LOW);
//   mcp1.digitalWrite(mcp1.eGPB4, LOW);
//   mcp1.digitalWrite(mcp1.eGPB5, LOW);
//   mcp1.digitalWrite(mcp1.eGPB6, LOW);
//   mcp1.digitalWrite(mcp1.eGPB7, LOW);
// }


// int16_t leeBusDirecciones() {
//   // Leer cada pin del MCP23017 (0x20) y combinar en un entero de 16 bits
//   int16_t direccion = 0;

//   // Puerto B (bits 0-7)
//   direccion |= mcp0.digitalRead(mcp0.eGPB0);
//   direccion |= mcp0.digitalRead(mcp0.eGPB1) << 1;
//   direccion |= mcp0.digitalRead(mcp0.eGPB2) << 2;
//   direccion |= mcp0.digitalRead(mcp0.eGPB3) << 3;
//   direccion |= mcp0.digitalRead(mcp0.eGPB4) << 4;
//   direccion |= mcp0.digitalRead(mcp0.eGPB5) << 5;
//   direccion |= mcp0.digitalRead(mcp0.eGPB6) << 6;
//   direccion |= mcp0.digitalRead(mcp0.eGPB7) << 7;

//   // Puerto A (bits 8-15)
//   direccion |= mcp0.digitalRead(mcp0.eGPA0) << 8;
//   direccion |= mcp0.digitalRead(mcp0.eGPA1) << 9;
//   direccion |= mcp0.digitalRead(mcp0.eGPA2) << 10;
//   direccion |= mcp0.digitalRead(mcp0.eGPA3) << 11;
//   direccion |= mcp0.digitalRead(mcp0.eGPA4) << 12;
//   direccion |= mcp0.digitalRead(mcp0.eGPA5) << 13;
//   direccion |= mcp0.digitalRead(mcp0.eGPA6) << 14;
//   direccion |= mcp0.digitalRead(mcp0.eGPA7) << 15;

//   return direccion;
// }
// int16_t leeBusDirecciones() {
//   mcp2.digitalWrite(mcp2.eGPA2, LOW);  // WAIT

//   int16_t d0 = mcp0.digitalRead(mcp0.eGPB0);
//   int16_t d1 = mcp0.digitalRead(mcp0.eGPB1) << 1;
//   int16_t d2 = mcp0.digitalRead(mcp0.eGPB2) << 2;
//   int16_t d3 = mcp0.digitalRead(mcp0.eGPB3) << 3;
//   int16_t d4 = mcp0.digitalRead(mcp0.eGPB4) << 4;
//   int16_t d5 = mcp0.digitalRead(mcp0.eGPB5) << 5;
//   int16_t d6 = mcp0.digitalRead(mcp0.eGPB6) << 6;
//   int16_t d7 = mcp0.digitalRead(mcp0.eGPB7) << 7;
//   int16_t d8 = mcp0.digitalRead(mcp0.eGPA0) << 8;
//   int16_t d9 = mcp0.digitalRead(mcp0.eGPA1) << 9;
//   int16_t d10 = mcp0.digitalRead(mcp0.eGPA2) << 10;
//   int16_t d11 = mcp0.digitalRead(mcp0.eGPA3) << 11;
//   int16_t d12 = mcp0.digitalRead(mcp0.eGPA4) << 12;
//   int16_t d13 = mcp0.digitalRead(mcp0.eGPA5) << 13;
//   int16_t d14 = mcp0.digitalRead(mcp0.eGPA6) << 14;
//   int16_t d15 = mcp0.digitalRead(mcp0.eGPA7) << 15;

//   mcp2.digitalWrite(mcp2.eGPA2, HIGH);  // WAIT


//   int16_t r = 0;
//   r = r | d0 | d1 | d2 | d3 | d4 | d5 | d6 | d7 | d8 | d9 | d10 | d11 | d12 | d13 | d14 | d15;



//   return r;
// }

// void handleLoadCommand() {
//   Serial.println("Carga iniciada...");
//   uint32_t address = 0;

//   while (address < sizeof(z80_memoria)) {
//     if (Serial.available()) {
//       z80_memoria[address++] = Serial.read();
//     }
//     if (millis() % 1000 == 0) {  // Feedback cada 1s
//       Serial.print("Progreso: ");
//       Serial.println(address);
//     }
//   }
//   Serial.println("OK_LOADED");
// }

// void handleRunCommand() {
//   Serial.println("Z80 en ejecucion...");
//   delay(1000);  // Simulación de ejecución

//   // Generar datos de prueba
//   uint8_t registers[] = { 0xA1, 0xB2, 0xC3, 0xD4, 0xE5, 0xF6, 0x01, 0x12, 0x23, 0x34 };
//   Serial.write(registers, sizeof(registers));
//   Serial.println("DATOS_ENVIADOS");
// }
