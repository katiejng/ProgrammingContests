//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;



ll work (ll l, ll r, vector<vector<double> > &memo){
    ll opt1 = memo[l-1][r-1]+1;

    ll opt2 = 0;
    for (ll k = 0; k<=min(l,r)-2;++k){
        ll temp = memo[l-2-k][r-2-k]+memo[k][k]+3;
        //cout << temp;
        if (temp>opt2){
            opt2 = temp;
        }
    }
    return max(opt1,opt2);


}


int main(){
    int n,d;
    string lillies;
    cin >> n >> d;
    cin >> lillies;
    int count = 0;
    int jumps = 0;
    for (int i = 0 ;i <n;i++){
      count ++;
      if (lillies[i] == '1'){
        if (count > d){
          cout << "-1";
          return 0;
        }
        count = 0;
      }
    }

    // It is possible
    int i = 0;
    while (i<n-1){
      for (int j = i+d;j>i;j--){
        if (j >=n-1){
          //cout << i<<endl;
          jumps ++;
          cout << jumps;
          return 0;
        }
        //cout << i<<j;
        if (lillies[j]=='1'){
          //cout << i<<endl;
          i = j;
          jumps ++;
          break;
        }
      }
    }


    cout << jumps;









    return 0;
}
