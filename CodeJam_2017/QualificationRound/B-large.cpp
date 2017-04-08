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



int main(){


	//ifstream cin ("C_small.in");
	//ofstream cout ("sher_small.out");
	ll T; //n = pebble types, k = number that can fit in one pocket
	cin >>T;
	ll res =0;
	string N;
	for (ll t0=0;t0<T;++t0){
		cin>>N;
		int n = N.length();
        int flag = 0;
		for (int i=0;i<N.length()-1;++i){
			if (flag==0){
                if (N[i]>N[i+1]){
                    N[i] = N[i]-1;
                }
            }else{
                N[i]= '9';
                
            }
            
		}
		
		cout <<"Case #"<<t0+1<<": "<<N<<endl;//fixed<<setprecision(0)<<
	
	}
	

	return 0;
}
