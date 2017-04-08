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


void test(int stalls[],int N){
		
	for(int i = 0;i<N+2;i++){
			cout <<stalls[i]<<" ";
		}
		cout<<endl;
}

struct Group{
    int start,end;
};


void work(int N, int K, vector<Group> groups,int stalls[]){
    while(K>0){
        Group current = groups[0];
        Group a;
        Group b;
        int split = (int)current.start+current.end/2;
        stalls[split]=1;
        a.start = current.start;
        a.end = split;
        b.start = split;
        b.end = current.end;
        if ((current.end-current.start) %2 ==0){
         
            groups.push_back(b);
            groups.push_back(a);

        }else{
            groups.push_back(a);
            groups.push_back(b);

        }
        
        groups.erase(0);
        k-=1;
    }
    test(stalls,N); 
}



int main(){


	int T; //n = pebble types, k = number that can fit in one pocket
	cin >>T;
	int res =0;
	int N,K;
	for (int t0=0;t0<T;++t0){
		cin>>N>>K;
		vector<Group> groups;
        int stalls[N+2]={0};
		stalls[0]= 1;
		stalls[N+1]= 1;
        
        Group a;
        a.start = 0;
        a.end = N+1;
        groups.push_back(a);
        
        work(N,K,groups,stalls);
		
        
		
		
		
		
	
	}
	

	return 0;
}
