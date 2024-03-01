/*
 * 
 */

#include<stdio.h>
#include<iostream> 
#include<vector>
#include<stack> 
#include<queue> 
#include<algorithm> 

using namespace std; 

int uwu;

#define FOR(i,lb,ub) for(int i=lb ; i<ub ; ++i)
#define ROF(i,ub,lb) for(int i=ub ; i>lb ; --i) 
#define pb(el) push_back(el) 
#define ri(i) uwu = scanf("%d",&i)
#define rii(i,j) uwu = scanf("%d %d", &i, &j) 
#define rl(l) uwu = scanf("%lld",&l)
#define rll(l,m) uwu = scanf("%lld %lld",&l, &m) 
#define SZ(el) el.size()
#define fst first
#define snd second

const int MAXI = 0x7fffffff;

typedef pair<int, int> pi ; 
typedef pair<int, pi> pii; 
typedef vector<long long> vl; 
typedef vector<char> vc; 
typedef vector<bool> vb; 
typedef vector<int> vi; 
typedef vector<pi> vii; 
typedef long long ll;

//GC

struct STATE {
    int curr, rem, first, second, idx;
    bool start_0;
    STATE(int curr, int rem) 
        : 
    curr(curr), start_0(true), rem(rem), first(1), second(2), idx(1) {};

    void next() {
        //cout<<"\tpre:\n";
        //cout<<"\tcurr: "<<curr<<" rem: "<<rem<<" start_0: " << (start_0 ? "yes":"no")<<'\n';

        if (start_0) 
            first = second;
        second = first + (1<<idx);

        ++idx;

        if (rem & 1) 
        {
            if (start_0) 
                curr = (rem + 1) >> 1;
            else 
                curr = rem >> 1;
            start_0 = !start_0;
        }
        else curr = rem >> 1;

        rem = rem - curr;

        //cout<<"\tpost:\n";
        //cout<<"\tcurr: "<<curr<<" rem: "<<rem<<" start_0: " << (start_0 ? "yes":"no")<<'\n';
    }

    int find_kth(int k) {
        int acc = 1;

        // Position on propper range
        while (acc + curr < k) 
        {
            acc += curr;
            next();
        }


        // Set step to move within range. here __range__ refers to `curr` amount of values you move corresponding 
        // `idx` iteration
        // Set first value for `output`, corresponding to first element in range
        int step = (1<<(idx-1)), output;

        if (start_0) {
            output = first;
        }
        else {
            output = second;
        }
        cout<<"DBG: acc:"<<acc<<" curr:"<<curr<<" step:"<<step<<" output_initial:"<<output<<'\n';

        while (acc < k)
        {
            ++acc;
            output += step;
        } // ! There might be overflow here

        return  output;
    }

    int return_next_idx(const vector<bool> &ref) const {
        int cnt = 0;
        ssize_t tmp = 0;

        ssize_t i;
        for (i = 0; i<ref.size() && cnt < 2; ++i)  {
            if (start_0) {
                if (!ref[i])
                    cnt = 2;
            }
            else {
                if (!ref[i]) {
                    if (rem == 1)
                        cnt = 2;
                    else
                        ++cnt;
                }
            }
        }

        return i-1;
    }

    void dbg() {
        cout<<"State: {curr:"<<curr<<", rem:"<<rem<<", start_0:"<<(start_0?"yes":"no")<<"}\n";
    }

};
int main() { 
    uint64_t n, k;
    cin>>n>>k;
    cout<<"n is: "<<n<<'\n';

    vector<bool> help(n, false);

    struct STATE st(0, n);

    cout<<"kth josephus is "<<st.find_kth(k)<<'\n';

    return 42;
    
    int it =1;

    while (st.rem > 0)
    {
        // DBG current mask state
        cout<<"-->";
        for (auto el : help)
            cout<<(el ? '1' : '0');
        cout<<' ';

        // DBG next idx to choose
        int idx = st.return_next_idx(help);
        cout<<"Next start point idx: "<<idx+1<<" val: "<<(help[idx] ? '1' : '0')<<'\n';

        st.dbg();

        // Mark 
        for (int i = idx; i < n; i += (1<<it))
            help[i] = true;

        ++it; st.next();
    }

    // DBG final mask state
    cout<<"-->";
    for (auto el : help)
        cout<<(el ? '1' : '0');
    cout<<"\n\n";

}

