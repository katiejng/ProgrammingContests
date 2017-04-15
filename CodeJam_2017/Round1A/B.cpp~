#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;




int main(){


	int T; //t = test cases
	cin >>T;
	int res =0;
	int R,C;
	for (int t0=0;t0<T;++t0){

		cin>>R>>C;
		
		vector<string> grid = vector<string>(R);
		for (int r = 0;r<R;r++){
			cin >> grid[r];
		}
		for (int r = 0;r<R;r++){
			//cout << grid[r]<<endl;
		}
		
		//First pass
		for (int r = 0;r<R;r++){
			
			char letter = '?';
			for (int c = 0;c<C;c++){
				if (grid[r][c] =='?'){
					grid[r][c] = letter;
				}else{
					letter = grid[r][c];
				}
			}
		}
		
		for (int r = R-1;r>=0;r--){
			
			char letter = '?';
			for (int c = C-1;c>=0;c--){
				if (grid[r][c] =='?'){
					grid[r][c] = letter;
				}else{
					letter = grid[r][c];
				}
			}
		}
		
		//down pass
		string sample = grid[0];
		for (int r = 1;r<R;r++){
			if (grid[r][0]=='?'){
				grid[r] = sample;
			}else{
				sample = grid[r];
			}
		}
		//up pass
		
		sample = grid[R-1];
		for (int r = R-2;r>=0;r--){
			if (grid[r][0]=='?'){
				grid[r] = sample;
			}else{
				sample = grid[r];
			}
		}
		
		
		
		



		cout <<"Case #"<<t0+1<<":"<<endl;//fixed<<setprecision(0)<<
		for (int r = 0;r<R;r++){
			cout << grid[r]<<endl;
		}
	
	}
	

	return 0;
}
