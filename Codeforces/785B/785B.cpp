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

  int CF, PS, PF, CS;
  int C,P, tempf, temps;

  cin >> C;
  cin >>CS>>CF;
  for (int i = 0; i<C-1;i++){
    cin >> temps>>tempf;
    if (temps>CS){
      CS = temps;
    }
    if (tempf<CF){
      CF = tempf;
    }
  }

  cin >> P;
  cin >>PS>>PF;
  for (int i = 0; i<P-1;i++){
    cin >> temps>>tempf;
    if (temps>PS){
      PS = temps;
    }
    if (tempf<PF){
      PF = tempf;
    }
  }
  int res = 0;
  if (PS-CF>res){
    res = PS-CF;
  }
  if (CS-PF>res){
    res = CS-PF;
  }
  cout << res<< endl;

  return 0;
}
