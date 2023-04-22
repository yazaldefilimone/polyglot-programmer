const ex = false;
if (!ex && 3) {
  console.log("it`s true");
} else {
  console.log("it`s false");
}

let nam = "yazalde";
if (Boolean(nam.includes("y") ?? 2)) {
  console.log("it`s true");
} else {
  console.log("it`s false");
}

for (let index = 0; index < 3; index++) {
  console.log(index);
}

let num = 1;
while (num !== 4) {
  console.log("i am while", num);
  num++;
}

const me = {
  name: "Yazalde",
  age: "18",
};
console.log("----for..in-----");

for (const key in me) {
  if (me.hasOwnProperty(key)) {
    console.log({ key, value: me[key] });
  }
}
console.log("----or-----");
// or
for (const key in me) {
  if (Object.hasOwnProperty.call(me, key)) {
    console.log({ key, value: me[key] });
  }
}

console.log("----for..of-----");

let meInfo = ["yazalde", 18];
for (const iterator of meInfo) {
  console.log(iterator);
}
