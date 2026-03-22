#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QGridLayout>
#include <QPushButton>
#include <QLabel>
#include <QLineEdit>
#include <QComboBox>
#include <QWidget>
#include <QTextEdit>
#include <QTableView>
#include <QHeaderView>
#include <QCoreApplication>
#include <QDir>
#include <iostream>

#include "bike.h"

const std::map<std::string, std::vector<std::string>> BRAND_MODEL = {
    {"Giant",   {"Trance", "Propel", "Roadster"}},
    {"Author",  {"Pavel", "Kolo"}},
    {"Bianchi", {"Stradale", "Puppisimo"}}
};

// 2. Vectors for simple lists
const std::vector<std::string> SIZE = {"S", "M", "L", "XL"};
const std::vector<std::string> WHEEL_SIZES = {"26", "27.5", "29"};
const std::vector<std::string> BIKE_TYPE = {"City", "Road", "Gravel", "MTB", "Fixed"};

void apply_dark_theme(QWidget *window);

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    QVBoxLayout * top_level_layout{nullptr};

    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
    void create_layout();
    void create_navbar();
    void create_action_bar();
    void create_table();
    void create_form();
    void create_inventory();

    std::vector<Bike> inventory;

private:
    Ui::MainWindow *ui;
};
#endif // MAINWINDOW_H
