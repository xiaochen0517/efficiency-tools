pub struct HMS {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
}

pub fn sec_to_hms(seconds: i32) -> HMS {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    HMS {
        hours: h,
        minutes: m,
        seconds: s,
    }
}
