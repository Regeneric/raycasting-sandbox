#ifndef MATH_UTILS_H
#define MATH_UTILS_H

// #include <cmath>

// #define DR 0.0174533        // One degree in radians

// // Faster than standard cosine function - to be tested
// inline float fcos(float x) {
//     float tp = 1.0f/(2.0f*PI);
//     x *= tp;
//     x -= (float)(0.25f) + floor(x + (float)(0.25f));
//     x *= (float)(16.0f) * (abs(x) - (float)(0.5f));

//     return x; 
// }

// // Faster than standard sine function - to be tested
// float fsine(float x) {
//     const float B =  4/PI;
//     const float C = -4/(PI*PI);

//     float y = B*x + C*x * abs(x);
//     return y;
// }

// #endif  // MATH_UTILS_H

#endif