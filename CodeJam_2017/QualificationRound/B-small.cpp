#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
#include <math.h>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;

int tidy(ll N){
	
	ll a = (ll)N/pow(10,0);
	for (ll i=0;pow(10,i)<N;++i){
		ll b = (ll)N/pow(10,i+1);
		if (a%10<b%10){
			return 0;
		}
		a = b;
	}
	return 1;

}


int main(){


	//ifstream cin ("C_small.in");
	//ofstream cout ("sher_small.out");
	ll T; //n = pebble types, k = number that can fit in one pocket
	cin >>T;
	ll res =0;
	ll N;
	for (ll t0=0;t0<T;++t0){
		cin>>N;
		for (ll n0=N;n0>=0;--n0){
			if (tidy(n0)){
				res = n0;
				break;
			}
		}
		
		cout <<"Case #"<<t0+1<<": "<<res<<endl;//fixed<<setprecision(0)<<
	
	}
	

	return 0;
}
