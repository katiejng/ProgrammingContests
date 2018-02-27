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
  ios::sync_with_stdio(0); cin.tie(0);
  int k;
  vi memo = vi(12,0);
  cin >> k;
  if (k == 0){
    cout << 0<<endl;
    return 0;
  }
  for (int i = 0;i<12;i++){
    cin >> memo[i];
  }
  sort(memo.begin(), memo.end());

  for (int i = 11;i>=0;i--){
    //cout << memo[i];
    k-=memo[i];
    if (k<=0){
      cout << (12-i)<<endl;
      return 0;
    }
  }
  cout << -1<<endl;
  return 0;
}
