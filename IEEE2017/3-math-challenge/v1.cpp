#include "../templates/stdc++.h"
//  ./file <infile >outfile. changes stdin and stdout
int modulo = 1000000007;
#define ll int
using namespace std;

ll factorial(ll n){
  ll res = 1;
  for (int i = n; i>1;i--){
    res*=i;
  }
  return res%modulo;
}


int main(){

    int t;
    cin >>t;
    for(int t0 = 0; t0<t; ++t0){
        ll a,b,c;
        ll res = 0;
        cin >> a>>b>>c;
        if (a==1){
          res = 1;
        }else{
          ll powerpart = factorial(b)/(factorial(c)*factorial(b-c));
          res = pow(a, powerpart);
          res =res % modulo;

        }

        cout <<"Case #"<<t0+1<<": "<<res<<endl; //<<fixed<<setprecision(8)

    }
    return 0;
}
