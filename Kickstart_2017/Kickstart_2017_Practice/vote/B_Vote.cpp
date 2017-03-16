#include <iostream>
#include <fstream> 
#include <string>
#include <iomanip>

#define ll long long
using namespace std;


ll work (ll A,ll B,ll n,ll m){
    if (B>=A){
        return 0;
    }
    if (n==0 && m==0){
        return 1;
    }
    if (n==0){
        return work(A,B+1,n,m-1)*m;
    }
    if (m==0){
        return work(A+1,B,n-1,m)*n;
    }
    return work(A+1,B,n-1,m)*n+work(A,B+1,n,m-1)*m;
    
    
    
}

ll fact(ll n){
    ll value = 1;
    while (n>0){
        value *= n;
        --n;
    }
}

int main(){
    
    
    ifstream cin ("B-large-practice.in");
    ofstream cout ("B_small.out");
    int t;    
    cin >>t;
    for(int t0 = 0; t0<t; ++t0){
        int n,m;
        cin >> n >> m;
        //cout <<t0<<" "<<n<<" "<<m<<" "<<"\n";
        ll X = work(1,0,n-1,m)*n;
        double res = (float)X/fact(n+m);
        
        cout <<"Case #"<<t0+1<<": "<<fixed<<setprecision(8)<<res<<endl;
    }
    
    
    
    
    
    
    
    return 0;
}


