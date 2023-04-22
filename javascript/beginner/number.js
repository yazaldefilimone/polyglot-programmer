let num = 2004;
let num2 = "2.004";

// console.log(typeof num);
// console.log(num.toFixed(1));
// console.log();

function compareTwoNum(num, num2, type) {
  console.log({ num, num2 });
  if (type === 1) {
    return num == num2;
  }
  return num === num2;
}

// console.log(compareTwoNum(1.4, Number.parseFloat("1.4"), 1));
// console.log(compareTwoNum(num, Number.parseInt("2004"), 1));

// console.log(!num);
// console.log(!num2);

console.log(isNaN("2"));
console.log(isNaN(1));
console.log(isNaN("oi"));
