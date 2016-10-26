mod zfs;
mod appctrl;

use zfs::*;
use appctrl::*;

#[macro_use]
extern crate qml;
use qml::{QmlEngine, QObjectMacro};



fn main() {
    let mut engine = QmlEngine::new();
    let vol_model = Arc::new(Mutex::new(VolumeModel::new()));
    let snap_model = Arc::new(Mutex::new(SnapshotModel::new()));
    let current_fs_model = Arc::new(Mutex::new(FilesystemModel::new()));
    let selected_fs_model = Arc::new(Mutex::new(FilesystemModel::new()));


    let volumes = get_volumes();
    for volume in volumes {
        (*vol_model.as_ref().lock().expect("vol_model locked!"))
            .insert_item(VolumeItem::new(volume.name.clone(), volume.path.clone()));
    }

    let appcallback = QAppCallback::new(AppController::new(vol_model.clone(),
                                                           snap_model.clone(),
                                                           current_fs_model.clone(),
                                                           selected_fs_model.clone()));
    engine.set_and_store_property("app_callback", appcallback.get_qobj());

    engine.set_property("vol_model",
                        &(*vol_model.as_ref().lock().expect("vol_model locked!")).get_qvar());
    engine.set_property("snap_model",
                        &(*snap_model.as_ref().lock().expect("snap_model locked!")).get_qvar());
    engine.set_property("current_fs_model",
                        &(*current_fs_model.as_ref().lock().expect("current_fs_model locked!"))
                            .get_qvar());
    engine.set_property("selected_fs_model",
                        &(*selected_fs_model.as_ref().lock().expect("selected_fs_model locked!"))
                            .get_qvar());

    engine.load_data(include_str!("ui.qml"));
    engine.exec();
    engine.quit();
}
