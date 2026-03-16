#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    this->resize(1400, 800);
    const QFont font = QFont("Roboto", 25);
    this->setFont(font);
    create_layout();
    create_navbar();
    create_action_bar();
    create_form();
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::create_layout() {
    QWidget * central_widget = new QWidget();
    this->top_level_layout = new QVBoxLayout();
    central_widget->setLayout(this->top_level_layout);
    this->setCentralWidget(central_widget);
}

void MainWindow::create_navbar() {
    QHBoxLayout * navbar_layout = new QHBoxLayout();
    navbar_layout->setAlignment(Qt::AlignTop);
    QLabel * app_name = new QLabel("Bike App");
    QPushButton* home = new QPushButton("Home");
    QPushButton* store = new QPushButton("Store");
    QPushButton* info = new QPushButton("Info");
    QLineEdit* search_bar = new QLineEdit();
    search_bar->setPlaceholderText("Search");
    navbar_layout->addWidget(app_name);
    navbar_layout->addWidget(home);
    navbar_layout->addWidget(store);
    navbar_layout->addWidget(info);
    navbar_layout->addStretch();
    navbar_layout->addWidget(search_bar);
    this->top_level_layout->addLayout(navbar_layout);
}

void MainWindow::create_action_bar() {
    QHBoxLayout * action_bar_layout = new QHBoxLayout();
    action_bar_layout->setAlignment(Qt::AlignTop);
    QPushButton* save_btn = new QPushButton("Save Data");
    QPushButton* load_btn = new QPushButton("Load Data");
    QPushButton* delete_btn = new QPushButton("Delete Selected");
    action_bar_layout->addWidget(save_btn);
    action_bar_layout->addWidget(load_btn);
    action_bar_layout->addWidget(delete_btn);
    action_bar_layout->addStretch();
    this->top_level_layout->addLayout(action_bar_layout);
}

void MainWindow::create_form() {
    QGridLayout* form_layout = new QGridLayout();

    QLabel* form_name = new QLabel("Add to inventory");
    form_layout->addWidget(form_name, 0, 0, Qt::AlignLeft);

    QLineEdit* brand_input = new QLineEdit();
    brand_input->setPlaceholderText("Brand");
    QLineEdit* model_input = new QLineEdit();
    model_input->setPlaceholderText("Model");
    form_layout->addWidget(brand_input, 1, 0, Qt::AlignLeft);
    form_layout->addWidget(model_input, 1, 1, Qt::AlignLeft);

    QComboBox* type_selection = new QComboBox();
    type_selection->addItems({"MTB", "Road", "Gravel"});
    form_layout->addWidget(type_selection, 1, 2, Qt::AlignLeft);

    QComboBox* size_selection = new QComboBox();
    size_selection->addItems({"S", "M", "L"});
    form_layout->addWidget(size_selection, 1, 3, Qt::AlignLeft);

    QTextEdit* description = new QTextEdit();
    description->setPlaceholderText("Description");
    form_layout->addWidget(description, 1, 4, 2, 4, Qt::AlignLeft);

    QComboBox* wheel_size_selection = new QComboBox();
    wheel_size_selection->addItems({"26", "27.5", "29"});
    form_layout->addWidget(wheel_size_selection, 2, 0, Qt::AlignLeft);

    QLineEdit* stock_input = new QLineEdit();
    stock_input->setPlaceholderText("Stock Quantity");
    QLineEdit* price_input = new QLineEdit();
    price_input->setPlaceholderText("Price");
    form_layout->addWidget(stock_input, 2, 1, Qt::AlignLeft);
    form_layout->addWidget(price_input, 2, 2, 1, 2,  Qt::AlignLeft);

    QPushButton* clear_btn = new QPushButton("Clear form");
    form_layout->addWidget(clear_btn, 3, 5, Qt::AlignLeft);

    QPushButton* add_btn = new QPushButton("Add to inventory");
    form_layout->addWidget(add_btn, 3, 6, 1, 2, Qt::AlignLeft);

    this->top_level_layout->addLayout(form_layout);
}
