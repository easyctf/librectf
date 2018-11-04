use chrono::NaiveDateTime;
use diesel::{debug_query, dsl::sql, prelude::*};
use diesel::{
    mysql::Mysql,
    sql_types::{Datetime, Integer},
};
use failure::Error;

use super::DbConn;

// TODO: make this a config option later
const RESULTS_PER_PAGE: i64 = 30;

#[derive(Serialize, Deserialize)]
pub struct ScoreboardOptions {
    #[serde(default)]
    page: i64,
}

#[derive(Debug, Queryable, Serialize)]
pub struct ScoreboardEntry {
    score: i32,
    #[serde(skip)]
    _last_update: NaiveDateTime,
    teamname: String,
}

/// Gets a public scoreboard using the options provided.
pub fn get_scoreboard(
    db: DbConn,
    options: &ScoreboardOptions,
) -> Result<Vec<ScoreboardEntry>, Error> {
    use core::schema::{chals, solves, teams};

    let query = solves::table
        .inner_join(teams::table)
        .inner_join(chals::table)
        .select((
            chals::value,
            teams::name,
            solves::chal_id,
            solves::team_id,
            solves::timestamp,
        )).filter(sql("TRUE GROUP BY `solves`.`chal_id`, `solves`.`team_id`"));

    let query = query
        .select((
            sql::<Integer>("SUM(`chals`.`value`) AS `score`"),
            sql::<Datetime>("MAX(`solves`.`timestamp`) AS `last_update`"),
            teams::name,
        )).order((
            sql::<Integer>("`score`").desc(),
            sql::<Datetime>("`last_update`"),
        ));

    let query = query
        .limit(RESULTS_PER_PAGE)
        .offset((options.page - 1) * RESULTS_PER_PAGE);

    let dbg = debug_query::<Mysql, _>(&query).to_string();
    info!("Debug scoreboard query: {}", dbg);

    // TOOD: don't unwrap
    query
        .load::<ScoreboardEntry>(&*db)
        .map_err(|err| err.into())
}
