pub use std::sync::{Arc, Mutex};
use std::fs::read_dir;
use std::fmt::Write;
use std::mem;

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

Q_LISTMODEL_ITEM!{
    pub FilesystemModel<FilesystemItem> {
        name: String,
        filetype: String,
        color: String,
        atime: String,
    }
}
impl FilesystemItem {
    pub fn new<S: Into<String>>(name: S, filetype: S, color: S, atime: S) -> FilesystemItem {
        FilesystemItem {
            name: name.into(),
            filetype: filetype.into(),
            color: color.into(),
            atime: atime.into(),
        }
    }
    pub fn new_from<S: Into<String>>(path: S) -> Vec<FilesystemItem> {
        let mut res: Vec<FilesystemItem> = vec![];
        res.push(FilesystemItem::new("..", "folder", "white", "today"));

        if let Ok(entries) = read_dir(path.into()) {
            for entry in entries {
                if let Ok(entry) = entry {
                    res.push(FilesystemItem::new(entry.file_name()
                                                     .to_str()
                                                     .expect("name not readable"),
                                                 "folder",
                                                 "white",
                                                 "today"));
                }
            }
        }

        res
    }
}


pub struct AppController {
    volume_model: Arc<Mutex<VolumeModel>>,
    snapshot_model: Arc<Mutex<SnapshotModel>>,
    current_fs_model: Arc<Mutex<FilesystemModel>>,
    selected_fs_model: Arc<Mutex<FilesystemModel>>,
    cwd: Arc<Mutex<String>>,
}
impl AppController {
    pub fn new(vol: Arc<Mutex<VolumeModel>>,
               snap: Arc<Mutex<SnapshotModel>>,
               cur: Arc<Mutex<FilesystemModel>>,
               sel: Arc<Mutex<FilesystemModel>>)
               -> AppController {
        AppController {
            volume_model: vol,
            snapshot_model: snap,
            current_fs_model: cur,
            selected_fs_model: sel,
            cwd: Arc::new(Mutex::new("/".into())),
        }
    }
    pub fn on_volume_index_changed(&self, index: usize) {
        let volumes = get_volumes();
        if volumes.len() > 0 {
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
        let ref mut c: FilesystemModel =
            *self.current_fs_model.as_ref().lock().expect("current_fs_model locked!");
        let ref mut cwd: String = *self.cwd.as_ref().lock().expect("current_fs_model locked!");
        c.clear();
        for entry in FilesystemItem::new_from(cwd.clone()) {
            c.insert_item(entry);
        }
    }

    pub fn on_current_fs_index_double_clicked(&self, index: usize) {
        let ref mut c: FilesystemModel =
            *self.current_fs_model.as_ref().lock().expect("current_fs_model locked!");
        let ref mut cwd: String = *self.cwd.as_ref().lock().expect("current_fs_model locked!");

        let files = FilesystemItem::new_from(cwd.clone());
        assert!(files.len() > index);
        let pwd = cwd.clone();
        mem::replace(cwd,format!("{:?}/{:?}",pwd,files[index]));
        c.clear();
        for entry in files {
            c.insert_item(entry);
        }
    }
}


Q_OBJECT!{
    pub AppController as QAppCallback {
        signals:
        slots:
            fn volume_index_changed(index: i32);
            fn snapshot_index_changed(index: i32);
            fn current_fs_index_changed(index: i32);
            fn current_fs_index_double_clicked(index: i32);
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
    pub fn current_fs_index_changed(&self, index: i32) -> Option<&QVariant> {
        println!("{:#?}", index);
        None
    }
    pub fn current_fs_index_double_clicked(&self, index: i32) -> Option<&QVariant> {
        (*self.origin).on_current_fs_index_double_clicked(index as usize);
        None
    }
}
