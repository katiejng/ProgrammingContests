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

  int n;
  cin >> n;
  string word;
  for (int i = 0;i < n;i++){
    cin >> word;
    string ans = word;
    if (word.size()>10){
      ans = word[0];
      //word = word[0]+str(word.size()-2)+word[-1];
      ans += to_string(word.size()-2);
      ans += word.back();
    }
    cout << ans<<endl;
  }

  return 0;
}
