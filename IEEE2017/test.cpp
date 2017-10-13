#include <iostream>
#include <vector>
using namespace std;

int doStuff(int i, vector<string> vect){
    for (int j = 0; j<i; j++){
		cout << vect[j]<<endl;
	}
	return 4;
}

int main() {
    int i;
    cin >>i;
    vector<string> a_vector(i);
    for (int j = 0; j<i;j++){
       cin >> a_vector[j];
    }

    cout << doStuff(i, a_vector);
}
