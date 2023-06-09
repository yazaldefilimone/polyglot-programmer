#include <iostream>

using namespace std;

int main()
{
  int b = 10, c = 20;

  if (b < 0 && c > 10)
  {
    std::cout << "b is less than 0 and c is greater than 10" << std::endl;
  }
  {

    int a = 5, b = -10;
    if (a > 0 || b > 0)
    {
      std::cout << "At least one value is positive." << endl;
    }
  }
}