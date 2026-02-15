use chrono::{Datelike, Months, NaiveDate, Utc};
use chrono_tz::{Asia, Tz};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::DatabaseError,
    models::{AttendanceRecord, Timestamp, Workplace},
    repositories::RepositoryFactory,
};

#[derive(Clone, Copy, Deserialize, Serialize)]
#[repr(transparent)]
pub(super) struct Year(i32);

impl From<Year> for i32 {
    fn from(value: Year) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[repr(transparent)]
pub(super) struct Month(u32);

impl From<Month> for u32 {
    fn from(value: Month) -> Self {
        value.0
    }
}

pub(super) struct TargetMonth {
    pub(super) year: Year,
    pub(super) month: Month,

    timezone: Tz,
}

impl TargetMonth {
    pub(super) fn new_with_default_timezone(year_opt: Option<i32>, month_opt: Option<u32>) -> Self {
        let timezone = Asia::Tokyo;
        let now = Utc::now().with_timezone(&timezone);

        let year = year_opt.unwrap_or(now.year());
        let month = month_opt.unwrap_or(now.month());

        Self {
            year: Year(year),
            month: Month(month),

            timezone,
        }
    }

    fn datetime_range(&self) -> (Timestamp, Timestamp) {
        let timezone = self.timezone;

        let local_start_time = NaiveDate::from_ymd_opt(self.year.into(), self.month.into(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(timezone)
            .unwrap();
        let local_end_time = local_start_time.checked_add_months(Months::new(1)).unwrap();

        let utc_start_time = local_start_time.to_utc().into();
        let utc_end_time = local_end_time.to_utc().into();
        (utc_start_time, utc_end_time)
    }
}

pub(super) struct AttendancesForMonth<'a> {
    app_state: &'a AppState,
    workplace: &'a Workplace,
    target_month: &'a TargetMonth,
}

impl<'a> AttendancesForMonth<'a> {
    pub(super) fn new(
        app_state: &'a AppState,
        workplace: &'a Workplace,
        target_month: &'a TargetMonth,
    ) -> Self {
        Self {
            app_state,
            workplace,
            target_month,
        }
    }

    pub(super) async fn execute(self) -> Result<Vec<AttendanceRecord>, DatabaseError> {
        let (start, end) = self.target_month.datetime_range();
        let repository = self.app_state.repositories.attendance_record();
        let attendance_records = repository.list(self.workplace, &start, &end).await?;
        Ok(attendance_records)
    }
}
