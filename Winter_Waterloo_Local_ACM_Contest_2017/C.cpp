//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <algorithm>
#include <vector>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;



double harmonic_mean (int sequence[], int start, int end){
  //start inclusive, end not inclusive
  double a=0;
  for (int i=start; i<end;++i){
      a+= (double)1/sequence[i];

  }
  return (double)(end-start)/a;

}

int k_mean_sorted( int sequence[],int len, int k){
  //cout <<sequence[3]<<"here1\n";
  int i;
  for (i=1;i<=len-k;++i){
    cout <<"\n(harmonic_mean["<<sequence[1]<< sequence[2]<<sequence[3]<<sequence[4]<<", "<<i<<", "<<i+k<<"]<harmonic_mean[sequence, i+1, i+k+1])"<<harmonic_mean(sequence, i, i+k)<<" "<<harmonic_mean(sequence, i+1, i+k+1);
    if (harmonic_mean(sequence, i, i+k)<harmonic_mean(sequence, i+1, i+k+1)){
        return false;
    }
  }

  return true;
}


int main(){

   int N,K;
   cin >>N>>K;

    int sequence[N+1];
    //create sequence
    for (int i=1; i<=N;++i){
        sequence[i]=i;
      }

    do{
      cout <<"testing";
      for (int j=1;j<=N;j++){
        cout << sequence[j]<< " ";

        }
        cout<<endl;

      if (k_mean_sorted(sequence,N,K)){

        for (int j=1;j<=N;j++){
          cout << sequence[j]<< " ";

          }
          return 0;
      }
    }while(next_permutation(sequence+1,sequence+N+1));


    cout <<0<<endl;


    return 0;
}
