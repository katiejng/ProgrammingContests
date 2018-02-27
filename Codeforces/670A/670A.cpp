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
  int n, weeks, days, mini, maxi;
  cin >> n;

  weeks = n/7;
  days = n%7;

  mini = weeks*2;
  maxi = weeks*2;
  if (days <=2){
    mini+=days;
  }else{
    mini+=2;
  }
  if (days>=6){
    maxi+=1;
  }
  cout << maxi << " "<<mini<<endl;


  return 0;
}
