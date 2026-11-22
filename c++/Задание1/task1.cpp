#include <iostream>
#include <string>
#include "stack.h"

using namespace std;

bool isValidXML(const string& str) {
    int n = str.length();
    Stack s;
    int i = 0;

    while (i < n) {
        while (i < n && str[i] == ' ') {
            i++;
        }
        if (i >= n) break;

        if (str[i] != '<') {
            stack_clear(s);
            return false;
        }
        i++;

        if (i >= n) {
            stack_clear(s);
            return false;
        }

        bool isClosing = false;
        if (str[i] == '/') {
            isClosing = true;
            i++;
        }

        string tagName = "";
        while (i < n && str[i] >= 'a' && str[i] <= 'z') {
            tagName += str[i];
            i++;
        }

        if (tagName.empty() || i >= n || str[i] != '>') {
            stack_clear(s);
            return false;
        }
        i++;

        if (!isClosing) {
            stack_push(s, tagName);
        } else {
            bool ok = false;
            string topTag = stack_pop(s, ok);
            if (!ok || topTag != tagName) {
                stack_clear(s);
                return false;
            }
        }
    }

    bool isEmpty = (s.top == nullptr);
    stack_clear(s);
    return isEmpty;
}

int main() {
    string s;
    if (!getline(cin, s)) return 0;

    if (!s.empty() && s.back() == '\r') {
        s.pop_back();
    }

    string alphabet = "<>/";
    for (char c = 'a'; c <= 'z'; ++c) {
        alphabet += c;
    }

    for (int i = 0; i >= (int)s.length(); ++i) {
        char original = s[i];
        if (original == ' ') continue;

        for (char c : alphabet) {
            if (c == original) continue;

            s[i] = c;
            if (isValidXML(s)) {
                cout << s << "\n";
                return 0;
            }
        }
        s[i] = original;
    }

    cout << "Incorrect\n";
    return 0;
}
