macro_rules! route_any {
    ($hm:ident $hp:tt => $h:expr $(, $tm:ident $tp:tt => $t:expr)* $(,)*) => {
        route_any!(@internal @path $hm $hp).and($h)
            $(.or(route_any!(@internal @path $tm $tp).and($t)))*
    };

    (@internal @path GET ()) => {{ warp::get2() }};
    (@internal @path POST ()) => {{ warp::post2() }};
    (@internal @path $m:ident $p:tt) => {{
        use warp::path;
        route_any!(@internal @path $m ()).and(path! $p)
    }};
}

macro_rules! Resp {
    () => { impl Clone + ::warp::Filter<Extract = (impl ::warp::Reply,), Error = ::warp::Rejection> }
}
