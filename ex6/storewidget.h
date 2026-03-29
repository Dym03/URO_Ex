#pragma once

#include <QWidget>
#include <QVBoxLayout>
#include <QScrollArea>

#include "bike.h"
#include "bikecard.h"

class StoreWidget : public QWidget
{
    Q_OBJECT
public:
    explicit StoreWidget(QWidget *parent, std::vector<Bike>& inventory);

    void set_inventory(std::vector<Bike>& inventory);
private:
    QVBoxLayout* layout;
    std::vector<Bike>& inventory;
    void create_items();
signals:
};
