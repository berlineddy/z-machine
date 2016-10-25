mod zfs;
mod appctrl;

use zfs::*;
use appctrl::*;

#[macro_use]
extern crate qml;
use qml::{QmlEngine,QObjectMacro};



fn main() {
    let mut engine = QmlEngine::new();
    let vol_model = Arc::new(Mutex::new(VolumeModel::new()));
    let snap_model = Arc::new(Mutex::new(SnapshotModel::new()));


    let volumes = get_volumes();
    for volume in volumes {
        (*vol_model.as_ref().lock().expect("vol_model locked!"))
            .insert_item(VolumeItem::new(volume.name.clone(), volume.path.clone()));
    }

    let appcallback = QAppCallback::new(AppController::new(vol_model.clone(), snap_model.clone()));
    engine.set_and_store_property("app_callback", appcallback.get_qobj());

    engine.set_property("vol_model",
                        &(*vol_model.as_ref().lock().expect("vol_model locked!")).get_qvar());
    engine.set_property("snap_model",
                        &(*snap_model.as_ref().lock().expect("snap_model locked!")).get_qvar());

    engine.load_data(include_str!("ui.qml"));
    engine.exec();
    engine.quit();
}
