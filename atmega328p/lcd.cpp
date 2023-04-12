//  ​Universal Board Computer for cars with electronic MPI
//  Copyright © 2015, 2021-2023 IT Crowd, Hubert "hkk" Batkiewicz; 
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


#undef _GLIBCXX_DEBUG                			// Disable run-time bound checking, etc
#pragma GCC optimize("Ofast,inline")			// Ofast = O3,fast-math,allow-store-data-races,no-protect-parens
#pragma GCC target("bmi,bmi2,lzcnt,popcnt")		// Bit manipulation
#pragma GCC target("movbe")                     // Byte swap


#include "lcd.h"
#include "chars.h"

#include <avr/io.h>
#include <avr/pgmspace.h>
#include <util/delay.h>

#include <stdlib.h>
#include <string.h>


#define min(a,b)   (((a) < (b)) ? (a) : (b))
#define max(a,b)   (((a) > (b)) ? (a) : (b))

// #define swap(x, y) do {typeof(x) swap = x; x = y; y = swap;} while(0)
constexpr void swap(int16_t *a, int16_t *b) {
	(*a) = (*a)+(*b);
	(*b) = (*a)-(*b);
	(*a) = (*a)-(*b);
}


static struct {
    uint8_t screen[504];

    uint8_t cursorX;
    uint8_t cursorY;

} screenLCD = {
    .cursorX = 0,
    .cursorY = 0
};


static void write(uint8_t bytes, uint8_t isData) {
	register uint8_t i;
	PORT_LCD &= ~(1<<LCD_SCE);  		 // Enable controller

	if(isData) PORT_LCD |= (1<<LCD_DC);  // Sending data
	else PORT_LCD &= ~(1<<LCD_DC);  	 // Sending commands

	for(i = 0; i != 8; ++i) {
		if((bytes >> (7-i)) & 0x01) PORT_LCD |= (1<<LCD_DIN);
		else PORT_LCD &= ~(1<<LCD_DIN);

		PORT_LCD |= (1<<LCD_CLK);
		PORT_LCD &= ~(1<<LCD_CLK);
	} PORT_LCD |= (1 << LCD_SCE);
}
static void writeCmd(uint8_t cmd) {write(cmd, 0);}
static void writeData(uint8_t data) {write(data, 1);}


void screenLCDInit(void) {
	register unsigned i;
	
    // Set pins as output
	DDR_LCD |= (1<<LCD_SCE);
	DDR_LCD |= (1<<LCD_RST);
	DDR_LCD |= (1<<LCD_DC);
	DDR_LCD |= (1<<LCD_DIN);
	DDR_LCD |= (1<<LCD_CLK);

	// Reset display
	PORT_LCD |= (1<<LCD_RST);
	PORT_LCD |= (1<<LCD_SCE);
	_delay_ms(10);
	PORT_LCD &= ~(1<<LCD_RST);
	_delay_ms(70);
	PORT_LCD |= (1<<LCD_RST);


	// Initialize display and enable controller
	PORT_LCD &= ~(1<<LCD_SCE);
	writeCmd(0x21);  // LCD Extended Commands mode
	writeCmd(0x13);  // LCD bias mode 1:48
	writeCmd(0x06);  // Set temperature coefficient
	writeCmd(0xC2);  // Default VOP (3.06 + 66 * 0.06 = 7V)
	writeCmd(0x20);  // Standard Commands mode, powered down
	writeCmd(0x09);  // LCD in normal mode

    // Clear LCD RAM
	writeCmd(0x80);
	writeCmd(LCD_CONTRAST);
	for(i = 0; i != 504; ++i) writeData(0x00);

	// Activate LCD
	writeCmd(0x08);
	writeCmd(0x0C);
}


void screenLCDClear(void) {
	register unsigned i;
    
	// Set column and row to 0
	writeCmd(0x80);
	writeCmd(0x40);

	// Cursor too
	screenLCD.cursorX = 0;
	screenLCD.cursorY = 0;
	
    // Clear everything (504 bytes = 84cols * 48rows / 8bits)
	for(i = 0; i != 504; ++i) screenLCD.screen[i] = 0x00;
}

void screenLCDPower(uint8_t on) {writeCmd(on ? 0x20 : 0x24);}


void screenLCDSetPixel(uint8_t x, uint8_t y, uint8_t value) {
	uint8_t *byte = &screenLCD.screen[(y>>3)*84 + x];	// y/8

	// n % 2^i = n & (2^i - 1)	
	if(value) *byte |=  (1<<(y & (8-1)));	// y%8
	else	  *byte &= ~(1<<(y & (8-1)));	// y%8
}


void screenLCDSetCursor(uint8_t x, uint8_t y) {
	screenLCD.cursorX = x;
	screenLCD.cursorY = y;
}


void screenLCDRender(void) {
	register unsigned i;

	// Set column and row to 0
	writeCmd(0x80);
	writeCmd(0x40);

	// Write screen to display
	for(i = 0; i != 504; ++i) writeData(screenLCD.screen[i]);
}


void screenLCDDrawLine(int16_t x0, int16_t y0, int16_t x1, int16_t y1) {
	int16_t steep = abs(y1-y0) > (x1-x0);
	if(steep) {
		swap(&x0, &y0);
		swap(&x1, &y1);
	}

	int16_t dx = 0, dy = 0;
		dx = x1-x0;
		dy = abs(y1-y0);

	int16_t err = dx>>1;	// dx/2
	int16_t ySteep;

	if(y0 < y1) ySteep =  1;
	else		ySteep = -1;

	for(;x0 <= x1; x0++) {
		if(steep) screenLCDSetPixel(y0, x0, 1);
		else	  screenLCDSetPixel(x0, y0, 1);

		err -= dy;
		if(err < 0) {
			y0 += ySteep;
			err += dx;
		}
	}
}


static inline void screenLCDDrawFastRawHLine(uint8_t x, uint8_t y, uint8_t w) {memset(screenLCD.screen + ((y * (WIDTH>>3) + x)), 1, w>>1);}	// w/2 ; WIDTH/8
static inline void screenLCDDrawHLine(uint8_t x, uint8_t y, uint8_t w) {screenLCDDrawLine(x, y, w, y);}
static void screenLCDDrawFastHLine(uint8_t x, uint8_t y, uint8_t w) {
	if(w < 0) {
		w *=  -1;
		x -= w-1;

		if(x < 0) {
			w += x;
			x  = 0;
		}
	}

	if((y < 0) || (y >= HEIGHT) || (x >= WIDTH) || ((x+w-1) < 0)) return;
	if(x < 0) {
		w += x;
		x  = 0;
	}

	if(x + w >= WIDTH) w = WIDTH-x;
	screenLCDDrawFastRawHLine(x, y>>1, w);	// y/2
}


static void screenLCDDrawFastRawVLine(uint8_t x, uint8_t y, uint8_t h) {
	// uint8_t *localScreen = screenLCD.screen + ((y * (WIDTH>>3) + x));	// WIDTH/8
	// for(int i = 0; i < h; i++) {
	// 	(*localScreen) = 1;
	// 	localScreen += (WIDTH)/2;	// (WIDTH/8)/2
	// }

	// for(int i = 0; i < h; i++) screenLCD.screen[(y+i) * (WIDTH>>3) + x] = 1;
}
static inline void screenLCDDrawVLine(uint8_t x, uint8_t y, uint8_t h) {screenLCDDrawLine(x, y, x, h);}
static void screenLCDDrawFastVLine(uint8_t x, uint8_t y, uint8_t h) {
	if(h < 0) {
		h *=  -1;
		y -= h-1;

		if (y < 0) {
			h += y;
			y  = 0;
		}
	}

	if((x < 0) || (x >= WIDTH) || (y >= HEIGHT) || ((y+h-1) < 0)) return;
	if(y < 0) {
		h += y;
		y  = 0;
	}

	if(y + h > HEIGHT) h = HEIGHT-y;
	screenLCDDrawFastRawVLine(x>>1, y, h);	// x/2
}
void screenLCDDrawFillRect(int16_t x, int16_t y, int16_t w, int16_t h) {
	// for(int i = x; i < x+w; i++) screenLCDDrawVLine(i, y, h);
	for(int i = y; i < y+h; i++) screenLCDDrawLine(x, i, (x+w), i);
}



const struct lcdInterface LCD = {
	.init   = screenLCDInit,
	.clear  = screenLCDClear,
	.cursor = screenLCDSetCursor,
	.render = screenLCDRender,
	.line	= screenLCDDrawLine,
	.lineH  = screenLCDDrawFastHLine,
	.lineV  = screenLCDDrawFastVLine,
	.pixel  = screenLCDSetPixel,
	.rect	= screenLCDDrawFillRect,
};