#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
#include <math.h>
#include <algorithm>
#include <queue>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;



void work(int t0, ll K, priority_queue<ll> groups){
   ll cur;
    while(K>1){
        cur = groups.top();
        groups.pop();
        groups.push(ceil((double)(cur-1)/2));
        groups.push(floor((double)(cur-1)/2));
        K-=1;
    }
    cur = groups.top();
    //cout <<cur;
    
    cout<<"Case #"<<t0+1<<": "<<ceil((double)(cur-1)/2)<<" "<<floor((double)(cur-1)/2)<<endl;
    
}



int main(){


	int T; //n = pebble types, k = number that can fit in one pocket
	cin >>T;
	int res =0;
	ll N,K;
	for (int t0=0;t0<T;++t0){
		cin>>N>>K;
		priority_queue<ll> groups;
        groups.push(N);
        
        work(t0,K,groups);
				
	
	}
	

	return 0;
}
