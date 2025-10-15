use chrono::{DateTime, Local, Months, NaiveDate, Utc};

use crate::{
    context::ApplicationContext, errors::DatabaseError, models::{AttendanceRecord, Workplace}
};

pub(super) struct TargetMonth {
    pub(super) year: i32,
    pub(super) month: u32,
}

impl TargetMonth {
    pub(super) fn new(year: &i32, month: &u32) -> Self {
        let m = month % 12;
        let y: i32 = (month / 12).try_into().unwrap();
        Self {
            year: year + y,
            month: m,
        }
    }

    fn datetime_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let timezone = Local::now().timezone();

        let local_start_time = NaiveDate::from_ymd_opt(self.year, self.month, 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_local_timezone(timezone).unwrap();
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
    pub(super) fn new(context: &'a ApplicationContext, workplace: &'a Workplace, target_month: &'a TargetMonth) -> Self {
        Self { context, workplace, target_month }
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
