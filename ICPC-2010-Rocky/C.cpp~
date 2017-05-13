#include <bits/stdc++.h>

using namespace std;

typedef vector<int> vi;

int main() {
  
	int N;
	while(1){
		cin >>N;
		if (N ==0){
			return 0;
		}
		vi pitches(N,0);
		
		int total_pitch_len=0;
		int longest_pitch=0;
		for (int i=0;i<N;i++){
			cin >> pitches[i];
			total_pitch_len+= pitches[i];
			if (pitches[i]>longest_pitch){
					longest_pitch = pitches[i];
			}
		}
	
	
		for(int rope_len = 50;rope_len<=70;rope_len+=10){
			if (rope_len/2 <total_pitch_len){
				if (rope_len==70){
					cout << "0";
				}else{
					cout <<"0 ";
				}
			}else{
				if (rope_len==70){
					cout <<rope_len/longest_pitch +1;
				}
				else{
					cout <<rope_len/longest_pitch +1<<" ";
				}
			}
			
		}
		cout <<endl;
	}


	
	return 0;
}
