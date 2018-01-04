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


int main(){
    int N;
    int a;
    cin >> N;
    int min = -1;
    int dist = -1;
    vector<int> dists;
    for (int i = 0; i<N;i++){
      cin >> a;


      if (min == -1){
        min = a;
        dist = 0;
      }else if (a < min){
          min = a;
          dist = 0;
          dists.clear();
      }else if (a> min){
        dist ++;
      }else if (a==min){
        dist ++;
        dists.push_back(dist);
        dist = 0;
      }


    }
    int res = -1;
    for (int i = 0; i<dists.size();i++){
      if (res ==-1 || res > dists[i]){
        res = dists[i];
      }
    }


    cout << res <<endl;

    return 0;
}
