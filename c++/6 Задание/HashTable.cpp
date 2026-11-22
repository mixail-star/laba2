#include "HashTable.h"
#include <functional>
#include <stdexcept>

// Вспомогательная функция поиска следующего простого числа для размера таблицы
static bool isPrime(size_t n) {
    if (n <= 1) return false;
    if (n <= 3) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    for (size_t i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return false;
    }
    return true;
}

static size_t nextPrime(size_t n) {
    while (!isPrime(n)) {
        ++n;
    }
    return n;
}

template <typename K, typename V>
HashTable<K, V>::HashTable(size_t initialCapacity, ProbingMethod method)
    : capacity(nextPrime(initialCapacity)), currentSize(0), totalCollisions(0), method(method) {
    table.resize(capacity);
}

template <typename K, typename V>
size_t HashTable<K, V>::hash1(const K& key) const {
    return std::hash<K>{}(key) % capacity;
}

template <typename K, typename V>
size_t HashTable<K, V>::hash2(const K& key) const {
    // Вторая хеш-функция не должна возвращать 0; используем простое число меньше capacity
    size_t prime = (capacity > 7) ? 7 : 3;
    return prime - (std::hash<K>{}(key) % prime);
}

template <typename K, typename V>
size_t HashTable<K, V>::getIndex(const K& key, size_t attempt) const {
    size_t h1 = hash1(key);
    if (method == ProbingMethod::LINEAR) {
        return (h1 + attempt) % capacity;
    } else {
        size_t h2 = hash2(key);
        return (h1 + attempt * h2) % capacity;
    }
}

template <typename K, typename V>
void HashTable<K, V>::rehash() {
    std::vector<HashEntry<K, V>> oldTable = table;
    capacity = nextPrime(capacity * 2);
    table.assign(capacity, HashEntry<K, V>{});
    currentSize = 0;

    for (const auto& entry : oldTable) {
        if (entry.state == EntryState::OCCUPIED) {
            insert(entry.key, entry.value);
        }
    }
}

template <typename K, typename V>
bool HashTable<K, V>::insert(const K& key, const V& value) {
    if (loadFactor() >= 0.7) {
        rehash();
    }

    size_t firstDeletedIndex = capacity;

    for (size_t attempt = 0; attempt < capacity; ++attempt) {
        size_t idx = getIndex(key, attempt);

        if (table[idx].state == EntryState::OCCUPIED) {
            if (table[idx].key == key) {
                table[idx].value = value; // Обновление значения
                return true;
            }
            ++totalCollisions; // Фиксируем коллизию при занятой ячейке
        } else if (table[idx].state == EntryState::DELETED) {
            if (firstDeletedIndex == capacity) {
                firstDeletedIndex = idx;
            }
        } else { // EntryState::EMPTY
            size_t targetIdx = (firstDeletedIndex != capacity) ? firstDeletedIndex : idx;
            table[targetIdx].key = key;
            table[targetIdx].value = value;
            table[targetIdx].state = EntryState::OCCUPIED;
            ++currentSize;
            return true;
        }
    }

    if (firstDeletedIndex != capacity) {
        table[firstDeletedIndex].key = key;
        table[firstDeletedIndex].value = value;
        table[firstDeletedIndex].state = EntryState::OCCUPIED;
        ++currentSize;
        return true;
    }

    return false;
}

template <typename K, typename V>
bool HashTable<K, V>::find(const K& key, V& valueOut) const {
    for (size_t attempt = 0; attempt < capacity; ++attempt) {
        size_t idx = getIndex(key, attempt);

        if (table[idx].state == EntryState::EMPTY) {
            return false;
        }
        if (table[idx].state == EntryState::OCCUPIED && table[idx].key == key) {
            valueOut = table[idx].value;
            return true;
        }
    }
    return false;
}

template <typename K, typename V>
bool HashTable<K, V>::remove(const K& key) {
    for (size_t attempt = 0; attempt < capacity; ++attempt) {
        size_t idx = getIndex(key, attempt);

        if (table[idx].state == EntryState::EMPTY) {
            return false;
        }
        if (table[idx].state == EntryState::OCCUPIED && table[idx].key == key) {
            table[idx].state = EntryState::DELETED;
            --currentSize;
            return true;
        }
    }
    return false;
}

template <typename K, typename V>
size_t HashTable<K, V>::size() const { return currentSize; }

template <typename K, typename V>
size_t HashTable<K, V>::getCapacity() const { return capacity; }

template <typename K, typename V>
size_t HashTable<K, V>::getCollisions() const { return totalCollisions; }

template <typename K, typename V>
double HashTable<K, V>::loadFactor() const {
    return static_cast<double>(currentSize) / capacity;
}

template <typename K, typename V>
void HashTable<K, V>::clear() {
    table.assign(capacity, HashEntry<K, V>{});
    currentSize = 0;
    totalCollisions = 0;
}

template class HashTable<int, int>;
template class HashTable<int, std::string>;
template class HashTable<std::string, int>;
template class HashTable<std::string, std::string>;
