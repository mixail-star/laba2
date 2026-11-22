#include "llist.h"

using namespace std;


void list_push_head(List& l, const string& val) {
    NodeL* n = new NodeL{val, l.head, nullptr};
    if (l.head) {
        l.head->prev = n;
    } else {
        l.tail = n;
    }
    l.head = n;
}

void list_push_tail(List& l, const string& val) {
    NodeL* n = new NodeL{val, nullptr, l.tail};
    if (l.tail) {
        l.tail->next = n;
    } else {
        l.head = n;
    }
    l.tail = n;
}

bool list_push_before(List& l, const string& val, const string& target) {
    NodeL* curr = l.head;
    while (curr) {
        if (curr->val == target) {
            NodeL* n = new NodeL{val, curr, curr->prev};
            if (curr->prev) {
                curr->prev->next = n;
            } else {
                l.head = n;
            }
            curr->prev = n;
            return true;
        }
        curr = curr->next;
    }
    return false;
}

bool list_push_after(List& l, const string& val, const string& target) {
    NodeL* curr = l.head;
    while (curr) {
        if (curr->val == target) {
            NodeL* n = new NodeL{val, curr->next, curr};
            if (curr->next) {
                curr->next->prev = n;
            } else {
                l.tail = n;
            }
            curr->next = n;
            return true;
        }
        curr = curr->next;
    }
    return false;
}


string list_pop_head(List& l) {
    if (!l.head) return "Empty";
    NodeL* temp = l.head;
    string res = temp->val;
    
    l.head = l.head->next;
    if (l.head) {
        l.head->prev = nullptr;
    } else {
        l.tail = nullptr;
    }
    
    delete temp;
    return res;
}

string list_pop_tail(List& l) {
    if (!l.tail) return "Empty";
    NodeL* temp = l.tail;
    string res = temp->val;
    
    l.tail = l.tail->prev;
    if (l.tail) {
        l.tail->next = nullptr;
    } else {
        l.head = nullptr;
    }
    
    delete temp;
    return res;
}

string list_pop_value(List& l, const string& target) {
    NodeL* curr = l.head;
    while (curr) {
        if (curr->val == target) {
            if (curr->prev) curr->prev->next = curr->next; else l.head = curr->next;
            if (curr->next) curr->next->prev = curr->prev; else l.tail = curr->prev;
            delete curr;
            return "OK";
        }
        curr = curr->next;
    }
    return "NOT_FOUND";
}

string list_pop_before(List& l, const string& target) {
    NodeL* curr = l.head;
    while (curr) {
        if (curr->val == target) {
            NodeL* temp = curr->prev;
            if (!temp) return "FALSE";
            
            if (temp->prev) temp->prev->next = curr; else l.head = curr;
            curr->prev = temp->prev;
            
            delete temp;
            return "OK";
        }
        curr = curr->next;
    }
    return "NOT_FOUND";
}

string list_pop_after(List& l, const string& target) {
    NodeL* curr = l.head;
    while (curr) {
        if (curr->val == target) {
            NodeL* temp = curr->next;
            if (!temp) return "FALSE";
            
            if (temp->next) temp->next->prev = curr; else l.tail = curr;
            curr->next = temp->next;
            
            delete temp;
            return "OK";
        }
        curr = curr->next;
    }
    return "NOT_FOUND";
}

string list_get(List& l) {
    if (l.head == nullptr && l.tail == nullptr) {
        return "Нет такой структуры";
    }
    NodeL* temp = l.head;
    string returnStr = "";
    while (temp)
    {   
        returnStr = returnStr + " " + temp->val;
        temp = temp->next;
    }
    return returnStr;
    
}

NodeL* list_find(List& l, const string& target) {
    NodeL* curr = l.head;
    while (curr) {
        if (curr->val == target) {
            return curr;
        }
        curr = curr->next;
    }
    return nullptr;
}


void list_clear(List& l) { 
    while (l.head) { 
        NodeL* t = l.head; 
        l.head = l.head->next; 
        delete t; 
    } 
    l.tail = nullptr; 
}
