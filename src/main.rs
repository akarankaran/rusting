let a = 10;
let b = 5;

// Addition
let sum1 = a + b;
let sum2 = b + a;
let sum3 = a + a;

// Subtraction
let diff1 = a - b;
let diff2 = b - a;
let diff3 = a - a;

// Multiplication
let prod1 = a * b;
let prod2 = b * a;
let prod3 = a * a;

// Division
let div1 = a / b;
let div2 = b / a; // This will be 0 as b < a
let div3 = a / 1; // Should return a
let div4 = a / a; // Should return 1

// Mixed operations
let mixed1 = (a + b) * a;
let mixed2 = a - (b * 2);
let mixed3 = (a / b) + b;
let mixed4 = (a + b) - (a * b);

// Negative values
let c = -3;
let d = -7;
let neg_sum = c + d;
let neg_diff = c - d;
let neg_prod = c * d;
let neg_div = c / d;

// Large numbers
let large1 = 10000;
let large2 = 20000;
let large_sum = large1 + large2;
let large_diff = large1 - large2;
let large_prod = large1 * large2;
let large_div = large1 / large2;

// Zero operations
let zero_add = 0 + a;
let zero_mul = 0 * a;
let zero_sub = a - 0;
let zero_div = a / 1;  // Division by 0 is not permitted

[sum1, sum2, sum3, diff1, diff2, diff3, prod1, prod2, prod3, div1, div2, div3, div4, mixed1, mixed2, mixed3, mixed4, neg_sum, neg_diff, neg_prod, neg_div, large_sum, large_diff, large_prod, large_div, zero_add, zero_mul, zero_sub, zero_div]