#pragma once

#include <QWidget>
#include <QVBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QFrame>

#include "bike.h"


class BikeCard : public QFrame {
    Q_OBJECT
public:
    explicit BikeCard(const Bike& bike, int h, int w, QWidget *parent = nullptr);
};
