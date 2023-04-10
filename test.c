#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>

#define PI 3.1415926535

__int32_t bitDiv(float a, float b) {
    __int32_t q, r, t;
    int bl = __CHAR_BIT__ * sizeof(float);

    q = a;
    r= 0;
    do {
        t = q;
        q = q+q;
        r = r+r + (q<t);

        if(r >= b) {
            r = r-b;
            q = q+1;
        } bl--;
    } while(bl);
    return q;
}


int main() {
    float ry = 3.10f;
    float py = 6.3123121258;

    float a = (int)py/6;
    float b = bitDiv((int)py, 6);

    a *= 6;
    b *= 6;

    a -= -0.001;
    b -= -0.001;

    printf("%f \n", a);
    printf("%f \n", b);

    return 0;
}