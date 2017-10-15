#include <iostream>
#include <tgmath.h>
#include <vector>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long int
using namespace std;
void getDivisors(ll n, vector<ll>* res)
{
    for (ll i=1; i<=sqrt(n); i++)
    {
        if (n%i==0)
        {
            if (n/i == i){
                res->push_back(i);
              }
            else{ // print both
                res->push_back(i);
                res->push_back(n/i);
              }
        }
    }
}

int main(){

    ll t,a,b,d;
    cin >>t>>a>>b;
    //create vector of divisors
    vector<ll>divisors(0);
    for (ll i=a;i<=b;i++){
      getDivisors(i,&divisors);
    }
    for(ll t0 = 0; t0<t; ++t0){
        ll res = 0;
        cin >> d;
        for (ll j=0; j<divisors.size();j++){
          if (!(divisors[j]%d==0)){
            res++;
          }
        }
        cout <<res<<fixed<<endl;
    }
    return 0;
  }
