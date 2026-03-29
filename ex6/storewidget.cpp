#include "storewidget.h"

StoreWidget::StoreWidget(QWidget *parent, std::vector<Bike>& inventory)
    : QWidget{parent}, inventory{inventory}
{
    this->layout = new QVBoxLayout(this);
    QScrollArea* scroll = new QScrollArea();
    scroll->setWidgetResizable(true);
    scroll->setStyleSheet("QScrollArea { background: transparent; border: none; }");

    QWidget* container = new QWidget();
    QGridLayout* grid = new QGridLayout(container);
    grid->setSpacing(20);
    grid->setAlignment(Qt::AlignTop | Qt::AlignLeft);

    int column_count = 4; // Number of cards per row
    int parent_width = parent->width() - 200;
    for (size_t i = 0; i < inventory.size(); ++i) {
        BikeCard* card = new BikeCard(inventory[i], 250, int(parent_width / column_count), parent);
        grid->addWidget(card, i / column_count, i % column_count);
    }

    scroll->setWidget(container);
    layout->addWidget(scroll);
}
