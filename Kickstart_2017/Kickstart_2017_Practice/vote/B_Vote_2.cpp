#include <iostream>
#include <fstream> 
#include <string>
#include <iomanip>
#include <vector>

#define ll long long
using namespace std;

// N As
// M Bs
// N-A
// 


double work (ll A,ll B,ll n,ll m, vector<vector<double> > &memo){
    if (B>=A){
        return 0;
    }
    if (n==0 && m==0){
        return 1;
    }
    if (n==0){
        return work(A,B+1,n,m-1,memo)*(m/(double)(n+m));
    }
    if (m==0){
        return work(A+1,B,n-1,m,memo)*((double)n/(n+m));
    }
    if (memo[A][B] < -0.5){
    
        memo[A][B] = work(A+1,B,n-1,m,memo)*((double)n/(n+m))+work(A,B+1,n,m-1,memo)*((double)m/(n+m));
    }
    return memo[A][B];
    
    
    
}


int main(){
    
    
    ifstream cin ("B-large-practice.in");
    ofstream cout ("B_small.out");
    int t;    
    cin >>t;
    for(int t0 = 0; t0<t; ++t0){
        int n,m;
        cin >> n >> m;
        //create a vector of vectors that saves result for n,m
        vector< vector<double> > memo(n, vector<double>(m, -1));
        //cout <<t0<<" "<<n<<" "<<m<<" "<<"\n";
        double res = work(1,0,n-1,m,memo)*((double)n/(n+m));
        
        cout <<"Case #"<<t0+1<<": "<<fixed<<setprecision(8)<<res<<endl;
    }
    
    
    
    
    
    
    
    return 0;
}


