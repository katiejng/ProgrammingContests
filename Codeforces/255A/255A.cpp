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
  int N, temp, res;
  vi memo = vi(3,0);
  cin >> N;
  for (int i = 0;i<N;i++){
    cin >> temp;
    memo[i%3] += temp;
  }
  res = 0;
  for (int i = 1;i<3;i++){
    if (memo[i]>memo[res]){
      res = i;
    }
  }
  //cout << res;
  if (res == 0){
    cout << "chest" << endl;
  }else if (res == 1){
    cout << "biceps" << endl;
  }else {
    cout << "back" << endl;
  }

  return 0;
}
