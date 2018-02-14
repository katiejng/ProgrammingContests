//https://codejam.withgoogle.com/codejam/contest/dashboard?c=6304486#s=p2
#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
#include <math.h>
#include <algorithm>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;

bool myfunction (vector<long> i,vector<long> j) { return (i[1]>j[1]); }

int main(){
  int n;
  cin >> n;
  string temp;
  vector<long> startletters;
  int l = 0;
  vector<vector<long> > letters(10, vector<long>(3,0));
  for (int i = 0;i<10;i++){
    letters[i][0]=i;
    letters[i][2]=-1;
  }
  for (int i = 0; i < n; i++){
    cin >> temp;
    startletters.push_back(temp[0]-'a');
    for (int j = 0; j<temp.length();j++){
      letters[temp[j]-'a'][1]+= pow(10,temp.length()-j-1);
    }
  }
  sort (letters.begin(), letters.end(), myfunction);


  // Choose 0!!
  int result = 0;
  int count = 0;
  for (int k = 0; k<letters.size();k++){
    if (find(startletters.begin(), startletters.end(),letters[k][0]) == startletters.end()){
      letters[k][2]=count;
      count +=1;
      break;
    }
  }

  for (int k = 0; k<letters.size();k++){
    //cout << char(letters[k][0]+'a')<<letters[k][1] <<" "<<letters[k][2]<< endl;
  }

  for (int k = 0; k<letters.size();k++){
    if (letters[k][2]==-1){
      result += letters[k][1]*count;
      count +=1;
      //cout << result <<endl;
    }
  }
    cout << result<<endl;





    return 0;
}
