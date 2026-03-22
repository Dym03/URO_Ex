#pragma once

#include <string>
#include <vector>
#include <random>
#include <map>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <QStandardItem>
#include <QString>
#include <QList>

class Bike {
public:
    // Static member to track the next available ID
    static int next_id;

    // Instance variables
    int bike_id;
    std::string brand;
    std::string model;
    std::string size;
    std::string type;
    std::string wheel_size;
    int quantity;
    double price;
    std::string description;

    // Constructor with default arguments
    Bike(std::string b, std::string m, std::string s,
         std::string t = "", std::string w = "",
         int q = 0, double p = 0.0, std::string d = "")
        : brand(b), model(m), size(s), type(t),
        wheel_size(w), quantity(q), price(p), description(d) {

        this->bike_id = Bike::next_id++;
    }

    // Static method to create a new random bike
    // Note: This assumes BRAND_MODEL and SIZE are defined elsewhere
    static Bike create_random(const std::map<std::string, std::vector<std::string>>& brand_model,
                              const std::vector<std::string>& sizes) {

        // Random engine setup
        std::random_device rd;
        std::mt19937 gen(rd());

        // Select random brand
        auto it = brand_model.begin();
        std::advance(it, std::uniform_int_distribution<>(0, brand_model.size() - 1)(gen));
        std::string brand = it->first;

        // Select random model
        std::string model = it->second[std::uniform_int_distribution<>(0, it->second.size() - 1)(gen)];

        // Select random size
        std::string size = sizes[std::uniform_int_distribution<>(0, sizes.size() - 1)(gen)];

        return Bike(brand, model, size);
    }

    // Equivalent to __str__ and __repr__
    std::string to_string() const;

    QList<QStandardItem*> to_row() const;

    // Equivalent to to_csv
    std::string to_csv() const;
};

std::vector<Bike> parse_bikes_csv(std::string filename);
