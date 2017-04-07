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


ll work(ll b,ll q,ll l, ll m, vector<ll> myvect){
	ll count = 0;
	if (abs(b)>l){
		return 0;
	}	
		
	//if b == 0, then it will stay 0 for infinity
	if (b==0){
		if (find(myvect.begin(), myvect.end(), 0) != myvect.end()){
			return 0;
		}else{
			return -1;
		}
	}

	//if q == 0, then it will become 0  forever unless the first number is >l.
	if (q==0){
		if (find(myvect.begin(), myvect.end(), 0) != myvect.end()){
			if (find(myvect.begin(), myvect.end(), b) != myvect.end()){return 0;
			}else{return 1;}
		}else{
			return -1;
		}
	}

	if (q==1){
		if (find(myvect.begin(), myvect.end(), b) != myvect.end()){
			return 0; //if the values are in bad list don't print anything
		}else{	
			return -1;//if both values aren't in bad list, print inf
		}
	}
	
	//if q == -1, then it'll flip signs forever
	if (q==-1){
		
		if (find(myvect.begin(), myvect.end(), b) != myvect.end() &&
			find(myvect.begin(), myvect.end(), -b) != myvect.end()){
			return 0; //if the values are in bad list don't print anything
		}else{	
			return -1;//if both values aren't in bad list, print inf
		}
		
	}

		
while(true){
	
	//check if b<l
	if (abs(b)<=l){
		//check if b is in a
		if (find(myvect.begin(), myvect.end(), b) != myvect.end()){
			if (q==1){//if b is in a and q = 1, it never prints
				return count;
			}
		}else{
			//it's not in a and is less than b, increment
			count++;
		}
	}else{
		return count;
	}

	
	b*=q;
}

}

int main(){


	//ifstream cin ("C_small.in");
	//ofstream cout ("sher_small.out");
	ll b,q,l,m; //n = pebble types, k = number that can fit in one pocket
	cin >>b>>q>>l>>m;

	vector<ll> myvect(m,0);
	
	for (ll i= 0; i<m;++i){
		cin >>myvect[i];
	}
	sort(myvect.begin(), myvect.end());

	ll result = work(b,q,l,m,myvect);
	if (result == -1){
			cout <<"inf";

}else{
	cout <<result;
}

	return 0;
}
