#pragma once
#include <string>

struct Node {
    int value;
    Node* left;
    Node* right;

    Node(int val) : value(val), left(nullptr), right(nullptr) {}
};

struct NodeQueue {
    Node* val;
    NodeQueue* next = nullptr;
};

struct Queue {
    NodeQueue* head = nullptr;
    NodeQueue* tail = nullptr;
};

void queue_push(Queue& q, Node* val);
Node* queue_pop(Queue& q, bool& ok);
void queue_clear(Queue& q);
//std::string queue_get(Queue& q);
int queue_size(const Queue& q);

