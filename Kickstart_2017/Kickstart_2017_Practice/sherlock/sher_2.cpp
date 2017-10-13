//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <fstream> 
#include <string>
#include <iomanip>
#include <vector>

#define ll long long
using namespace std;



ll work (ll l){
    return (l*(l+1))/2;
    
    
}


int main(){
    
    
    ifstream cin ("C-large-practice.in");
    ofstream cout ("C-large-practice.out");
    int t;    
    cin >>t;
    for(int t0 = 0; t0<t; ++t0){
        int L,R;
        cin >> L >> R;
        //cout <<L<<R;
        //create a vector of vectors that saves result for n,m
        
       ll res;
        res = work(min(L,R));
       
        
        cout <<"Case #"<<t0+1<<": "<<res<<endl; //<<fixed<<setprecision(8)
    
    }
    
    
    
    
    
    
    
    return 0;
}


