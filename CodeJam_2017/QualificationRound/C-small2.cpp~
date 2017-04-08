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



void work(int t0,int stalls[], int N, int K){
	int finalmin;
	int finalmax;
	int start= 0;
	int end = N+1;
	vector<int> minLR(N+2,-1);
	vector<int> maxLR(N+2,-1);
	for (int j = 0;j<K;j++){
		
		//calculate Ls
		int count = 0;
		for (int i=start;i<=end;++i){
			minLR[i]=count;
			if (stalls[i]==1){
				minLR[i]=0;
				count = 0;
			}else{
				count++;
			}
		}
		//calculate Rs
		count = 0;
		int largestMinValue=0;
		for (int i=end;i>=start;--i){
			if (stalls[i]==1){
				maxLR[i]=0;
				count = 0;
			}else{
				if (minLR[i]>count){
					maxLR[i] = minLR[i];
					minLR[i] = count;
				}else{
					maxLR[i]=count;
				}
				if (minLR[i]>largestMinValue){
					largestMinValue = minLR[i];
				
				}
				count++;
			
			}
		}
		
		
		//if it's not the largest MINLR value, set the MAXLR to -1
		int largestMaxValue=0;
		int largestMaxIndex=0;
		for (int i = 0;i<N+2;++i){
			if (minLR[i]==largestMinValue){
				if (largestMaxValue<maxLR[i]){
					largestMaxValue = maxLR[i];
					largestMaxIndex = i;
				}
			}
		}
		
		stalls[largestMaxIndex]=1;
		finalmin = minLR[largestMaxIndex];
		finalmax = maxLR[largestMaxIndex];
		
		//test(stalls,N);
	}
	cout <<"Case #"<<t0+1<<": "<<finalmax<<" "<<finalmin<<endl;//fixed<<setprecision(0)<<

}

int main(){


	//ifstream cin ("C_smaint.in");
	//ofstream cout ("sher_smaint.out");
	int T; //n = pebble types, k = number that can fit in one pocket
	cin >>T;
	int res =0;
	int N,K;
	for (int t0=0;t0<T;++t0){
		cin>>N>>K;
		
		int stalls[N+2]={0};
		
		stalls[0]= 1;
		stalls[N+1]= 1;
		
		//test(stalls,N);
		work(t0,stalls, N,K);
		
		
		
	
	}
	

	return 0;
}
