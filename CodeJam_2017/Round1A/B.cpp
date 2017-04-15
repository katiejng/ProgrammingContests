#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;




int main(){


	int T; //t = test cases
	cin >>T;
	int res =0;
	int N,P;
	for (int t0=0;t0<T;++t0){

		cin>>N>>P;
		
		vector<int> recipe(N);
		vector<vector<int> > packets(N,vector<int> (P,-1));
			
		for (int n = 0;n<N;n++){
			cin >> recipe[n];
		}
		
		for (int n = 0;n<N;n++){
			cout << recipe[n]<<endl;
		}
		for (int n = 0;n<N;n++){
			for (int p = 0;p<P;p++){
				cin >> packets[n][p];
			}
		}
		
		for (int n = 0;n<N;n++){
			for (int p = 0;p<P;p++){
				cout << packets[n][p]<<endl;
			}
		}
		

		cout <<"Case #"<<t0+1<<":"<<endl;//fixed<<setprecision(0)<<
		
	
	}
	

	return 0;
}
