#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
#include <algorithm>    // std::sort
#include <math.h>
#include <cmath>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;


int work(int l, int r, vector<vector<int> > &memo){
	if (memo[l][r] == -1){
	
		memo[l][r] =  work(l,r-1,memo)+(memo[r-1][r]*pow(-1,r-l+1));
	}
	return memo[l][r];
}

int main(){


	//ifstream cin ("C_small.in");
	//ofstream cout ("sher_small.out");
	int n; //n = pebble types, k = number that can fit in one pocket
	cin >>n;
	int temp,l0,r0;
	vector<int> myvect(n,0);
	 vector< vector<int> > memo(n, vector<int>(n, -1));
	for (int i= 0; i<n;++i){
		cin >>myvect[i];
	}
	
	for (int i= 0; i<n-1;++i){
		memo[i][i+1]= abs(myvect[i]-myvect[i+1]);
	}
	int maxValue = memo[0][1];
	//for j = 1 value ... all values (n-1)
	for(l0 = 0;l0<n-1;++l0){
		for (r0 = l0+1;r0<n;++r0){
			temp = work(l0,r0,memo); 
			//printf("%d, %d, %d\n",l0,r0, temp); 

			if (temp >maxValue){
				maxValue =temp; 
			}
		}
	}
	cout <<maxValue;

	return 0;
}
