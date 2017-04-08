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


	int T; //n = pebble types, k = number that can fit in one pocket
	cin >>T;
	string N;
	for (ll t0=0;t0<T;++t0){
		cin>>N;
		int n = N.length();
        for (int i=0;i<N.length()-1;++i){
            //cout <<N<<endl;
			if (N[i]<=N[i+1]){
                //cout <<"THis happened"<<endl;
            }else{
                N[i]-=1;
                
                for (int j=i+1;j<N.length();++j){
                    N[j]='9';
                }
                //cout<<"now it is"<<N<<endl;
                i-=2;
                
            }
            
		}
		
		cout <<"Case #"<<t0+1<<": "<<N<<endl;//fixed<<setprecision(0)<<
	
	}
	

	return 0;
}
