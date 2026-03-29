#include "bikecard.h"

BikeCard::BikeCard(const Bike& bike, int h, int w, QWidget *parent) : QFrame(parent) {
    this->setObjectName("formCard");
    this->setFixedSize(w, h);

    QVBoxLayout* layout = new QVBoxLayout(this);
    layout->setContentsMargins(20, 20, 20, 20);
    layout->setSpacing(4);

    QLabel* brand_lbl = new QLabel(QString::fromStdString(bike.brand).toUpper());
    brand_lbl->setObjectName("cardLabel");

    QLabel* name_lbl = new QLabel(QString::fromStdString(bike.model));
    name_lbl->setObjectName("formTitle");
    name_lbl->setWordWrap(true);

    QLabel* specs_lbl = new QLabel(QString("%1 | %2\" Wheel")
                                       .arg(QString::fromStdString(bike.type))
                                       .arg(QString::fromStdString(bike.wheel_size)));
    specs_lbl->setObjectName("cardLabel");

    layout->addWidget(brand_lbl);
    layout->addWidget(name_lbl);
    layout->addWidget(specs_lbl);

    layout->addStretch();

    QHBoxLayout* bottom_row = new QHBoxLayout();

    QLabel* price_lbl = new QLabel(QString("$%1").arg(bike.price));
    price_lbl->setStyleSheet("color: #3b82f6; font-size: 22px; font-weight: bold; background: #282c37;");

    QPushButton* view_btn = new QPushButton("View Details");

    view_btn->setStyleSheet("QPushButton { "
                            "background: transparent; "
                            "color: #ffffff; "
                            "font-weight: bold; "
                            "font-size: 14px; "
                            "border: none; "
                            "text-align: right; "
                            "} "
                            "QPushButton:hover { color: #3b82f6; }");
    view_btn->setCursor(Qt::PointingHandCursor);

    bottom_row->addWidget(price_lbl);
    bottom_row->addStretch();
    bottom_row->addWidget(view_btn);

    layout->addLayout(bottom_row);
}
