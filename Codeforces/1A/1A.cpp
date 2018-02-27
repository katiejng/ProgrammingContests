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
  ll n,m,a ;
  cin >> n>>m>>a;

  ll result = ((n+a-1)/a)*((m+a-1)/a);
  cout << result<<endl;

  return 0;
}
