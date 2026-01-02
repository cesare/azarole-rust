use chrono::{DateTime, Datelike, Months, NaiveDate, Utc};
use chrono_tz::{Asia, Tz};
use serde::{Deserialize, Serialize};

use crate::{
    context::ApplicationContext,
    errors::DatabaseError,
    models::{AttendanceRecord, Workplace},
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

    fn datetime_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let timezone = self.timezone;

        let local_start_time = NaiveDate::from_ymd_opt(self.year.into(), self.month.into(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(timezone)
            .unwrap();
        let local_end_time = local_start_time.checked_add_months(Months::new(1)).unwrap();

        let utc_start_time = local_start_time.to_utc();
        let utc_end_time = local_end_time.to_utc();
        (utc_start_time, utc_end_time)
    }
}

pub(super) struct AttendancesForMonth<'a> {
    context: &'a ApplicationContext,
    workplace: &'a Workplace,
    target_month: &'a TargetMonth,
}

impl<'a> AttendancesForMonth<'a> {
    pub(super) fn new(
        context: &'a ApplicationContext,
        workplace: &'a Workplace,
        target_month: &'a TargetMonth,
    ) -> Self {
        Self {
            context,
            workplace,
            target_month,
        }
    }

    pub(super) async fn execute(self) -> Result<Vec<AttendanceRecord>, DatabaseError> {
        let (start, end) = self.target_month.datetime_range();
        let statement = "select id, workplace_id, event, recorded_at from attendance_records where workplace_id = $1 and recorded_at >= $2 and recorded_at < $3 order by recorded_at";
        let attendance_records: Vec<AttendanceRecord> = sqlx::query_as(statement)
            .bind(self.workplace.id)
            .bind(start)
            .bind(end)
            .fetch_all(&self.context.database.pool)
            .await?;
        Ok(attendance_records)
    }
}
