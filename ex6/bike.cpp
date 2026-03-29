#include "bike.h"

// Initialize the static ID counter outside the class
int Bike::next_id = 0;

// Equivalent to __str__ and __repr__
std::string Bike::to_string() const{
    return brand + " - " + model + " - " + size;
}

QList<QStandardItem*> Bike::to_row() const {
    QList<QStandardItem*> row;
    row.append(new QStandardItem(QString::fromStdString(this->brand)));
    row.append(new QStandardItem(QString::fromStdString(this->model)));
    row.append(new QStandardItem(QString::fromStdString(this->type)));
    row.append(new QStandardItem(QString::fromStdString(this->size)));
    row.append(new QStandardItem(QString::fromStdString(this->wheel_size)));
    row.append(new QStandardItem(QString::number(this->quantity)));
    row.append(new QStandardItem(QString::number(this->price)));

    return row;
}

// Equivalent to to_csv
std::string Bike::to_csv() const{
    return std::to_string(bike_id) + ", " + brand + ", " + model + ", " + size;
}


std::vector<Bike> parse_bikes_csv(std::string filename) {
    std::vector<Bike> inventory;
    std::ifstream file(filename);

    if (!file.is_open()) {
        throw std::runtime_error("Could not open file");
    }

    std::string line;
    // Skip the header row
    std::getline(file, line);

    while (std::getline(file, line)) {
        std::stringstream ss(line);
        std::string brand, model, type, size, wheel, stock_str, price_str;

        // Extract each column separated by a comma
        std::getline(ss, brand, ',');
        std::getline(ss, model, ',');
        std::getline(ss, type, ',');
        std::getline(ss, size, ',');
        std::getline(ss, wheel, ',');
        std::getline(ss, stock_str, ',');
        std::getline(ss, price_str, ',');

        // 1. Clean the price (remove the '$' sign if present)
        price_str.erase(std::remove(price_str.begin(), price_str.end(), '$'), price_str.end());

        // 2. Convert strings to numeric values
        int stock = std::stoi(stock_str);
        double price = std::stod(price_str);

        // 3. Create Bike object and add to list
        // Note: we map your CSV columns to the Bike constructor parameters
        inventory.emplace_back(brand, model, size, type, wheel, stock, price);
    }

    file.close();
    return inventory;
}
