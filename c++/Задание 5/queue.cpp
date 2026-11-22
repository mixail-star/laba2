#include "queue.h"

void queue_push(Queue& q, Node* val) {
    NodeQueue* n = new NodeQueue{val, nullptr};
    if (!q.head) {
        q.head = n;
        q.tail = n;
    } else {
        q.tail->next = n;
        q.tail = n;
    }
}

Node* queue_pop(Queue& q, bool& ok) {
    if (!q.head) {
        ok = false;
        return nullptr;
    }
    ok = true;
    NodeQueue* temp = q.head;
    Node* v = temp->val;
    q.head = q.head->next;
    if (!q.head) {
        q.tail = nullptr;
    }
    delete temp;
    return v;
}

int queue_size(const Queue& q) {
    int count = 0;
    NodeQueue* temp = q.head;
    while (temp) {
        count++;
        temp = temp->next;
    }
    return count;
}

void queue_clear(Queue& q) {
    while (q.head) {
        NodeQueue* temp = q.head;
        q.head = q.head->next;
        delete temp;
    }
    q.tail = nullptr;
}
