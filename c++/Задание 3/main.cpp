#include <iostream>
#include <string>

using namespace std;

struct Set {
    string* data = nullptr;
    size_t size = 0;
    size_t capacity = 0;
};

void set_resize(Set& s) {
    s.capacity = (s.capacity == 0) ? 4 : s.capacity * 2;
    string* new_data = new string[s.capacity];
    for (size_t i = 0; i < s.size; ++i) {
        new_data[i] = s.data[i];
    }
    delete[] s.data;
    s.data = new_data;
}

bool set_contains(const Set& s, const string& val) {
    for (size_t i = 0; i < s.size; ++i) {
        if (s.data[i] == val) {
            return true;
        }
    }
    return false;
}

bool set_add(Set& s, const string& val) {
    if (set_contains(s, val)) {
        return false;
    }
    if (s.size >= s.capacity) {
        set_resize(s);
    }
    s.data[s.size++] = val;
    return true;
}

void set_clear(Set& s) {
    delete[] s.data;
    s.data = nullptr;
    s.size = 0;
    s.capacity = 0;
}

int main() {
    string genome1, genome2;
    if (!(cin >> genome1 >> genome2)) {
        return 0;
    }

    Set second_pairs;

    for (size_t i = 0; i + 1 < genome2.size(); ++i) {
        set_add(second_pairs, genome2.substr(i, 2));
    }

    int closeness = 0;
    for (size_t i = 0; i + 1 < genome1.size(); ++i) {
        if (set_contains(second_pairs, genome1.substr(i, 2))) {
            closeness++;
        }
    }

    cout << closeness << "\n";
    set_clear(second_pairs);

    return 0;
}
