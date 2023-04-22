var message;
var let_message;
const const_global = "const,hei";
console.log(message);
{
  let create_new_scope = "scope object";
  message = "var, new scope";
  let_message = "let, new scope";
  console.log(const_global);
}

console.log(message);
// console.log(create_new_scope);
console.log(let_message);
console.log(const_global);
