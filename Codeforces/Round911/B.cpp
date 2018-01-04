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
    int x, n,a,b;
    cin >> n >> a>>b;

    x = 0;

    for (int i = 1; i <= max(a,b)+1; i++){
      if ((a/i + b/i)<n){
        if (i-1 > a){
          cout <<a;
        }else if (i-1 > b){
          cout <<b;
        }else{

          cout <<i-1 <<endl;
        }
        break;
      }
    }


    return 0;
}
