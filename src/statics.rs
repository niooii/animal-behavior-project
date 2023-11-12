use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::LanternFly;
use crate::Transform;
use crate::Fork;

lazy_static!(
    pub static ref FLIES: Mutex<Vec<LanternFly>> = {
        let v = Vec::<LanternFly>::new();
        
        Mutex::new(v)
    };

    pub static ref FIRES: Mutex<Vec<Transform>> = {
        let v = Vec::<Transform>::new();

        Mutex::new(v)
    };

    pub static ref FORK: Mutex<Option<Fork>> = Mutex::new(None);
);