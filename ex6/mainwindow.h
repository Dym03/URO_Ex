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
#include <QDialog>
#include <QTextBlock>
#include <iostream>
#include <QIcon>
#include <QStackedWidget>

#include "bike.h"
#include "homewidget.h"
#include "storewidget.h"

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
    std::vector<Bike> inventory;
    QStackedWidget* view_stack;
    QStandardItemModel* model;
    QDialog* info_window;

    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
    void create_layout();
    void create_navbar();
    void create_inventory();
    void create_menu();
    void create_actions();


public slots:
    void on_info_clicked();
    void on_send_clicked();
    void on_home_clicked();
    void on_store_clicked();

private:
    Ui::MainWindow *ui;

    QMenu *fileMenu;
    QMenu *editMenu;
    QMenu *formatMenu;
    QMenu *helpMenu;

    QActionGroup *alignmentGroup;
    QAction *newAct;
    QAction *openAct;
    QAction *saveAct;
    QAction *undoAct;
    QAction *redoAct;
};
#endif // MAINWINDOW_H
