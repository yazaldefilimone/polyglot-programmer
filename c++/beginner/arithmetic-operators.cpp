#include <iostream>

int main()
{
  int a = 10;
  int b = 20;

  // addition
  int sum = a + b;

  std::cout << "Sum result: " << sum << std::endl;

  // subtraction

  int difrenrence = a - b;

  std::cout << " Diference : " << difrenrence << std::endl;

  // Multiplication

  int product = a * b;
  std::cout << " product : " << product << std::endl;

  int quotient = a / b;
  std::cout << " quotient : " << quotient << std::endl;

  float quotient_with_float = float(a) / float(b);

  std::cout << " quotient_with_float : " << quotient_with_float << std::endl;
  //  Modulus Operator (%)
  int remainder = a % b;
  std::cout << " remainder : " << remainder << std::endl;

  return 0;
}