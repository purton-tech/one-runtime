pub mod api_keys {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/api-keys")]
    pub struct Index {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/api-keys/create")]
    pub struct Create {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/api-keys/revoke/{id}")]
    pub struct Revoke {
        pub org_id: String,
        pub id: String,
    }
}

pub mod integrations {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations")]
    pub struct Index {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations/new")]
    pub struct New {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations/edit/{id}")]
    pub struct Edit {
        pub org_id: String,
        pub id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations/upsert")]
    pub struct Upsert {
        pub org_id: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/o/{org_id}/integrations/delete/{id}")]
    pub struct Delete {
        pub org_id: String,
        pub id: String,
    }
}
