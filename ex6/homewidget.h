#pragma once

#include <QWidget>
#include <QVBoxLayout>
#include <QStandardItemModel>
#include <QTableView>
#include <QHeaderView>
#include <QPushButton>
#include <QLineEdit>
#include <QComboBox>
#include <QTextEdit>
#include <QFrame>
#include <QLabel>
#include <QDebug>
#include "bike.h"

class HomeWidget : public QWidget {
    Q_OBJECT
public:
    explicit HomeWidget(QWidget *parent, std::vector<Bike>& inventory);

private:
    QVBoxLayout* layout;
    QStandardItemModel* model;
    std::vector<Bike>& inventory;

    void create_action_bar();
    void create_table();
    void create_form();

private slots:
    void on_item_selected(QModelIndex idx);
};
