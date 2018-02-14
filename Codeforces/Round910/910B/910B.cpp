//fail

#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>
#include <vector>
//#include <bits/stdc++.h>
//  ./file <infile >outfile. changes stdin and stdout

#define ll long long
using namespace std;


int main()
{
	int n,a,b,s;
	while(scanf("%d%d%d",&n,&a,&b)!=EOF){
	if(4*a+2*b<=n)	s=1;
	else if(2*a+b<=n)	s=2;
	else if(4*a<=n||(2*a<=n&&2*b<=n)||(2*a<=n&&a+b<=n))	s=3;
	else if(a+b<=n||2*a<=n)	s=4;
	else if(2*b<=n)	s=5;
	else s=6;
	printf("%d\n",s);
	}
	return 0;
}
