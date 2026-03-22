// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateOauthClientParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub org_id: T1,
    pub provider: T2,
    pub client_id: T3,
    pub client_secret: T4,
}
#[derive(Debug, Clone, PartialEq)]
pub struct OAuthClientCard {
    pub id: uuid::Uuid,
    pub provider: String,
    pub client_id: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct OAuthClientCardBorrowed<'a> {
    pub id: uuid::Uuid,
    pub provider: &'a str,
    pub client_id: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<OAuthClientCardBorrowed<'a>> for OAuthClientCard {
    fn from(
        OAuthClientCardBorrowed {
            id,
            provider,
            client_id,
            created_at,
        }: OAuthClientCardBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            provider: provider.into(),
            client_id: client_id.into(),
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OAuthClientMutation {
    pub changed: bool,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct OAuthClientCardQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<OAuthClientCardBorrowed, tokio_postgres::Error>,
    mapper: fn(OAuthClientCardBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> OAuthClientCardQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(OAuthClientCardBorrowed) -> R,
    ) -> OAuthClientCardQuery<'c, 'a, 's, C, R, N> {
        OAuthClientCardQuery {
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
pub struct OAuthClientMutationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<OAuthClientMutation, tokio_postgres::Error>,
    mapper: fn(OAuthClientMutation) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> OAuthClientMutationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(OAuthClientMutation) -> R,
    ) -> OAuthClientMutationQuery<'c, 'a, 's, C, R, N> {
        OAuthClientMutationQuery {
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
pub struct ListOauthClientsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_oauth_clients() -> ListOauthClientsStmt {
    ListOauthClientsStmt(
        "SELECT id, provider, client_id, created_at FROM public.oauth_clients WHERE org_id = public.b64url_to_uuid($1::TEXT) ORDER BY LOWER(provider), LOWER(client_id), created_at DESC",
        None,
    )
}
impl ListOauthClientsStmt {
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
        org_id: &'a T1,
    ) -> OAuthClientCardQuery<'c, 'a, 's, C, OAuthClientCard, 1> {
        OAuthClientCardQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<OAuthClientCardBorrowed, tokio_postgres::Error> {
                Ok(OAuthClientCardBorrowed {
                    id: row.try_get(0)?,
                    provider: row.try_get(1)?,
                    client_id: row.try_get(2)?,
                    created_at: row.try_get(3)?,
                })
            },
            mapper: |it| OAuthClientCard::from(it),
        }
    }
}
pub struct CreateOauthClientStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_oauth_client() -> CreateOauthClientStmt {
    CreateOauthClientStmt(
        "WITH inserted AS ( INSERT INTO public.oauth_clients ( org_id, created_by_user_id, provider, client_id, client_secret ) VALUES ( public.b64url_to_uuid($1::TEXT), auth.uid(), $2::TEXT, $3::TEXT, $4::TEXT ) RETURNING id ) SELECT EXISTS(SELECT 1 FROM inserted) AS changed",
        None,
    )
}
impl CreateOauthClientStmt {
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
    >(
        &'s self,
        client: &'c C,
        org_id: &'a T1,
        provider: &'a T2,
        client_id: &'a T3,
        client_secret: &'a T4,
    ) -> OAuthClientMutationQuery<'c, 'a, 's, C, OAuthClientMutation, 4> {
        OAuthClientMutationQuery {
            client,
            params: [org_id, provider, client_id, client_secret],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<OAuthClientMutation, tokio_postgres::Error> {
                    Ok(OAuthClientMutation {
                        changed: row.try_get(0)?,
                    })
                },
            mapper: |it| OAuthClientMutation::from(it),
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
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateOauthClientParams<T1, T2, T3, T4>,
        OAuthClientMutationQuery<'c, 'a, 's, C, OAuthClientMutation, 4>,
        C,
    > for CreateOauthClientStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateOauthClientParams<T1, T2, T3, T4>,
    ) -> OAuthClientMutationQuery<'c, 'a, 's, C, OAuthClientMutation, 4> {
        self.bind(
            client,
            &params.org_id,
            &params.provider,
            &params.client_id,
            &params.client_secret,
        )
    }
}
