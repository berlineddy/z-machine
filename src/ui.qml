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

    Rectangle {
        id: title
        height: 40
        anchors.left: parent.left
        anchors.leftMargin: 0
        anchors.right: parent.right
        anchors.rightMargin: 0
        anchors.top: parent.top
        anchors.topMargin: 0

        RowLayout {
            id: rowLayout1
            spacing: 5
            anchors.bottom: parent.bottom
            anchors.bottomMargin: 0
            anchors.top: parent.top
            anchors.topMargin: 0
            anchors.right: parent.right
            anchors.rightMargin: 0
            anchors.left: parent.left
            anchors.leftMargin: 0

            Rectangle {
                id: rectangle1
                anchors.right: parent.horizontalCenter
                anchors.rightMargin: 0
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.top: parent.top

                ComboBox  {
                    id: listViewVolume
                    anchors.fill: parent
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

                ComboBox  {
                    id: listViewSnapshots
                    anchors.fill: parent
                    model: snap_model
                    textRole: "name"
                    onCurrentIndexChanged: {
                        app_callback.snapshot_index_changed(currentIndex);
                    }
                }

            }

        }
    }



}
