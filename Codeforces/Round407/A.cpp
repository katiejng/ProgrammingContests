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
	int N,K; //n = pebble types, k = number that can fit in one pocket
	cin >>N>>K;

	vector<int> myvect(N,0);
	
	for (int i= 0; i<N;++i){
		cin >>myvect[i];
	}
	

	
	int pockets = 0;
	for (int i= 0; i<N;++i){
		pockets+=ceil((double)myvect[i]/K);
	}


	cout <<fixed<<setprecision(0)<<ceil((double)pockets/2);
	return 0;
}
