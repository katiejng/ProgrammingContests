//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
#include <cstring>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
#define MAX_BUFFER 2000
using namespace std;

int getMinIndex(int array[]){
  int smallValue = array[0];
  int smallIndex= 0;

  for (int i=1;i<sizeof(array);++i){
    if (array[i]<smallValue){
      smallValue=array[i];
      smallIndex=i;
    }
  }
  return smallIndex;
}


int main(){


    int N,K;
    cin >>N>>K;
    string A;
    cin >> A;
    if (K>N){
      cout << "WRONGANSWER";
      return 0;
    }

    int letterCount[26]={0};

    for (int i=0;i<N;i++){
      letterCount[A[i]-'a']++;
    }

    

    int leastLetterIndex = getMinIndex(letterCount);

    if (letterCount[leastLetterIndex]>K){
      cout<<"WRONGANSWER";
      return 0;
    }

    string ans(N,leastLetterIndex+'a');
    K-= letterCount[leastLetterIndex];
    int pos=0;
    while (K>0){
      if (A[pos]!=leastLetterIndex+'a'){//if we're not looking at the least common letterCount
        ans[pos]=A[pos];
        K-=1;

      }
      ++pos;
    }
    cout << ans;



    return 0;
}
