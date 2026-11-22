#include <iostream>
#include <vector>
#include <string>
#include <functional>
#include <chrono>
#include "HashTable.h"

using namespace std;


template <typename K>
class BloomFilter {
private:
    vector<bool> bitArray;
    size_t bitSize;
    size_t numHashes;

    size_t nthHash(const K& key, size_t i) const {
        size_t h1 = hash<K>{}(key);
        size_t h2 = (h1 >> 16) ^ (h1 * 0x45d9f3b);
        if (h2 == 0) h2 = 1;
        return (h1 + i * h2) % bitSize;
    }

public:
    BloomFilter(size_t size = 10000, size_t hashCount = 4)
        : bitSize(size), numHashes(hashCount) {
        bitArray.assign(bitSize, false);
    }

    void add(const K& key) {
        for (size_t i = 0; i < numHashes; ++i) {
            bitArray[nthHash(key, i)] = true;
        }
    }

    bool possiblyContains(const K& key) const {
        for (size_t i = 0; i < numHashes; ++i) {
            if (!bitArray[nthHash(key, i)]) {
                return false; 
            }
        }
        return true; 
    }

    void clear() {
        fill(bitArray.begin(), bitArray.end(), false);
    }
};

template <typename K, typename V>
class HashTableWithBloom {
private:
    HashTable<K, V> table;
    BloomFilter<K> filter;
    mutable size_t rejectedByBloom = 0; 

public:
    HashTableWithBloom(size_t initialCapacity, ProbingMethod method, size_t bloomBits, size_t bloomHashes)
        : table(initialCapacity, method), filter(bloomBits, bloomHashes) {}

    bool insert(const K& key, const V& value) {
        if (table.insert(key, value)) {
            filter.add(key); 
            return true;
        }
        return false;
    }

    bool find(const K& key, V& valueOut) const {
        if (!filter.possiblyContains(key)) {
            ++rejectedByBloom;
            return false; 
        }
        return table.find(key, valueOut);
    }

    size_t getRejectedCount() const {
        return rejectedByBloom;
    }
};


int main() {

    const size_t N = 50000;
    HashTableWithBloom<string, int> smartTable(1009, ProbingMethod::DOUBLE_HASHING, N * 10, 4);

    for (size_t i = 0; i < N; ++i) {
        smartTable.insert("user_" + to_string(i), static_cast<int>(i));
    }

    int val = 0;
    if (smartTable.find("user_777", val)) {
        cout << "Ключ 'user_777' найден! Значение: " << val << "\n";
    }

    size_t foundFalsePositives = 0;
    auto start = chrono::high_resolution_clock::now();

    for (size_t i = 0; i < N; ++i) {
        if (smartTable.find("guest_" + to_string(i), val)) {
            ++foundFalsePositives;
        }
    }

    auto end = chrono::high_resolution_clock::now();
    chrono::duration<double, milli> elapsed = end - start;

    cout << "\n=== Результаты поиска " << N << " отсутствующих ключей ===\n";
    cout << "Время выполнения: " << elapsed.count() << " мс\n";
    cout << "Отсечено фильтром Блума (без поиска в таблице): "
              << smartTable.getRejectedCount() << " из " << N << "\n";
    cout << "Найдено ложных элементов: " << foundFalsePositives << "\n";

    return 0;
}
