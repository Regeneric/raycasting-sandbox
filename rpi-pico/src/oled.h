//  ​Universal Board Computer for cars with electronic MPI
//  Copyright © 2021-2023 IT Crowd, Hubert "hkk" Batkiewicz
// 
//  This file is part of UBC.
//  UBC is free software: you can redistribute it and/or modify
//  ​it under the terms of the GNU Affero General Public License as
//  published by the Free Software Foundation, either version 3 of the
//  ​License, or (at your option) any later version.
// 
//  ​This program is distributed in the hope that it will be useful,
//  ​but WITHOUT ANY WARRANTY; without even the implied warranty of
//  ​MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. 
//  See the ​GNU Affero General Public License for more details.
// 
//  ​You should have received a copy of the GNU Affero General Public License
//  ​along with this program.  If not, see <https://www.gnu.org/licenses/>

// <https://itcrowd.net.pl/>


#ifndef OLED_H
#define OLED_H

#include "hardware/spi.h"

#define USE_SPI     0       // 0 - SPI off ; 1 - SPI on
#define SPI_SPEED   4       // MHz

#define USE_I2C     1       // 0 - I2C off ; 1 - SPI on
#define I2C_SPEED   3000    // KHz, around 3.0 Mb/s - maximum speed of I2C standard is ~3200 KHz

#define USE_SSD1327     1
#define USE_SSD1351     0

#if USE_SSD1327 == 1
    #define ADDRESS               0x3D  // 0x3C or 0x3D

    #define WIDTH                 128
    #define HEIGHT                128

    #define SET_COL_ADDR          0x15
    #define SET_SCROLL_DEACTIVATE 0x2E
    #define SET_ROW_ADDR          0x75
    #define SET_CONTRAST          0x81
    #define SET_SEG_REMAP         0xA0
    #define SET_DISP_START_LINE   0xA1
    #define SET_DISP_OFFSET       0xA2
    #define SET_DISP_MODE         0xA4
    #define SET_MUX_RATIO         0xA8
    #define SET_FN_SELECT_A       0xAB
    #define SET_DISP              0xAE 
    #define SET_PHASE_LEN         0xB1
    #define SET_DISP_CLK_DIV      0xB3
    #define SET_SECOND_PRECHARGE  0xB6
    #define SET_GRAYSCALE_TABLE   0xB8
    #define SET_GRAYSCALE_LINEAR  0xB9
    #define SET_PRECHARGE         0xBC
    #define SET_VCOM_DESEL        0xBE
    #define SET_FN_SELECT_B       0xD5
    #define SET_COMMAND_LOCK      0xFD

    #define OLED_ALL_ON  0x02
    #define OLED_ON      0x01
    #define OLED_OFF     0x00

    #define VDD_ON       0x01
    #define VDD_OFF      0x00

    #define REG_CMD      0x80
    #define REG_DATA     0x40
#endif  // USE_SSD1327


void screenTest();
void initOLED();

void powerOn();
void powerOff();

void displayOn();
void displayOff();

void display();
void brightness(byte brigthness); // 0-255
void contrast(byte contrast);     // 0-15
void refresh(byte refresh);       // 0-15
void invert(byte invert);         // 0-1
void clear(byte color);

void setPixel(byte x, byte y, byte color);
void fillRect(byte x, byte y, byte w, byte h, byte color);

void drawLine(word x0, word y0, word x1, word y1, byte color);

void drawHLine(word x, word y, word w, byte color);
void drawFastHLine(word x, word y, word w, byte color);

void drawVLine(word x, word y, word h, byte color);
void drawFastVLine(word x, word y, word h, byte color);


void charSize(byte size);
void charColor(byte color);
void putChar(byte x, byte y, char c, byte color, byte bg, byte size);

void sendc(byte x, byte y, char c);
void sends(byte x, byte y, char *str);
void sendi(byte x, byte y, int num);
void sendf(byte x, byte y, float num, byte precision);


struct OLED {
    void (*powerOn)(void);
    void (*powerOff)(void);

    void (*on)(void);
    void (*off)(void);

    void (*init)(void);
    void (*display)(void);
    void (*clear)(byte color);
    void (*invert)(byte invert);
    void (*refresh)(byte refresh);
    void (*contrast)(byte contrast);
    void (*brightness)(byte brightness);

    void (*pixel)(byte x, byte y, byte color);
    void (*rect)(byte x, byte y, byte w, byte h, byte color);
    void (*line)(word x0, word y0, word x1, word y1, byte color);
    void (*hline)(word x, word y, word w, byte color);
    void (*fhline)(word x, word y, word w, byte color);
    void (*vline)(word x, word y, word h, byte color);
    void (*fvline)(word x, word y, word h, byte color);

    void (*font)(byte size);
    void (*color)(byte color);
    void (*cursor)(byte x,  byte y);

    void (*sendc)(byte x, byte y, char c);
    void (*sends)(byte x, byte y, char *str);
    void (*sendi)(byte x, byte y, int num);
    void (*sendf)(byte x, byte y, float nums, byte precision);
}; extern const struct OLED oled;

#endif  // OLED_H