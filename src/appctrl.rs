pub use std::sync::{Arc, Mutex};

use zfs::*;
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
    pub fn new(vol: Arc<Mutex<VolumeModel>>,
           snap: Arc<Mutex<SnapshotModel>>)
           -> AppController {
        AppController {
            volume_model: vol,
            snapshot_model: snap,
        }
    }
    pub fn on_volume_index_changed(&self, index: usize) {
        let volumes = get_volumes();
        if  volumes.len() > 0
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
    pub fn on_snapshot_index_changed(&self, index: usize) {
        
    }
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
