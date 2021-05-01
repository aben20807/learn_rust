#include <iostream>
#include <cstdio>
#include <cstdlib>
using namespace std;

struct User { // C++
    string username;
    string email;
    unsigned int sign_in_count;
    bool active;
};

int main()
{
    User *u = new User();
    u->username = "ouo";
    cout << u->username << endl;
    return 0;
}
