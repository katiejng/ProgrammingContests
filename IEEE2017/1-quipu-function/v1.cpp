#include "../templates/stdc++.h"
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;


int main(){

    int t,a,b,d;
    cin >>t>>a>>b;
    //create vector of divisors
    vector<int>divisors(0);
    for (int i = a; i<=b;i++){
      //cout <<"I loop. i: "<<i<<endl;
      //create a vector of divisors
      for (int divisor = 1;divisor<=sqrt(i);divisor++){
        //cout << "divisor loop. div: "<<divisor<<endl;
        if (i % divisor == 0){
          if (divisor%d){
            divisors.push_back(i);
            cout<<i<<endl;
          }
        }
      }
    for(int t0 = 0; t0<t; ++t0){
        int res = 0;
        cin >> d;

        cout <<res<<endl; //<<fixed<<setprecision(8)

    }
    return 0;
}
