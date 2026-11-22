#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

using namespace std;

struct Node {
    string key;
    Node* next;
    Node(const string& k) : key(k), next(nullptr) {}
};

class HashSet {
private:
    static const size_t TABLE_SIZE = 1009; 
    Node* table[TABLE_SIZE];


    size_t hashFunction(const string& key) const {
        size_t hash = 0;
        const size_t p = 31;
        for (char c : key) {
            hash = (hash * p + static_cast<unsigned char>(c)) % TABLE_SIZE;
        }
        return hash;
    }

public:
    HashSet() {
        for (size_t i = 0; i < TABLE_SIZE; ++i) {
            table[i] = nullptr;
        }
    }

    ~HashSet() {
        clear();
    }

    void clear() {
        for (size_t i = 0; i < TABLE_SIZE; ++i) {
            Node* curr = table[i];
            while (curr != nullptr) {
                Node* temp = curr;
                curr = curr->next;
                delete temp;
            }
            table[i] = nullptr;
        }
    }

    void add(const string& key) {
        if (contains(key)) return; // Множество хранит только уникальные элемент
        size_t idx = hashFunction(key);
        Node* newNode = new Node(key);
        newNode->next = table[idx];
        table[idx] = newNode;
    }

    // SETDEL: Удаление элемента за O(1)
    bool remove(const string& key) {
        size_t idx = hashFunction(key);
        Node* curr = table[idx];
        Node* prev = nullptr;

        while (curr != nullptr) {
            if (curr->key == key) {
                if (prev == nullptr) {
                    table[idx] = curr->next;
                } else {
                    prev->next = curr->next;
                }
                delete curr;
                return true;
            }
            prev = curr;
            curr = curr->next;
        }
        return false;
    }

    bool contains(const string& key) const {
        size_t idx = hashFunction(key);
        Node* curr = table[idx];
        while (curr != nullptr) {
            if (curr->key == key) {
                return true;
            }
            curr = curr->next;
        }
        return false;
    }

    string serialize() const {
        string result;
        for (size_t i = 0; i < TABLE_SIZE; ++i) {
            Node* curr = table[i];
            while (curr != nullptr) {
                if (!result.empty()) result += " ";
                result += curr->key;
                curr = curr->next;
            }
        }
        return result;
    }
};

struct NamedSet {
    string name;
    HashSet set;
    NamedSet* next;
    NamedSet(const string& n) : name(n), next(nullptr) {}
};

class SetStorage {
private:
    NamedSet* head = nullptr;

public:
    ~SetStorage() {
        while (head != nullptr) {
            NamedSet* temp = head;
            head = head->next;
            delete temp;
        }
    }

    NamedSet* getOrCreate(const string& name) {
        NamedSet* curr = head;
        while (curr != nullptr) {
            if (curr->name == name) return curr;
            curr = curr->next;
        }
        NamedSet* newSet = new NamedSet(name);
        newSet->next = head;
        head = newSet;
        return newSet;
    }

    void loadFromFile(const string& filename) {
        ifstream file(filename);
        if (!file.is_open()) return; // Если файла ещё нет, он создастся при сохранении

        string line;
        while (getline(file, line)) {
            if (line.empty()) continue;
            istringstream iss(line);
            string setName;
            if (!(iss >> setName)) continue;

            NamedSet* ns = getOrCreate(setName);
            string item;
            while (iss >> item) {
                ns->set.add(item);
            }
        }
        file.close();
    }

    void saveToFile(const string& filename) const {
        ofstream file(filename);
        if (!file.is_open()) {
            cerr << "Ошибка: не удалось открыть файл для записи." << endl;
            return;
        }

        NamedSet* curr = head;
        while (curr != nullptr) {
            string items = curr->set.serialize();
            if (!items.empty()) {
                file << curr->name << " " << items << "\n";
            }
            curr = curr->next;
        }
        file.close();
    }
};

int main(int argc, char* argv[]) {
    string filename;
    string query;

    for (int i = 1; i < argc; ++i) {
        string arg = argv[i];
        if (arg == "--file" && i + 1 < argc) {
            filename = argv[++i];
        } else if (arg == "--query" && i + 1 < argc) {
            query = argv[++i];
        }
    }

    if (filename.empty() || query.empty()) {
        return 1;
    }

    istringstream qss(query);
    string command, setName, value;
    qss >> command >> setName >> value;

    if (command.empty() || setName.empty() || value.empty()) {
        return 1;
    }

    SetStorage storage;
    storage.loadFromFile(filename);
    NamedSet* targetSet = storage.getOrCreate(setName);

    if (command == "SETADD") {
        targetSet->set.add(value);
        storage.saveToFile(filename);
        cout << "OK" << endl;
    } else if (command == "SETDEL") {
        if (targetSet->set.remove(value)) {
            storage.saveToFile(filename);
            cout << "OK" << endl;
        } else {
            cout << "NOT FOUND" << endl;
        }
    } else if (command == "SET_AT") {
        if (targetSet->set.contains(value)) {
            cout << "TRUE" << endl;
        } else {
            cout << "FALSE" << endl;
        }
    } else {
        cerr << "Неизвестная команда: " << command << endl;
        return 1;
    }

    return 0;
}
