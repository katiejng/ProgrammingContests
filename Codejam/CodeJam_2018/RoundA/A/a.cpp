#include <cstring>
#include <string.h>
#include <map>
#include <deque>
#include <queue>
#include <stack>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <ctime>
#include <algorithm>
#include <vector>
#include <set>
#include <complex>
#include <list>

using namespace std;

int getLengthCount (int i){
  int lengthCount = 0;
  for(; i != 0; i /= 10, lengthCount++);
  return lengthCount;
}


int main(int argc, char *args[]) {
    if (argc == 2 && strcmp(args[1], "small") == 0) {
        freopen("small.in","rt",stdin);
        freopen("small.out","wt",stdout);
    }
    else if (argc == 2 && strcmp(args[1], "large") == 0) {
        freopen("large.in","rt",stdin);
        freopen("large.out","wt",stdout);
    }
    else {
	    freopen("a.txt", "rt", stdin);
    }
    int N;
    string anum;
    cin>>N;
    for (int i = 0; i<N;i++){

      cin >> anum;
      cout << anum[0];
      //printf("Case #%d: %c", i, anum);

      cout<<endl;
    }




    return 0;
}
