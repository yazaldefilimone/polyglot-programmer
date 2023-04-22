function fatorial(n) {
  if (n === 0 || n === 1) {
    return 1;
  } else {
    return n * fatorial(n - 1);
  }
}

console.log(fatorial(5)); // Saída: 120

// Se n é par, x^n = (x^(n/2))^2
// Se n é ímpar, x^n = x * (x^((n-1)/2))^2
function multiplication(num, times) {
  if (times <= 1) {
    return num;
  }
  if (times % 2 === 0) {
    const result = multiplication(num, times / 2);
    return result + result;
  } else {
    const result = multiplication(num, (times - 1) / 2);
    return result + result + num;
  }
}

console.log(multiplication(12, 6));

function teste(...args) {
  console.log(args);
}

teste("2", 3, 5, 6);
