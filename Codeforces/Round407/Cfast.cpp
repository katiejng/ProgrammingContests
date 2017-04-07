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

ll maxSubArraySum(ll a[], ll size)
{
    ll max_so_far = 0, max_ending_here = 0;
 
    for (ll i = 0; i < size; i++)
    {
	//max ending here calculates sums of positive numbers in a row. If the sum is negative, it starts again at 0. max so far stores the biggest number max ending has ever been.
        max_ending_here = max_ending_here + a[i];
        if (max_so_far < max_ending_here)
            max_so_far = max_ending_here;
 
        if (max_ending_here < 0)
            max_ending_here = 0;
    }
    return max_so_far;
}

int main(){


	//ifstream cin ("C_small.in");
	//ofstream cout ("sher_small.out");
	ll n; //n = pebble types, k = number that can fit in one pocket
	cin >>n;
	ll temp,l0,r0;
	vector<ll> myvect(n,0);
	ll memo[n-1];
	for (ll i= 0; i<n;++i){
		cin >>myvect[i];
	}
	ll a = 1;
	for (ll i= 0; i<n-1;++i){
		memo[i]= abs(myvect[i]-myvect[i+1])*a;
		a*=-1;
	}

	ll maxValue = maxSubArraySum(memo,n-1);
	for (ll i= 0; i<n-1;++i){
		memo[i]= memo[i]*(-1);
		
	}
	ll maxValue2 = maxSubArraySum(memo, n-1);
	cout <<max(maxValue, maxValue2);

	return 0;
}
