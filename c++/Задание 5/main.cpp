#include <iostream>
#include "queue.h"

using namespace std;

Node* insert(Node* root, int val) {
    if (root == nullptr) {
        return new Node(val);
    }
    if (val < root->value) {
        root->left = insert(root->left, val);
    } else {
        root->right = insert(root->right, val);
    }
    return root;
}

void printRightSideView(Node* root) {
    if (root == nullptr) {
        return;
    }

    Queue q;
    queue_push(q, root);
    bool isFirstPrinted = true;

    cout << "Результат: ";

    while (q.head != nullptr) {
        int levelSize = queue_size(q); 

        for (int i = 0; i < levelSize; ++i) {
            bool ok = false;
            Node* current = queue_pop(q, ok);

            if (!ok || current == nullptr) continue;

            if (i == levelSize - 1) {
                if (!isFirstPrinted) {
                    cout << ", ";
                }
                cout << current->value;
                isFirstPrinted = false;
            }

            if (current->left != nullptr) {
                queue_push(q, current->left);
            }
            if (current->right != nullptr) {
                queue_push(q, current->right);
            }
        }
    }
    cout << endl;
    
    queue_clear(q);
}

int main() {
    Node* root = nullptr;

    int values[] = {8, 5, 10, 3, 9, 15, 12};
    for (int val : values) {
        root = insert(root, val);
    }

    printRightSideView(root);

    return 0;
}
