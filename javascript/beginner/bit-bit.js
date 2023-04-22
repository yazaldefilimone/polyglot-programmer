// `&`(AND bit a bit)
// `|` (OR bit a bit)
// `~` (NOT bit a bit)
// `^` (XOR bit a bit)
// `<<` (Shift left)
// `>>` (Shift right)
// `>>>` (Shift right sem sinal)
let a = 5; // valor binário 0101
let b = 3; // valor binário 0011

let c = a & b; // resultado: 1 (valor binário 0001)
let d = a | b; // resultado: 7 (valor binário 0111)
let e = ~a; // resultado: -6 (valor binário 1010)
let f = a ^ b; // resultado: 6 (valor binário 0110)
let g = a << 1; // resultado: 10 (valor binário 1010)
let h = a >> 1; // resultado: 2 (valor binário 0010)

console.log({ a, b, c, d, e, f, g, h });
