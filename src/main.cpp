#include <iostream>
#include <cstddef>

#include "csdemo.h"

void print_person(const Person* person);
void print_count();

int main(int, char**){
    const Version* version = csdemo_version();

    std::cout << "Library.....: " << version->name << std::endl << 
                 "Version.....: " << (int) version->major << "." << (int) version->minor << "." << (int) version->patch << std::endl <<
                 "Description.: " << version->description << std::endl <<
                 "Author......: " << version->author << std::endl <<
                 "System......: " << version->system << std::endl <<
                 "Architecture: " << version->architecture << std::endl;

    std::cout << "=============================================================" << std::endl;
    std::cout << "Storing 5 people..." << std::endl;
    std::cout << "=============================================================" << std::endl;
    for (int i = 1; i <= 5; ++i) {
        std::string name = "Person " + std::to_string(i);
        const Person *person = csdemo_person_push(name.c_str(), 10+i, Sex::MALE);
        print_person(person);
    }
    print_count();

    std::cout << "=============================================================" << std::endl;
    std::cout << "Retrieving people..." << std::endl;
    std::cout << "=============================================================" << std::endl;
    while (csdemo_person_count() > 0) {
        Person *person = csdemo_person_pop();
        if (person != nullptr) {
            print_person(person);
            // free(person); It works, but it is preferable to free the pointer in the RUST layer.
            csdemo_person_free(person);
        }
    }
    print_count();

    return EXIT_SUCCESS;
}

void print_person(const Person* person) {
    std::cout << "Person.: " << person->id << std::endl
              << "   Name: " << person->name << std::endl
              << "   Age.: " << person->age << std::endl
              << "   Sex.: " << (person->sex == Sex::MALE ? "Male" : "Female") << std::endl;
}

void print_count() {
    std::cout << std::endl;
    std::cout << "Has " << csdemo_person_count() << " people in the database." << std::endl;
    std::cout << std::endl;
}