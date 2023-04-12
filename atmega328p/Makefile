TARGET = atmega328p
PROGM = usbasp
PROGM_UC = m328p

CC = avr-g++
CFLAGS = -fshort-enums -ffunction-sections -funsigned-char -lm -lstdc++ -I /home/hkk/Documents/cpp/raycasting/atmega328p/avr-libstdcpp/include -Os -w -ffreestanding -DF_CPU=20000000UL -mmcu=$(TARGET) -fno-rtti

all: main.o lcd.o app ./build/app.bin
	avr-objcopy -O ihex -R .eeprom ./build/app.bin ./rel/app.hex

flash: all
	sudo avrdude -c ${PROGM} -p ${PROGM_UC} -U flash:w:rel/app.hex


main.o: main.cpp
	$(CC) $(CFLAGS) -c -o ./build/main.o main.cpp

lcd.o: ./lcd.cpp ./lcd.h
	$(CC) $(CFLAGS) -c -o ./build/lcd.o ./lcd.cpp


app: main.o lcd.o
	$(CC) -mmcu=$(TARGET) ./build/main.o ./build/lcd.o -o ./build/app.bin


clean:
	rm -rf ./build/*
