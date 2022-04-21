//! Record related types used for deserializing HTTP requests and serializing HTTP responses.

use super::{Component, ComponentTest, ValidName};
use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker, StringFaker};
#[cfg(test)]
use quickcheck;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecordAdd {
    pub record_id: ValidName,
    pub site_id: ValidName,
    pub user_id: ValidName,
    pub group_id: ValidName,
    pub components: Vec<Component>,
    pub start_time: DateTime<Utc>,
    pub stop_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecordUpdate {
    pub record_id: ValidName,
    pub site_id: ValidName,
    pub user_id: ValidName,
    pub group_id: ValidName,
    pub components: Vec<Component>,
    pub start_time: Option<DateTime<Utc>>,
    pub stop_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    pub record_id: String,
    pub site_id: Option<String>,
    pub user_id: Option<String>,
    pub group_id: Option<String>,
    pub components: Option<Vec<Component>>,
    pub start_time: DateTime<Utc>,
    pub stop_time: Option<DateTime<Utc>>,
    pub runtime: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RecordTest {
    pub record_id: Option<String>,
    pub site_id: Option<String>,
    pub user_id: Option<String>,
    pub group_id: Option<String>,
    pub components: Option<Vec<ComponentTest>>,
    pub start_time: Option<DateTime<Utc>>,
    pub stop_time: Option<DateTime<Utc>>,
}

impl RecordTest {
    pub fn new() -> Self {
        RecordTest::default()
    }

    pub fn with_record_id<T: AsRef<str>>(mut self, record_id: T) -> Self {
        self.record_id = Some(record_id.as_ref().to_string());
        self
    }

    pub fn with_site_id<T: AsRef<str>>(mut self, site_id: T) -> Self {
        self.site_id = Some(site_id.as_ref().to_string());
        self
    }

    pub fn with_user_id<T: AsRef<str>>(mut self, user_id: T) -> Self {
        self.user_id = Some(user_id.as_ref().to_string());
        self
    }

    pub fn with_group_id<T: AsRef<str>>(mut self, group_id: T) -> Self {
        self.group_id = Some(group_id.as_ref().to_string());
        self
    }

    pub fn with_component<T: AsRef<str>>(mut self, name: T, amount: i64, factor: f64) -> Self {
        if self.components.is_none() {
            self.components = Some(vec![])
        }
        self.components.as_mut().unwrap().push(ComponentTest {
            name: Some(name.as_ref().to_string()),
            amount: Some(amount),
            factor: Some(factor),
        });
        self
    }

    pub fn with_start_time<T: AsRef<str>>(mut self, start_time: T) -> Self {
        self.start_time = Some(
            DateTime::parse_from_rfc3339(start_time.as_ref())
                .unwrap()
                .with_timezone(&Utc),
        );
        self
    }

    pub fn with_stop_time<T: AsRef<str>>(mut self, stop_time: T) -> Self {
        self.stop_time = Some(
            DateTime::parse_from_rfc3339(stop_time.as_ref())
                .unwrap()
                .with_timezone(&Utc),
        );
        self
    }
}

impl Dummy<Faker> for RecordTest {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> RecordTest {
        let fakename = || -> String {
            StringFaker::with(
                String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789*&^%$#@!~").into_bytes(),
                1..256,
            )
            .fake()
        };
        let fakeamount = || (0..i64::MAX).fake();
        let fakefactor = || (0.0..f64::MAX).fake();
        let fakedate = || -> DateTime<Utc> { Faker.fake() };

        let mut out = RecordTest::new()
            .with_record_id(fakename())
            .with_site_id(fakename())
            .with_user_id(fakename())
            .with_group_id(fakename())
            .with_start_time(fakedate().to_rfc3339())
            .with_stop_time(fakedate().to_rfc3339());
        for _ in 0..(1..10).fake_with_rng(rng) {
            out = out.with_component(fakename(), fakeamount(), fakefactor());
        }
        out
    }
}

impl PartialEq<Record> for RecordTest {
    fn eq(&self, other: &Record) -> bool {
        let RecordTest {
            record_id: s_rid,
            site_id: s_sid,
            user_id: s_uid,
            group_id: s_gid,
            components: s_comp,
            start_time: s_start,
            stop_time: s_stop,
        } = self;
        let Record {
            record_id: o_rid,
            site_id: o_sid,
            user_id: o_uid,
            group_id: o_gid,
            components: o_comp,
            start_time: o_start,
            stop_time: o_stop,
            runtime: _,
        } = other;

        // Can't be equal if record ID and start_time are not set in `RecordTest`.
        if s_rid.is_none() || s_start.is_none() {
            return false;
        }

        let s_start = s_start.as_ref().unwrap();

        let start_diff = if s_start > o_start {
            *s_start - *o_start
        } else {
            *o_start - *s_start
        };

        let s_stop = s_stop.as_ref().unwrap();
        let o_stop = o_stop.as_ref().unwrap();

        let stop_diff = if s_stop > o_stop {
            *s_stop - *o_stop
        } else {
            *o_stop - *s_stop
        };

        s_rid.as_ref().unwrap() == o_rid
            && s_sid == o_sid
            && s_uid == o_uid
            && s_gid == o_gid
            && start_diff < chrono::Duration::milliseconds(1)
            && stop_diff < chrono::Duration::milliseconds(1)
            && ((s_comp.is_none() && o_comp.is_none())
                || (s_comp.as_ref().unwrap().len() == o_comp.as_ref().unwrap().len()
                    && s_comp
                        .as_ref()
                        .unwrap()
                        .iter()
                        .zip(o_comp.as_ref().unwrap().iter())
                        .fold(true, |acc, (a, b)| acc && (a == b))))
    }
}

impl PartialEq<RecordTest> for Record {
    fn eq(&self, other: &RecordTest) -> bool {
        other.eq(self)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for RecordTest {
    fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
        Faker.fake()
    }
}
