#include "homewidget.h"

HomeWidget::HomeWidget(QWidget *parent, std::vector<Bike>& inventory) : QWidget(parent), inventory(inventory) {
    this->layout = new QVBoxLayout(this);
    this->layout->setContentsMargins(0, 0, 0, 0);
    this->layout->setSpacing(10);

    create_action_bar();
    create_table();
    create_form();
}

void HomeWidget::create_action_bar() {
    QHBoxLayout * action_bar_layout = new QHBoxLayout();
    action_bar_layout->setAlignment(Qt::AlignTop);
    QPushButton* load_btn = new QPushButton("Load Data");
    load_btn->setObjectName("primaryBtn");
    QPushButton* save_btn = new QPushButton("Save Data");
    save_btn->setObjectName("primaryBtn");
    QPushButton* delete_btn = new QPushButton("Delete Selected");
    delete_btn->setObjectName("dangerBtn");

    action_bar_layout->addWidget(save_btn);
    action_bar_layout->addWidget(load_btn);
    action_bar_layout->addWidget(delete_btn);
    action_bar_layout->addStretch();
    this->layout->addLayout(action_bar_layout);
}

void HomeWidget::create_table() {
    QFrame* table_container = new QFrame();
    table_container->setObjectName("formCard");
    QVBoxLayout* table_layout = new QVBoxLayout(table_container);

    std::vector<QString> headers = {"Brand", "Name", "Type", "Size", "Wheel Size", "Stock", "Price"};
    this->model = new QStandardItemModel(0, headers.size(), this);

    for (size_t i = 0; i < headers.size(); i++) {
        this->model->setHeaderData(i, Qt::Horizontal, headers[i]);
    }

    QTableView* table = new QTableView();
    table->setModel(this->model);
    table->horizontalHeader()->setSectionResizeMode(QHeaderView::Stretch);
    table->verticalHeader()->setVisible(false);
    table->setEditTriggers(QAbstractItemView::NoEditTriggers);
    table->setSelectionBehavior(QAbstractItemView::SelectRows);

    connect(table, SIGNAL(clicked(QModelIndex)), this, SLOT(on_item_selected(QModelIndex)));

    table_layout->addWidget(table);
    for (auto bike : this->inventory) {
        model->appendRow(bike.to_row());
    }
    this->layout->addWidget(table_container, 1);
}

void HomeWidget::on_item_selected(QModelIndex idx) {
    if (idx.isValid()) {
        int row = idx.row();
        for (int col = 0; col < model->columnCount(); ++col) {
            QStandardItem *item = model->item(row, col);
            if (item) qDebug() << "Column" << col << ":" << item->text();
        }
    }
}

void HomeWidget::create_form() {
    QFrame* form_container = new QFrame();
    form_container->setObjectName("formCard");

    QGridLayout* form_layout = new QGridLayout(form_container);
    form_layout->setContentsMargins(20, 20, 20, 20);
    form_layout->setSpacing(15);

    QLabel* form_name = new QLabel("Add to inventory");
    form_name->setObjectName("formTitle");
    form_layout->addWidget(form_name, 0, 0);

    QLineEdit* brand_input = new QLineEdit();
    brand_input->setPlaceholderText("Brand");
    QLineEdit* model_input = new QLineEdit();
    model_input->setPlaceholderText("Model");
    form_layout->addWidget(brand_input, 1, 0);
    form_layout->addWidget(model_input, 1, 1);

    QComboBox* type_selection = new QComboBox();
    type_selection->addItems({"MTB", "Road", "Gravel"});
    form_layout->addWidget(type_selection, 1, 2);

    QComboBox* size_selection = new QComboBox();
    size_selection->addItems({"S", "M", "L"});
    form_layout->addWidget(size_selection, 1, 3);

    QTextEdit* description = new QTextEdit();
    description->setPlaceholderText("Description");
    form_layout->addWidget(description, 1, 4, 2, 4);

    QComboBox* wheel_size_selection = new QComboBox();
    wheel_size_selection->addItems({"26", "27.5", "29"});
    form_layout->addWidget(wheel_size_selection, 2, 0);

    QLineEdit* stock_input = new QLineEdit();
    stock_input->setPlaceholderText("Stock Quantity");
    QLineEdit* price_input = new QLineEdit();
    price_input->setPlaceholderText("Price");
    form_layout->addWidget(stock_input, 2, 1);
    form_layout->addWidget(price_input, 2, 2, 1, 2);

    QPushButton* clear_btn = new QPushButton("Clear form");
    clear_btn->setObjectName("secondaryBtn");
    form_layout->addWidget(clear_btn, 3, 5);


    QPushButton* add_btn = new QPushButton("Add to inventory");
    add_btn->setObjectName("primaryBtn");
    form_layout->addWidget(add_btn, 3, 6, 1, 2);

    this->layout->addWidget(form_container);
}
