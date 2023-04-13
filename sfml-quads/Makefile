FCC = gfortran
CC  = g++

CFLAGS = -pg -g3 -lX11 -lGL -lpthread -lpng -lstdc++fs -std=c++20 -lsfml-graphics -lsfml-window -lsfml-system  


test: app
	./app

cpp: *.cpp SelbaWard/*.cpp
	$(CC) -c *.cpp -c SelbaWard/Line.cpp $(CFLAGS)

app: *.o
	$(CC) *.o $(CFLAGS) -o app


main: main.cpp
	$(CC) -c main.cpp $(CFLAGS)

player: Player.cpp
	$(CC) -c Player.cpp $(CFLAGS)

ray: Ray.cpp
	$(CC) -c Ray.cpp $(CFLAGS)

wall: Wall.cpp
	$(CC) -c Wall.cpp $(CFLAGS)

pixel: Pixel.cpp
	$(CC) -c Pixle.cpp $(CFLAGS)

line: SelbaWard/Line.cpp
	$(CC) -c SelbaWard/Line.cpp $(CFLAGS)


clean:
	rm *.o
	rm app
