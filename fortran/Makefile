FCC = gfortran
CC  = g++

CFLAGS = -pg -g3 -lX11 -lGL -lpthread -lpng -lstdc++fs -std=c++20 -lsfml-graphics -lsfml-window -lsfml-system -lgfortran  

test: app
	./app

cpp: *.cpp SelbaWard/*.cpp
	$(CC) -c *.cpp $(CFLAGS)

fortran: *.f90
	$(FCC) -c *.f90

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

line: SelbaWard/Line.cpp
	$(CC) -c SelbaWard/Line.cpp $(CFLAGS)


clean:
	rm -rf *.o
	rm -rf *.out
	rm -rf app
