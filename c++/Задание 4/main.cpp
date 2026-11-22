#include <iostream>
#include <string>
#include <sstream>
#include <cctype>
#include "array.h"

using namespace std;

string to_lower_str(const string& s) {
    string res = s;
    for (size_t i = 0; i < res.size(); ++i) {
        res[i] = tolower(static_cast<unsigned char>(res[i]));
    }
    return res;
}

int count_stresses(const string& s) {
    int count = 0;
    for (size_t i = 0; i < s.size(); ++i) {
        if (isupper(static_cast<unsigned char>(s[i]))) {
            count++;
        }
    }
    return count;
}

int main() {
    int n;
    if (!(cin >> n)) return 0;

    string dummy;
    getline(cin, dummy);

    Array dict;
    Array dict_lower;

    for (int i = 0; i < n; ++i) {
        string word;
        getline(cin, word);
        arr_push(dict, word);
        arr_push(dict_lower, to_lower_str(word));
    }

    string text;
    getline(cin, text);

    stringstream ss(text);
    string word;
    int errors = 0;

    while (ss >> word) {
        string lower_w = to_lower_str(word);
        bool in_dict = false;
        bool exact_match = false;

        for (size_t i = 0; i < dict.size; ++i) {
            bool ok1, ok2;
            string d_low = arr_get(dict_lower, i, ok1);
            string d_orig = arr_get(dict, i, ok2);

            if (d_low == lower_w) {
                in_dict = true;
                if (d_orig == word) {
                    exact_match = true;
                    break;
                }
            }
        }

        if (in_dict) {
            if (!exact_match) {
                errors++;
            }
        } else {
            if (count_stresses(word) != 1) {
                errors++;
            }
        }
    }

    cout << errors << endl;

   
    arr_clear(dict);
    arr_clear(dict_lower);

    return 0;
}
