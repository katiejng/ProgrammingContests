#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
#include <math.h>
#include <algorithm>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;


void test(int stalls[],int N){
		
	for(int i = 0;i<N+2;i++){
			cout <<stalls[i]<<" ";
		}
		cout<<endl;
}


void work(int stalls[], int occupied[], int N, int K){
	stalls[(int)N/2]= 1;//The Kth person goes in this spot
	
	
	work(stalls[],(int)N/2,K-1)

}

struct group {
	
	int index;
	int length;
};

int main(){


	//ifstream cin ("C_smaint.in");
	//ofstream cout ("sher_smaint.out");
	int T; //n = pebble types, k = number that can fit in one pocket
	cin >>T;
	int res =0;
	int N,K;
	for (int t0=0;t0<T;++t0){
		cin>>N>>K;
		
		int stalls[N+2] = {0};
		vector<group> groups[K+2];
		stalls[0]= 1;
		stalls[N+1]= 1;
		
		
		work(stalls, occupied, N,K);
		
		
		
	
	}
	

	return 0;
}
