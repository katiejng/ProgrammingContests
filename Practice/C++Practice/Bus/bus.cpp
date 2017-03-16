#include <iostream>
#include <fstream> 
#include <string>
#include <iomanip>
#include <vector>
#include <algorithm>

//#include <bits/stdc++.h>
using namespace std;


int main(){
    
    //ofstream cout ("sher_small.out");
    int N,C,K;
    cin >> N>>C>>K;
    vector<int> T(N);
    for (int i=0;i<N; i++){//for each element in vector t, call it x. auto - type. could say (int& x:T)
        cin >>T[i];
    }
    sort(T.begin(),T.end());
    
    int ans = 0;
    
    for (int i=0; i<N;){
        int leavetime = T[i]+K;
        int filled = 0;
        while (i<N && filled <C && T[i]<=leavetime){
            ++i;
            ++filled;
        }
        ans++;
    }
    cout<<ans<<endl;
    
    
    return 0;
}