import QtQuick 2.2
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.1
import QtQuick.Window 2.1



ApplicationWindow
{
    id: applicationWindow1
    width: 400
    height: 300
    Component.onCompleted: visible = true

    ColumnLayout {
        id: columnLayout1
        anchors.fill: parent
        spacing: 5

        Rectangle {
            id: title
            height: 40
            anchors.right: parent.right
            anchors.rightMargin: 0
            anchors.left: parent.left
            anchors.leftMargin: 0
            anchors.top: parent.top
            anchors.topMargin: 0
            antialiasing: true

            RowLayout {
                id: rowLayout1
                anchors.fill: parent
                spacing: 5

                Rectangle {
                    id: rectangle1
                    anchors.right: parent.horizontalCenter
                    anchors.rightMargin: 0
                    anchors.bottom: parent.bottom
                    anchors.left: parent.left
                    anchors.top: parent.top

                    Label {
                        id: txtVol
                        text: qsTr("Volume:  ")
                        verticalAlignment: Text.AlignVCenter
                        horizontalAlignment: Text.AlignHCenter
                        anchors.left: parent.left
                        anchors.leftMargin: 10
                        anchors.bottom: parent.bottom
                        anchors.bottomMargin: 0
                        anchors.top: parent.top
                        anchors.topMargin: 0
                    }

                    ComboBox  {
                        id: listViewVolume
                        anchors.left: txtVol.right
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        anchors.top: parent.top
                        anchors.leftMargin: 0
                        antialiasing: true
                        model: vol_model
                        textRole: "name"
                        onCurrentIndexChanged: {
                            app_callback.volume_index_changed(currentIndex);
                        }

                    }

                }

                Rectangle {
                    id: rectangle2
                    anchors.bottom: parent.bottom
                    anchors.bottomMargin: 0
                    anchors.top: parent.top
                    anchors.topMargin: 0
                    anchors.left: parent.horizontalCenter
                    anchors.leftMargin: 0
                    anchors.right: parent.right
                    anchors.rightMargin: 0

                    Label {
                        id: txtSnap
                        text: qsTr("Snapshot:  ")
                        verticalAlignment: Text.AlignVCenter
                        horizontalAlignment: Text.AlignHCenter
                        anchors.left: parent.left
                        anchors.leftMargin: 10
                        anchors.bottom: parent.bottom
                        anchors.bottomMargin: 0
                        anchors.top: parent.top
                        anchors.topMargin: 0
                    }

                    ComboBox  {
                        id: listViewSnapshots
                        anchors.left: txtSnap.right
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        anchors.top: parent.top
                        anchors.leftMargin: 0
                        model: snap_model
                        textRole: "name"
                        onCurrentIndexChanged: {
                            app_callback.snapshot_index_changed(currentIndex);
                        }
                    }

                }

            }
        }

        Rectangle {
            id: rectangle5
            anchors.top: title.bottom
            anchors.topMargin: 0
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            anchors.left: parent.left


            ListModel {
                id: libraryModel

                ListElement {
                    title: "A Masterpiece"
                    author: "Gabriel"
                }

                ListElement {
                    title: "Brilliance"
                    author: "Jens"
                }

                ListElement {
                    title: "Outstanding"
                    author: "Frederik"
                }
            }


            TableView {
                id: tbLeft
                anchors.right: parent.horizontalCenter
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.top: parent.top
                anchors.rightMargin: 0

                TableViewColumn {
                    role: "title"
                    title: "Title"
                    width: 100
                }

                TableViewColumn {
                    role: "author"
                    title: "Author"
                    width: 100
                }

                model: libraryModel

                onClicked: {
                    tbRight.selection.clear()
                    tbRight.selection.select(row);
                }

            }
            TableView {
                id: tbRight
                anchors.left: parent.horizontalCenter
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                anchors.top: parent.top
                anchors.leftMargin: 0


                TableViewColumn {
                    role: "title"
                    title: "Title"
                    width: 100
                }

                TableViewColumn {
                    role: "author"
                    title: "Author"
                    width: 100
                }

                model: libraryModel
                onClicked: {
                    tbLeft.selection.clear()
                    tbLeft.selection.select(row);
                }

            }
        }
    }
}
