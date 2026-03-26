// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct ListPublicHostedIntegrationsParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub org_public_id: T1,
    pub end_user_id: T2,
}
#[derive(Debug)]
pub struct CreateHostedConnectionSessionParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
> {
    pub org_id: uuid::Uuid,
    pub created_by_user_id: uuid::Uuid,
    pub created_by_api_key_id: uuid::Uuid,
    pub integration_id: uuid::Uuid,
    pub integration_slug: T1,
    pub end_user_id: T2,
    pub end_user_name: T3,
    pub end_user_email: T4,
    pub suggested_connection_name: T5,
    pub auth_type: crate::types::IntegrationAuthType,
    pub token: T6,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Debug)]
pub struct CreateApiKeyIntegrationConnectionParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
> {
    pub org_public_id: T1,
    pub integration_id: uuid::Uuid,
    pub name: T2,
    pub api_key: T3,
    pub end_user_id: T4,
    pub end_user_name: T5,
    pub end_user_email: T6,
}
#[derive(Debug)]
pub struct DisconnectPublicHostedIntegrationsParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
> {
    pub org_public_id: T1,
    pub integration_slug: T2,
    pub end_user_id: T3,
}
#[derive(Debug, Clone, PartialEq)]
pub struct HostedIntegration {
    pub id: uuid::Uuid,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub openapi_spec: String,
}
pub struct HostedIntegrationBorrowed<'a> {
    pub id: uuid::Uuid,
    pub slug: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub openapi_spec: &'a str,
}
impl<'a> From<HostedIntegrationBorrowed<'a>> for HostedIntegration {
    fn from(
        HostedIntegrationBorrowed {
            id,
            slug,
            name,
            description,
            openapi_spec,
        }: HostedIntegrationBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            name: name.into(),
            description: description.into(),
            openapi_spec: openapi_spec.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PublicHostedIntegration {
    pub id: uuid::Uuid,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub openapi_spec: String,
    pub connected: bool,
}
pub struct PublicHostedIntegrationBorrowed<'a> {
    pub id: uuid::Uuid,
    pub slug: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub openapi_spec: &'a str,
    pub connected: bool,
}
impl<'a> From<PublicHostedIntegrationBorrowed<'a>> for PublicHostedIntegration {
    fn from(
        PublicHostedIntegrationBorrowed {
            id,
            slug,
            name,
            description,
            openapi_spec,
            connected,
        }: PublicHostedIntegrationBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            name: name.into(),
            description: description.into(),
            openapi_spec: openapi_spec.into(),
            connected,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HostedConnectionSession {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct HostedConnectionSessionBorrowed<'a> {
    pub token: &'a str,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<HostedConnectionSessionBorrowed<'a>> for HostedConnectionSession {
    fn from(
        HostedConnectionSessionBorrowed { token, expires_at }: HostedConnectionSessionBorrowed<'a>,
    ) -> Self {
        Self {
            token: token.into(),
            expires_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HostedConnectionSessionContext {
    pub id: uuid::Uuid,
    pub org_public_id: String,
    pub integration_id: uuid::Uuid,
    pub integration_slug: String,
    pub integration_name: String,
    pub created_by_issuer: String,
    pub created_by_sub: String,
    pub end_user_id: String,
    pub end_user_name: String,
    pub end_user_email: String,
    pub suggested_connection_name: String,
    pub auth_type: String,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
    pub expired: bool,
    pub used: bool,
}
pub struct HostedConnectionSessionContextBorrowed<'a> {
    pub id: uuid::Uuid,
    pub org_public_id: &'a str,
    pub integration_id: uuid::Uuid,
    pub integration_slug: &'a str,
    pub integration_name: &'a str,
    pub created_by_issuer: &'a str,
    pub created_by_sub: &'a str,
    pub end_user_id: &'a str,
    pub end_user_name: &'a str,
    pub end_user_email: &'a str,
    pub suggested_connection_name: &'a str,
    pub auth_type: &'a str,
    pub expires_at: chrono::DateTime<chrono::FixedOffset>,
    pub expired: bool,
    pub used: bool,
}
impl<'a> From<HostedConnectionSessionContextBorrowed<'a>> for HostedConnectionSessionContext {
    fn from(
        HostedConnectionSessionContextBorrowed {
            id,
            org_public_id,
            integration_id,
            integration_slug,
            integration_name,
            created_by_issuer,
            created_by_sub,
            end_user_id,
            end_user_name,
            end_user_email,
            suggested_connection_name,
            auth_type,
            expires_at,
            expired,
            used,
        }: HostedConnectionSessionContextBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            org_public_id: org_public_id.into(),
            integration_id,
            integration_slug: integration_slug.into(),
            integration_name: integration_name.into(),
            created_by_issuer: created_by_issuer.into(),
            created_by_sub: created_by_sub.into(),
            end_user_id: end_user_id.into(),
            end_user_name: end_user_name.into(),
            end_user_email: end_user_email.into(),
            suggested_connection_name: suggested_connection_name.into(),
            auth_type: auth_type.into(),
            expires_at,
            expired,
            used,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreatedHostedConnection {
    pub id: uuid::Uuid,
    pub name: String,
}
pub struct CreatedHostedConnectionBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
}
impl<'a> From<CreatedHostedConnectionBorrowed<'a>> for CreatedHostedConnection {
    fn from(
        CreatedHostedConnectionBorrowed { id, name }: CreatedHostedConnectionBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DisconnectedHostedConnections {
    pub deleted_count: i64,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct HostedIntegrationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<HostedIntegrationBorrowed, tokio_postgres::Error>,
    mapper: fn(HostedIntegrationBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> HostedIntegrationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(HostedIntegrationBorrowed) -> R,
    ) -> HostedIntegrationQuery<'c, 'a, 's, C, R, N> {
        HostedIntegrationQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct PublicHostedIntegrationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<PublicHostedIntegrationBorrowed, tokio_postgres::Error>,
    mapper: fn(PublicHostedIntegrationBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> PublicHostedIntegrationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(PublicHostedIntegrationBorrowed) -> R,
    ) -> PublicHostedIntegrationQuery<'c, 'a, 's, C, R, N> {
        PublicHostedIntegrationQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct HostedConnectionSessionQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<HostedConnectionSessionBorrowed, tokio_postgres::Error>,
    mapper: fn(HostedConnectionSessionBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> HostedConnectionSessionQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(HostedConnectionSessionBorrowed) -> R,
    ) -> HostedConnectionSessionQuery<'c, 'a, 's, C, R, N> {
        HostedConnectionSessionQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct HostedConnectionSessionContextQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(
        &tokio_postgres::Row,
    ) -> Result<HostedConnectionSessionContextBorrowed, tokio_postgres::Error>,
    mapper: fn(HostedConnectionSessionContextBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> HostedConnectionSessionContextQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(HostedConnectionSessionContextBorrowed) -> R,
    ) -> HostedConnectionSessionContextQuery<'c, 'a, 's, C, R, N> {
        HostedConnectionSessionContextQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CreatedHostedConnectionQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<CreatedHostedConnectionBorrowed, tokio_postgres::Error>,
    mapper: fn(CreatedHostedConnectionBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreatedHostedConnectionQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreatedHostedConnectionBorrowed) -> R,
    ) -> CreatedHostedConnectionQuery<'c, 'a, 's, C, R, N> {
        CreatedHostedConnectionQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct DisconnectedHostedConnectionsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<DisconnectedHostedConnections, tokio_postgres::Error>,
    mapper: fn(DisconnectedHostedConnections) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> DisconnectedHostedConnectionsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(DisconnectedHostedConnections) -> R,
    ) -> DisconnectedHostedConnectionsQuery<'c, 'a, 's, C, R, N> {
        DisconnectedHostedConnectionsQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct GetSystemIntegrationBySlugStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_system_integration_by_slug() -> GetSystemIntegrationBySlugStmt {
    GetSystemIntegrationBySlugStmt(
        "SELECT i.id, COALESCE(i.slug, '') AS slug, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name, COALESCE(i.openapi_spec #>> '{info,description}', '') AS description, i.openapi_spec::TEXT AS openapi_spec FROM public.integrations i WHERE i.owner_kind = 'system' AND i.slug = $1::TEXT LIMIT 1",
        None,
    )
}
impl GetSystemIntegrationBySlugStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        integration_slug: &'a T1,
    ) -> HostedIntegrationQuery<'c, 'a, 's, C, HostedIntegration, 1> {
        HostedIntegrationQuery {
            client,
            params: [integration_slug],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<HostedIntegrationBorrowed, tokio_postgres::Error> {
                Ok(HostedIntegrationBorrowed {
                    id: row.try_get(0)?,
                    slug: row.try_get(1)?,
                    name: row.try_get(2)?,
                    description: row.try_get(3)?,
                    openapi_spec: row.try_get(4)?,
                })
            },
            mapper: |it| HostedIntegration::from(it),
        }
    }
}
pub struct ListPublicHostedIntegrationsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_public_hosted_integrations() -> ListPublicHostedIntegrationsStmt {
    ListPublicHostedIntegrationsStmt(
        "SELECT i.id, COALESCE(i.slug, '') AS slug, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name, COALESCE(i.openapi_spec #>> '{info,description}', '') AS description, i.openapi_spec::TEXT AS openapi_spec, EXISTS( SELECT 1 FROM public.integration_connections c WHERE c.org_id = public.b64url_to_uuid($1::TEXT) AND c.integration_id = i.id AND c.end_user_id = $2::TEXT AND ( c.visibility = 'org' OR c.created_by_user_id = auth.uid() ) ) AS connected FROM public.integrations i WHERE i.owner_kind = 'system' ORDER BY LOWER(COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled')), i.updated_at DESC",
        None,
    )
}
impl ListPublicHostedIntegrationsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        org_public_id: &'a T1,
        end_user_id: &'a T2,
    ) -> PublicHostedIntegrationQuery<'c, 'a, 's, C, PublicHostedIntegration, 2> {
        PublicHostedIntegrationQuery {
            client,
            params: [org_public_id, end_user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<PublicHostedIntegrationBorrowed, tokio_postgres::Error> {
                Ok(PublicHostedIntegrationBorrowed {
                    id: row.try_get(0)?,
                    slug: row.try_get(1)?,
                    name: row.try_get(2)?,
                    description: row.try_get(3)?,
                    openapi_spec: row.try_get(4)?,
                    connected: row.try_get(5)?,
                })
            },
            mapper: |it| PublicHostedIntegration::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ListPublicHostedIntegrationsParams<T1, T2>,
        PublicHostedIntegrationQuery<'c, 'a, 's, C, PublicHostedIntegration, 2>,
        C,
    > for ListPublicHostedIntegrationsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ListPublicHostedIntegrationsParams<T1, T2>,
    ) -> PublicHostedIntegrationQuery<'c, 'a, 's, C, PublicHostedIntegration, 2> {
        self.bind(client, &params.org_public_id, &params.end_user_id)
    }
}
pub struct ListPublicCatalogIntegrationsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_public_catalog_integrations() -> ListPublicCatalogIntegrationsStmt {
    ListPublicCatalogIntegrationsStmt(
        "SELECT i.id, COALESCE(i.slug, '') AS slug, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name, COALESCE(i.openapi_spec #>> '{info,description}', '') AS description, i.openapi_spec::TEXT AS openapi_spec FROM public.integrations i WHERE i.owner_kind = 'system' ORDER BY LOWER(COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled')), i.updated_at DESC",
        None,
    )
}
impl ListPublicCatalogIntegrationsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> HostedIntegrationQuery<'c, 'a, 's, C, HostedIntegration, 0> {
        HostedIntegrationQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<HostedIntegrationBorrowed, tokio_postgres::Error> {
                Ok(HostedIntegrationBorrowed {
                    id: row.try_get(0)?,
                    slug: row.try_get(1)?,
                    name: row.try_get(2)?,
                    description: row.try_get(3)?,
                    openapi_spec: row.try_get(4)?,
                })
            },
            mapper: |it| HostedIntegration::from(it),
        }
    }
}
pub struct CreateHostedConnectionSessionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_hosted_connection_session() -> CreateHostedConnectionSessionStmt {
    CreateHostedConnectionSessionStmt(
        "INSERT INTO public.hosted_connection_sessions ( org_id, created_by_user_id, created_by_api_key_id, integration_id, integration_slug, end_user_id, end_user_name, end_user_email, suggested_connection_name, auth_type, token, expires_at ) VALUES ( $1::UUID, $2::UUID, $3::UUID, $4::UUID, $5::TEXT, $6::TEXT, NULLIF($7::TEXT, ''), NULLIF($8::TEXT, ''), NULLIF($9::TEXT, ''), $10::integration_auth_type, $11::TEXT, $12::TIMESTAMPTZ ) RETURNING token, expires_at",
        None,
    )
}
impl CreateHostedConnectionSessionStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        org_id: &'a uuid::Uuid,
        created_by_user_id: &'a uuid::Uuid,
        created_by_api_key_id: &'a uuid::Uuid,
        integration_id: &'a uuid::Uuid,
        integration_slug: &'a T1,
        end_user_id: &'a T2,
        end_user_name: &'a T3,
        end_user_email: &'a T4,
        suggested_connection_name: &'a T5,
        auth_type: &'a crate::types::IntegrationAuthType,
        token: &'a T6,
        expires_at: &'a chrono::DateTime<chrono::FixedOffset>,
    ) -> HostedConnectionSessionQuery<'c, 'a, 's, C, HostedConnectionSession, 12> {
        HostedConnectionSessionQuery {
            client,
            params: [
                org_id,
                created_by_user_id,
                created_by_api_key_id,
                integration_id,
                integration_slug,
                end_user_id,
                end_user_name,
                end_user_email,
                suggested_connection_name,
                auth_type,
                token,
                expires_at,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<HostedConnectionSessionBorrowed, tokio_postgres::Error> {
                Ok(HostedConnectionSessionBorrowed {
                    token: row.try_get(0)?,
                    expires_at: row.try_get(1)?,
                })
            },
            mapper: |it| HostedConnectionSession::from(it),
        }
    }
}
impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateHostedConnectionSessionParams<T1, T2, T3, T4, T5, T6>,
        HostedConnectionSessionQuery<'c, 'a, 's, C, HostedConnectionSession, 12>,
        C,
    > for CreateHostedConnectionSessionStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateHostedConnectionSessionParams<T1, T2, T3, T4, T5, T6>,
    ) -> HostedConnectionSessionQuery<'c, 'a, 's, C, HostedConnectionSession, 12> {
        self.bind(
            client,
            &params.org_id,
            &params.created_by_user_id,
            &params.created_by_api_key_id,
            &params.integration_id,
            &params.integration_slug,
            &params.end_user_id,
            &params.end_user_name,
            &params.end_user_email,
            &params.suggested_connection_name,
            &params.auth_type,
            &params.token,
            &params.expires_at,
        )
    }
}
pub struct GetHostedConnectionSessionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_hosted_connection_session() -> GetHostedConnectionSessionStmt {
    GetHostedConnectionSessionStmt(
        "SELECT s.id, public.uuid_to_b64url(s.org_id) AS org_public_id, s.integration_id, s.integration_slug, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS integration_name, u.issuer AS created_by_issuer, u.sub AS created_by_sub, s.end_user_id, COALESCE(s.end_user_name, '') AS end_user_name, COALESCE(s.end_user_email, '') AS end_user_email, COALESCE(s.suggested_connection_name, '') AS suggested_connection_name, s.auth_type::TEXT AS auth_type, s.expires_at, (s.expires_at <= NOW()) AS expired, (s.used_at IS NOT NULL) AS used FROM public.hosted_connection_sessions s INNER JOIN public.integrations i ON i.id = s.integration_id INNER JOIN auth.users u ON u.id = s.created_by_user_id WHERE s.token = $1::TEXT LIMIT 1",
        None,
    )
}
impl GetHostedConnectionSessionStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        token: &'a T1,
    ) -> HostedConnectionSessionContextQuery<'c, 'a, 's, C, HostedConnectionSessionContext, 1> {
        HostedConnectionSessionContextQuery {
            client,
            params: [token],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<
                HostedConnectionSessionContextBorrowed,
                tokio_postgres::Error,
            > {
                Ok(HostedConnectionSessionContextBorrowed {
                    id: row.try_get(0)?,
                    org_public_id: row.try_get(1)?,
                    integration_id: row.try_get(2)?,
                    integration_slug: row.try_get(3)?,
                    integration_name: row.try_get(4)?,
                    created_by_issuer: row.try_get(5)?,
                    created_by_sub: row.try_get(6)?,
                    end_user_id: row.try_get(7)?,
                    end_user_name: row.try_get(8)?,
                    end_user_email: row.try_get(9)?,
                    suggested_connection_name: row.try_get(10)?,
                    auth_type: row.try_get(11)?,
                    expires_at: row.try_get(12)?,
                    expired: row.try_get(13)?,
                    used: row.try_get(14)?,
                })
            },
            mapper: |it| HostedConnectionSessionContext::from(it),
        }
    }
}
pub struct GetHostedConnectionSessionForUpdateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_hosted_connection_session_for_update() -> GetHostedConnectionSessionForUpdateStmt {
    GetHostedConnectionSessionForUpdateStmt(
        "SELECT s.id, public.uuid_to_b64url(s.org_id) AS org_public_id, s.integration_id, s.integration_slug, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS integration_name, u.issuer AS created_by_issuer, u.sub AS created_by_sub, s.end_user_id, COALESCE(s.end_user_name, '') AS end_user_name, COALESCE(s.end_user_email, '') AS end_user_email, COALESCE(s.suggested_connection_name, '') AS suggested_connection_name, s.auth_type::TEXT AS auth_type, s.expires_at, (s.expires_at <= NOW()) AS expired, (s.used_at IS NOT NULL) AS used FROM public.hosted_connection_sessions s INNER JOIN public.integrations i ON i.id = s.integration_id INNER JOIN auth.users u ON u.id = s.created_by_user_id WHERE s.token = $1::TEXT LIMIT 1 FOR UPDATE OF s",
        None,
    )
}
impl GetHostedConnectionSessionForUpdateStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        token: &'a T1,
    ) -> HostedConnectionSessionContextQuery<'c, 'a, 's, C, HostedConnectionSessionContext, 1> {
        HostedConnectionSessionContextQuery {
            client,
            params: [token],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<
                HostedConnectionSessionContextBorrowed,
                tokio_postgres::Error,
            > {
                Ok(HostedConnectionSessionContextBorrowed {
                    id: row.try_get(0)?,
                    org_public_id: row.try_get(1)?,
                    integration_id: row.try_get(2)?,
                    integration_slug: row.try_get(3)?,
                    integration_name: row.try_get(4)?,
                    created_by_issuer: row.try_get(5)?,
                    created_by_sub: row.try_get(6)?,
                    end_user_id: row.try_get(7)?,
                    end_user_name: row.try_get(8)?,
                    end_user_email: row.try_get(9)?,
                    suggested_connection_name: row.try_get(10)?,
                    auth_type: row.try_get(11)?,
                    expires_at: row.try_get(12)?,
                    expired: row.try_get(13)?,
                    used: row.try_get(14)?,
                })
            },
            mapper: |it| HostedConnectionSessionContext::from(it),
        }
    }
}
pub struct CreateApiKeyIntegrationConnectionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_api_key_integration_connection() -> CreateApiKeyIntegrationConnectionStmt {
    CreateApiKeyIntegrationConnectionStmt(
        "INSERT INTO public.integration_connections ( org_id, integration_id, created_by_user_id, visibility, name, auth_type, api_key, end_user_id, end_user_name, end_user_email ) VALUES ( public.b64url_to_uuid($1::TEXT), $2::UUID, auth.uid(), 'private'::resource_visibility, $3::TEXT, 'api_key'::integration_auth_type, $4::TEXT, $5::TEXT, NULLIF($6::TEXT, ''), NULLIF($7::TEXT, '') ) RETURNING id, name",
        None,
    )
}
impl CreateApiKeyIntegrationConnectionStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        org_public_id: &'a T1,
        integration_id: &'a uuid::Uuid,
        name: &'a T2,
        api_key: &'a T3,
        end_user_id: &'a T4,
        end_user_name: &'a T5,
        end_user_email: &'a T6,
    ) -> CreatedHostedConnectionQuery<'c, 'a, 's, C, CreatedHostedConnection, 7> {
        CreatedHostedConnectionQuery {
            client,
            params: [
                org_public_id,
                integration_id,
                name,
                api_key,
                end_user_id,
                end_user_name,
                end_user_email,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<CreatedHostedConnectionBorrowed, tokio_postgres::Error> {
                Ok(CreatedHostedConnectionBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                })
            },
            mapper: |it| CreatedHostedConnection::from(it),
        }
    }
}
impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateApiKeyIntegrationConnectionParams<T1, T2, T3, T4, T5, T6>,
        CreatedHostedConnectionQuery<'c, 'a, 's, C, CreatedHostedConnection, 7>,
        C,
    > for CreateApiKeyIntegrationConnectionStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateApiKeyIntegrationConnectionParams<T1, T2, T3, T4, T5, T6>,
    ) -> CreatedHostedConnectionQuery<'c, 'a, 's, C, CreatedHostedConnection, 7> {
        self.bind(
            client,
            &params.org_public_id,
            &params.integration_id,
            &params.name,
            &params.api_key,
            &params.end_user_id,
            &params.end_user_name,
            &params.end_user_email,
        )
    }
}
pub struct DisconnectPublicHostedIntegrationsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn disconnect_public_hosted_integrations() -> DisconnectPublicHostedIntegrationsStmt {
    DisconnectPublicHostedIntegrationsStmt(
        "WITH deleted AS ( DELETE FROM public.integration_connections c USING public.integrations i WHERE c.org_id = public.b64url_to_uuid($1::TEXT) AND c.integration_id = i.id AND i.owner_kind = 'system' AND i.slug = $2::TEXT AND c.end_user_id = $3::TEXT RETURNING c.id ) SELECT COUNT(*)::BIGINT AS deleted_count FROM deleted",
        None,
    )
}
impl DisconnectPublicHostedIntegrationsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        org_public_id: &'a T1,
        integration_slug: &'a T2,
        end_user_id: &'a T3,
    ) -> DisconnectedHostedConnectionsQuery<'c, 'a, 's, C, DisconnectedHostedConnections, 3> {
        DisconnectedHostedConnectionsQuery {
            client,
            params: [org_public_id, integration_slug, end_user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<DisconnectedHostedConnections, tokio_postgres::Error> {
                Ok(DisconnectedHostedConnections {
                    deleted_count: row.try_get(0)?,
                })
            },
            mapper: |it| DisconnectedHostedConnections::from(it),
        }
    }
}
impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        DisconnectPublicHostedIntegrationsParams<T1, T2, T3>,
        DisconnectedHostedConnectionsQuery<'c, 'a, 's, C, DisconnectedHostedConnections, 3>,
        C,
    > for DisconnectPublicHostedIntegrationsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a DisconnectPublicHostedIntegrationsParams<T1, T2, T3>,
    ) -> DisconnectedHostedConnectionsQuery<'c, 'a, 's, C, DisconnectedHostedConnections, 3> {
        self.bind(
            client,
            &params.org_public_id,
            &params.integration_slug,
            &params.end_user_id,
        )
    }
}
pub struct MarkHostedConnectionSessionUsedStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn mark_hosted_connection_session_used() -> MarkHostedConnectionSessionUsedStmt {
    MarkHostedConnectionSessionUsedStmt(
        "UPDATE public.hosted_connection_sessions SET used_at = NOW(), updated_at = NOW() WHERE token = $1::TEXT AND used_at IS NULL",
        None,
    )
}
impl MarkHostedConnectionSessionUsedStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        token: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[token]).await
    }
}
