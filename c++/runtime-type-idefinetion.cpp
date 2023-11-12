#include <iostream>
#include <typeinfo>

class Base {
  virtual void dummy(){};
};

class Derived : public Base {};

int main() {
  Base *base_ptr = new Derived;

  // using typeid to get the type of the object

  std::cout << "Type: " << typeid(*base_ptr).name() << '\n';

  delete base_ptr;
  return 0;
}