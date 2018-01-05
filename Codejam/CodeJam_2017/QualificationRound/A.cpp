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
	int T; //n = pebble types, k = number that can fit in one pocket
	cin >>T;
	int res;
	int N;
	int num_flips;
	for (int t0=0;t0<T;++t0){
		res = 0;
		string pancakes;
		cin>>pancakes>>N;
		num_flips = 0;
		//cout <<pancakes<<endl;
		int pancake_len = pancakes.length();
		
		for (int i=0;i<pancake_len-N+1;++i){
			if (pancakes[i]=='-'){
				num_flips++;
				for (int j=0;j<N;++j){
					if(pancakes[i+j]=='-'){
						pancakes[i+j]='+';
					}else{
						pancakes[i+j]='-';
					}
				}
				
			
			}
			//cout <<pancakes<<" "<<num_flips<<endl;
		}
		for (int i=0;i<pancake_len;++i){
			if (pancakes[i]=='-'){
				res = 1;
			}
		}
		if (res==1){
			cout <<"Case #"<<t0+1<<": "<<"IMPOSSIBLE"<<endl;//fixed<<setprecision(0)<<
	
		}else{
			cout <<"Case #"<<t0+1<<": "<<num_flips<<endl;//fixed<<setprecision(0)<<
	
		}
		
		//
	}
	

	return 0;
}
