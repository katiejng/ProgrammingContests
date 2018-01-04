//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <vector>
#include <algorithm>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;
// 0 is even, 1 is odd

int main(){

    int sum = 0;

    int n, m;

    cin >> n;
    vector<int> perm = vector<int>(n);
    for (int i =0; i<n;i++){
      cin >> perm[i];
    }

    cin >> m;

    vector< vector<double> > steps(n, vector<double>(2, -1));


    //calculate initial value

    for (int i = 0;i<n-1;i++){
      for (int j = i;j<n;j++){
        if (perm[i]>perm[j]){
          sum++;
        }
      }
    }
    //cout << sum;
    sum = sum %2;
    int a, b, diff, swaps;

    for (int i =0; i<m;i++){
      cin >> a >> b;
      diff = b-a;
      if (diff >0){
        
        sum = (((diff*(diff+1))/2)%2 + sum)%2;
      }
      if (sum ==0){
        cout << "even"<<endl;
      }else{
        cout <<"odd"<<endl;
      }

    }

    return 0;
}
