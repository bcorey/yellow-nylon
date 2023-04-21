use dioxus::prelude::*;

pub fn use_database_path(cx: &ScopeState) -> UseSharedState<Option<String>> {
    use_shared_state::<Option<String>>(cx).expect("Could not read database_path").clone()
}

pub fn use_dist_path(cx: &ScopeState) -> UseSharedState<Option<String>> {
    use_shared_state::<Option<String>>(cx).expect("Could not read dist path").clone()
}