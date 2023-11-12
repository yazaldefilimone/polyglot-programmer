#include <any>
#include <iostream>

int main() {
  // int x = 42;
  // float y = 3.14f;
  // std::string z = "Hello, world!";

  // void *void_ptr;

  // void_ptr = &x;
  // std::cout << "int value: " << *(static_cast<int *>(void_ptr)) << std::endl;

  // void_ptr = &y;
  // std::cout << "float value: " << *(static_cast<float *>(void_ptr))
  //           << std::endl;

  // void_ptr = &z;
  // std::cout << "string value: " << *(static_cast<std::string *>(void_ptr))
  //           << std::endl;
  // ------ dynamic type using "any"

  std::any anything;

  anything = 5.78;

  std::cout << "anything" << std::any_cast<double>(anything) << std::endl;

  anything = 100;
  std::cout << "anything" << std::any_cast<int>(anything) << std::endl;

  anything = std::string("Hello word");

  std::cout << "anything" << std::any_cast<std::string>(anything) << std::endl;

  return 0;
}