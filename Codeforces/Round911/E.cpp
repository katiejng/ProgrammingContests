//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <vector>
#include <algorithm>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout
// INCOMPLETE SOLUTION
#define ll long long
using namespace std;
// 0 is even, 1 is odd

int main(){

  int n, k;
  cin >> n >> k;
  int cur = 1;
  vector<int> a(n,-1);
  vector<int> s;
  for (int i = 0;i<n;i++){
    cin >> a[i];
  }

  while (!a.empty() || !s.empty()){
      // check if can take from stack
      if (!s.empty() && s.back() == cur){
          s.pop_back();
          cur++;

      //check if can take from array
      }else if (!a.empty()){
        if (a.back() == cur){
          a.pop_back();
          cur++;
        }else{

          s.push_back(a.back());
          a.pop_back();
        }
      //can't take from stack (even with items) or array
    }else{
      cout << cur;
      if (cur > n){
        // cout result
        cout << "result";
      }else{
        cout << -1;
      }
      break;
    }


  }



  return 0;
}
