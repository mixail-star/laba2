#pragma once

#include <string>

struct NodeL {
    std::string val;
    NodeL* next;
    NodeL* prev;
};

struct List {
    NodeL* head = nullptr;
    NodeL* tail = nullptr;
};

void list_clear(List& l);

void list_push_head(List& l, const std::string& val);
void list_push_tail(List& l, const std::string& val);
bool list_push_before(List& l, const std::string& val, const std::string& target);
bool list_push_after(List& l, const std::string& val, const std::string& target);

std::string list_pop_head(List& l);
std::string list_pop_tail(List& l);
std::string list_pop_value(List& l, const std::string& target);
std::string list_pop_before(List& l, const std::string& target);
std::string list_pop_after(List& l, const std::string& target);
std::string list_get(List& l);
NodeL* list_find(List& l, const std::string& target);


