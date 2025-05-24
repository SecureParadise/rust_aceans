#include <iostream>
using namespace std;

int gcd(int a, int b)
{
    // ensure a > b
    if (b > a)
    {
        swap(a, b);
    }
    if (b == 0)
    {
        return a;
    }
    else
    {
       return gcd(b, a % b);
    }
}

int main(){
    cout << 5%2 <<endl;
    return 0;
}