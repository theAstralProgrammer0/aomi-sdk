#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///`CreateQuoteRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "maker_owner_id",
    ///    "maker_shard",
    ///    "text"
    ///  ],
    ///  "properties": {
    ///    "maker_owner_id": {
    ///      "description": "Maker's owner identifier.",
    ///      "type": "string"
    ///    },
    ///    "maker_shard": {
    ///      "description": "Maker's shard number.",
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "text": {
    ///      "description": "Natural-language quote (e.g. \"Buy 10 dETH at most 2000 USDD,\nexpires in 5 minutes\"). The backend compiles this into a Local Law.\n",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateQuoteRequest {
        ///Maker's owner identifier.
        pub maker_owner_id: ::std::string::String,
        ///Maker's shard number.
        pub maker_shard: i64,
        /**Natural-language quote (e.g. "Buy 10 dETH at most 2000 USDD,
        expires in 5 minutes"). The backend compiles this into a Local Law.
        */
        pub text: ::std::string::String,
    }
    ///`FeedEvidence`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "asset",
    ///    "price",
    ///    "signature",
    ///    "source",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "asset": {
    ///      "description": "Asset the price is for.",
    ///      "type": "string"
    ///    },
    ///    "price": {
    ///      "description": "Price reported by this feed.",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "signature": {
    ///      "description": "Cryptographic signature over the feed payload.",
    ///      "type": "string"
    ///    },
    ///    "source": {
    ///      "description": "Price-feed source name.",
    ///      "type": "string"
    ///    },
    ///    "timestamp": {
    ///      "description": "Unix timestamp of the price.",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FeedEvidence {
        ///Asset the price is for.
        pub asset: ::std::string::String,
        ///Price reported by this feed.
        pub price: f64,
        ///Cryptographic signature over the feed payload.
        pub signature: ::std::string::String,
        ///Price-feed source name.
        pub source: ::std::string::String,
        ///Unix timestamp of the price.
        pub timestamp: i64,
    }
    ///`FillQuoteRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "feed_evidence",
    ///    "price",
    ///    "size",
    ///    "taker_owner_id",
    ///    "taker_shard"
    ///  ],
    ///  "properties": {
    ///    "feed_evidence": {
    ///      "description": "Signed price-feed evidence backing the fill.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FeedEvidence"
    ///      }
    ///    },
    ///    "price": {
    ///      "description": "Price at which to fill.",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "size": {
    ///      "description": "Size to fill.",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "taker_owner_id": {
    ///      "description": "Taker's owner identifier.",
    ///      "type": "string"
    ///    },
    ///    "taker_shard": {
    ///      "description": "Taker's shard number.",
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FillQuoteRequest {
        ///Signed price-feed evidence backing the fill.
        pub feed_evidence: ::std::vec::Vec<FeedEvidence>,
        ///Price at which to fill.
        pub price: f64,
        ///Size to fill.
        pub size: f64,
        ///Taker's owner identifier.
        pub taker_owner_id: ::std::string::String,
        ///Taker's shard number.
        pub taker_shard: i64,
    }
    ///`FillResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "fill_id",
    ///    "message",
    ///    "quote_id",
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "description": "Constraint-violation details when success=false. Opaque shape.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": true
    ///    },
    ///    "fill_id": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "proof": {
    ///      "description": "ZK proof artifact emitted on success. Opaque shape.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": true
    ///    },
    ///    "quote_id": {
    ///      "type": "string"
    ///    },
    ///    "receipt": {
    ///      "description": "Embedded receipt object on success. Opaque shape.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": true
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FillResponse {
        ///Constraint-violation details when success=false. Opaque shape.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub fill_id: ::std::string::String,
        pub message: ::std::string::String,
        ///ZK proof artifact emitted on success. Opaque shape.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub proof:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub quote_id: ::std::string::String,
        ///Embedded receipt object on success. Opaque shape.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub receipt:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub success: bool,
    }
    ///`Quote`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "asset",
    ///    "created_at",
    ///    "currency",
    ///    "direction",
    ///    "expires_at",
    ///    "id",
    ///    "maker_owner_id",
    ///    "maker_shard",
    ///    "size",
    ///    "status",
    ///    "text"
    ///  ],
    ///  "properties": {
    ///    "asset": {
    ///      "type": "string"
    ///    },
    ///    "constraints_summary": {
    ///      "description": "Human-readable summary of the compiled Local Law.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "created_at": {
    ///      "description": "Unix timestamp at which the quote was created.",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "currency": {
    ///      "type": "string"
    ///    },
    ///    "direction": {
    ///      "description": "Direction (e.g. buy or sell).",
    ///      "type": "string"
    ///    },
    ///    "expires_at": {
    ///      "description": "Unix timestamp at which the quote expires.",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "local_law": {
    ///      "description": "Compiled Local-Law document. Internal shape is opaque and may\nevolve across backend versions; treated as a JSON passthrough.\n",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": true
    ///    },
    ///    "maker_owner_id": {
    ///      "type": "string"
    ///    },
    ///    "maker_shard": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    },
    ///    "message": {
    ///      "description": "Optional informational message from the backend.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "price_limit": {
    ///      "description": "Optional price bound parsed from the natural-language text.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "size": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "status": {
    ///      "description": "Lifecycle status (e.g. active, filled, expired, cancelled).",
    ///      "type": "string"
    ///    },
    ///    "text": {
    ///      "description": "Original natural-language quote.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Quote {
        pub asset: ::std::string::String,
        ///Human-readable summary of the compiled Local Law.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub constraints_summary: ::std::option::Option<::std::string::String>,
        ///Unix timestamp at which the quote was created.
        pub created_at: i64,
        pub currency: ::std::string::String,
        ///Direction (e.g. buy or sell).
        pub direction: ::std::string::String,
        ///Unix timestamp at which the quote expires.
        pub expires_at: i64,
        pub id: ::std::string::String,
        /**Compiled Local-Law document. Internal shape is opaque and may
        evolve across backend versions; treated as a JSON passthrough.
        */
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub local_law:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub maker_owner_id: ::std::string::String,
        pub maker_shard: i64,
        ///Optional informational message from the backend.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        ///Optional price bound parsed from the natural-language text.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price_limit: ::std::option::Option<f64>,
        pub size: f64,
        ///Lifecycle status (e.g. active, filled, expired, cancelled).
        pub status: ::std::string::String,
        ///Original natural-language quote.
        pub text: ::std::string::String,
    }
    ///`Receipt`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "attempted_at",
    ///    "id",
    ///    "price",
    ///    "quote_id",
    ///    "size",
    ///    "status",
    ///    "success",
    ///    "taker_owner_id",
    ///    "taker_shard"
    ///  ],
    ///  "properties": {
    ///    "attempted_at": {
    ///      "description": "Unix timestamp when the fill was attempted.",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "error_code": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "error_message": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quote_id": {
    ///      "type": "string"
    ///    },
    ///    "sdl_hash": {
    ///      "description": "Hash of the SDL/Local-Law document the fill was checked against.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "size": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "status": {
    ///      "description": "Receipt status (e.g. settled, rejected).",
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    },
    ///    "taker_owner_id": {
    ///      "type": "string"
    ///    },
    ///    "taker_shard": {
    ///      "type": "integer",
    ///      "format": "int64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Receipt {
        ///Unix timestamp when the fill was attempted.
        pub attempted_at: i64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_code: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_message: ::std::option::Option<::std::string::String>,
        pub id: ::std::string::String,
        pub price: f64,
        pub quote_id: ::std::string::String,
        ///Hash of the SDL/Local-Law document the fill was checked against.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sdl_hash: ::std::option::Option<::std::string::String>,
        pub size: f64,
        ///Receipt status (e.g. settled, rejected).
        pub status: ::std::string::String,
        pub success: bool,
        pub taker_owner_id: ::std::string::String,
        pub taker_shard: i64,
    }
}
#[derive(Clone, Debug)]
/**Client for Delta RFQ Arena API

HTTP API for the Delta RFQ Arena — an OTC Request-For-Quote trading
backend that compiles natural-language quote text into machine-checkable
"Local Laws" and verifies fills via ZK proofs.

This spec describes ONLY the surface consumed by the `apps/delta` Aomi
app crate. It is hand-drafted from `apps/delta/src/{client,tool,types}.rs`
because there is no public OpenAPI document published upstream.

## Auth
No authentication is declared on the wire — the crate hits a
locally-running backend (default `http://localhost:3335`, overridable
via the `DELTA_RFQ_API_URL` environment variable). If the deployment
later adds bearer/API-key auth, add it under `securitySchemes` and apply
via a top-level `security:` block.

## Loose schemas
Several response fields are JSON-`object`-shaped passthroughs whose
internal structure is not stable across backend versions:
  - `Quote.local_law` — compiled constraint document
  - `FillResponse.error`, `FillResponse.receipt`, `FillResponse.proof`
These are typed `additionalProperties: true` rather than fabricated.


Version: 0.1.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.1.0"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    /**List all active quotes in the arena

    Sends a `GET` request to `/quotes`

    */
    pub async fn list_quotes<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Quote>>, Error<()>> {
        let url = format!("{}/quotes", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_quotes",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a new RFQ quote from natural language

    Submit a plain-English quote (e.g. "Buy 10 dETH at most 2000 USDD,
    expires in 5 minutes"). The backend parses the text and compiles a
    Local Law that subsequent fill attempts must satisfy.


    Sends a `POST` request to `/quotes`

    */
    pub async fn create_quote<'a>(
        &'a self,
        body: &'a types::CreateQuoteRequest,
    ) -> Result<ResponseValue<types::Quote>, Error<()>> {
        let url = format!("{}/quotes", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_quote",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get detailed information about a specific quote

    Sends a `GET` request to `/quotes/{quote_id}`

    Arguments:
    - `quote_id`: Identifier of a quote in the arena.
    */
    pub async fn get_quote<'a>(
        &'a self,
        quote_id: &'a str,
    ) -> Result<ResponseValue<types::Quote>, Error<()>> {
        let url = format!(
            "{}/quotes/{}",
            self.baseurl,
            encode_path(&quote_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_quote",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Attempt to fill a quote with price-feed evidence

    Submit a fill proposal with size, price and a list of signed price
    feeds. The fill succeeds only if it satisfies all Local-Law
    constraints attached to the quote; otherwise the response carries
    an error describing the violated constraint.


    Sends a `POST` request to `/quotes/{quote_id}/fill`

    Arguments:
    - `quote_id`: Identifier of a quote in the arena.
    - `body`
    */
    pub async fn fill_quote<'a>(
        &'a self,
        quote_id: &'a str,
        body: &'a types::FillQuoteRequest,
    ) -> Result<ResponseValue<types::FillResponse>, Error<()>> {
        let url = format!(
            "{}/quotes/{}/fill",
            self.baseurl,
            encode_path(&quote_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "fill_quote",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get all fill receipts for a quote

    Each receipt records one fill attempt, success or failure.

    Sends a `GET` request to `/quotes/{quote_id}/receipts`

    Arguments:
    - `quote_id`: Identifier of a quote in the arena.
    */
    pub async fn get_receipts<'a>(
        &'a self,
        quote_id: &'a str,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Receipt>>, Error<()>> {
        let url = format!(
            "{}/quotes/{}/receipts",
            self.baseurl,
            encode_path(&quote_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_receipts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
