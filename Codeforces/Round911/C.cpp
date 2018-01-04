//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <vector>
#include <algorithm>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;


int main(){
    int x, c,a,b;
    vector<int> values;

    cin >> a >> b>>c;
    values.push_back(a);
    values.push_back(b);
    values.push_back(c);
    sort(values.begin(), values.end());

    a = values[0];
    b = values[1];
    c = values[2];

    if (a == 1){
      cout << "YES";
    } else if ( a == 2 && b == 2){
      cout << "YES";
    } else if (a == 2 && b == 4 && c == 4){
      cout << "YES";
    }else if (a == 3 && b == 3 && c == 3){
      cout << "YES";
    }else{
      cout <<"NO";
    }


    return 0;
}
