#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
#include <math.h>
#include <stdlib.h>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;




int main(){


	//ifstream cin ("C_small.in");
	//ofstream cout ("sher_small.out");
	int N,K; //n = pebble types, k = number that can fit in one pocket
	N= 100000;
	K = 100;
	vector<int> myvect(N,0);
	int i;
	for (i= 0; i<N;++i){
		myvect[i]=rand() %10000+1;
	}
	


	cout << N << " "<<K<<endl;
	for (i= 0; i<N;++i){
		cout <<myvect[i]<<" ";
	}
	
	return 0;
}
