#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
#include <algorithm>

using namespace std;

typedef long long ll;
typedef vector<int> vi;
typedef pair<int, int> pii;

int main() {
  int i ;
  cin >> i;
  if (i>2 && i%2==0){
    cout << "YES";
  }else{
    cout << "NO";
  }

  return 0;
}
