#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;


int main(){
  int N;
  cin >> N;
  vector< string > memo(N, "");

  for (int j = 0 ;j<N;j++){
    cin >> memo[j];
  }
  sort(memo.begin(), memo.end());

  string target;
  int Q;
  int sum;
  cin >> Q;
  int index;
  for (int i = 0; i<Q;i++){
    cin >> target;
    sum = 0;
    for (int j = 0 ;j<N;j++){
      if (memo[j] == target){
        index = j;
      }
    }

    for (int j = 0;j<target.size();j++){
      cout << int(target[j]-'A')<<endl;
      sum+=target[j]-'A'+1;
    }
    cout << sum * (index+1<< endl;

  }

  return 0;
}
