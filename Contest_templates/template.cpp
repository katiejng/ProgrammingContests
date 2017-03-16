//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <fstream> 
#include <string>
#include <iomanip>
#include <vector>

#define ll long long
using namespace std;



ll work (ll l, ll r, vector<vector<double> > &memo){
    ll opt1 = memo[l-1][r-1]+1;
    
    ll opt2 = 0;
    for (ll k = 0; k<=min(l,r)-2;++k){
        ll temp = memo[l-2-k][r-2-k]+memo[k][k]+3;
        //cout << temp;
        if (temp>opt2){
            opt2 = temp;
        }
    }
    return max(opt1,opt2);
    
    
}


int main(){
    
    
    ifstream cin ("C_small.in");
    ofstream cout ("sher_small.out");
    int t;    
    cin >>t;
    for(int t0 = 0; t0<t; ++t0){
        int L,R;
        cin >> L >> R;
        //cout <<L<<R;
        //create a vector of vectors that saves result for n,m
        
        vector< vector<double> > memo(L+1, vector<double>(R+1, -1));
        ll res;
        if (L==0 || R==0){
            res = 0;
        }else{
            memo[0][0] = 0;
            for (int l = 0; l<L+1; ++l){
                memo[l][0] = 0;
                memo[l][1] = 1;

            }

            for (int r = 1; r<R+1; ++r){
                memo[0][r] = 0;
                memo[1][r] = 1;

            }

            for (ll l = 1;l<L+1; ++l){
                for (ll r = 2; r<R+1; ++r){
                    memo[l][r] = work(l,r,memo);
                }
            }
            res = memo[L][R];
        }
        
        cout <<"Case #"<<t0+1<<": "<<res<<endl; //<<fixed<<setprecision(8)
    
    }
    
    
    
    
    
    
    
    return 0;
}


