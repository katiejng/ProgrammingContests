//Incomplete solution. IsConnected is unneccessary for the solution

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

int BFS(int cities[],int currCity, int roads[][2],int n, int m){


vector<int> nextCity;
for(int ab = 0;ab<n;++ab){//repeat the number of cities
	for (int i =0;i<m;++i){
		//add adjacent cities that haven't been visited
		if (roads[i][0]==currCity && cities[roads[i][1]]==-1) {
			cities[roads[i][1]]=1;			
			nextCity.push_back(roads[i][1]);
		}
		if (roads[i][1]==currCity && cities[roads[i][0]]==-1) {
			cities[roads[i][1]]=1;
			nextCity.push_back(roads[i][0]);
		}
	}
	if (nextCity.size()==0)break;
	currCity =nextCity[nextCity.size()-1]; 
	nextCity.pop_back();
}
for (int cb = 0;cb<n;cb++){
	if (cities[cb]==-1){
		return -1;
}
}
return 1;

}

int isconnected(int roads[][2],int n,int m){


int cities[n]={-1};

int currCity = 0;
cities[currCity]=1;

return BFS(cities, currCity, roads,n,m);
	

}

int work(int i, int j, int roads[][2], int n, int m){
	
	int cities[n]={0};
	for (int k=0;k<m;++k){
		if (k!=i && k != j){
		
			cities[roads[i][0]]++;
			cities[roads[i][1]]++;
		}
		//only do once
		cities[roads[i][0]]++;
		cities[roads[i][1]]++;
	
	}
	int count = 0;
	for (int a =0;a<n;++a){
		if (cities[a]%2!=0) count++;
		if (count >2) return 0;
 
	}
	if (count == 0 ||count ==2){return 1;}
	else{ return 0;}

}

int main(){


	//ifstream cin ("C_small.in");
	//ofstream cout ("sher_small.out");
	int n,m; //n = pebble types, k = number that can fit in one pocket
	int x,y;	
	cin >>n>>m;
	int roads[m][2];
	for (ll i= 0; i<m;++i){
		cin >>x>>y;
		roads[i][0] = x;
		roads[i][1] = y;
	}
	int res = 0;
	if (isconnected(roads,n,m)){
		for (int i = 0;i<m-1;++i){
			for (int j = i+1; j<m;++j){
				res +=work(i,j,roads, n, m);
		}
	}
}
	printf("%d", res);
	
	return 0;
}
