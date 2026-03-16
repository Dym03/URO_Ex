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
    void create_form();

private:
    Ui::MainWindow *ui;
};
#endif // MAINWINDOW_H
