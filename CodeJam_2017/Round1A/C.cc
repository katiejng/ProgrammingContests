#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;

#define X first
#define Y second

#define debug(a) cerr << #a << " = " << (a) << endl;

template<typename T> ostream& operator<<(ostream& o, const vector<T>& v) {
	for (const auto& a : v) o << a << " ";
	return o;
}

int cities;
double dp[200];
ll dist[200][200];
ll speed[200];
ll energy[200];

double recurse(int n) {
	// one indexed
	double &z = dp[n];
	if (z) {
		return z;
	}
	if (n == cities - 1) {
		return 0;
	}

	double out = numeric_limits<double>::infinity();

	int cur_city = n+1;
	ll walked = dist[n][n+1];
	while (cur_city < cities && walked <= energy[n]) {
		out = min(out, recurse(cur_city) + walked / 1.0 / speed[n]);
		walked += dist[cur_city][cur_city+1];
		cur_city++;
	}

	return z = out;
}

double do_case() {
	// catch case where middle is -1
	fill(dp, dp+200, 0);
	int q;
	cin >> cities >> q;
	for (int i = 0; i < cities; i++) {
		cin >> energy[i] >> speed[i];
	}
	for (int i = 0; i < cities; i++) {
		for (int j = 0; j < cities; j++) {
			cin >> dist[i][j];
	}
	}
	int a, b;
	for (int i = 0; i < q; i++) {
		cin >> a >> b;
	}
	return recurse(0);
}

int main() {
	int t;
	cin >> t;
	for (int c = 0; c < t; c++) {
		const auto ans = do_case();
		cout << setprecision(11);
		cout << "Case #" << (c+1) << ": " << ans << endl;
	}
}
