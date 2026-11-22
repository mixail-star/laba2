#include <iostream>
#include <string>
#include "HashTable.h"
#include "llist.h"

using namespace std;

class LRUCache {
private:
    int capacity;
    int currentSize;
    HashTable<int, int> cacheMap; 
    List orderList;               

public:
    LRUCache(int cap) 
        : capacity(cap), currentSize(0), cacheMap(cap * 2) {
        orderList.head = nullptr;
        orderList.tail = nullptr;
    }

    ~LRUCache() {
        list_clear(orderList);
    }

    int get(int x) {
        int val = 0;
        if (!cacheMap.find(x, val)) {
            return -1;
        }

        string keyStr = to_string(x);
        list_pop_value(orderList, keyStr);
        list_push_head(orderList, keyStr);

        return val;
    }

    void set(int x, int y) {
        if (capacity <= 0) return;

        int existingVal = 0;
        string keyStr = to_string(x);

        if (cacheMap.find(x, existingVal)) {
            cacheMap.insert(x, y); 
            list_pop_value(orderList, keyStr);
            list_push_head(orderList, keyStr);
            return;
        }

        if (currentSize == capacity) {
            string oldestKeyStr = list_pop_tail(orderList);
            int oldestKey = stoi(oldestKeyStr);
            cacheMap.remove(oldestKey);
            --currentSize;
        }

        cacheMap.insert(x, y);
        list_push_head(orderList, keyStr);
        ++currentSize;
    }
};

int main() {
    int cap, Q;
    if (!(cin >> cap >> Q)) return 0;

    LRUCache cache(cap);

    for (int i = 0; i < Q; ++i) {
        string type;
        cin >> type;
        if (type == "SET") {
            int x, y;
            cin >> x >> y;
            cache.set(x, y);
        } else if (type == "GET") {
            int x;
            cin >> x;
            cout << cache.get(x) << " ";
        }
    }
    cout << endl;

    return 0;
}
