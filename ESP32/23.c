#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <Adafruit_MCP23X17.h>

const int I2C_SDA = 8;
const int I2C_SCL = 9;
const int numBytes = 65536;

Adafruit_SSD1306 display(128, 64, &Wire, -1);
Adafruit_MCP23X17 mcp0, mcp1, mcp2;

// Variables compartidas (volatile!)
volatile uint16_t currentAddress = 0;
volatile bool dataReady = false;
byte z80_memoria[numBytes];
int ciclos = 0;

// Pines de control (ajusta según tu hardware)
#define CLOCK_PIN 9  // GPB1 en MCP2
#define RESET_PIN 8  // GPB0 en MCP2
#define RD_PIN 0     // GPA0 en MCP2
#define WR_PIN 1     // GPA1 en MCP2
#define MREQ_PIN 2   // GPA2 en MCP2
#define RFSH_PIN 5   // GPA5 en MCP2


// Temporización
const unsigned long CLOCK_DELAY = 100;  // 10µs ~ 50 kHz
void imprime(const char* txt) {
  Serial.println(txt);
  //display.clearDisplay();
  //display.setCursor(0,0);
  display.println(txt);
  display.display();
}
void setup() {
  Serial.begin(115200);
  Wire.begin(I2C_SDA, I2C_SCL);

  // Inicializar OLED
  if (!display.begin(SSD1306_SWITCHCAPVCC, 0x3C))
    //if (!display.begin(SSD1306_SWITCHCAPVCC, 0x27))
    while (1)
      ;
  display.setTextSize(1);
  display.setTextColor(SSD1306_WHITE);

  // Configurar MCP23017
  mcp0.begin_I2C(0x20);
  mcp1.begin_I2C(0x21);
  mcp2.begin_I2C(0x22);

  // Configurar pines
  for (int i = 0; i < 16; i++) mcp0.pinMode(i, INPUT);   // Direcciones con pull-up
  for (int i = 8; i < 16; i++) mcp1.pinMode(i, OUTPUT);  // Datos como salida inicial

  mcp2.pinMode(RESET_PIN, OUTPUT);
  mcp2.pinMode(CLOCK_PIN, OUTPUT);
  mcp2.pinMode(RD_PIN, INPUT);
  mcp2.pinMode(WR_PIN, INPUT);
  mcp2.pinMode(RFSH_PIN, INPUT);


  // Reset inicial
  // mcp2.digitalWrite(RESET_PIN, LOW);
  // delayMicroseconds(1000);
  // mcp2.digitalWrite(RESET_PIN, HIGH);
  // delayMicroseconds(1000);


  int resetticks = 0;
  mcp2.digitalWrite(RESET_PIN, LOW);  // Reset debe estar al menos 3 clocks
  while (resetticks < 3) {
    resetticks++;
    mcp2.digitalWrite(CLOCK_PIN, LOW);
    delayMicroseconds(100);
    mcp2.digitalWrite(CLOCK_PIN, HIGH);
    delayMicroseconds(100);
  }
  mcp2.digitalWrite(RESET_PIN, HIGH);

  // Inicializar memoria
  memset(z80_memoria, 0, numBytes);
  z80_memoria[0x0000] = 0xF3;  //         DI
  z80_memoria[0x0001] = 0xC3;  //         JP 0x0104
  z80_memoria[0x0002] = 0x04;
  z80_memoria[0x0003] = 0x01;
  z80_memoria[0x0104] = 0x3E;  // 0x0032  LD A,0x01
  z80_memoria[0x0105] = 0x01;
  z80_memoria[0x0106] = 0xC3;  //         JP 0x0104
  z80_memoria[0x0107] = 0x04;
  z80_memoria[0x0108] = 0x01;
}

void loop() {
  // Generar pulso de reloj manual (como en el código original)
  mcp2.digitalWrite(CLOCK_PIN, HIGH);
  delayMicroseconds(CLOCK_DELAY);

  // Leer dirección en flanco de subida
  //currentAddress = (mcp0.readGPIOA() << 8) | mcp0.readGPIOB();

  delayMicroseconds(100);
  // *************** Gestionar bus de datos *********************
  // RD && MREQ && no hay RFSH     Z80 leyendo
  if ((mcp2.digitalRead(RD_PIN) == LOW) & (mcp2.digitalRead(MREQ_PIN) == LOW) & (mcp2.digitalRead(RFSH_PIN) == HIGH)) {
    // Leer dirección en flanco de subida
    currentAddress = (mcp0.readGPIOA() << 8) | mcp0.readGPIOB();
    mcp1.writeGPIOB(z80_memoria[currentAddress]);
    //mcp1.writeGPIOB(z80_memoria[currentAddress]);
    //mcp1.writeGPIOB(z80_memoria[currentAddress]);
    //mcp1.writeGPIOB(z80_memoria[currentAddress]);
    //delayMicroseconds(10000);
  } else if (mcp2.digitalRead(WR_PIN) == LOW) {  // Z80 escribiendo
    z80_memoria[currentAddress] = mcp1.readGPIOB();
  }
  delayMicroseconds(100);
  mcp2.digitalWrite(CLOCK_PIN, LOW);
  delayMicroseconds(CLOCK_DELAY);
  ciclos++;

  // Actualizar pantalla cada 100 ciclos
  //if (ciclos % 100 == 0) {
  display.clearDisplay();
  display.setCursor(0, 0);
  char msg1[80];
  snprintf(msg1, sizeof(msg1), "Direccion: 0x%04X\nDatos: 0x%02X\nCiclos: %d\n",
           currentAddress, z80_memoria[currentAddress], ciclos);

  imprime(msg1);

  display.display();
  //}

  // Manejar reset por serial
  if (Serial.available() && Serial.read() == 'r') {
    char msg3[80];
    snprintf(msg3, sizeof(msg3), "RESET **********\n");
    imprime(msg3);

    // mcp2.digitalWrite(RESET_PIN, LOW);
    // delay(100);
    // mcp2.digitalWrite(RESET_PIN, HIGH);

    int resetticks = 0;
    mcp2.digitalWrite(RESET_PIN, LOW);  // Reset debe estar al menos 3 clocks
    while (resetticks < 3) {
      resetticks++;
      mcp2.digitalWrite(CLOCK_PIN, LOW);
      delayMicroseconds(1000);
      mcp2.digitalWrite(CLOCK_PIN, HIGH);
      delayMicroseconds(1000);
    }
    mcp2.digitalWrite(RESET_PIN, HIGH);
  }
}