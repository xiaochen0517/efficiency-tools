use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum PomodoroTimeMode {
    Short,
    Medium,
    Long,
}

pub struct PomodoroTime {
    pub work_time: i32,
    pub short_rest_time: i32,
    pub long_rest_time: i32,
}

impl PomodoroTimeMode {
    pub fn get_config(&self) -> PomodoroTime {
        match self {
            Self::Short => PomodoroTime {
                work_time: 25 * 60,      // 25分钟
                short_rest_time: 5 * 60, // 5分钟
                long_rest_time: 15 * 60, // 15分钟
            },
            Self::Medium => PomodoroTime {
                work_time: 45 * 60,
                short_rest_time: 10 * 60,
                long_rest_time: 20 * 60,
            },
            Self::Long => PomodoroTime {
                work_time: 60 * 60,
                short_rest_time: 15 * 60,
                long_rest_time: 30 * 60,
            },
        }
    }
}
