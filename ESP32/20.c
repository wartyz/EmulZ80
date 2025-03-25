#include <DFRobot_MCP23017.h>
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>

const int I2C_SDA = 8;
const int I2C_SCL = 9;

const int numBytes = 65536;
const int dly = 1;


// parametros OLED
#define SCREEN_WIDTH 128  // ancho en pixels del display OLED
#define SCREEN_HEIGHT 64  // alto en pixels del display OLED
#define OLED_RESET -1     // pin de reset (o -1 si se comparte pin de Arduino)
//#define SCREEN_ADDRESS 0x3C  // Direccion I2C
#define ROTATION 0  // Rota texto en OLED 1=90 grados, 2=180 grados

Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);
DFRobot_MCP23017 mcp0(Wire, 0x20);
DFRobot_MCP23017 mcp1(Wire, 0x21);
DFRobot_MCP23017 mcp2(Wire, 0x22);

//const byte DATA_EXPANDER_ADDR = 0x3C;

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
byte direcc[16];
byte datos[8];
int relojadas = 0;


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
}

void imprime(const char* txt) {
  Serial.println(txt);
  //display.clearDisplay();
  //display.setCursor(0,0);
  display.println(txt);
  display.display();
}
void resetRam() {
  int x = 0;

  while (x < numBytes) {
    z80_memoria[x] = 0;
    x++;
  }
}

void progEjemplo() {
  z80_memoria[0] = 0xC3;  // jmp 16
  z80_memoria[1] = 0x10;
  z80_memoria[2] = 0x00;

  z80_memoria[16] = 0x3E;  //Draw a circle in bits... i.e. hello world
  z80_memoria[17] = 0x18;
  z80_memoria[18] = 0x21;
  z80_memoria[19] = 0x03;
  z80_memoria[20] = 0x00;
  z80_memoria[21] = 0x77;
  z80_memoria[22] = 0x21;
  z80_memoria[23] = 0x08;
  z80_memoria[24] = 0x00;
  z80_memoria[25] = 0x77;

  z80_memoria[26] = 0x3E;
  z80_memoria[27] = 0x24;
  z80_memoria[28] = 0x21;
  z80_memoria[29] = 0x04;
  z80_memoria[30] = 0x00;
  z80_memoria[31] = 0x77;
  z80_memoria[32] = 0x21;
  z80_memoria[33] = 0x07;
  z80_memoria[34] = 0x00;
  z80_memoria[35] = 0x77;

  z80_memoria[36] = 0x3E;
  z80_memoria[37] = 0x42;
  z80_memoria[38] = 0x21;
  z80_memoria[39] = 0x05;
  z80_memoria[40] = 0x00;
  z80_memoria[41] = 0x77;
  z80_memoria[42] = 0x21;
  z80_memoria[43] = 0x06;
  z80_memoria[44] = 0x00;
  z80_memoria[45] = 0x77;

  z80_memoria[46] = 0xC3;
  z80_memoria[47] = 0x00;
  z80_memoria[49] = 0x00;  // jump back to the start
}

void escaneaI2C() {
  byte direccion = 0;
  byte count = 0;
  // Solo escaneo direcciones de expansores en I2C
  for (byte address = 0x20; address < 0x28; address++) {
    Wire.beginTransmission(address);
    byte error = Wire.endTransmission();

    if (error == 0) {
      char msg[50];
      snprintf(msg, sizeof(msg), "Expander en 0x%02X", address);
      imprime(msg);
      count++;
    }
  }

  if (count == 0) {
    imprime("No se encontraron dispositivos");
  }

  imprime("Escaneo completado");
}
void setup() {
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
  mcp0.pinMode(mcp0.eGPA, INPUT);
  mcp0.pinMode(mcp0.eGPB, INPUT);
  // Bus de datos bidireccional
  //mcp1.pinMode(mcp1.eGPB, INPUT_PULLUP);
  mcp1.pinMode(mcp1.eGPB, OUTPUT);
  // bus de control parte de entrada
  mcp2.pinMode(mcp2.eGPA, INPUT);
  // bus de control parte de salida
  mcp2.pinMode(mcp2.eGPB, OUTPUT);

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

  Wire.begin(I2C_SDA, I2C_SCL);
  escaneaI2C();
  // Envia un reset (primero un clock)
  int r;
  //mcp2.digitalWrite(mcp2.eGPB1, HIGH);  // CLOCK
  //delay(dly);
  //for (r = 0; r < 5; r++) {
  mcp2.digitalWrite(mcp2.eGPB0, LOW);  // RESET
  //}
  delay(dly);
  mcp2.digitalWrite(mcp2.eGPB0, HIGH);  // RESET

  mcp2.digitalWrite(mcp2.eGPB1, LOW);  // CLOCK
  delay(dly);
}

void loop() {

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

  poneDato();
  delay(dly);
  // Envia 4 clocks
  for (int ciclo = 0; ciclo < 1000; ciclo++) {
    mcp2.digitalWrite(mcp2.eGPB1, HIGH);
    //delay(dly);
    mcp2.digitalWrite(mcp2.eGPB1, LOW);
    //delay(dly);
  }
  char msg[50];
  snprintf(msg, sizeof(msg), "Pasado el bucle %d", relojadas);
  imprime(msg);
  relojadas++;
  display.clearDisplay();
  display.setCursor(0,0);
}

void poneDato() {
  mcp1.digitalWrite(mcp1.eGPB0, LOW);
  mcp1.digitalWrite(mcp1.eGPB1, LOW);
  mcp1.digitalWrite(mcp1.eGPB2, LOW);
  mcp1.digitalWrite(mcp1.eGPB3, LOW);
  mcp1.digitalWrite(mcp1.eGPB4, LOW);
  mcp1.digitalWrite(mcp1.eGPB5, LOW);
  mcp1.digitalWrite(mcp1.eGPB6, LOW);
  mcp1.digitalWrite(mcp1.eGPB7, LOW);
}

void handleLoadCommand() {
  Serial.println("Carga iniciada...");
  uint32_t address = 0;

  while (address < sizeof(z80_memoria)) {
    if (Serial.available()) {
      z80_memoria[address++] = Serial.read();
    }
    if (millis() % 1000 == 0) {  // Feedback cada 1s
      Serial.print("Progreso: ");
      Serial.println(address);
    }
  }
  Serial.println("OK_LOADED");
}

void handleRunCommand() {
  Serial.println("Z80 en ejecucion...");
  delay(1000);  // Simulación de ejecución

  // Generar datos de prueba
  uint8_t registers[] = { 0xA1, 0xB2, 0xC3, 0xD4, 0xE5, 0xF6, 0x01, 0x12, 0x23, 0x34 };
  Serial.write(registers, sizeof(registers));
  Serial.println("DATOS_ENVIADOS");
}
