#include <bits/stdc++.h>

using namespace std;

typedef vector<int> vi;
#define ll long long

int main() {
  
	string N;
	while(cin >>N){
			
		vector<vi> points(N.length()+1 ,vi(2,0));
		int current[] = {0,0};
		
		
		for (ll i =0;i<(int)N.length();i++){
			int direction =N[i]-'0';
			
			if (direction == 1 || direction == 0 || direction ==7){
				current[0]+=1;
			}
			
			if (direction == 3 || direction == 4 || direction ==5){
				
				current[0]-=1;
			}
			if (direction == 1 || direction == 2 || direction ==3){
				current[1]+=1;
			}
			
			if (direction == 5 || direction == 6 || direction ==7){
				
				current[1]-=1;
			}
			points[i+1][0] = current[0];
			points[i+1][1] = current[1];
			
		}
		
		
		
		//find area of polygon
		
		//find number of points inside polygon using picks theorem
		
		double area = 0;
		double trap = 0;
		ll x1,x2,y1,y2,x;
		double y;
		for (ll i =0;i<N.length();i++){
			//cout << points[i][0]; 
			x1 = points[i][0];
			x2 = points[i+1][0];
			y1 = points[i][1];
			y2 = points[i+1][1];
			//cout <<x1<<" "<<y1<<" "<<x2<<" "<<y2<<endl;
			 x = abs(x2-x1);
			 
			 y = (double)abs(y2+y1)/2;
			 trap = x*y;
			 //cout <<x<<" "<<y<<"TRPAP::"<<trap<<endl;
			 if (x2>x1){
			 	 area+= trap;
			 }else{
			 	 area-= trap;
			 }
			 		
			
		}
		area = abs(area);
		
		//cout << area <<endl;
        
		double I = area +1-(double)N.length()/2;
		cout << I+N.length()<<setprecision(0)<<fixed<<endl;
		
		
	}


	
	return 0;
}
