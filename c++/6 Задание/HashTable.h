#ifndef HASHTABLE_H
#define HASHTABLE_H

#include <vector>
#include <string>
#include <cstddef>

enum class ProbingMethod {
    LINEAR,
    DOUBLE_HASHING
};

enum class EntryState {
    EMPTY,
    OCCUPIED,
    DELETED
};

template <typename K, typename V>
struct HashEntry {
    K key;
    V value;
    EntryState state = EntryState::EMPTY;
};

template <typename K, typename V>
class HashTable {
private:
    std::vector<HashEntry<K, V>> table;
    size_t capacity;
    size_t currentSize;
    size_t totalCollisions; // Счетчик коллизий для эмпирического анализа
    ProbingMethod method;

    size_t hash1(const K& key) const;
    size_t hash2(const K& key) const;
    size_t getIndex(const K& key, size_t attempt) const;
    void rehash();

public:
    explicit HashTable(size_t initialCapacity = 101, ProbingMethod method = ProbingMethod::LINEAR);

    bool insert(const K& key, const V& value);
    bool find(const K& key, V& valueOut) const;
    bool remove(const K& key);

    size_t size() const;
    size_t getCapacity() const;
    size_t getCollisions() const;
    double loadFactor() const;
    void clear();
};

#endif // HASHTABLE_H
