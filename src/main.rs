use std::sync::{Arc};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Datelike, Duration, FixedOffset, Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Deserialize, Serialize, Debug)]
struct NewCourseResp {
    data: Vec<NewCourseRespData>
}

#[derive(Deserialize, Serialize, Debug)]
struct NewCourseRespData {
    id: i32,
    name: String,
    season_id: i32,
    period_id: i32,
    score: i32,
    #[serde(rename = "type")]
    ty: i32,
    admin_id: String,
    sort: Option<String>,
    study_number: i32,
    cover: String,
    link: String,
    status: i32,
    publish_time: String,
    is_study: i32,
    created_at: String,
    updated_at: String
}

#[derive(Deserialize, Serialize, Debug)]
struct StudyCourseRsp {
    status: i32,
    message: String,
    error: String,
    data: StudyCourseData
}

#[derive(Deserialize, Serialize, Debug)]
struct StudyCourseData {
    user_score: i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tz = FixedOffset::east_opt(8 * 3600).unwrap();
    loop {
        let now = Utc::now().with_timezone(&tz);
        let midnight = now.date()
            .succ_opt()
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let duration = (midnight - now).to_std().unwrap();
        sleep(duration).await;
        task().await?;
    }
}


async fn task() -> Result<(), Box<dyn std::error::Error>> {
    let openid = std::env::var("OPENID")?;

    // 拿到最新的课程数据
    let time = SystemTime::now();
    let timestamp = time.duration_since(UNIX_EPOCH)?.as_secs();
    let resp: NewCourseResp = reqwest::get(format!("http://qndxx.cqyouths.com/new_course.json?time={}", timestamp))
        .await?
        .json()
        .await?;
    let data = resp.data.get(0).unwrap();
    // 进行打卡
    let resp: StudyCourseRsp = reqwest::get(format!("http://qndxx.cqyouths.com/api/course/studyCourse?openid={}&id={}", openid, data.id))
        .await?
        .json()
        .await?;

    println!("{:?}", resp);
    Ok(())
}