#include "stack.h"

using namespace std;

void stack_push(Stack& s, const string& val) {
    NodeStack* n = new NodeStack{val, s.top};
    s.top = n;
}

string stack_pop(Stack& s, bool& ok) {
    if (!s.top) {
        ok = false;
        return "";
    }
    ok = true;
    NodeStack* temp = s.top;
    string v = temp->val;
    s.top = s.top->next;
    delete temp;
    return v;
}

string stack_get(const Stack& s) {
    if (s.top == nullptr) {
        return "Нет такой структуры";
    }
    NodeStack* temp = s.top;
    string returnString = "";
    while (temp) {
        returnString = returnString + " " + temp->val;
        temp = temp->next;
    }
    return returnString;
}

void stack_clear(Stack& s) {
    while (s.top) {
        NodeStack* temp = s.top;
        s.top = s.top->next;
        delete temp;
    }
}
