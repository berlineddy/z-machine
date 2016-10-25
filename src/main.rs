use std::sync::{Arc, Mutex};
mod zfs;
use zfs::*;

#[macro_use]
extern crate qml;
use qml::*;

Q_LISTMODEL_ITEM!{
    pub VolumeModel<VolumeItem> {
        name: String,
        path: String,
    }
}
impl VolumeItem {
    pub fn new<S: Into<String>>(name: S, path: S) -> VolumeItem {
        VolumeItem {
            name: name.into(),
            path: path.into(),
        }
    }
}


Q_LISTMODEL_ITEM!{
    pub SnapshotModel<SnapshotItem> {
        name: String,
        path: String,
    }
}
impl SnapshotItem {
    pub fn new<S: Into<String>>(name: S, path: S) -> SnapshotItem {
        SnapshotItem {
            name: name.into(),
            path: path.into(),
        }
    }
}


pub struct AppController {
    volume_model: Arc<Mutex<VolumeModel>>,
    snapshot_model: Arc<Mutex<SnapshotModel>>,
}
impl AppController {
    fn new(vol: Arc<Mutex<VolumeModel>>,
           snap: Arc<Mutex<SnapshotModel>>)
           -> AppController {
        AppController {
            volume_model: vol,
            snapshot_model: snap,
        }
    }
    pub fn on_volume_index_changed(&self, index: usize) {
        let volumes = get_volumes();
        if ! volumes.len() == 0
        {
            let ref mut s: SnapshotModel =
                *self.snapshot_model.as_ref().lock().expect("snapshot_model locked!");

            s.clear();
            let snapshots = get_snapshots(volumes[index].path.clone());
            for snapshot in snapshots {
                s.insert_item(SnapshotItem::new(snapshot.name.clone(), snapshot.path.clone()));
            }
        }
    }
    pub fn on_snapshot_index_changed(&self, index: usize) {}
}


Q_OBJECT!{
    pub AppController as QAppCallback {
        signals:
        slots:
            fn volume_index_changed(index: i32);
            fn snapshot_index_changed(index: i32);
        properties:
    }
}
impl QAppCallback {
    pub fn volume_index_changed(&self, index: i32) -> Option<&QVariant> {
        (*self.origin).on_volume_index_changed(index as usize);
        None
    }
    pub fn snapshot_index_changed(&self, index: i32) -> Option<&QVariant> {
        (*self.origin).on_snapshot_index_changed(index as usize);
        None
    }
}


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
