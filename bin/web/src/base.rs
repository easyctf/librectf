use actix_web::{App, HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use diesel::{debug_query, dsl::sql, prelude::*};
use diesel::{
    mysql::Mysql,
    sql_types::{Datetime, Integer},
};

use super::{DbConn, State};

// TODO: make this a config option later
const RESULTS_PER_PAGE: i64 = 30;

pub fn app(state: State) -> App<State> {
    App::with_state(state)
        .prefix("/base")
        .resource("/scoreboard/{n}", |r| r.with(scoreboard))
        .resource("/scoreboard", |r| r.with(scoreboard))
}

#[derive(Queryable, Serialize)]
struct ScoreboardEntry {
    score: i32,
    #[serde(skip)]
    _last_update: NaiveDateTime,
    teamname: String,
}

fn scoreboard((req, db): (HttpRequest<State>, DbConn)) -> HttpResponse {
    use openctf_core::schema::{chals, solves, teams};

    let params = req.match_info();
    let page: i64 = match params.query("n") {
        Ok(n) if n >= 1 => n,
        _ => 1,
    };

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
        .offset((page - 1) * RESULTS_PER_PAGE);

    let dbg = debug_query::<Mysql, _>(&query).to_string();

    // TOOD: don't unwrap
    let results: Vec<ScoreboardEntry> = query.load(&*db).unwrap();
    HttpResponse::Ok().json(json!({ "query": dbg, "results": results }))
}
