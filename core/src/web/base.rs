use actix_web::{App, HttpResponse};
use chrono::NaiveDateTime;
use diesel::{debug_query, dsl::sql, prelude::*};
use diesel::{
    mysql::Mysql,
    sql_types::{Datetime, Integer},
};

use super::{DbConn, State};

pub fn app(state: State) -> App<State> {
    App::with_state(state).resource("/scoreboard", |r| r.with(scoreboard))
}

fn scoreboard(db: DbConn) -> HttpResponse {
    use schema::{chals, solves, teams};

    #[derive(Queryable, Serialize)]
    struct ScoreboardEntry {
        score: i32,
        #[serde(skip)]
        _last_update: NaiveDateTime,
        teamname: String,
    }

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

    let dbg = debug_query::<Mysql, _>(&query).to_string();

    // TOOD: don't unwrap
    let results: Vec<ScoreboardEntry> = query.load(&*db).unwrap();
    HttpResponse::Ok().json(json!({ "query": dbg, "results": results }))
}
