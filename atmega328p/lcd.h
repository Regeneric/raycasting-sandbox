//  ​Universal Board Computer for cars with electronic MPI
//  Copyright © 2015, 2021-2022 IT Crowd, Hubert "hkk" Batkiewicz; 
//  Sergey Denisov aka LittleBuster (DenisovS21@gmail.com)
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
// <https://github.com/LittleBuster>


#ifndef LCD_H
#define LCD_H

#include <avr/pgmspace.h>
#include <stdint.h>

// LCD's port
#define PORT_LCD PORTB
#define DDR_LCD  DDRB

// LCD's pins
#define LCD_RST PB5  // RST
#define LCD_SCE PB4  // CS
#define LCD_DC  PB3  // D/C
#define LCD_DIN PB2  // DIN
#define LCD_CLK PB1  // CLK

#define LCD_CONTRAST 0x40


#define WIDTH  84
#define HEIGHT 48


struct lcdInterface {
    void (*init)(void);
    void (*clear)(void);                                          //__attribute__((optimize("-O3")));
    void (*cursor)(uint8_t xPos, uint8_t yPos);
    void (*render)(void);                                         //__attribute__((optimize("-O3")));
    void (*line)(int16_t x0, int16_t y0, int16_t x1, int16_t y1); //__attribute__((optimize("-O3")));
    void (*lineH)(uint8_t x, uint8_t y, uint8_t w);
    void (*lineV)(uint8_t x, uint8_t y, uint8_t h);
    void (*pixel)(uint8_t x, uint8_t y, uint8_t value);          //__attribute__((optimize("-O3")));
    void (*rect)(int16_t x, int16_t y, int16_t w, int16_t h);    //__attribute__((optimize("-O3")));
}; extern const struct lcdInterface LCD;

#endif  // LCD_H