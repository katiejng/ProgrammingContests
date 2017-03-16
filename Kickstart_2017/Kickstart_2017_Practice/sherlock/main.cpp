#include <bits/stdc++.h>

void solve(int l, int r);

using namespace std;

int main() {
    freopen("C-small-practice.in","r",stdin);
    freopen("ans.out","w",stdout);

    int test, l, r, count = 1;
    cin>>test;
    for (int i = 0; i < test; ++i) {
        cin>>l>>r;
        cout<<"Case #"<<count<<": ";
        solve(l,r);
        count++;
    }
    return 0;
}

void solve(int l, int r) {
    int sum = 0;
    if (r == 0 || l == 0){
        cout<<0<<endl;
        return;
    }
    if ( l >= r){
        for (int i = 1; i <= r; ++i) {
            sum += (r-(i-1));
        }
    }
    else{
        for (int i = 1; i <= l; ++i) {
            sum += (l-(i-1));
        }
    }
    cout<<sum<<endl;
}