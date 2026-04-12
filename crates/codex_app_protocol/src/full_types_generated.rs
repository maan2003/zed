#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
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
#[doc = "A path that is guaranteed to be absolute and normalized (though it is not guaranteed to be canonicalized or exist on the filesystem).\n\nIMPORTANT: When deserializing an `AbsolutePathBuf`, a base path must be set using [AbsolutePathBufGuard::new]. If no base path is set, the deserialization will fail unless the path being deserialized is already absolute."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A path that is guaranteed to be absolute and normalized (though it is not guaranteed to be canonicalized or exist on the filesystem).\\n\\nIMPORTANT: When deserializing an `AbsolutePathBuf`, a base path must be set using [AbsolutePathBufGuard::new]. If no base path is set, the deserialization will fail unless the path being deserialized is already absolute.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct AbsolutePathBuf(pub ::std::string::String);
impl ::std::ops::Deref for AbsolutePathBuf {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AbsolutePathBuf> for ::std::string::String {
    fn from(value: AbsolutePathBuf) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for AbsolutePathBuf {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for AbsolutePathBuf {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for AbsolutePathBuf {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Account`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"ApiKeyAccount\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ApiKeyAccountType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"apiKey\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ChatgptAccount\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"email\","]
#[doc = "        \"planType\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"email\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"planType\": {"]
#[doc = "          \"$ref\": \"#/definitions/PlanType\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ChatgptAccountType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"chatgpt\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Account {
    #[serde(rename = "apiKey")]
    ApiKey,
    #[doc = "ChatgptAccount"]
    #[serde(rename = "chatgpt")]
    Chatgpt {
        email: ::std::string::String,
        #[serde(rename = "planType")]
        plan_type: PlanType,
    },
}
#[doc = "`AccountLoginCompletedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AccountLoginCompletedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"success\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"loginId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"success\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AccountLoginCompletedNotification {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "loginId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub login_id: ::std::option::Option<::std::string::String>,
    pub success: bool,
}
#[doc = "`AccountRateLimitsUpdatedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AccountRateLimitsUpdatedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"rateLimits\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"rateLimits\": {"]
#[doc = "      \"$ref\": \"#/definitions/RateLimitSnapshot\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AccountRateLimitsUpdatedNotification {
    #[serde(rename = "rateLimits")]
    pub rate_limits: RateLimitSnapshot,
}
#[doc = "`AccountUpdatedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AccountUpdatedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"authMode\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AuthMode\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"planType\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PlanType\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AccountUpdatedNotification {
    #[serde(
        rename = "authMode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub auth_mode: ::std::option::Option<AuthMode>,
    #[serde(
        rename = "planType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub plan_type: ::std::option::Option<PlanType>,
}
impl ::std::default::Default for AccountUpdatedNotification {
    fn default() -> Self {
        Self {
            auth_mode: Default::default(),
            plan_type: Default::default(),
        }
    }
}
#[doc = "`AgentMessageDeltaNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AgentMessageDeltaNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"delta\","]
#[doc = "    \"itemId\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"delta\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentMessageDeltaNotification {
    pub delta: ::std::string::String,
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`AgentPath`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct AgentPath(pub ::std::string::String);
impl ::std::ops::Deref for AgentPath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgentPath> for ::std::string::String {
    fn from(value: AgentPath) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for AgentPath {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for AgentPath {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for AgentPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`AnalyticsConfig`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AnalyticsConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
}
impl ::std::default::Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
        }
    }
}
#[doc = "EXPERIMENTAL - app metadata returned by app-list APIs."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"EXPERIMENTAL - app metadata returned by app-list APIs.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"isDiscoverableApp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"category\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"developer\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isDiscoverableApp\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"privacyPolicy\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"termsOfService\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"website\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppBranding {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub developer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isDiscoverableApp")]
    pub is_discoverable_app: bool,
    #[serde(
        rename = "privacyPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub privacy_policy: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "termsOfService",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terms_of_service: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub website: ::std::option::Option<::std::string::String>,
}
#[doc = "`AppConfig`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"default_tools_approval_mode\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AppToolApproval\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"default_tools_enabled\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"destructive_enabled\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"open_world_enabled\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tools\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AppToolsConfig\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub default_tools_approval_mode: ::std::option::Option<AppToolApproval>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub default_tools_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub destructive_enabled: ::std::option::Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub open_world_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<AppToolsConfig>,
}
impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            default_tools_approval_mode: Default::default(),
            default_tools_enabled: Default::default(),
            destructive_enabled: Default::default(),
            enabled: defaults::default_bool::<true>(),
            open_world_enabled: Default::default(),
            tools: Default::default(),
        }
    }
}
#[doc = "EXPERIMENTAL - app metadata returned by app-list APIs."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"EXPERIMENTAL - app metadata returned by app-list APIs.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"appMetadata\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AppMetadata\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"branding\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AppBranding\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"distributionChannel\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"installUrl\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isAccessible\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isEnabled\": {"]
#[doc = "      \"description\": \"Whether this app is enabled in config.toml. Example: ```toml [apps.bad_app] enabled = false ```\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"labels\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"logoUrl\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"logoUrlDark\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pluginDisplayNames\": {"]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppInfo {
    #[serde(
        rename = "appMetadata",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub app_metadata: ::std::option::Option<AppMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub branding: ::std::option::Option<AppBranding>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "distributionChannel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub distribution_channel: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(
        rename = "installUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub install_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isAccessible", default)]
    pub is_accessible: bool,
    #[doc = "Whether this app is enabled in config.toml. Example: ```toml [apps.bad_app] enabled = false ```"]
    #[serde(rename = "isEnabled", default = "defaults::default_bool::<true>")]
    pub is_enabled: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub labels: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[serde(
        rename = "logoUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub logo_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "logoUrlDark",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub logo_url_dark: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(
        rename = "pluginDisplayNames",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub plugin_display_names: ::std::vec::Vec<::std::string::String>,
}
#[doc = "EXPERIMENTAL - notification emitted when the app list changes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AppListUpdatedNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - notification emitted when the app list changes.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AppInfo\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppListUpdatedNotification {
    pub data: ::std::vec::Vec<AppInfo>,
}
#[doc = "`AppMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"categories\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"developer\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"firstPartyRequiresInstall\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"firstPartyType\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"review\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AppReview\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"screenshots\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AppScreenshot\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"seoDescription\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"showInComposerWhenUnlinked\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"subCategories\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"versionId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"versionNotes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub developer: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "firstPartyRequiresInstall",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_party_requires_install: ::std::option::Option<bool>,
    #[serde(
        rename = "firstPartyType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_party_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub review: ::std::option::Option<AppReview>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub screenshots: ::std::option::Option<::std::vec::Vec<AppScreenshot>>,
    #[serde(
        rename = "seoDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub seo_description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "showInComposerWhenUnlinked",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_in_composer_when_unlinked: ::std::option::Option<bool>,
    #[serde(
        rename = "subCategories",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "versionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "versionNotes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_notes: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AppMetadata {
    fn default() -> Self {
        Self {
            categories: Default::default(),
            developer: Default::default(),
            first_party_requires_install: Default::default(),
            first_party_type: Default::default(),
            review: Default::default(),
            screenshots: Default::default(),
            seo_description: Default::default(),
            show_in_composer_when_unlinked: Default::default(),
            sub_categories: Default::default(),
            version: Default::default(),
            version_id: Default::default(),
            version_notes: Default::default(),
        }
    }
}
#[doc = "`AppReview`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"status\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppReview {
    pub status: ::std::string::String,
}
#[doc = "`AppScreenshot`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"userPrompt\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fileId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"userPrompt\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppScreenshot {
    #[serde(
        rename = "fileId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userPrompt")]
    pub user_prompt: ::std::string::String,
}
#[doc = "EXPERIMENTAL - app metadata summary for plugin responses."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"EXPERIMENTAL - app metadata summary for plugin responses.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"needsAuth\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"installUrl\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"needsAuth\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppSummary {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(
        rename = "installUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub install_url: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(rename = "needsAuth")]
    pub needs_auth: bool,
}
#[doc = "`AppToolApproval`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"auto\","]
#[doc = "    \"prompt\","]
#[doc = "    \"approve\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AppToolApproval {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "prompt")]
    Prompt,
    #[serde(rename = "approve")]
    Approve,
}
impl ::std::fmt::Display for AppToolApproval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Prompt => f.write_str("prompt"),
            Self::Approve => f.write_str("approve"),
        }
    }
}
impl ::std::str::FromStr for AppToolApproval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "prompt" => Ok(Self::Prompt),
            "approve" => Ok(Self::Approve),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AppToolApproval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AppToolApproval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AppToolApproval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AppToolConfig`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"approval_mode\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AppToolApproval\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppToolConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub approval_mode: ::std::option::Option<AppToolApproval>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
}
impl ::std::default::Default for AppToolConfig {
    fn default() -> Self {
        Self {
            approval_mode: Default::default(),
            enabled: Default::default(),
        }
    }
}
#[doc = "`AppToolsConfig`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct AppToolsConfig(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for AppToolsConfig {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<AppToolsConfig>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: AppToolsConfig) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for AppToolsConfig
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Configures who approval requests are routed to for review. Examples include sandbox escapes, blocked network access, MCP approval prompts, and ARC escalations. Defaults to `user`. `guardian_subagent` uses a carefully prompted subagent to gather relevant context and apply a risk-based decision framework before approving or denying the request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Configures who approval requests are routed to for review. Examples include sandbox escapes, blocked network access, MCP approval prompts, and ARC escalations. Defaults to `user`. `guardian_subagent` uses a carefully prompted subagent to gather relevant context and apply a risk-based decision framework before approving or denying the request.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"user\","]
#[doc = "    \"guardian_subagent\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApprovalsReviewer {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "guardian_subagent")]
    GuardianSubagent,
}
impl ::std::fmt::Display for ApprovalsReviewer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::GuardianSubagent => f.write_str("guardian_subagent"),
        }
    }
}
impl ::std::str::FromStr for ApprovalsReviewer {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "guardian_subagent" => Ok(Self::GuardianSubagent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApprovalsReviewer {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ApprovalsReviewer {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApprovalsReviewer {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AppsConfig`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_default\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AppsDefaultConfig\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppsConfig {
    #[serde(
        rename = "_default",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default: ::std::option::Option<AppsDefaultConfig>,
}
impl ::std::default::Default for AppsConfig {
    fn default() -> Self {
        Self {
            default: Default::default(),
        }
    }
}
#[doc = "`AppsDefaultConfig`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"destructive_enabled\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"open_world_enabled\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppsDefaultConfig {
    #[serde(default = "defaults::default_bool::<true>")]
    pub destructive_enabled: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub enabled: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub open_world_enabled: bool,
}
impl ::std::default::Default for AppsDefaultConfig {
    fn default() -> Self {
        Self {
            destructive_enabled: defaults::default_bool::<true>(),
            enabled: defaults::default_bool::<true>(),
            open_world_enabled: defaults::default_bool::<true>(),
        }
    }
}
#[doc = "EXPERIMENTAL - list available apps/connectors."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AppsListParams\","]
#[doc = "  \"description\": \"EXPERIMENTAL - list available apps/connectors.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"Opaque pagination cursor returned by a previous call.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"forceRefetch\": {"]
#[doc = "      \"description\": \"When true, bypass app caches and fetch the latest data from sources.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"limit\": {"]
#[doc = "      \"description\": \"Optional page size; defaults to a reasonable server-side value.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"description\": \"Optional thread id used to evaluate app feature gating from that thread's config.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppsListParams {
    #[doc = "Opaque pagination cursor returned by a previous call."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[doc = "When true, bypass app caches and fetch the latest data from sources."]
    #[serde(
        rename = "forceRefetch",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub force_refetch: ::std::option::Option<bool>,
    #[doc = "Optional page size; defaults to a reasonable server-side value."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<u32>,
    #[doc = "Optional thread id used to evaluate app feature gating from that thread's config."]
    #[serde(
        rename = "threadId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub thread_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AppsListParams {
    fn default() -> Self {
        Self {
            cursor: Default::default(),
            force_refetch: Default::default(),
            limit: Default::default(),
            thread_id: Default::default(),
        }
    }
}
#[doc = "EXPERIMENTAL - app list response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"AppsListResponse\","]
#[doc = "  \"description\": \"EXPERIMENTAL - app list response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AppInfo\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"Opaque cursor to pass to the next call to continue after the last item. If None, there are no more items to return.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AppsListResponse {
    pub data: ::std::vec::Vec<AppInfo>,
    #[doc = "Opaque cursor to pass to the next call to continue after the last item. If None, there are no more items to return."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
}
#[doc = "`AskForApproval`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"untrusted\","]
#[doc = "        \"on-failure\","]
#[doc = "        \"on-request\","]
#[doc = "        \"never\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"GranularAskForApproval\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"granular\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"granular\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"mcp_elicitations\","]
#[doc = "            \"rules\","]
#[doc = "            \"sandbox_approval\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"mcp_elicitations\": {"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"request_permissions\": {"]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"rules\": {"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"sandbox_approval\": {"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"skill_approval\": {"]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum AskForApproval {
    #[serde(rename = "untrusted")]
    Untrusted,
    #[serde(rename = "on-failure")]
    OnFailure,
    #[serde(rename = "on-request")]
    OnRequest,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "granular")]
    Granular {
        mcp_elicitations: bool,
        #[serde(default)]
        request_permissions: bool,
        rules: bool,
        sandbox_approval: bool,
        #[serde(default)]
        skill_approval: bool,
    },
}
#[doc = "Authentication mode for OpenAI-backed providers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Authentication mode for OpenAI-backed providers.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"OpenAI API key provided by the caller and stored by Codex.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"apikey\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"ChatGPT OAuth managed by Codex (tokens persisted and refreshed by Codex).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"chatgpt\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"[UNSTABLE] FOR OPENAI INTERNAL USE ONLY - DO NOT USE.\\n\\nChatGPT auth tokens are supplied by an external host app and are only stored in memory. Token refresh must be handled by the external host app.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"chatgptAuthTokens\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AuthMode {
    #[doc = "OpenAI API key provided by the caller and stored by Codex."]
    #[serde(rename = "apikey")]
    Apikey,
    #[doc = "ChatGPT OAuth managed by Codex (tokens persisted and refreshed by Codex)."]
    #[serde(rename = "chatgpt")]
    Chatgpt,
    #[doc = "[UNSTABLE] FOR OPENAI INTERNAL USE ONLY - DO NOT USE.\n\nChatGPT auth tokens are supplied by an external host app and are only stored in memory. Token refresh must be handled by the external host app."]
    #[serde(rename = "chatgptAuthTokens")]
    ChatgptAuthTokens,
}
impl ::std::fmt::Display for AuthMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Apikey => f.write_str("apikey"),
            Self::Chatgpt => f.write_str("chatgpt"),
            Self::ChatgptAuthTokens => f.write_str("chatgptAuthTokens"),
        }
    }
}
impl ::std::str::FromStr for AuthMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apikey" => Ok(Self::Apikey),
            "chatgpt" => Ok(Self::Chatgpt),
            "chatgptAuthTokens" => Ok(Self::ChatgptAuthTokens),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuthMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AuthMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuthMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "[UNSTABLE] Source that produced a terminal guardian approval review decision."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[UNSTABLE] Source that produced a terminal guardian approval review decision.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"agent\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AutoReviewDecisionSource {
    #[serde(rename = "agent")]
    Agent,
}
impl ::std::fmt::Display for AutoReviewDecisionSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Agent => f.write_str("agent"),
        }
    }
}
impl ::std::str::FromStr for AutoReviewDecisionSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agent" => Ok(Self::Agent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AutoReviewDecisionSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AutoReviewDecisionSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AutoReviewDecisionSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ByteRange`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"end\","]
#[doc = "    \"start\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"end\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ByteRange {
    pub end: u32,
    pub start: u32,
}
#[doc = "`CancelLoginAccountParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CancelLoginAccountParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"loginId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"loginId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelLoginAccountParams {
    #[serde(rename = "loginId")]
    pub login_id: ::std::string::String,
}
#[doc = "`CancelLoginAccountResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CancelLoginAccountResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/CancelLoginAccountStatus\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelLoginAccountResponse {
    pub status: CancelLoginAccountStatus,
}
#[doc = "`CancelLoginAccountStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"canceled\","]
#[doc = "    \"notFound\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CancelLoginAccountStatus {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "notFound")]
    NotFound,
}
impl ::std::fmt::Display for CancelLoginAccountStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Canceled => f.write_str("canceled"),
            Self::NotFound => f.write_str("notFound"),
        }
    }
}
impl ::std::str::FromStr for CancelLoginAccountStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "canceled" => Ok(Self::Canceled),
            "notFound" => Ok(Self::NotFound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancelLoginAccountStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CancelLoginAccountStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancelLoginAccountStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ClientInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClientInfo {
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    pub version: ::std::string::String,
}
#[doc = "Request from the client to the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ClientRequest\","]
#[doc = "  \"description\": \"Request from the client to the server.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"InitializeRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"InitializeRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"initialize\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/InitializeParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/startRequest\","]
#[doc = "      \"description\": \"NEW APIs\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/startRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/start\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadStartParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/resumeRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/resumeRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/resume\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadResumeParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/forkRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/forkRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/fork\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadForkParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/archiveRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/archiveRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/archive\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadArchiveParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/unsubscribeRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/unsubscribeRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/unsubscribe\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadUnsubscribeParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/name/setRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/name/setRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/name/set\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadSetNameParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/metadata/updateRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/metadata/updateRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/metadata/update\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadMetadataUpdateParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/unarchiveRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/unarchiveRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/unarchive\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadUnarchiveParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/compact/startRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/compact/startRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/compact/start\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadCompactStartParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/shellCommandRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/shellCommandRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/shellCommand\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadShellCommandParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/rollbackRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/rollbackRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/rollback\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadRollbackParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/listRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/listRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/list\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/loaded/listRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/loaded/listRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/loaded/list\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadLoadedListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/readRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/readRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/read\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadReadParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Skills/listRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Skills/listRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"skills/list\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/SkillsListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Plugin/listRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Plugin/listRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"plugin/list\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/PluginListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Plugin/readRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Plugin/readRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"plugin/read\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/PluginReadParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"App/listRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"App/listRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"app/list\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/AppsListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/readFileRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/readFileRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/readFile\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsReadFileParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/writeFileRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/writeFileRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/writeFile\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsWriteFileParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/createDirectoryRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/createDirectoryRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/createDirectory\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsCreateDirectoryParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/getMetadataRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/getMetadataRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/getMetadata\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsGetMetadataParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/readDirectoryRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/readDirectoryRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/readDirectory\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsReadDirectoryParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/removeRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/removeRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/remove\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsRemoveParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/copyRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/copyRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/copy\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsCopyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/watchRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/watchRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/watch\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsWatchParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/unwatchRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/unwatchRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/unwatch\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsUnwatchParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Skills/config/writeRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Skills/config/writeRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"skills/config/write\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/SkillsConfigWriteParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Plugin/installRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Plugin/installRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"plugin/install\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/PluginInstallParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Plugin/uninstallRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Plugin/uninstallRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"plugin/uninstall\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/PluginUninstallParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Turn/startRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Turn/startRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"turn/start\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/TurnStartParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Turn/steerRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Turn/steerRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"turn/steer\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/TurnSteerParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Turn/interruptRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Turn/interruptRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"turn/interrupt\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/TurnInterruptParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Review/startRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Review/startRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"review/start\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ReviewStartParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Model/listRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Model/listRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"model/list\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ModelListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ExperimentalFeature/listRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"ExperimentalFeature/listRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"experimentalFeature/list\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ExperimentalFeatureListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ExperimentalFeature/enablement/setRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"ExperimentalFeature/enablement/setRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"experimentalFeature/enablement/set\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ExperimentalFeatureEnablementSetParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"McpServer/oauth/loginRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"McpServer/oauth/loginRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mcpServer/oauth/login\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/McpServerOauthLoginParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Config/mcpServer/reloadRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Config/mcpServer/reloadRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"config/mcpServer/reload\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"McpServerStatus/listRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"McpServerStatus/listRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mcpServerStatus/list\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ListMcpServerStatusParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"McpServer/resource/readRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"McpServer/resource/readRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mcpServer/resource/read\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/McpResourceReadParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"McpServer/tool/callRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"McpServer/tool/callRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mcpServer/tool/call\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/McpServerToolCallParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"WindowsSandbox/setupStartRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"WindowsSandbox/setupStartRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"windowsSandbox/setupStart\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/WindowsSandboxSetupStartParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Account/login/startRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Account/login/startRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"account/login/start\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/LoginAccountParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Account/login/cancelRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Account/login/cancelRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"account/login/cancel\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/CancelLoginAccountParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Account/logoutRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Account/logoutRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"account/logout\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Account/rateLimits/readRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Account/rateLimits/readRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"account/rateLimits/read\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Feedback/uploadRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Feedback/uploadRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"feedback/upload\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FeedbackUploadParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Command/execRequest\","]
#[doc = "      \"description\": \"Execute a standalone command (argv vector) under the server's sandbox.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Command/execRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"command/exec\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Command/exec/writeRequest\","]
#[doc = "      \"description\": \"Write stdin bytes to a running `command/exec` session or close stdin.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Command/exec/writeRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"command/exec/write\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecWriteParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Command/exec/terminateRequest\","]
#[doc = "      \"description\": \"Terminate a running `command/exec` session by client-supplied `processId`.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Command/exec/terminateRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"command/exec/terminate\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecTerminateParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Command/exec/resizeRequest\","]
#[doc = "      \"description\": \"Resize a running PTY-backed `command/exec` session by client-supplied `processId`.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Command/exec/resizeRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"command/exec/resize\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecResizeParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Config/readRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Config/readRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"config/read\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigReadParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ExternalAgentConfig/detectRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"ExternalAgentConfig/detectRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"externalAgentConfig/detect\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ExternalAgentConfigDetectParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ExternalAgentConfig/importRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"ExternalAgentConfig/importRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"externalAgentConfig/import\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ExternalAgentConfigImportParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Config/value/writeRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Config/value/writeRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"config/value/write\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigValueWriteParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Config/batchWriteRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Config/batchWriteRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"config/batchWrite\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigBatchWriteParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ConfigRequirements/readRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"ConfigRequirements/readRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"configRequirements/read\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Account/readRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Account/readRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"account/read\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/GetAccountParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FuzzyFileSearchRequest\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        },"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"FuzzyFileSearchRequestMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fuzzyFileSearch\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FuzzyFileSearchParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "method")]
pub enum ClientRequest {
    #[doc = "InitializeRequest"]
    #[serde(rename = "initialize")]
    Initialize {
        id: RequestId,
        params: InitializeParams,
    },
    #[doc = "Thread/startRequest\n\nNEW APIs"]
    #[serde(rename = "thread/start")]
    ThreadStart {
        id: RequestId,
        params: ThreadStartParams,
    },
    #[doc = "Thread/resumeRequest"]
    #[serde(rename = "thread/resume")]
    ThreadResume {
        id: RequestId,
        params: ThreadResumeParams,
    },
    #[doc = "Thread/forkRequest"]
    #[serde(rename = "thread/fork")]
    ThreadFork {
        id: RequestId,
        params: ThreadForkParams,
    },
    #[doc = "Thread/archiveRequest"]
    #[serde(rename = "thread/archive")]
    ThreadArchive {
        id: RequestId,
        params: ThreadArchiveParams,
    },
    #[doc = "Thread/unsubscribeRequest"]
    #[serde(rename = "thread/unsubscribe")]
    ThreadUnsubscribe {
        id: RequestId,
        params: ThreadUnsubscribeParams,
    },
    #[doc = "Thread/name/setRequest"]
    #[serde(rename = "thread/name/set")]
    ThreadNameSet {
        id: RequestId,
        params: ThreadSetNameParams,
    },
    #[doc = "Thread/metadata/updateRequest"]
    #[serde(rename = "thread/metadata/update")]
    ThreadMetadataUpdate {
        id: RequestId,
        params: ThreadMetadataUpdateParams,
    },
    #[doc = "Thread/unarchiveRequest"]
    #[serde(rename = "thread/unarchive")]
    ThreadUnarchive {
        id: RequestId,
        params: ThreadUnarchiveParams,
    },
    #[doc = "Thread/compact/startRequest"]
    #[serde(rename = "thread/compact/start")]
    ThreadCompactStart {
        id: RequestId,
        params: ThreadCompactStartParams,
    },
    #[doc = "Thread/shellCommandRequest"]
    #[serde(rename = "thread/shellCommand")]
    ThreadShellCommand {
        id: RequestId,
        params: ThreadShellCommandParams,
    },
    #[doc = "Thread/rollbackRequest"]
    #[serde(rename = "thread/rollback")]
    ThreadRollback {
        id: RequestId,
        params: ThreadRollbackParams,
    },
    #[doc = "Thread/listRequest"]
    #[serde(rename = "thread/list")]
    ThreadList {
        id: RequestId,
        params: ThreadListParams,
    },
    #[doc = "Thread/loaded/listRequest"]
    #[serde(rename = "thread/loaded/list")]
    ThreadLoadedList {
        id: RequestId,
        params: ThreadLoadedListParams,
    },
    #[doc = "Thread/readRequest"]
    #[serde(rename = "thread/read")]
    ThreadRead {
        id: RequestId,
        params: ThreadReadParams,
    },
    #[doc = "Skills/listRequest"]
    #[serde(rename = "skills/list")]
    SkillsList {
        id: RequestId,
        params: SkillsListParams,
    },
    #[doc = "Plugin/listRequest"]
    #[serde(rename = "plugin/list")]
    PluginList {
        id: RequestId,
        params: PluginListParams,
    },
    #[doc = "Plugin/readRequest"]
    #[serde(rename = "plugin/read")]
    PluginRead {
        id: RequestId,
        params: PluginReadParams,
    },
    #[doc = "App/listRequest"]
    #[serde(rename = "app/list")]
    AppList {
        id: RequestId,
        params: AppsListParams,
    },
    #[doc = "Fs/readFileRequest"]
    #[serde(rename = "fs/readFile")]
    FsReadFile {
        id: RequestId,
        params: FsReadFileParams,
    },
    #[doc = "Fs/writeFileRequest"]
    #[serde(rename = "fs/writeFile")]
    FsWriteFile {
        id: RequestId,
        params: FsWriteFileParams,
    },
    #[doc = "Fs/createDirectoryRequest"]
    #[serde(rename = "fs/createDirectory")]
    FsCreateDirectory {
        id: RequestId,
        params: FsCreateDirectoryParams,
    },
    #[doc = "Fs/getMetadataRequest"]
    #[serde(rename = "fs/getMetadata")]
    FsGetMetadata {
        id: RequestId,
        params: FsGetMetadataParams,
    },
    #[doc = "Fs/readDirectoryRequest"]
    #[serde(rename = "fs/readDirectory")]
    FsReadDirectory {
        id: RequestId,
        params: FsReadDirectoryParams,
    },
    #[doc = "Fs/removeRequest"]
    #[serde(rename = "fs/remove")]
    FsRemove {
        id: RequestId,
        params: FsRemoveParams,
    },
    #[doc = "Fs/copyRequest"]
    #[serde(rename = "fs/copy")]
    FsCopy { id: RequestId, params: FsCopyParams },
    #[doc = "Fs/watchRequest"]
    #[serde(rename = "fs/watch")]
    FsWatch {
        id: RequestId,
        params: FsWatchParams,
    },
    #[doc = "Fs/unwatchRequest"]
    #[serde(rename = "fs/unwatch")]
    FsUnwatch {
        id: RequestId,
        params: FsUnwatchParams,
    },
    #[doc = "Skills/config/writeRequest"]
    #[serde(rename = "skills/config/write")]
    SkillsConfigWrite {
        id: RequestId,
        params: SkillsConfigWriteParams,
    },
    #[doc = "Plugin/installRequest"]
    #[serde(rename = "plugin/install")]
    PluginInstall {
        id: RequestId,
        params: PluginInstallParams,
    },
    #[doc = "Plugin/uninstallRequest"]
    #[serde(rename = "plugin/uninstall")]
    PluginUninstall {
        id: RequestId,
        params: PluginUninstallParams,
    },
    #[doc = "Turn/startRequest"]
    #[serde(rename = "turn/start")]
    TurnStart {
        id: RequestId,
        params: TurnStartParams,
    },
    #[doc = "Turn/steerRequest"]
    #[serde(rename = "turn/steer")]
    TurnSteer {
        id: RequestId,
        params: TurnSteerParams,
    },
    #[doc = "Turn/interruptRequest"]
    #[serde(rename = "turn/interrupt")]
    TurnInterrupt {
        id: RequestId,
        params: TurnInterruptParams,
    },
    #[doc = "Review/startRequest"]
    #[serde(rename = "review/start")]
    ReviewStart {
        id: RequestId,
        params: ReviewStartParams,
    },
    #[doc = "Model/listRequest"]
    #[serde(rename = "model/list")]
    ModelList {
        id: RequestId,
        params: ModelListParams,
    },
    #[doc = "ExperimentalFeature/listRequest"]
    #[serde(rename = "experimentalFeature/list")]
    ExperimentalFeatureList {
        id: RequestId,
        params: ExperimentalFeatureListParams,
    },
    #[doc = "ExperimentalFeature/enablement/setRequest"]
    #[serde(rename = "experimentalFeature/enablement/set")]
    ExperimentalFeatureEnablementSet {
        id: RequestId,
        params: ExperimentalFeatureEnablementSetParams,
    },
    #[doc = "McpServer/oauth/loginRequest"]
    #[serde(rename = "mcpServer/oauth/login")]
    McpServerOauthLogin {
        id: RequestId,
        params: McpServerOauthLoginParams,
    },
    #[doc = "Config/mcpServer/reloadRequest"]
    #[serde(rename = "config/mcpServer/reload")]
    ConfigMcpServerReload {
        id: RequestId,
        #[serde(default)]
        params: (),
    },
    #[doc = "McpServerStatus/listRequest"]
    #[serde(rename = "mcpServerStatus/list")]
    McpServerStatusList {
        id: RequestId,
        params: ListMcpServerStatusParams,
    },
    #[doc = "McpServer/resource/readRequest"]
    #[serde(rename = "mcpServer/resource/read")]
    McpServerResourceRead {
        id: RequestId,
        params: McpResourceReadParams,
    },
    #[doc = "McpServer/tool/callRequest"]
    #[serde(rename = "mcpServer/tool/call")]
    McpServerToolCall {
        id: RequestId,
        params: McpServerToolCallParams,
    },
    #[doc = "WindowsSandbox/setupStartRequest"]
    #[serde(rename = "windowsSandbox/setupStart")]
    WindowsSandboxSetupStart {
        id: RequestId,
        params: WindowsSandboxSetupStartParams,
    },
    #[doc = "Account/login/startRequest"]
    #[serde(rename = "account/login/start")]
    AccountLoginStart {
        id: RequestId,
        params: LoginAccountParams,
    },
    #[doc = "Account/login/cancelRequest"]
    #[serde(rename = "account/login/cancel")]
    AccountLoginCancel {
        id: RequestId,
        params: CancelLoginAccountParams,
    },
    #[doc = "Account/logoutRequest"]
    #[serde(rename = "account/logout")]
    AccountLogout {
        id: RequestId,
        #[serde(default)]
        params: (),
    },
    #[doc = "Account/rateLimits/readRequest"]
    #[serde(rename = "account/rateLimits/read")]
    AccountRateLimitsRead {
        id: RequestId,
        #[serde(default)]
        params: (),
    },
    #[doc = "Feedback/uploadRequest"]
    #[serde(rename = "feedback/upload")]
    FeedbackUpload {
        id: RequestId,
        params: FeedbackUploadParams,
    },
    #[doc = "Command/execRequest\n\nExecute a standalone command (argv vector) under the server's sandbox."]
    #[serde(rename = "command/exec")]
    CommandExec {
        id: RequestId,
        params: CommandExecParams,
    },
    #[doc = "Command/exec/writeRequest\n\nWrite stdin bytes to a running `command/exec` session or close stdin."]
    #[serde(rename = "command/exec/write")]
    CommandExecWrite {
        id: RequestId,
        params: CommandExecWriteParams,
    },
    #[doc = "Command/exec/terminateRequest\n\nTerminate a running `command/exec` session by client-supplied `processId`."]
    #[serde(rename = "command/exec/terminate")]
    CommandExecTerminate {
        id: RequestId,
        params: CommandExecTerminateParams,
    },
    #[doc = "Command/exec/resizeRequest\n\nResize a running PTY-backed `command/exec` session by client-supplied `processId`."]
    #[serde(rename = "command/exec/resize")]
    CommandExecResize {
        id: RequestId,
        params: CommandExecResizeParams,
    },
    #[doc = "Config/readRequest"]
    #[serde(rename = "config/read")]
    ConfigRead {
        id: RequestId,
        params: ConfigReadParams,
    },
    #[doc = "ExternalAgentConfig/detectRequest"]
    #[serde(rename = "externalAgentConfig/detect")]
    ExternalAgentConfigDetect {
        id: RequestId,
        params: ExternalAgentConfigDetectParams,
    },
    #[doc = "ExternalAgentConfig/importRequest"]
    #[serde(rename = "externalAgentConfig/import")]
    ExternalAgentConfigImport {
        id: RequestId,
        params: ExternalAgentConfigImportParams,
    },
    #[doc = "Config/value/writeRequest"]
    #[serde(rename = "config/value/write")]
    ConfigValueWrite {
        id: RequestId,
        params: ConfigValueWriteParams,
    },
    #[doc = "Config/batchWriteRequest"]
    #[serde(rename = "config/batchWrite")]
    ConfigBatchWrite {
        id: RequestId,
        params: ConfigBatchWriteParams,
    },
    #[doc = "ConfigRequirements/readRequest"]
    #[serde(rename = "configRequirements/read")]
    ConfigRequirementsRead {
        id: RequestId,
        #[serde(default)]
        params: (),
    },
    #[doc = "Account/readRequest"]
    #[serde(rename = "account/read")]
    AccountRead {
        id: RequestId,
        params: GetAccountParams,
    },
    #[doc = "FuzzyFileSearchRequest"]
    #[serde(rename = "fuzzyFileSearch")]
    FuzzyFileSearch {
        id: RequestId,
        params: FuzzyFileSearchParams,
    },
}
#[doc = "`CodexAppServerProtocolV2`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CodexAppServerProtocolV2\","]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CodexAppServerProtocolV2(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for CodexAppServerProtocolV2 {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<CodexAppServerProtocolV2>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: CodexAppServerProtocolV2) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for CodexAppServerProtocolV2
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "This translation layer make sure that we expose codex error code in camel case.\n\nWhen an upstream HTTP status is available (for example, from the Responses API or a provider), it is forwarded in `httpStatusCode` on the relevant `codexErrorInfo` variant."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This translation layer make sure that we expose codex error code in camel case.\\n\\nWhen an upstream HTTP status is available (for example, from the Responses API or a provider), it is forwarded in `httpStatusCode` on the relevant `codexErrorInfo` variant.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"contextWindowExceeded\","]
#[doc = "        \"usageLimitExceeded\","]
#[doc = "        \"serverOverloaded\","]
#[doc = "        \"internalServerError\","]
#[doc = "        \"unauthorized\","]
#[doc = "        \"badRequest\","]
#[doc = "        \"threadRollbackFailed\","]
#[doc = "        \"sandboxError\","]
#[doc = "        \"other\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"HttpConnectionFailedCodexErrorInfo\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"httpConnectionFailed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"httpConnectionFailed\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"httpStatusCode\": {"]
#[doc = "              \"type\": ["]
#[doc = "                \"integer\","]
#[doc = "                \"null\""]
#[doc = "              ],"]
#[doc = "              \"format\": \"uint16\","]
#[doc = "              \"minimum\": 0.0"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ResponseStreamConnectionFailedCodexErrorInfo\","]
#[doc = "      \"description\": \"Failed to connect to the response SSE stream.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"responseStreamConnectionFailed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"responseStreamConnectionFailed\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"httpStatusCode\": {"]
#[doc = "              \"type\": ["]
#[doc = "                \"integer\","]
#[doc = "                \"null\""]
#[doc = "              ],"]
#[doc = "              \"format\": \"uint16\","]
#[doc = "              \"minimum\": 0.0"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ResponseStreamDisconnectedCodexErrorInfo\","]
#[doc = "      \"description\": \"The response SSE stream disconnected in the middle of a turn before completion.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"responseStreamDisconnected\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"responseStreamDisconnected\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"httpStatusCode\": {"]
#[doc = "              \"type\": ["]
#[doc = "                \"integer\","]
#[doc = "                \"null\""]
#[doc = "              ],"]
#[doc = "              \"format\": \"uint16\","]
#[doc = "              \"minimum\": 0.0"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ResponseTooManyFailedAttemptsCodexErrorInfo\","]
#[doc = "      \"description\": \"Reached the retry limit for responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"responseTooManyFailedAttempts\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"responseTooManyFailedAttempts\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"httpStatusCode\": {"]
#[doc = "              \"type\": ["]
#[doc = "                \"integer\","]
#[doc = "                \"null\""]
#[doc = "              ],"]
#[doc = "              \"format\": \"uint16\","]
#[doc = "              \"minimum\": 0.0"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ActiveTurnNotSteerableCodexErrorInfo\","]
#[doc = "      \"description\": \"Returned when `turn/start` or `turn/steer` is submitted while the current active turn cannot accept same-turn steering, for example `/review` or manual `/compact`.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"activeTurnNotSteerable\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"activeTurnNotSteerable\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"turnKind\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"turnKind\": {"]
#[doc = "              \"$ref\": \"#/definitions/NonSteerableTurnKind\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum CodexErrorInfo {
    #[serde(rename = "contextWindowExceeded")]
    ContextWindowExceeded,
    #[serde(rename = "usageLimitExceeded")]
    UsageLimitExceeded,
    #[serde(rename = "serverOverloaded")]
    ServerOverloaded,
    #[serde(rename = "internalServerError")]
    InternalServerError,
    #[serde(rename = "unauthorized")]
    Unauthorized,
    #[serde(rename = "badRequest")]
    BadRequest,
    #[serde(rename = "threadRollbackFailed")]
    ThreadRollbackFailed,
    #[serde(rename = "sandboxError")]
    SandboxError,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "httpConnectionFailed")]
    HttpConnectionFailed {
        #[serde(
            rename = "httpStatusCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        http_status_code: ::std::option::Option<u16>,
    },
    #[doc = "Failed to connect to the response SSE stream."]
    #[serde(rename = "responseStreamConnectionFailed")]
    ResponseStreamConnectionFailed {
        #[serde(
            rename = "httpStatusCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        http_status_code: ::std::option::Option<u16>,
    },
    #[doc = "The response SSE stream disconnected in the middle of a turn before completion."]
    #[serde(rename = "responseStreamDisconnected")]
    ResponseStreamDisconnected {
        #[serde(
            rename = "httpStatusCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        http_status_code: ::std::option::Option<u16>,
    },
    #[doc = "Reached the retry limit for responses."]
    #[serde(rename = "responseTooManyFailedAttempts")]
    ResponseTooManyFailedAttempts {
        #[serde(
            rename = "httpStatusCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        http_status_code: ::std::option::Option<u16>,
    },
    #[doc = "Returned when `turn/start` or `turn/steer` is submitted while the current active turn cannot accept same-turn steering, for example `/review` or manual `/compact`."]
    #[serde(rename = "activeTurnNotSteerable")]
    ActiveTurnNotSteerable {
        #[serde(rename = "turnKind")]
        turn_kind: NonSteerableTurnKind,
    },
}
#[doc = "`CollabAgentState`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/CollabAgentStatus\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CollabAgentState {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    pub status: CollabAgentStatus,
}
#[doc = "`CollabAgentStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"pendingInit\","]
#[doc = "    \"running\","]
#[doc = "    \"interrupted\","]
#[doc = "    \"completed\","]
#[doc = "    \"errored\","]
#[doc = "    \"shutdown\","]
#[doc = "    \"notFound\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CollabAgentStatus {
    #[serde(rename = "pendingInit")]
    PendingInit,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "interrupted")]
    Interrupted,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "errored")]
    Errored,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "notFound")]
    NotFound,
}
impl ::std::fmt::Display for CollabAgentStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PendingInit => f.write_str("pendingInit"),
            Self::Running => f.write_str("running"),
            Self::Interrupted => f.write_str("interrupted"),
            Self::Completed => f.write_str("completed"),
            Self::Errored => f.write_str("errored"),
            Self::Shutdown => f.write_str("shutdown"),
            Self::NotFound => f.write_str("notFound"),
        }
    }
}
impl ::std::str::FromStr for CollabAgentStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pendingInit" => Ok(Self::PendingInit),
            "running" => Ok(Self::Running),
            "interrupted" => Ok(Self::Interrupted),
            "completed" => Ok(Self::Completed),
            "errored" => Ok(Self::Errored),
            "shutdown" => Ok(Self::Shutdown),
            "notFound" => Ok(Self::NotFound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CollabAgentStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CollabAgentStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CollabAgentStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CollabAgentTool`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"spawnAgent\","]
#[doc = "    \"sendInput\","]
#[doc = "    \"resumeAgent\","]
#[doc = "    \"wait\","]
#[doc = "    \"closeAgent\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CollabAgentTool {
    #[serde(rename = "spawnAgent")]
    SpawnAgent,
    #[serde(rename = "sendInput")]
    SendInput,
    #[serde(rename = "resumeAgent")]
    ResumeAgent,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "closeAgent")]
    CloseAgent,
}
impl ::std::fmt::Display for CollabAgentTool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SpawnAgent => f.write_str("spawnAgent"),
            Self::SendInput => f.write_str("sendInput"),
            Self::ResumeAgent => f.write_str("resumeAgent"),
            Self::Wait => f.write_str("wait"),
            Self::CloseAgent => f.write_str("closeAgent"),
        }
    }
}
impl ::std::str::FromStr for CollabAgentTool {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "spawnAgent" => Ok(Self::SpawnAgent),
            "sendInput" => Ok(Self::SendInput),
            "resumeAgent" => Ok(Self::ResumeAgent),
            "wait" => Ok(Self::Wait),
            "closeAgent" => Ok(Self::CloseAgent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CollabAgentTool {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CollabAgentTool {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CollabAgentTool {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CollabAgentToolCallStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inProgress\","]
#[doc = "    \"completed\","]
#[doc = "    \"failed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CollabAgentToolCallStatus {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for CollabAgentToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("inProgress"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for CollabAgentToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inProgress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CollabAgentToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CollabAgentToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CollabAgentToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Collaboration mode for a Codex session."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Collaboration mode for a Codex session.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"mode\","]
#[doc = "    \"settings\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"mode\": {"]
#[doc = "      \"$ref\": \"#/definitions/ModeKind\""]
#[doc = "    },"]
#[doc = "    \"settings\": {"]
#[doc = "      \"$ref\": \"#/definitions/Settings\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CollaborationMode {
    pub mode: ModeKind,
    pub settings: Settings,
}
#[doc = "EXPERIMENTAL - collaboration mode preset metadata for clients."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"EXPERIMENTAL - collaboration mode preset metadata for clients.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"mode\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ModeKind\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reasoning_effort\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CollaborationModeMask {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<ModeKind>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<ReasoningEffort>,
}
#[doc = "`CommandAction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"ReadCommandAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"command\","]
#[doc = "        \"name\","]
#[doc = "        \"path\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"command\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ReadCommandActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"read\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ListFilesCommandAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"command\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"command\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ListFilesCommandActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"listFiles\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"SearchCommandAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"command\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"command\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"query\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"SearchCommandActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"search\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"UnknownCommandAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"command\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"command\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"UnknownCommandActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"unknown\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum CommandAction {
    #[doc = "ReadCommandAction"]
    #[serde(rename = "read")]
    Read {
        command: ::std::string::String,
        name: ::std::string::String,
        path: ::std::string::String,
    },
    #[doc = "ListFilesCommandAction"]
    #[serde(rename = "listFiles")]
    ListFiles {
        command: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        path: ::std::option::Option<::std::string::String>,
    },
    #[doc = "SearchCommandAction"]
    #[serde(rename = "search")]
    Search {
        command: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        query: ::std::option::Option<::std::string::String>,
    },
    #[doc = "UnknownCommandAction"]
    #[serde(rename = "unknown")]
    Unknown { command: ::std::string::String },
}
#[doc = "Base64-encoded output chunk emitted for a streaming `command/exec` request.\n\nThese notifications are connection-scoped. If the originating connection closes, the server terminates the process."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecOutputDeltaNotification\","]
#[doc = "  \"description\": \"Base64-encoded output chunk emitted for a streaming `command/exec` request.\\n\\nThese notifications are connection-scoped. If the originating connection closes, the server terminates the process.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capReached\","]
#[doc = "    \"deltaBase64\","]
#[doc = "    \"processId\","]
#[doc = "    \"stream\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capReached\": {"]
#[doc = "      \"description\": \"`true` on the final streamed chunk for a stream when `outputBytesCap` truncated later output on that stream.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"deltaBase64\": {"]
#[doc = "      \"description\": \"Base64-encoded output bytes.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"processId\": {"]
#[doc = "      \"description\": \"Client-supplied, connection-scoped `processId` from the original `command/exec` request.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stream\": {"]
#[doc = "      \"description\": \"Output stream for this chunk.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecOutputStream\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandExecOutputDeltaNotification {
    #[doc = "`true` on the final streamed chunk for a stream when `outputBytesCap` truncated later output on that stream."]
    #[serde(rename = "capReached")]
    pub cap_reached: bool,
    #[doc = "Base64-encoded output bytes."]
    #[serde(rename = "deltaBase64")]
    pub delta_base64: ::std::string::String,
    #[doc = "Client-supplied, connection-scoped `processId` from the original `command/exec` request."]
    #[serde(rename = "processId")]
    pub process_id: ::std::string::String,
    #[doc = "Output stream for this chunk."]
    pub stream: CommandExecOutputStream,
}
#[doc = "Stream label for `command/exec/outputDelta` notifications."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stream label for `command/exec/outputDelta` notifications.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"stdout stream. PTY mode multiplexes terminal output here.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"stdout\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"stderr stream.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"stderr\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CommandExecOutputStream {
    #[doc = "stdout stream. PTY mode multiplexes terminal output here."]
    #[serde(rename = "stdout")]
    Stdout,
    #[doc = "stderr stream."]
    #[serde(rename = "stderr")]
    Stderr,
}
impl ::std::fmt::Display for CommandExecOutputStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stdout => f.write_str("stdout"),
            Self::Stderr => f.write_str("stderr"),
        }
    }
}
impl ::std::str::FromStr for CommandExecOutputStream {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stdout" => Ok(Self::Stdout),
            "stderr" => Ok(Self::Stderr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CommandExecOutputStream {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CommandExecOutputStream {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CommandExecOutputStream {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Run a standalone command (argv vector) in the server sandbox without creating a thread or turn.\n\nThe final `command/exec` response is deferred until the process exits and is sent only after all `command/exec/outputDelta` notifications for that connection have been emitted."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecParams\","]
#[doc = "  \"description\": \"Run a standalone command (argv vector) in the server sandbox without creating a thread or turn.\\n\\nThe final `command/exec` response is deferred until the process exits and is sent only after all `command/exec/outputDelta` notifications for that connection have been emitted.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command\": {"]
#[doc = "      \"description\": \"Command argv vector. Empty arrays are rejected.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"description\": \"Optional working directory. Defaults to the server cwd.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"disableOutputCap\": {"]
#[doc = "      \"description\": \"Disable stdout/stderr capture truncation for this request.\\n\\nCannot be combined with `outputBytesCap`.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"disableTimeout\": {"]
#[doc = "      \"description\": \"Disable the timeout entirely for this request.\\n\\nCannot be combined with `timeoutMs`.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"env\": {"]
#[doc = "      \"description\": \"Optional environment overrides merged into the server-computed environment.\\n\\nMatching names override inherited values. Set a key to `null` to unset an inherited variable.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": ["]
#[doc = "          \"string\","]
#[doc = "          \"null\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"outputBytesCap\": {"]
#[doc = "      \"description\": \"Optional per-stream stdout/stderr capture cap in bytes.\\n\\nWhen omitted, the server default applies. Cannot be combined with `disableOutputCap`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"processId\": {"]
#[doc = "      \"description\": \"Optional client-supplied, connection-scoped process id.\\n\\nRequired for `tty`, `streamStdin`, `streamStdoutStderr`, and follow-up `command/exec/write`, `command/exec/resize`, and `command/exec/terminate` calls. When omitted, buffered execution gets an internal id that is not exposed to the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandboxPolicy\": {"]
#[doc = "      \"description\": \"Optional sandbox policy for this command.\\n\\nUses the same shape as thread/turn execution sandbox configuration and defaults to the user's configured policy when omitted.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SandboxPolicy\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"Optional initial PTY size in character cells. Only valid when `tty` is true.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecTerminalSize\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"streamStdin\": {"]
#[doc = "      \"description\": \"Allow follow-up `command/exec/write` requests to write stdin bytes.\\n\\nRequires a client-supplied `processId`.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"streamStdoutStderr\": {"]
#[doc = "      \"description\": \"Stream stdout/stderr via `command/exec/outputDelta` notifications.\\n\\nStreamed bytes are not duplicated into the final response and require a client-supplied `processId`.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"timeoutMs\": {"]
#[doc = "      \"description\": \"Optional timeout in milliseconds.\\n\\nWhen omitted, the server default applies. Cannot be combined with `disableTimeout`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"tty\": {"]
#[doc = "      \"description\": \"Enable PTY mode.\\n\\nThis implies `streamStdin` and `streamStdoutStderr`.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandExecParams {
    #[doc = "Command argv vector. Empty arrays are rejected."]
    pub command: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional working directory. Defaults to the server cwd."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[doc = "Disable stdout/stderr capture truncation for this request.\n\nCannot be combined with `outputBytesCap`."]
    #[serde(
        rename = "disableOutputCap",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disable_output_cap: ::std::option::Option<bool>,
    #[doc = "Disable the timeout entirely for this request.\n\nCannot be combined with `timeoutMs`."]
    #[serde(
        rename = "disableTimeout",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disable_timeout: ::std::option::Option<bool>,
    #[doc = "Optional environment overrides merged into the server-computed environment.\n\nMatching names override inherited values. Set a key to `null` to unset an inherited variable."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub env: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::option::Option<::std::string::String>,
        >,
    >,
    #[doc = "Optional per-stream stdout/stderr capture cap in bytes.\n\nWhen omitted, the server default applies. Cannot be combined with `disableOutputCap`."]
    #[serde(
        rename = "outputBytesCap",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub output_bytes_cap: ::std::option::Option<u32>,
    #[doc = "Optional client-supplied, connection-scoped process id.\n\nRequired for `tty`, `streamStdin`, `streamStdoutStderr`, and follow-up `command/exec/write`, `command/exec/resize`, and `command/exec/terminate` calls. When omitted, buffered execution gets an internal id that is not exposed to the client."]
    #[serde(
        rename = "processId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub process_id: ::std::option::Option<::std::string::String>,
    #[doc = "Optional sandbox policy for this command.\n\nUses the same shape as thread/turn execution sandbox configuration and defaults to the user's configured policy when omitted."]
    #[serde(
        rename = "sandboxPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sandbox_policy: ::std::option::Option<SandboxPolicy>,
    #[doc = "Optional initial PTY size in character cells. Only valid when `tty` is true."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<CommandExecTerminalSize>,
    #[doc = "Allow follow-up `command/exec/write` requests to write stdin bytes.\n\nRequires a client-supplied `processId`."]
    #[serde(
        rename = "streamStdin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stream_stdin: ::std::option::Option<bool>,
    #[doc = "Stream stdout/stderr via `command/exec/outputDelta` notifications.\n\nStreamed bytes are not duplicated into the final response and require a client-supplied `processId`."]
    #[serde(
        rename = "streamStdoutStderr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stream_stdout_stderr: ::std::option::Option<bool>,
    #[doc = "Optional timeout in milliseconds.\n\nWhen omitted, the server default applies. Cannot be combined with `disableTimeout`."]
    #[serde(
        rename = "timeoutMs",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub timeout_ms: ::std::option::Option<i64>,
    #[doc = "Enable PTY mode.\n\nThis implies `streamStdin` and `streamStdoutStderr`."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tty: ::std::option::Option<bool>,
}
#[doc = "Resize a running PTY-backed `command/exec` session."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecResizeParams\","]
#[doc = "  \"description\": \"Resize a running PTY-backed `command/exec` session.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"processId\","]
#[doc = "    \"size\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"processId\": {"]
#[doc = "      \"description\": \"Client-supplied, connection-scoped `processId` from the original `command/exec` request.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"New PTY size in character cells.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecTerminalSize\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandExecResizeParams {
    #[doc = "Client-supplied, connection-scoped `processId` from the original `command/exec` request."]
    #[serde(rename = "processId")]
    pub process_id: ::std::string::String,
    #[doc = "New PTY size in character cells."]
    pub size: CommandExecTerminalSize,
}
#[doc = "Empty success response for `command/exec/resize`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecResizeResponse\","]
#[doc = "  \"description\": \"Empty success response for `command/exec/resize`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CommandExecResizeResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for CommandExecResizeResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<CommandExecResizeResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: CommandExecResizeResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for CommandExecResizeResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Final buffered result for `command/exec`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecResponse\","]
#[doc = "  \"description\": \"Final buffered result for `command/exec`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"exitCode\","]
#[doc = "    \"stderr\","]
#[doc = "    \"stdout\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"exitCode\": {"]
#[doc = "      \"description\": \"Process exit code.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"stderr\": {"]
#[doc = "      \"description\": \"Buffered stderr capture.\\n\\nEmpty when stderr was streamed via `command/exec/outputDelta`.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stdout\": {"]
#[doc = "      \"description\": \"Buffered stdout capture.\\n\\nEmpty when stdout was streamed via `command/exec/outputDelta`.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandExecResponse {
    #[doc = "Process exit code."]
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    #[doc = "Buffered stderr capture.\n\nEmpty when stderr was streamed via `command/exec/outputDelta`."]
    pub stderr: ::std::string::String,
    #[doc = "Buffered stdout capture.\n\nEmpty when stdout was streamed via `command/exec/outputDelta`."]
    pub stdout: ::std::string::String,
}
#[doc = "PTY size in character cells for `command/exec` PTY sessions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"PTY size in character cells for `command/exec` PTY sessions.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cols\","]
#[doc = "    \"rows\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cols\": {"]
#[doc = "      \"description\": \"Terminal width in character cells.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"rows\": {"]
#[doc = "      \"description\": \"Terminal height in character cells.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandExecTerminalSize {
    #[doc = "Terminal width in character cells."]
    pub cols: u16,
    #[doc = "Terminal height in character cells."]
    pub rows: u16,
}
#[doc = "Terminate a running `command/exec` session."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecTerminateParams\","]
#[doc = "  \"description\": \"Terminate a running `command/exec` session.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"processId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"processId\": {"]
#[doc = "      \"description\": \"Client-supplied, connection-scoped `processId` from the original `command/exec` request.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandExecTerminateParams {
    #[doc = "Client-supplied, connection-scoped `processId` from the original `command/exec` request."]
    #[serde(rename = "processId")]
    pub process_id: ::std::string::String,
}
#[doc = "Empty success response for `command/exec/terminate`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecTerminateResponse\","]
#[doc = "  \"description\": \"Empty success response for `command/exec/terminate`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CommandExecTerminateResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for CommandExecTerminateResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<CommandExecTerminateResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: CommandExecTerminateResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for CommandExecTerminateResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Write stdin bytes to a running `command/exec` session, close stdin, or both."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecWriteParams\","]
#[doc = "  \"description\": \"Write stdin bytes to a running `command/exec` session, close stdin, or both.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"processId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"closeStdin\": {"]
#[doc = "      \"description\": \"Close stdin after writing `deltaBase64`, if present.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"deltaBase64\": {"]
#[doc = "      \"description\": \"Optional base64-encoded stdin bytes to write.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"processId\": {"]
#[doc = "      \"description\": \"Client-supplied, connection-scoped `processId` from the original `command/exec` request.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandExecWriteParams {
    #[doc = "Close stdin after writing `deltaBase64`, if present."]
    #[serde(
        rename = "closeStdin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub close_stdin: ::std::option::Option<bool>,
    #[doc = "Optional base64-encoded stdin bytes to write."]
    #[serde(
        rename = "deltaBase64",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub delta_base64: ::std::option::Option<::std::string::String>,
    #[doc = "Client-supplied, connection-scoped `processId` from the original `command/exec` request."]
    #[serde(rename = "processId")]
    pub process_id: ::std::string::String,
}
#[doc = "Empty success response for `command/exec/write`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecWriteResponse\","]
#[doc = "  \"description\": \"Empty success response for `command/exec/write`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CommandExecWriteResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for CommandExecWriteResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<CommandExecWriteResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: CommandExecWriteResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for CommandExecWriteResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`CommandExecutionOutputDeltaNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CommandExecutionOutputDeltaNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"delta\","]
#[doc = "    \"itemId\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"delta\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandExecutionOutputDeltaNotification {
    pub delta: ::std::string::String,
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`CommandExecutionSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"agent\","]
#[doc = "    \"userShell\","]
#[doc = "    \"unifiedExecStartup\","]
#[doc = "    \"unifiedExecInteraction\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CommandExecutionSource {
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "userShell")]
    UserShell,
    #[serde(rename = "unifiedExecStartup")]
    UnifiedExecStartup,
    #[serde(rename = "unifiedExecInteraction")]
    UnifiedExecInteraction,
}
impl ::std::fmt::Display for CommandExecutionSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Agent => f.write_str("agent"),
            Self::UserShell => f.write_str("userShell"),
            Self::UnifiedExecStartup => f.write_str("unifiedExecStartup"),
            Self::UnifiedExecInteraction => f.write_str("unifiedExecInteraction"),
        }
    }
}
impl ::std::str::FromStr for CommandExecutionSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agent" => Ok(Self::Agent),
            "userShell" => Ok(Self::UserShell),
            "unifiedExecStartup" => Ok(Self::UnifiedExecStartup),
            "unifiedExecInteraction" => Ok(Self::UnifiedExecInteraction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CommandExecutionSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CommandExecutionSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CommandExecutionSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CommandExecutionStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inProgress\","]
#[doc = "    \"completed\","]
#[doc = "    \"failed\","]
#[doc = "    \"declined\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CommandExecutionStatus {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "declined")]
    Declined,
}
impl ::std::fmt::Display for CommandExecutionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("inProgress"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
            Self::Declined => f.write_str("declined"),
        }
    }
}
impl ::std::str::FromStr for CommandExecutionStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inProgress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "declined" => Ok(Self::Declined),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CommandExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CommandExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CommandExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Config`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"analytics\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AnalyticsConfig\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"approval_policy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"approvals_reviewer\": {"]
#[doc = "      \"description\": \"[UNSTABLE] Optional default for where approval requests are routed for review.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"compact_prompt\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"developer_instructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"forced_chatgpt_workspace_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"forced_login_method\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ForcedLoginMethod\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"instructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model_auto_compact_token_limit\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"model_context_window\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"model_provider\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model_reasoning_effort\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model_reasoning_summary\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningSummary\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model_verbosity\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Verbosity\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"profile\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"profiles\": {"]
#[doc = "      \"default\": {},"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/ProfileV2\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"review_model\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandbox_mode\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SandboxMode\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandbox_workspace_write\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SandboxWorkspaceWrite\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"service_tier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tools\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ToolsV2\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"web_search\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/WebSearchMode\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Config {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub analytics: ::std::option::Option<AnalyticsConfig>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub approval_policy: ::std::option::Option<AskForApproval>,
    #[doc = "[UNSTABLE] Optional default for where approval requests are routed for review."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub approvals_reviewer: ::std::option::Option<ApprovalsReviewer>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub compact_prompt: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub developer_instructions: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub forced_chatgpt_workspace_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub forced_login_method: ::std::option::Option<ForcedLoginMethod>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_auto_compact_token_limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_context_window: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_provider: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_reasoning_effort: ::std::option::Option<ReasoningEffort>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_reasoning_summary: ::std::option::Option<ReasoningSummary>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_verbosity: ::std::option::Option<Verbosity>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub profile: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub profiles: ::std::collections::HashMap<::std::string::String, ProfileV2>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub review_model: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sandbox_mode: ::std::option::Option<SandboxMode>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sandbox_workspace_write: ::std::option::Option<SandboxWorkspaceWrite>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<ToolsV2>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub web_search: ::std::option::Option<WebSearchMode>,
}
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            analytics: Default::default(),
            approval_policy: Default::default(),
            approvals_reviewer: Default::default(),
            compact_prompt: Default::default(),
            developer_instructions: Default::default(),
            forced_chatgpt_workspace_id: Default::default(),
            forced_login_method: Default::default(),
            instructions: Default::default(),
            model: Default::default(),
            model_auto_compact_token_limit: Default::default(),
            model_context_window: Default::default(),
            model_provider: Default::default(),
            model_reasoning_effort: Default::default(),
            model_reasoning_summary: Default::default(),
            model_verbosity: Default::default(),
            profile: Default::default(),
            profiles: Default::default(),
            review_model: Default::default(),
            sandbox_mode: Default::default(),
            sandbox_workspace_write: Default::default(),
            service_tier: Default::default(),
            tools: Default::default(),
            web_search: Default::default(),
        }
    }
}
#[doc = "`ConfigBatchWriteParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ConfigBatchWriteParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"edits\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"edits\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ConfigEdit\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"expectedVersion\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"filePath\": {"]
#[doc = "      \"description\": \"Path to the config file to write; defaults to the user's `config.toml` when omitted.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"reloadUserConfig\": {"]
#[doc = "      \"description\": \"When true, hot-reload the updated user config into all loaded threads after writing.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigBatchWriteParams {
    pub edits: ::std::vec::Vec<ConfigEdit>,
    #[serde(
        rename = "expectedVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expected_version: ::std::option::Option<::std::string::String>,
    #[doc = "Path to the config file to write; defaults to the user's `config.toml` when omitted."]
    #[serde(
        rename = "filePath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_path: ::std::option::Option<::std::string::String>,
    #[doc = "When true, hot-reload the updated user config into all loaded threads after writing."]
    #[serde(
        rename = "reloadUserConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reload_user_config: ::std::option::Option<bool>,
}
#[doc = "`ConfigEdit`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"keyPath\","]
#[doc = "    \"mergeStrategy\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"keyPath\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mergeStrategy\": {"]
#[doc = "      \"$ref\": \"#/definitions/MergeStrategy\""]
#[doc = "    },"]
#[doc = "    \"value\": true"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigEdit {
    #[serde(rename = "keyPath")]
    pub key_path: ::std::string::String,
    #[serde(rename = "mergeStrategy")]
    pub merge_strategy: MergeStrategy,
    pub value: ::serde_json::Value,
}
#[doc = "`ConfigLayer`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"config\","]
#[doc = "    \"name\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"config\": true,"]
#[doc = "    \"disabledReason\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/definitions/ConfigLayerSource\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigLayer {
    pub config: ::serde_json::Value,
    #[serde(
        rename = "disabledReason",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disabled_reason: ::std::option::Option<::std::string::String>,
    pub name: ConfigLayerSource,
    pub version: ::std::string::String,
}
#[doc = "`ConfigLayerMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/definitions/ConfigLayerSource\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigLayerMetadata {
    pub name: ConfigLayerSource,
    pub version: ::std::string::String,
}
#[doc = "`ConfigLayerSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"MdmConfigLayerSource\","]
#[doc = "      \"description\": \"Managed preferences layer delivered by MDM (macOS only).\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"domain\","]
#[doc = "        \"key\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"domain\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"key\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"MdmConfigLayerSourceType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mdm\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"SystemConfigLayerSource\","]
#[doc = "      \"description\": \"Managed config layer from a file (usually `managed_config.toml`).\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"file\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"file\": {"]
#[doc = "          \"description\": \"This is the path to the system config.toml file, though it is not guaranteed to exist.\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"SystemConfigLayerSourceType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"system\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"UserConfigLayerSource\","]
#[doc = "      \"description\": \"User config layer from $CODEX_HOME/config.toml. This layer is special in that it is expected to be: - writable by the user - generally outside the workspace directory\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"file\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"file\": {"]
#[doc = "          \"description\": \"This is the path to the user's config.toml file, though it is not guaranteed to exist.\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"UserConfigLayerSourceType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"user\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ProjectConfigLayerSource\","]
#[doc = "      \"description\": \"Path to a .codex/ folder within a project. There could be multiple of these between `cwd` and the project/repo root.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"dotCodexFolder\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"dotCodexFolder\": {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ProjectConfigLayerSourceType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"project\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"SessionFlagsConfigLayerSource\","]
#[doc = "      \"description\": \"Session-layer overrides supplied via `-c`/`--config`.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"SessionFlagsConfigLayerSourceType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"sessionFlags\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"LegacyManagedConfigTomlFromFileConfigLayerSource\","]
#[doc = "      \"description\": \"`managed_config.toml` was designed to be a config that was loaded as the last layer on top of everything else. This scheme did not quite work out as intended, but we keep this variant as a \\\"best effort\\\" while we phase out `managed_config.toml` in favor of `requirements.toml`.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"file\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"file\": {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"LegacyManagedConfigTomlFromFileConfigLayerSourceType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"legacyManagedConfigTomlFromFile\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"LegacyManagedConfigTomlFromMdmConfigLayerSource\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"LegacyManagedConfigTomlFromMdmConfigLayerSourceType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"legacyManagedConfigTomlFromMdm\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ConfigLayerSource {
    #[doc = "MdmConfigLayerSource\n\nManaged preferences layer delivered by MDM (macOS only)."]
    #[serde(rename = "mdm")]
    Mdm {
        domain: ::std::string::String,
        key: ::std::string::String,
    },
    #[doc = "SystemConfigLayerSource\n\nManaged config layer from a file (usually `managed_config.toml`)."]
    #[serde(rename = "system")]
    System {
        #[doc = "This is the path to the system config.toml file, though it is not guaranteed to exist."]
        file: AbsolutePathBuf,
    },
    #[doc = "UserConfigLayerSource\n\nUser config layer from $CODEX_HOME/config.toml. This layer is special in that it is expected to be: - writable by the user - generally outside the workspace directory"]
    #[serde(rename = "user")]
    User {
        #[doc = "This is the path to the user's config.toml file, though it is not guaranteed to exist."]
        file: AbsolutePathBuf,
    },
    #[doc = "ProjectConfigLayerSource\n\nPath to a .codex/ folder within a project. There could be multiple of these between `cwd` and the project/repo root."]
    #[serde(rename = "project")]
    Project {
        #[serde(rename = "dotCodexFolder")]
        dot_codex_folder: AbsolutePathBuf,
    },
    #[serde(rename = "sessionFlags")]
    SessionFlags,
    #[doc = "LegacyManagedConfigTomlFromFileConfigLayerSource\n\n`managed_config.toml` was designed to be a config that was loaded as the last layer on top of everything else. This scheme did not quite work out as intended, but we keep this variant as a \"best effort\" while we phase out `managed_config.toml` in favor of `requirements.toml`."]
    #[serde(rename = "legacyManagedConfigTomlFromFile")]
    LegacyManagedConfigTomlFromFile { file: AbsolutePathBuf },
    #[serde(rename = "legacyManagedConfigTomlFromMdm")]
    LegacyManagedConfigTomlFromMdm,
}
#[doc = "`ConfigReadParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ConfigReadParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"description\": \"Optional working directory to resolve project config layers. If specified, return the effective config as seen from that directory (i.e., including any project layers between `cwd` and the project/repo root).\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"includeLayers\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigReadParams {
    #[doc = "Optional working directory to resolve project config layers. If specified, return the effective config as seen from that directory (i.e., including any project layers between `cwd` and the project/repo root)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(rename = "includeLayers", default)]
    pub include_layers: bool,
}
impl ::std::default::Default for ConfigReadParams {
    fn default() -> Self {
        Self {
            cwd: Default::default(),
            include_layers: Default::default(),
        }
    }
}
#[doc = "`ConfigReadResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ConfigReadResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"config\","]
#[doc = "    \"origins\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"config\": {"]
#[doc = "      \"$ref\": \"#/definitions/Config\""]
#[doc = "    },"]
#[doc = "    \"layers\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ConfigLayer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/ConfigLayerMetadata\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigReadResponse {
    pub config: Config,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub layers: ::std::option::Option<::std::vec::Vec<ConfigLayer>>,
    pub origins: ::std::collections::HashMap<::std::string::String, ConfigLayerMetadata>,
}
#[doc = "`ConfigRequirements`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowedApprovalPolicies\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"allowedSandboxModes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SandboxMode\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"allowedWebSearchModes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/WebSearchMode\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enforceResidency\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ResidencyRequirement\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"featureRequirements\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigRequirements {
    #[serde(
        rename = "allowedApprovalPolicies",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allowed_approval_policies: ::std::option::Option<::std::vec::Vec<AskForApproval>>,
    #[serde(
        rename = "allowedSandboxModes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allowed_sandbox_modes: ::std::option::Option<::std::vec::Vec<SandboxMode>>,
    #[serde(
        rename = "allowedWebSearchModes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allowed_web_search_modes: ::std::option::Option<::std::vec::Vec<WebSearchMode>>,
    #[serde(
        rename = "enforceResidency",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub enforce_residency: ::std::option::Option<ResidencyRequirement>,
    #[serde(
        rename = "featureRequirements",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub feature_requirements:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, bool>>,
}
impl ::std::default::Default for ConfigRequirements {
    fn default() -> Self {
        Self {
            allowed_approval_policies: Default::default(),
            allowed_sandbox_modes: Default::default(),
            allowed_web_search_modes: Default::default(),
            enforce_residency: Default::default(),
            feature_requirements: Default::default(),
        }
    }
}
#[doc = "`ConfigRequirementsReadResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ConfigRequirementsReadResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"requirements\": {"]
#[doc = "      \"description\": \"Null if no requirements are configured (e.g. no requirements.toml/MDM entries).\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigRequirements\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigRequirementsReadResponse {
    #[doc = "Null if no requirements are configured (e.g. no requirements.toml/MDM entries)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub requirements: ::std::option::Option<ConfigRequirements>,
}
impl ::std::default::Default for ConfigRequirementsReadResponse {
    fn default() -> Self {
        Self {
            requirements: Default::default(),
        }
    }
}
#[doc = "`ConfigValueWriteParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ConfigValueWriteParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"keyPath\","]
#[doc = "    \"mergeStrategy\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expectedVersion\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"filePath\": {"]
#[doc = "      \"description\": \"Path to the config file to write; defaults to the user's `config.toml` when omitted.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"keyPath\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mergeStrategy\": {"]
#[doc = "      \"$ref\": \"#/definitions/MergeStrategy\""]
#[doc = "    },"]
#[doc = "    \"value\": true"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigValueWriteParams {
    #[serde(
        rename = "expectedVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expected_version: ::std::option::Option<::std::string::String>,
    #[doc = "Path to the config file to write; defaults to the user's `config.toml` when omitted."]
    #[serde(
        rename = "filePath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyPath")]
    pub key_path: ::std::string::String,
    #[serde(rename = "mergeStrategy")]
    pub merge_strategy: MergeStrategy,
    pub value: ::serde_json::Value,
}
#[doc = "`ConfigWarningNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ConfigWarningNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"summary\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"details\": {"]
#[doc = "      \"description\": \"Optional extra guidance or error details.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Optional path to the config file that triggered the warning.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"range\": {"]
#[doc = "      \"description\": \"Optional range for the error location inside the config file.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TextRange\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"Concise summary of the warning.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigWarningNotification {
    #[doc = "Optional extra guidance or error details."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub details: ::std::option::Option<::std::string::String>,
    #[doc = "Optional path to the config file that triggered the warning."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[doc = "Optional range for the error location inside the config file."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub range: ::std::option::Option<TextRange>,
    #[doc = "Concise summary of the warning."]
    pub summary: ::std::string::String,
}
#[doc = "`ConfigWriteResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ConfigWriteResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"filePath\","]
#[doc = "    \"status\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"filePath\": {"]
#[doc = "      \"description\": \"Canonical path to the config file that was written.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"overriddenMetadata\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/OverriddenMetadata\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/WriteStatus\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigWriteResponse {
    #[doc = "Canonical path to the config file that was written."]
    #[serde(rename = "filePath")]
    pub file_path: AbsolutePathBuf,
    #[serde(
        rename = "overriddenMetadata",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub overridden_metadata: ::std::option::Option<OverriddenMetadata>,
    pub status: WriteStatus,
    pub version: ::std::string::String,
}
#[doc = "`ContentItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"InputTextContentItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"InputTextContentItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"input_text\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"InputImageContentItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"image_url\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"image_url\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"InputImageContentItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"input_image\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"OutputTextContentItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"OutputTextContentItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"output_text\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ContentItem {
    #[doc = "InputTextContentItem"]
    #[serde(rename = "input_text")]
    InputText { text: ::std::string::String },
    #[doc = "InputImageContentItem"]
    #[serde(rename = "input_image")]
    InputImage { image_url: ::std::string::String },
    #[doc = "OutputTextContentItem"]
    #[serde(rename = "output_text")]
    OutputText { text: ::std::string::String },
}
#[doc = "Deprecated: Use `ContextCompaction` item type instead."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ContextCompactedNotification\","]
#[doc = "  \"description\": \"Deprecated: Use `ContextCompaction` item type instead.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ContextCompactedNotification {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`CreditsSnapshot`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"hasCredits\","]
#[doc = "    \"unlimited\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"balance\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"hasCredits\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"unlimited\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreditsSnapshot {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub balance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hasCredits")]
    pub has_credits: bool,
    pub unlimited: bool,
}
#[doc = "`DeprecationNoticeNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"DeprecationNoticeNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"summary\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"details\": {"]
#[doc = "      \"description\": \"Optional extra guidance, such as migration steps or rationale.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"Concise summary of what is deprecated.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DeprecationNoticeNotification {
    #[doc = "Optional extra guidance, such as migration steps or rationale."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub details: ::std::option::Option<::std::string::String>,
    #[doc = "Concise summary of what is deprecated."]
    pub summary: ::std::string::String,
}
#[doc = "`DynamicToolCallOutputContentItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"InputTextDynamicToolCallOutputContentItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"InputTextDynamicToolCallOutputContentItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"inputText\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"InputImageDynamicToolCallOutputContentItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"imageUrl\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"imageUrl\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"InputImageDynamicToolCallOutputContentItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"inputImage\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum DynamicToolCallOutputContentItem {
    #[doc = "InputTextDynamicToolCallOutputContentItem"]
    #[serde(rename = "inputText")]
    InputText { text: ::std::string::String },
    #[doc = "InputImageDynamicToolCallOutputContentItem"]
    #[serde(rename = "inputImage")]
    InputImage {
        #[serde(rename = "imageUrl")]
        image_url: ::std::string::String,
    },
}
#[doc = "`DynamicToolCallStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inProgress\","]
#[doc = "    \"completed\","]
#[doc = "    \"failed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DynamicToolCallStatus {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for DynamicToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("inProgress"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for DynamicToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inProgress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DynamicToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DynamicToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DynamicToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DynamicToolSpec`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"inputSchema\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"deferLoading\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"inputSchema\": true,"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DynamicToolSpec {
    #[serde(
        rename = "deferLoading",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub defer_loading: ::std::option::Option<bool>,
    pub description: ::std::string::String,
    #[serde(rename = "inputSchema")]
    pub input_schema: ::serde_json::Value,
    pub name: ::std::string::String,
}
#[doc = "`ErrorNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ErrorNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"error\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\","]
#[doc = "    \"willRetry\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"$ref\": \"#/definitions/TurnError\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"willRetry\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ErrorNotification {
    pub error: TurnError,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
    #[serde(rename = "willRetry")]
    pub will_retry: bool,
}
#[doc = "`ExperimentalFeature`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"defaultEnabled\","]
#[doc = "    \"enabled\","]
#[doc = "    \"name\","]
#[doc = "    \"stage\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"announcement\": {"]
#[doc = "      \"description\": \"Announcement copy shown to users when the feature is introduced. Null when this feature is not in beta.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"defaultEnabled\": {"]
#[doc = "      \"description\": \"Whether this feature is enabled by default.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short summary describing what the feature does. Null when this feature is not in beta.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"displayName\": {"]
#[doc = "      \"description\": \"User-facing display name shown in the experimental features UI. Null when this feature is not in beta.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"description\": \"Whether this feature is currently enabled in the loaded config.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Stable key used in config.toml and CLI flag toggles.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stage\": {"]
#[doc = "      \"description\": \"Lifecycle stage of this feature flag.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ExperimentalFeatureStage\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExperimentalFeature {
    #[doc = "Announcement copy shown to users when the feature is introduced. Null when this feature is not in beta."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub announcement: ::std::option::Option<::std::string::String>,
    #[doc = "Whether this feature is enabled by default."]
    #[serde(rename = "defaultEnabled")]
    pub default_enabled: bool,
    #[doc = "Short summary describing what the feature does. Null when this feature is not in beta."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "User-facing display name shown in the experimental features UI. Null when this feature is not in beta."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[doc = "Whether this feature is currently enabled in the loaded config."]
    pub enabled: bool,
    #[doc = "Stable key used in config.toml and CLI flag toggles."]
    pub name: ::std::string::String,
    #[doc = "Lifecycle stage of this feature flag."]
    pub stage: ExperimentalFeatureStage,
}
#[doc = "`ExperimentalFeatureEnablementSetParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExperimentalFeatureEnablementSetParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"enablement\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"enablement\": {"]
#[doc = "      \"description\": \"Process-wide runtime feature enablement keyed by canonical feature name.\\n\\nOnly named features are updated. Omitted features are left unchanged. Send an empty map for a no-op.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExperimentalFeatureEnablementSetParams {
    #[doc = "Process-wide runtime feature enablement keyed by canonical feature name.\n\nOnly named features are updated. Omitted features are left unchanged. Send an empty map for a no-op."]
    pub enablement: ::std::collections::HashMap<::std::string::String, bool>,
}
#[doc = "`ExperimentalFeatureEnablementSetResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExperimentalFeatureEnablementSetResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"enablement\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"enablement\": {"]
#[doc = "      \"description\": \"Feature enablement entries updated by this request.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExperimentalFeatureEnablementSetResponse {
    #[doc = "Feature enablement entries updated by this request."]
    pub enablement: ::std::collections::HashMap<::std::string::String, bool>,
}
#[doc = "`ExperimentalFeatureListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExperimentalFeatureListParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"Opaque pagination cursor returned by a previous call.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limit\": {"]
#[doc = "      \"description\": \"Optional page size; defaults to a reasonable server-side value.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExperimentalFeatureListParams {
    #[doc = "Opaque pagination cursor returned by a previous call."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[doc = "Optional page size; defaults to a reasonable server-side value."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<u32>,
}
impl ::std::default::Default for ExperimentalFeatureListParams {
    fn default() -> Self {
        Self {
            cursor: Default::default(),
            limit: Default::default(),
        }
    }
}
#[doc = "`ExperimentalFeatureListResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExperimentalFeatureListResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ExperimentalFeature\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"Opaque cursor to pass to the next call to continue after the last item. If None, there are no more items to return.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExperimentalFeatureListResponse {
    pub data: ::std::vec::Vec<ExperimentalFeature>,
    #[doc = "Opaque cursor to pass to the next call to continue after the last item. If None, there are no more items to return."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
}
#[doc = "`ExperimentalFeatureStage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Feature is available for user testing and feedback.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"beta\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Feature is still being built and not ready for broad use.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"underDevelopment\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Feature is production-ready.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"stable\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Feature is deprecated and should be avoided.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"deprecated\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Feature flag is retained only for backwards compatibility.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"removed\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExperimentalFeatureStage {
    #[doc = "Feature is available for user testing and feedback."]
    #[serde(rename = "beta")]
    Beta,
    #[doc = "Feature is still being built and not ready for broad use."]
    #[serde(rename = "underDevelopment")]
    UnderDevelopment,
    #[doc = "Feature is production-ready."]
    #[serde(rename = "stable")]
    Stable,
    #[doc = "Feature is deprecated and should be avoided."]
    #[serde(rename = "deprecated")]
    Deprecated,
    #[doc = "Feature flag is retained only for backwards compatibility."]
    #[serde(rename = "removed")]
    Removed,
}
impl ::std::fmt::Display for ExperimentalFeatureStage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Beta => f.write_str("beta"),
            Self::UnderDevelopment => f.write_str("underDevelopment"),
            Self::Stable => f.write_str("stable"),
            Self::Deprecated => f.write_str("deprecated"),
            Self::Removed => f.write_str("removed"),
        }
    }
}
impl ::std::str::FromStr for ExperimentalFeatureStage {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "beta" => Ok(Self::Beta),
            "underDevelopment" => Ok(Self::UnderDevelopment),
            "stable" => Ok(Self::Stable),
            "deprecated" => Ok(Self::Deprecated),
            "removed" => Ok(Self::Removed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExperimentalFeatureStage {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExperimentalFeatureStage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExperimentalFeatureStage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ExternalAgentConfigDetectParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExternalAgentConfigDetectParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwds\": {"]
#[doc = "      \"description\": \"Zero or more working directories to include for repo-scoped detection.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"includeHome\": {"]
#[doc = "      \"description\": \"If true, include detection under the user's home (~/.claude, ~/.codex, etc.).\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExternalAgentConfigDetectParams {
    #[doc = "Zero or more working directories to include for repo-scoped detection."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "If true, include detection under the user's home (~/.claude, ~/.codex, etc.)."]
    #[serde(
        rename = "includeHome",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_home: ::std::option::Option<bool>,
}
impl ::std::default::Default for ExternalAgentConfigDetectParams {
    fn default() -> Self {
        Self {
            cwds: Default::default(),
            include_home: Default::default(),
        }
    }
}
#[doc = "`ExternalAgentConfigDetectResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExternalAgentConfigDetectResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"items\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ExternalAgentConfigMigrationItem\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExternalAgentConfigDetectResponse {
    pub items: ::std::vec::Vec<ExternalAgentConfigMigrationItem>,
}
#[doc = "`ExternalAgentConfigImportParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExternalAgentConfigImportParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"migrationItems\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"migrationItems\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ExternalAgentConfigMigrationItem\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExternalAgentConfigImportParams {
    #[serde(rename = "migrationItems")]
    pub migration_items: ::std::vec::Vec<ExternalAgentConfigMigrationItem>,
}
#[doc = "`ExternalAgentConfigImportResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExternalAgentConfigImportResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ExternalAgentConfigImportResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for ExternalAgentConfigImportResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ExternalAgentConfigImportResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: ExternalAgentConfigImportResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for ExternalAgentConfigImportResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`ExternalAgentConfigMigrationItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"itemType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"description\": \"Null or empty means home-scoped migration; non-empty means repo-scoped migration.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"itemType\": {"]
#[doc = "      \"$ref\": \"#/definitions/ExternalAgentConfigMigrationItemType\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExternalAgentConfigMigrationItem {
    #[doc = "Null or empty means home-scoped migration; non-empty means repo-scoped migration."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    pub description: ::std::string::String,
    #[serde(rename = "itemType")]
    pub item_type: ExternalAgentConfigMigrationItemType,
}
#[doc = "`ExternalAgentConfigMigrationItemType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"AGENTS_MD\","]
#[doc = "    \"CONFIG\","]
#[doc = "    \"SKILLS\","]
#[doc = "    \"MCP_SERVER_CONFIG\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExternalAgentConfigMigrationItemType {
    #[serde(rename = "AGENTS_MD")]
    AgentsMd,
    #[serde(rename = "CONFIG")]
    Config,
    #[serde(rename = "SKILLS")]
    Skills,
    #[serde(rename = "MCP_SERVER_CONFIG")]
    McpServerConfig,
}
impl ::std::fmt::Display for ExternalAgentConfigMigrationItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AgentsMd => f.write_str("AGENTS_MD"),
            Self::Config => f.write_str("CONFIG"),
            Self::Skills => f.write_str("SKILLS"),
            Self::McpServerConfig => f.write_str("MCP_SERVER_CONFIG"),
        }
    }
}
impl ::std::str::FromStr for ExternalAgentConfigMigrationItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "AGENTS_MD" => Ok(Self::AgentsMd),
            "CONFIG" => Ok(Self::Config),
            "SKILLS" => Ok(Self::Skills),
            "MCP_SERVER_CONFIG" => Ok(Self::McpServerConfig),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExternalAgentConfigMigrationItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalAgentConfigMigrationItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalAgentConfigMigrationItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FeedbackUploadParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FeedbackUploadParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"classification\","]
#[doc = "    \"includeLogs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"classification\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extraLogFiles\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"includeLogs\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FeedbackUploadParams {
    pub classification: ::std::string::String,
    #[serde(
        rename = "extraLogFiles",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub extra_log_files: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "includeLogs")]
    pub include_logs: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[serde(
        rename = "threadId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub thread_id: ::std::option::Option<::std::string::String>,
}
#[doc = "`FeedbackUploadResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FeedbackUploadResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FeedbackUploadResponse {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`FileChangeOutputDeltaNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FileChangeOutputDeltaNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"delta\","]
#[doc = "    \"itemId\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"delta\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FileChangeOutputDeltaNotification {
    pub delta: ::std::string::String,
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`FileUpdateChange`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"diff\","]
#[doc = "    \"kind\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"diff\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"$ref\": \"#/definitions/PatchChangeKind\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FileUpdateChange {
    pub diff: ::std::string::String,
    pub kind: PatchChangeKind,
    pub path: ::std::string::String,
}
#[doc = "`ForcedLoginMethod`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"chatgpt\","]
#[doc = "    \"api\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ForcedLoginMethod {
    #[serde(rename = "chatgpt")]
    Chatgpt,
    #[serde(rename = "api")]
    Api,
}
impl ::std::fmt::Display for ForcedLoginMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Chatgpt => f.write_str("chatgpt"),
            Self::Api => f.write_str("api"),
        }
    }
}
impl ::std::str::FromStr for ForcedLoginMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatgpt" => Ok(Self::Chatgpt),
            "api" => Ok(Self::Api),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ForcedLoginMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ForcedLoginMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ForcedLoginMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Filesystem watch notification emitted for `fs/watch` subscribers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsChangedNotification\","]
#[doc = "  \"description\": \"Filesystem watch notification emitted for `fs/watch` subscribers.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"changedPaths\","]
#[doc = "    \"watchId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"changedPaths\": {"]
#[doc = "      \"description\": \"File or directory paths associated with this event.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"watchId\": {"]
#[doc = "      \"description\": \"Watch identifier previously provided to `fs/watch`.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsChangedNotification {
    #[doc = "File or directory paths associated with this event."]
    #[serde(rename = "changedPaths")]
    pub changed_paths: ::std::vec::Vec<AbsolutePathBuf>,
    #[doc = "Watch identifier previously provided to `fs/watch`."]
    #[serde(rename = "watchId")]
    pub watch_id: ::std::string::String,
}
#[doc = "Copy a file or directory tree on the host filesystem."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsCopyParams\","]
#[doc = "  \"description\": \"Copy a file or directory tree on the host filesystem.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"destinationPath\","]
#[doc = "    \"sourcePath\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"destinationPath\": {"]
#[doc = "      \"description\": \"Absolute destination path.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"recursive\": {"]
#[doc = "      \"description\": \"Required for directory copies; ignored for file copies.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"sourcePath\": {"]
#[doc = "      \"description\": \"Absolute source path.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsCopyParams {
    #[doc = "Absolute destination path."]
    #[serde(rename = "destinationPath")]
    pub destination_path: AbsolutePathBuf,
    #[doc = "Required for directory copies; ignored for file copies."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub recursive: ::std::option::Option<bool>,
    #[doc = "Absolute source path."]
    #[serde(rename = "sourcePath")]
    pub source_path: AbsolutePathBuf,
}
#[doc = "Successful response for `fs/copy`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsCopyResponse\","]
#[doc = "  \"description\": \"Successful response for `fs/copy`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FsCopyResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for FsCopyResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<FsCopyResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: FsCopyResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for FsCopyResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Create a directory on the host filesystem."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsCreateDirectoryParams\","]
#[doc = "  \"description\": \"Create a directory on the host filesystem.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Absolute directory path to create.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"recursive\": {"]
#[doc = "      \"description\": \"Whether parent directories should also be created. Defaults to `true`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsCreateDirectoryParams {
    #[doc = "Absolute directory path to create."]
    pub path: AbsolutePathBuf,
    #[doc = "Whether parent directories should also be created. Defaults to `true`."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub recursive: ::std::option::Option<bool>,
}
#[doc = "Successful response for `fs/createDirectory`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsCreateDirectoryResponse\","]
#[doc = "  \"description\": \"Successful response for `fs/createDirectory`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FsCreateDirectoryResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for FsCreateDirectoryResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<FsCreateDirectoryResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: FsCreateDirectoryResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for FsCreateDirectoryResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Request metadata for an absolute path."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsGetMetadataParams\","]
#[doc = "  \"description\": \"Request metadata for an absolute path.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Absolute path to inspect.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsGetMetadataParams {
    #[doc = "Absolute path to inspect."]
    pub path: AbsolutePathBuf,
}
#[doc = "Metadata returned by `fs/getMetadata`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsGetMetadataResponse\","]
#[doc = "  \"description\": \"Metadata returned by `fs/getMetadata`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"createdAtMs\","]
#[doc = "    \"isDirectory\","]
#[doc = "    \"isFile\","]
#[doc = "    \"modifiedAtMs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"createdAtMs\": {"]
#[doc = "      \"description\": \"File creation time in Unix milliseconds when available, otherwise `0`.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"isDirectory\": {"]
#[doc = "      \"description\": \"Whether the path currently resolves to a directory.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isFile\": {"]
#[doc = "      \"description\": \"Whether the path currently resolves to a regular file.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"modifiedAtMs\": {"]
#[doc = "      \"description\": \"File modification time in Unix milliseconds when available, otherwise `0`.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsGetMetadataResponse {
    #[doc = "File creation time in Unix milliseconds when available, otherwise `0`."]
    #[serde(rename = "createdAtMs")]
    pub created_at_ms: i64,
    #[doc = "Whether the path currently resolves to a directory."]
    #[serde(rename = "isDirectory")]
    pub is_directory: bool,
    #[doc = "Whether the path currently resolves to a regular file."]
    #[serde(rename = "isFile")]
    pub is_file: bool,
    #[doc = "File modification time in Unix milliseconds when available, otherwise `0`."]
    #[serde(rename = "modifiedAtMs")]
    pub modified_at_ms: i64,
}
#[doc = "A directory entry returned by `fs/readDirectory`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A directory entry returned by `fs/readDirectory`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fileName\","]
#[doc = "    \"isDirectory\","]
#[doc = "    \"isFile\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fileName\": {"]
#[doc = "      \"description\": \"Direct child entry name only, not an absolute or relative path.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"isDirectory\": {"]
#[doc = "      \"description\": \"Whether this entry resolves to a directory.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isFile\": {"]
#[doc = "      \"description\": \"Whether this entry resolves to a regular file.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsReadDirectoryEntry {
    #[doc = "Direct child entry name only, not an absolute or relative path."]
    #[serde(rename = "fileName")]
    pub file_name: ::std::string::String,
    #[doc = "Whether this entry resolves to a directory."]
    #[serde(rename = "isDirectory")]
    pub is_directory: bool,
    #[doc = "Whether this entry resolves to a regular file."]
    #[serde(rename = "isFile")]
    pub is_file: bool,
}
#[doc = "List direct child names for a directory."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsReadDirectoryParams\","]
#[doc = "  \"description\": \"List direct child names for a directory.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Absolute directory path to read.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsReadDirectoryParams {
    #[doc = "Absolute directory path to read."]
    pub path: AbsolutePathBuf,
}
#[doc = "Directory entries returned by `fs/readDirectory`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsReadDirectoryResponse\","]
#[doc = "  \"description\": \"Directory entries returned by `fs/readDirectory`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"entries\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"entries\": {"]
#[doc = "      \"description\": \"Direct child entries in the requested directory.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/FsReadDirectoryEntry\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsReadDirectoryResponse {
    #[doc = "Direct child entries in the requested directory."]
    pub entries: ::std::vec::Vec<FsReadDirectoryEntry>,
}
#[doc = "Read a file from the host filesystem."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsReadFileParams\","]
#[doc = "  \"description\": \"Read a file from the host filesystem.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Absolute path to read.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsReadFileParams {
    #[doc = "Absolute path to read."]
    pub path: AbsolutePathBuf,
}
#[doc = "Base64-encoded file contents returned by `fs/readFile`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsReadFileResponse\","]
#[doc = "  \"description\": \"Base64-encoded file contents returned by `fs/readFile`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dataBase64\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dataBase64\": {"]
#[doc = "      \"description\": \"File contents encoded as base64.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsReadFileResponse {
    #[doc = "File contents encoded as base64."]
    #[serde(rename = "dataBase64")]
    pub data_base64: ::std::string::String,
}
#[doc = "Remove a file or directory tree from the host filesystem."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsRemoveParams\","]
#[doc = "  \"description\": \"Remove a file or directory tree from the host filesystem.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"force\": {"]
#[doc = "      \"description\": \"Whether missing paths should be ignored. Defaults to `true`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Absolute path to remove.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"recursive\": {"]
#[doc = "      \"description\": \"Whether directory removal should recurse. Defaults to `true`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsRemoveParams {
    #[doc = "Whether missing paths should be ignored. Defaults to `true`."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub force: ::std::option::Option<bool>,
    #[doc = "Absolute path to remove."]
    pub path: AbsolutePathBuf,
    #[doc = "Whether directory removal should recurse. Defaults to `true`."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub recursive: ::std::option::Option<bool>,
}
#[doc = "Successful response for `fs/remove`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsRemoveResponse\","]
#[doc = "  \"description\": \"Successful response for `fs/remove`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FsRemoveResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for FsRemoveResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<FsRemoveResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: FsRemoveResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for FsRemoveResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Stop filesystem watch notifications for a prior `fs/watch`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsUnwatchParams\","]
#[doc = "  \"description\": \"Stop filesystem watch notifications for a prior `fs/watch`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"watchId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"watchId\": {"]
#[doc = "      \"description\": \"Watch identifier previously provided to `fs/watch`.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsUnwatchParams {
    #[doc = "Watch identifier previously provided to `fs/watch`."]
    #[serde(rename = "watchId")]
    pub watch_id: ::std::string::String,
}
#[doc = "Successful response for `fs/unwatch`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsUnwatchResponse\","]
#[doc = "  \"description\": \"Successful response for `fs/unwatch`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FsUnwatchResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for FsUnwatchResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<FsUnwatchResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: FsUnwatchResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for FsUnwatchResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Start filesystem watch notifications for an absolute path."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsWatchParams\","]
#[doc = "  \"description\": \"Start filesystem watch notifications for an absolute path.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\","]
#[doc = "    \"watchId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Absolute file or directory path to watch.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"watchId\": {"]
#[doc = "      \"description\": \"Connection-scoped watch identifier used for `fs/unwatch` and `fs/changed`.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsWatchParams {
    #[doc = "Absolute file or directory path to watch."]
    pub path: AbsolutePathBuf,
    #[doc = "Connection-scoped watch identifier used for `fs/unwatch` and `fs/changed`."]
    #[serde(rename = "watchId")]
    pub watch_id: ::std::string::String,
}
#[doc = "Successful response for `fs/watch`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsWatchResponse\","]
#[doc = "  \"description\": \"Successful response for `fs/watch`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Canonicalized path associated with the watch.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsWatchResponse {
    #[doc = "Canonicalized path associated with the watch."]
    pub path: AbsolutePathBuf,
}
#[doc = "Write a file on the host filesystem."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsWriteFileParams\","]
#[doc = "  \"description\": \"Write a file on the host filesystem.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dataBase64\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dataBase64\": {"]
#[doc = "      \"description\": \"File contents encoded as base64.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Absolute path to write.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FsWriteFileParams {
    #[doc = "File contents encoded as base64."]
    #[serde(rename = "dataBase64")]
    pub data_base64: ::std::string::String,
    #[doc = "Absolute path to write."]
    pub path: AbsolutePathBuf,
}
#[doc = "Successful response for `fs/writeFile`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FsWriteFileResponse\","]
#[doc = "  \"description\": \"Successful response for `fs/writeFile`.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FsWriteFileResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for FsWriteFileResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<FsWriteFileResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: FsWriteFileResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for FsWriteFileResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`FunctionCallOutputBody`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/FunctionCallOutputContentItem\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum FunctionCallOutputBody {
    String(::std::string::String),
    Array(::std::vec::Vec<FunctionCallOutputContentItem>),
}
impl ::std::convert::From<::std::vec::Vec<FunctionCallOutputContentItem>>
    for FunctionCallOutputBody
{
    fn from(value: ::std::vec::Vec<FunctionCallOutputContentItem>) -> Self {
        Self::Array(value)
    }
}
#[doc = "Responses API compatible content items that can be returned by a tool call. This is a subset of ContentItem with the types we support as function call outputs."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Responses API compatible content items that can be returned by a tool call. This is a subset of ContentItem with the types we support as function call outputs.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"InputTextFunctionCallOutputContentItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"InputTextFunctionCallOutputContentItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"input_text\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"InputImageFunctionCallOutputContentItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"image_url\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"detail\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ImageDetail\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"image_url\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"InputImageFunctionCallOutputContentItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"input_image\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum FunctionCallOutputContentItem {
    #[doc = "InputTextFunctionCallOutputContentItem"]
    #[serde(rename = "input_text")]
    InputText { text: ::std::string::String },
    #[doc = "InputImageFunctionCallOutputContentItem"]
    #[serde(rename = "input_image")]
    InputImage {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        detail: ::std::option::Option<ImageDetail>,
        image_url: ::std::string::String,
    },
}
#[doc = "`FuzzyFileSearchMatchType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"file\","]
#[doc = "    \"directory\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FuzzyFileSearchMatchType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "directory")]
    Directory,
}
impl ::std::fmt::Display for FuzzyFileSearchMatchType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => f.write_str("file"),
            Self::Directory => f.write_str("directory"),
        }
    }
}
impl ::std::str::FromStr for FuzzyFileSearchMatchType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            "directory" => Ok(Self::Directory),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FuzzyFileSearchMatchType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FuzzyFileSearchMatchType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FuzzyFileSearchMatchType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FuzzyFileSearchParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FuzzyFileSearchParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"query\","]
#[doc = "    \"roots\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cancellationToken\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"query\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"roots\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FuzzyFileSearchParams {
    #[serde(
        rename = "cancellationToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cancellation_token: ::std::option::Option<::std::string::String>,
    pub query: ::std::string::String,
    pub roots: ::std::vec::Vec<::std::string::String>,
}
#[doc = "Superset of [`codex_file_search::FileMatch`]"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Superset of [`codex_file_search::FileMatch`]\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"file_name\","]
#[doc = "    \"match_type\","]
#[doc = "    \"path\","]
#[doc = "    \"root\","]
#[doc = "    \"score\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"file_name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"indices\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\","]
#[doc = "        \"format\": \"uint32\","]
#[doc = "        \"minimum\": 0.0"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"match_type\": {"]
#[doc = "      \"$ref\": \"#/definitions/FuzzyFileSearchMatchType\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"root\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"score\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FuzzyFileSearchResult {
    pub file_name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub indices: ::std::option::Option<::std::vec::Vec<u32>>,
    pub match_type: FuzzyFileSearchMatchType,
    pub path: ::std::string::String,
    pub root: ::std::string::String,
    pub score: u32,
}
#[doc = "`FuzzyFileSearchSessionCompletedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FuzzyFileSearchSessionCompletedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"sessionId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"sessionId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FuzzyFileSearchSessionCompletedNotification {
    #[serde(rename = "sessionId")]
    pub session_id: ::std::string::String,
}
#[doc = "`FuzzyFileSearchSessionUpdatedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FuzzyFileSearchSessionUpdatedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"files\","]
#[doc = "    \"query\","]
#[doc = "    \"sessionId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"files\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/FuzzyFileSearchResult\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"query\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"sessionId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FuzzyFileSearchSessionUpdatedNotification {
    pub files: ::std::vec::Vec<FuzzyFileSearchResult>,
    pub query: ::std::string::String,
    #[serde(rename = "sessionId")]
    pub session_id: ::std::string::String,
}
#[doc = "`GetAccountParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"GetAccountParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"refreshToken\": {"]
#[doc = "      \"description\": \"When `true`, requests a proactive token refresh before returning.\\n\\nIn managed auth mode this triggers the normal refresh-token flow. In external auth mode this flag is ignored. Clients should refresh tokens themselves and call `account/login/start` with `chatgptAuthTokens`.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetAccountParams {
    #[doc = "When `true`, requests a proactive token refresh before returning.\n\nIn managed auth mode this triggers the normal refresh-token flow. In external auth mode this flag is ignored. Clients should refresh tokens themselves and call `account/login/start` with `chatgptAuthTokens`."]
    #[serde(rename = "refreshToken", default)]
    pub refresh_token: bool,
}
impl ::std::default::Default for GetAccountParams {
    fn default() -> Self {
        Self {
            refresh_token: Default::default(),
        }
    }
}
#[doc = "`GetAccountRateLimitsResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"GetAccountRateLimitsResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"rateLimits\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"rateLimits\": {"]
#[doc = "      \"description\": \"Backward-compatible single-bucket view; mirrors the historical payload.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/RateLimitSnapshot\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"rateLimitsByLimitId\": {"]
#[doc = "      \"description\": \"Multi-bucket view keyed by metered `limit_id` (for example, `codex`).\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/RateLimitSnapshot\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetAccountRateLimitsResponse {
    #[doc = "Backward-compatible single-bucket view; mirrors the historical payload."]
    #[serde(rename = "rateLimits")]
    pub rate_limits: RateLimitSnapshot,
    #[doc = "Multi-bucket view keyed by metered `limit_id` (for example, `codex`)."]
    #[serde(
        rename = "rateLimitsByLimitId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rate_limits_by_limit_id: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, RateLimitSnapshot>,
    >,
}
#[doc = "`GetAccountResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"GetAccountResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"requiresOpenaiAuth\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Account\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requiresOpenaiAuth\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetAccountResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub account: ::std::option::Option<Account>,
    #[serde(rename = "requiresOpenaiAuth")]
    pub requires_openai_auth: bool,
}
#[doc = "Details of a ghost commit created from a repository state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Details of a ghost commit created from a repository state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"preexisting_untracked_dirs\","]
#[doc = "    \"preexisting_untracked_files\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"preexisting_untracked_dirs\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"preexisting_untracked_files\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GhostCommit {
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent: ::std::option::Option<::std::string::String>,
    pub preexisting_untracked_dirs: ::std::vec::Vec<::std::string::String>,
    pub preexisting_untracked_files: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`GitInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"branch\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"originUrl\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sha\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GitInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "originUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub origin_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sha: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GitInfo {
    fn default() -> Self {
        Self {
            branch: Default::default(),
            origin_url: Default::default(),
            sha: Default::default(),
        }
    }
}
#[doc = "[UNSTABLE] Temporary guardian approval review payload used by `item/autoApprovalReview/*` notifications. This shape is expected to change soon."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[UNSTABLE] Temporary guardian approval review payload used by `item/autoApprovalReview/*` notifications. This shape is expected to change soon.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"rationale\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"riskLevel\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/GuardianRiskLevel\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/GuardianApprovalReviewStatus\""]
#[doc = "    },"]
#[doc = "    \"userAuthorization\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/GuardianUserAuthorization\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GuardianApprovalReview {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rationale: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "riskLevel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub risk_level: ::std::option::Option<GuardianRiskLevel>,
    pub status: GuardianApprovalReviewStatus,
    #[serde(
        rename = "userAuthorization",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_authorization: ::std::option::Option<GuardianUserAuthorization>,
}
#[doc = "`GuardianApprovalReviewAction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"CommandGuardianApprovalReviewAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"command\","]
#[doc = "        \"cwd\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"command\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"cwd\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/definitions/GuardianCommandSource\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"CommandGuardianApprovalReviewActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"command\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ExecveGuardianApprovalReviewAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"argv\","]
#[doc = "        \"cwd\","]
#[doc = "        \"program\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"argv\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"cwd\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"program\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/definitions/GuardianCommandSource\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ExecveGuardianApprovalReviewActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"execve\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ApplyPatchGuardianApprovalReviewAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"cwd\","]
#[doc = "        \"files\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"cwd\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"files\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ApplyPatchGuardianApprovalReviewActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"applyPatch\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"NetworkAccessGuardianApprovalReviewAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"host\","]
#[doc = "        \"port\","]
#[doc = "        \"protocol\","]
#[doc = "        \"target\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"host\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"port\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint16\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"protocol\": {"]
#[doc = "          \"$ref\": \"#/definitions/NetworkApprovalProtocol\""]
#[doc = "        },"]
#[doc = "        \"target\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"NetworkAccessGuardianApprovalReviewActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"networkAccess\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"McpToolCallGuardianApprovalReviewAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"server\","]
#[doc = "        \"toolName\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"connectorId\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"connectorName\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"server\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"toolName\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"toolTitle\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"McpToolCallGuardianApprovalReviewActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mcpToolCall\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum GuardianApprovalReviewAction {
    #[doc = "CommandGuardianApprovalReviewAction"]
    #[serde(rename = "command")]
    Command {
        command: ::std::string::String,
        cwd: ::std::string::String,
        source: GuardianCommandSource,
    },
    #[doc = "ExecveGuardianApprovalReviewAction"]
    #[serde(rename = "execve")]
    Execve {
        argv: ::std::vec::Vec<::std::string::String>,
        cwd: ::std::string::String,
        program: ::std::string::String,
        source: GuardianCommandSource,
    },
    #[doc = "ApplyPatchGuardianApprovalReviewAction"]
    #[serde(rename = "applyPatch")]
    ApplyPatch {
        cwd: ::std::string::String,
        files: ::std::vec::Vec<::std::string::String>,
    },
    #[doc = "NetworkAccessGuardianApprovalReviewAction"]
    #[serde(rename = "networkAccess")]
    NetworkAccess {
        host: ::std::string::String,
        port: u16,
        protocol: NetworkApprovalProtocol,
        target: ::std::string::String,
    },
    #[doc = "McpToolCallGuardianApprovalReviewAction"]
    #[serde(rename = "mcpToolCall")]
    McpToolCall {
        #[serde(
            rename = "connectorId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        connector_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "connectorName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        connector_name: ::std::option::Option<::std::string::String>,
        server: ::std::string::String,
        #[serde(rename = "toolName")]
        tool_name: ::std::string::String,
        #[serde(
            rename = "toolTitle",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        tool_title: ::std::option::Option<::std::string::String>,
    },
}
#[doc = "[UNSTABLE] Lifecycle state for a guardian approval review."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[UNSTABLE] Lifecycle state for a guardian approval review.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inProgress\","]
#[doc = "    \"approved\","]
#[doc = "    \"denied\","]
#[doc = "    \"timedOut\","]
#[doc = "    \"aborted\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GuardianApprovalReviewStatus {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "timedOut")]
    TimedOut,
    #[serde(rename = "aborted")]
    Aborted,
}
impl ::std::fmt::Display for GuardianApprovalReviewStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("inProgress"),
            Self::Approved => f.write_str("approved"),
            Self::Denied => f.write_str("denied"),
            Self::TimedOut => f.write_str("timedOut"),
            Self::Aborted => f.write_str("aborted"),
        }
    }
}
impl ::std::str::FromStr for GuardianApprovalReviewStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inProgress" => Ok(Self::InProgress),
            "approved" => Ok(Self::Approved),
            "denied" => Ok(Self::Denied),
            "timedOut" => Ok(Self::TimedOut),
            "aborted" => Ok(Self::Aborted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GuardianApprovalReviewStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GuardianApprovalReviewStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GuardianApprovalReviewStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GuardianCommandSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"shell\","]
#[doc = "    \"unifiedExec\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GuardianCommandSource {
    #[serde(rename = "shell")]
    Shell,
    #[serde(rename = "unifiedExec")]
    UnifiedExec,
}
impl ::std::fmt::Display for GuardianCommandSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shell => f.write_str("shell"),
            Self::UnifiedExec => f.write_str("unifiedExec"),
        }
    }
}
impl ::std::str::FromStr for GuardianCommandSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell" => Ok(Self::Shell),
            "unifiedExec" => Ok(Self::UnifiedExec),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GuardianCommandSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GuardianCommandSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GuardianCommandSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "[UNSTABLE] Risk level assigned by guardian approval review."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[UNSTABLE] Risk level assigned by guardian approval review.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"low\","]
#[doc = "    \"medium\","]
#[doc = "    \"high\","]
#[doc = "    \"critical\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GuardianRiskLevel {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}
impl ::std::fmt::Display for GuardianRiskLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Critical => f.write_str("critical"),
        }
    }
}
impl ::std::str::FromStr for GuardianRiskLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "critical" => Ok(Self::Critical),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GuardianRiskLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GuardianRiskLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GuardianRiskLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "[UNSTABLE] Authorization level assigned by guardian approval review."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[UNSTABLE] Authorization level assigned by guardian approval review.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"unknown\","]
#[doc = "    \"low\","]
#[doc = "    \"medium\","]
#[doc = "    \"high\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GuardianUserAuthorization {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for GuardianUserAuthorization {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("unknown"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for GuardianUserAuthorization {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GuardianUserAuthorization {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GuardianUserAuthorization {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GuardianUserAuthorization {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HookCompletedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"HookCompletedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"run\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"run\": {"]
#[doc = "      \"$ref\": \"#/definitions/HookRunSummary\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HookCompletedNotification {
    pub run: HookRunSummary,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(
        rename = "turnId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub turn_id: ::std::option::Option<::std::string::String>,
}
#[doc = "`HookEventName`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"preToolUse\","]
#[doc = "    \"postToolUse\","]
#[doc = "    \"sessionStart\","]
#[doc = "    \"userPromptSubmit\","]
#[doc = "    \"stop\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HookEventName {
    #[serde(rename = "preToolUse")]
    PreToolUse,
    #[serde(rename = "postToolUse")]
    PostToolUse,
    #[serde(rename = "sessionStart")]
    SessionStart,
    #[serde(rename = "userPromptSubmit")]
    UserPromptSubmit,
    #[serde(rename = "stop")]
    Stop,
}
impl ::std::fmt::Display for HookEventName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PreToolUse => f.write_str("preToolUse"),
            Self::PostToolUse => f.write_str("postToolUse"),
            Self::SessionStart => f.write_str("sessionStart"),
            Self::UserPromptSubmit => f.write_str("userPromptSubmit"),
            Self::Stop => f.write_str("stop"),
        }
    }
}
impl ::std::str::FromStr for HookEventName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "preToolUse" => Ok(Self::PreToolUse),
            "postToolUse" => Ok(Self::PostToolUse),
            "sessionStart" => Ok(Self::SessionStart),
            "userPromptSubmit" => Ok(Self::UserPromptSubmit),
            "stop" => Ok(Self::Stop),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HookEventName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HookEventName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HookEventName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HookExecutionMode`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"sync\","]
#[doc = "    \"async\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HookExecutionMode {
    #[serde(rename = "sync")]
    Sync,
    #[serde(rename = "async")]
    Async,
}
impl ::std::fmt::Display for HookExecutionMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sync => f.write_str("sync"),
            Self::Async => f.write_str("async"),
        }
    }
}
impl ::std::str::FromStr for HookExecutionMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sync" => Ok(Self::Sync),
            "async" => Ok(Self::Async),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HookExecutionMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HookExecutionMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HookExecutionMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HookHandlerType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"command\","]
#[doc = "    \"prompt\","]
#[doc = "    \"agent\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HookHandlerType {
    #[serde(rename = "command")]
    Command,
    #[serde(rename = "prompt")]
    Prompt,
    #[serde(rename = "agent")]
    Agent,
}
impl ::std::fmt::Display for HookHandlerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Command => f.write_str("command"),
            Self::Prompt => f.write_str("prompt"),
            Self::Agent => f.write_str("agent"),
        }
    }
}
impl ::std::str::FromStr for HookHandlerType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "command" => Ok(Self::Command),
            "prompt" => Ok(Self::Prompt),
            "agent" => Ok(Self::Agent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HookHandlerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HookHandlerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HookHandlerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HookOutputEntry`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"$ref\": \"#/definitions/HookOutputEntryKind\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HookOutputEntry {
    pub kind: HookOutputEntryKind,
    pub text: ::std::string::String,
}
#[doc = "`HookOutputEntryKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"warning\","]
#[doc = "    \"stop\","]
#[doc = "    \"feedback\","]
#[doc = "    \"context\","]
#[doc = "    \"error\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HookOutputEntryKind {
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "feedback")]
    Feedback,
    #[serde(rename = "context")]
    Context,
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for HookOutputEntryKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Warning => f.write_str("warning"),
            Self::Stop => f.write_str("stop"),
            Self::Feedback => f.write_str("feedback"),
            Self::Context => f.write_str("context"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for HookOutputEntryKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "warning" => Ok(Self::Warning),
            "stop" => Ok(Self::Stop),
            "feedback" => Ok(Self::Feedback),
            "context" => Ok(Self::Context),
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HookOutputEntryKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HookOutputEntryKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HookOutputEntryKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HookPromptFragment`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"hookRunId\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"hookRunId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HookPromptFragment {
    #[serde(rename = "hookRunId")]
    pub hook_run_id: ::std::string::String,
    pub text: ::std::string::String,
}
#[doc = "`HookRunStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"running\","]
#[doc = "    \"completed\","]
#[doc = "    \"failed\","]
#[doc = "    \"blocked\","]
#[doc = "    \"stopped\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HookRunStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "stopped")]
    Stopped,
}
impl ::std::fmt::Display for HookRunStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Running => f.write_str("running"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
            Self::Blocked => f.write_str("blocked"),
            Self::Stopped => f.write_str("stopped"),
        }
    }
}
impl ::std::str::FromStr for HookRunStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "blocked" => Ok(Self::Blocked),
            "stopped" => Ok(Self::Stopped),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HookRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HookRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HookRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HookRunSummary`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"displayOrder\","]
#[doc = "    \"entries\","]
#[doc = "    \"eventName\","]
#[doc = "    \"executionMode\","]
#[doc = "    \"handlerType\","]
#[doc = "    \"id\","]
#[doc = "    \"scope\","]
#[doc = "    \"sourcePath\","]
#[doc = "    \"startedAt\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"completedAt\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"displayOrder\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"durationMs\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"entries\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/HookOutputEntry\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"eventName\": {"]
#[doc = "      \"$ref\": \"#/definitions/HookEventName\""]
#[doc = "    },"]
#[doc = "    \"executionMode\": {"]
#[doc = "      \"$ref\": \"#/definitions/HookExecutionMode\""]
#[doc = "    },"]
#[doc = "    \"handlerType\": {"]
#[doc = "      \"$ref\": \"#/definitions/HookHandlerType\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"$ref\": \"#/definitions/HookScope\""]
#[doc = "    },"]
#[doc = "    \"sourcePath\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"startedAt\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/HookRunStatus\""]
#[doc = "    },"]
#[doc = "    \"statusMessage\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HookRunSummary {
    #[serde(
        rename = "completedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub completed_at: ::std::option::Option<i64>,
    #[serde(rename = "displayOrder")]
    pub display_order: i64,
    #[serde(
        rename = "durationMs",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub duration_ms: ::std::option::Option<i64>,
    pub entries: ::std::vec::Vec<HookOutputEntry>,
    #[serde(rename = "eventName")]
    pub event_name: HookEventName,
    #[serde(rename = "executionMode")]
    pub execution_mode: HookExecutionMode,
    #[serde(rename = "handlerType")]
    pub handler_type: HookHandlerType,
    pub id: ::std::string::String,
    pub scope: HookScope,
    #[serde(rename = "sourcePath")]
    pub source_path: ::std::string::String,
    #[serde(rename = "startedAt")]
    pub started_at: i64,
    pub status: HookRunStatus,
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_message: ::std::option::Option<::std::string::String>,
}
#[doc = "`HookScope`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"thread\","]
#[doc = "    \"turn\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HookScope {
    #[serde(rename = "thread")]
    Thread,
    #[serde(rename = "turn")]
    Turn,
}
impl ::std::fmt::Display for HookScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Thread => f.write_str("thread"),
            Self::Turn => f.write_str("turn"),
        }
    }
}
impl ::std::str::FromStr for HookScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread" => Ok(Self::Thread),
            "turn" => Ok(Self::Turn),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HookScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HookScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HookScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HookStartedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"HookStartedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"run\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"run\": {"]
#[doc = "      \"$ref\": \"#/definitions/HookRunSummary\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HookStartedNotification {
    pub run: HookRunSummary,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(
        rename = "turnId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub turn_id: ::std::option::Option<::std::string::String>,
}
#[doc = "`ImageDetail`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"auto\","]
#[doc = "    \"low\","]
#[doc = "    \"high\","]
#[doc = "    \"original\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "original")]
    Original,
}
impl ::std::fmt::Display for ImageDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
            Self::Original => f.write_str("original"),
        }
    }
}
impl ::std::str::FromStr for ImageDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            "original" => Ok(Self::Original),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Client-declared capabilities negotiated during initialize."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Client-declared capabilities negotiated during initialize.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"experimentalApi\": {"]
#[doc = "      \"description\": \"Opt into receiving experimental API methods and fields.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optOutNotificationMethods\": {"]
#[doc = "      \"description\": \"Exact notification method names that should be suppressed for this connection (for example `thread/started`).\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializeCapabilities {
    #[doc = "Opt into receiving experimental API methods and fields."]
    #[serde(rename = "experimentalApi", default)]
    pub experimental_api: bool,
    #[doc = "Exact notification method names that should be suppressed for this connection (for example `thread/started`)."]
    #[serde(
        rename = "optOutNotificationMethods",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub opt_out_notification_methods: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ::std::default::Default for InitializeCapabilities {
    fn default() -> Self {
        Self {
            experimental_api: Default::default(),
            opt_out_notification_methods: Default::default(),
        }
    }
}
#[doc = "`InitializeParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"InitializeParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"clientInfo\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/InitializeCapabilities\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"clientInfo\": {"]
#[doc = "      \"$ref\": \"#/definitions/ClientInfo\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializeParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub capabilities: ::std::option::Option<InitializeCapabilities>,
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
}
#[doc = "Canonical user-input modality tags advertised by a model."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Canonical user-input modality tags advertised by a model.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Plain text turns and tool payloads.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"text\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Image attachments included in user turns.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"image\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputModality {
    #[doc = "Plain text turns and tool payloads."]
    #[serde(rename = "text")]
    Text,
    #[doc = "Image attachments included in user turns."]
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display for InputModality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr for InputModality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputModality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InputModality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputModality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ItemCompletedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ItemCompletedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"item\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"item\": {"]
#[doc = "      \"$ref\": \"#/definitions/ThreadItem\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ItemCompletedNotification {
    pub item: ThreadItem,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "[UNSTABLE] Temporary notification payload for guardian automatic approval review. This shape is expected to change soon."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ItemGuardianApprovalReviewCompletedNotification\","]
#[doc = "  \"description\": \"[UNSTABLE] Temporary notification payload for guardian automatic approval review. This shape is expected to change soon.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"action\","]
#[doc = "    \"decisionSource\","]
#[doc = "    \"review\","]
#[doc = "    \"reviewId\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"action\": {"]
#[doc = "      \"$ref\": \"#/definitions/GuardianApprovalReviewAction\""]
#[doc = "    },"]
#[doc = "    \"decisionSource\": {"]
#[doc = "      \"$ref\": \"#/definitions/AutoReviewDecisionSource\""]
#[doc = "    },"]
#[doc = "    \"review\": {"]
#[doc = "      \"$ref\": \"#/definitions/GuardianApprovalReview\""]
#[doc = "    },"]
#[doc = "    \"reviewId\": {"]
#[doc = "      \"description\": \"Stable identifier for this review.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"targetItemId\": {"]
#[doc = "      \"description\": \"Identifier for the reviewed item or tool call when one exists.\\n\\nIn most cases, one review maps to one target item. The exceptions are - execve reviews, where a single command may contain multiple execve calls to review (only possible when using the shell_zsh_fork feature) - network policy reviews, where there is no target item\\n\\nA network call is triggered by a CommandExecution item, so having a target_item_id set to the CommandExecution item would be misleading because the review is about the network call, not the command execution. Therefore, target_item_id is set to None for network policy reviews.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ItemGuardianApprovalReviewCompletedNotification {
    pub action: GuardianApprovalReviewAction,
    #[serde(rename = "decisionSource")]
    pub decision_source: AutoReviewDecisionSource,
    pub review: GuardianApprovalReview,
    #[doc = "Stable identifier for this review."]
    #[serde(rename = "reviewId")]
    pub review_id: ::std::string::String,
    #[doc = "Identifier for the reviewed item or tool call when one exists.\n\nIn most cases, one review maps to one target item. The exceptions are - execve reviews, where a single command may contain multiple execve calls to review (only possible when using the shell_zsh_fork feature) - network policy reviews, where there is no target item\n\nA network call is triggered by a CommandExecution item, so having a target_item_id set to the CommandExecution item would be misleading because the review is about the network call, not the command execution. Therefore, target_item_id is set to None for network policy reviews."]
    #[serde(
        rename = "targetItemId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "[UNSTABLE] Temporary notification payload for guardian automatic approval review. This shape is expected to change soon."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ItemGuardianApprovalReviewStartedNotification\","]
#[doc = "  \"description\": \"[UNSTABLE] Temporary notification payload for guardian automatic approval review. This shape is expected to change soon.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"action\","]
#[doc = "    \"review\","]
#[doc = "    \"reviewId\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"action\": {"]
#[doc = "      \"$ref\": \"#/definitions/GuardianApprovalReviewAction\""]
#[doc = "    },"]
#[doc = "    \"review\": {"]
#[doc = "      \"$ref\": \"#/definitions/GuardianApprovalReview\""]
#[doc = "    },"]
#[doc = "    \"reviewId\": {"]
#[doc = "      \"description\": \"Stable identifier for this review.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"targetItemId\": {"]
#[doc = "      \"description\": \"Identifier for the reviewed item or tool call when one exists.\\n\\nIn most cases, one review maps to one target item. The exceptions are - execve reviews, where a single command may contain multiple execve calls to review (only possible when using the shell_zsh_fork feature) - network policy reviews, where there is no target item\\n\\nA network call is triggered by a CommandExecution item, so having a target_item_id set to the CommandExecution item would be misleading because the review is about the network call, not the command execution. Therefore, target_item_id is set to None for network policy reviews.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ItemGuardianApprovalReviewStartedNotification {
    pub action: GuardianApprovalReviewAction,
    pub review: GuardianApprovalReview,
    #[doc = "Stable identifier for this review."]
    #[serde(rename = "reviewId")]
    pub review_id: ::std::string::String,
    #[doc = "Identifier for the reviewed item or tool call when one exists.\n\nIn most cases, one review maps to one target item. The exceptions are - execve reviews, where a single command may contain multiple execve calls to review (only possible when using the shell_zsh_fork feature) - network policy reviews, where there is no target item\n\nA network call is triggered by a CommandExecution item, so having a target_item_id set to the CommandExecution item would be misleading because the review is about the network call, not the command execution. Therefore, target_item_id is set to None for network policy reviews."]
    #[serde(
        rename = "targetItemId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`ItemStartedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ItemStartedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"item\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"item\": {"]
#[doc = "      \"$ref\": \"#/definitions/ThreadItem\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ItemStartedNotification {
    pub item: ThreadItem,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`ListMcpServerStatusParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ListMcpServerStatusParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"Opaque pagination cursor returned by a previous call.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"detail\": {"]
#[doc = "      \"description\": \"Controls how much MCP inventory data to fetch for each server. Defaults to `Full` when omitted.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/McpServerStatusDetail\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limit\": {"]
#[doc = "      \"description\": \"Optional page size; defaults to a server-defined value.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListMcpServerStatusParams {
    #[doc = "Opaque pagination cursor returned by a previous call."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[doc = "Controls how much MCP inventory data to fetch for each server. Defaults to `Full` when omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<McpServerStatusDetail>,
    #[doc = "Optional page size; defaults to a server-defined value."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<u32>,
}
impl ::std::default::Default for ListMcpServerStatusParams {
    fn default() -> Self {
        Self {
            cursor: Default::default(),
            detail: Default::default(),
            limit: Default::default(),
        }
    }
}
#[doc = "`ListMcpServerStatusResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ListMcpServerStatusResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/McpServerStatus\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"Opaque cursor to pass to the next call to continue after the last item. If None, there are no more items to return.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListMcpServerStatusResponse {
    pub data: ::std::vec::Vec<McpServerStatus>,
    #[doc = "Opaque cursor to pass to the next call to continue after the last item. If None, there are no more items to return."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
}
#[doc = "`LocalShellAction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"ExecLocalShellAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"command\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"command\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"env\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"object\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"timeout_ms\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"uint64\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ExecLocalShellActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"exec\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"user\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"working_directory\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum LocalShellAction {
    #[doc = "ExecLocalShellAction"]
    #[serde(rename = "exec")]
    Exec {
        command: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        env: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        timeout_ms: ::std::option::Option<u64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        user: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        working_directory: ::std::option::Option<::std::string::String>,
    },
}
#[doc = "`LocalShellStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"completed\","]
#[doc = "    \"in_progress\","]
#[doc = "    \"incomplete\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LocalShellStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for LocalShellStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::InProgress => f.write_str("in_progress"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for LocalShellStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalShellStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LocalShellStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalShellStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LoginAccountParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"LoginAccountParams\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"ApiKeyv2::LoginAccountParams\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"apiKey\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"apiKey\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ApiKeyv2::LoginAccountParamsType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"apiKey\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Chatgptv2::LoginAccountParams\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"Chatgptv2::LoginAccountParamsType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"chatgpt\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ChatgptDeviceCodev2::LoginAccountParams\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ChatgptDeviceCodev2::LoginAccountParamsType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"chatgptDeviceCode\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ChatgptAuthTokensv2::LoginAccountParams\","]
#[doc = "      \"description\": \"[UNSTABLE] FOR OPENAI INTERNAL USE ONLY - DO NOT USE. The access token must contain the same scopes that Codex-managed ChatGPT auth tokens have.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"accessToken\","]
#[doc = "        \"chatgptAccountId\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"accessToken\": {"]
#[doc = "          \"description\": \"Access token (JWT) supplied by the client. This token is used for backend API requests and email extraction.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"chatgptAccountId\": {"]
#[doc = "          \"description\": \"Workspace/account identifier supplied by the client.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"chatgptPlanType\": {"]
#[doc = "          \"description\": \"Optional plan type supplied by the client.\\n\\nWhen `null`, Codex attempts to derive the plan type from access-token claims. If unavailable, the plan defaults to `unknown`.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ChatgptAuthTokensv2::LoginAccountParamsType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"chatgptAuthTokens\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum LoginAccountParams {
    #[doc = "ApiKeyv2::LoginAccountParams"]
    #[serde(rename = "apiKey")]
    ApiKey {
        #[serde(rename = "apiKey")]
        api_key: ::std::string::String,
    },
    #[serde(rename = "chatgpt")]
    Chatgpt,
    #[serde(rename = "chatgptDeviceCode")]
    ChatgptDeviceCode,
    #[doc = "ChatgptAuthTokensv2::LoginAccountParams\n\n[UNSTABLE] FOR OPENAI INTERNAL USE ONLY - DO NOT USE. The access token must contain the same scopes that Codex-managed ChatGPT auth tokens have."]
    #[serde(rename = "chatgptAuthTokens")]
    ChatgptAuthTokens {
        #[doc = "Access token (JWT) supplied by the client. This token is used for backend API requests and email extraction."]
        #[serde(rename = "accessToken")]
        access_token: ::std::string::String,
        #[doc = "Workspace/account identifier supplied by the client."]
        #[serde(rename = "chatgptAccountId")]
        chatgpt_account_id: ::std::string::String,
        #[doc = "Optional plan type supplied by the client.\n\nWhen `null`, Codex attempts to derive the plan type from access-token claims. If unavailable, the plan defaults to `unknown`."]
        #[serde(
            rename = "chatgptPlanType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        chatgpt_plan_type: ::std::option::Option<::std::string::String>,
    },
}
#[doc = "`LoginAccountResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"LoginAccountResponse\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"ApiKeyv2::LoginAccountResponse\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ApiKeyv2::LoginAccountResponseType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"apiKey\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Chatgptv2::LoginAccountResponse\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"authUrl\","]
#[doc = "        \"loginId\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"authUrl\": {"]
#[doc = "          \"description\": \"URL the client should open in a browser to initiate the OAuth flow.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"loginId\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"Chatgptv2::LoginAccountResponseType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"chatgpt\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ChatgptDeviceCodev2::LoginAccountResponse\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"loginId\","]
#[doc = "        \"type\","]
#[doc = "        \"userCode\","]
#[doc = "        \"verificationUrl\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"loginId\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ChatgptDeviceCodev2::LoginAccountResponseType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"chatgptDeviceCode\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"userCode\": {"]
#[doc = "          \"description\": \"One-time code the user must enter after signing in.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"verificationUrl\": {"]
#[doc = "          \"description\": \"URL the client should open in a browser to complete device code authorization.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ChatgptAuthTokensv2::LoginAccountResponse\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ChatgptAuthTokensv2::LoginAccountResponseType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"chatgptAuthTokens\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum LoginAccountResponse {
    #[serde(rename = "apiKey")]
    ApiKey,
    #[doc = "Chatgptv2::LoginAccountResponse"]
    #[serde(rename = "chatgpt")]
    Chatgpt {
        #[doc = "URL the client should open in a browser to initiate the OAuth flow."]
        #[serde(rename = "authUrl")]
        auth_url: ::std::string::String,
        #[serde(rename = "loginId")]
        login_id: ::std::string::String,
    },
    #[doc = "ChatgptDeviceCodev2::LoginAccountResponse"]
    #[serde(rename = "chatgptDeviceCode")]
    ChatgptDeviceCode {
        #[serde(rename = "loginId")]
        login_id: ::std::string::String,
        #[doc = "One-time code the user must enter after signing in."]
        #[serde(rename = "userCode")]
        user_code: ::std::string::String,
        #[doc = "URL the client should open in a browser to complete device code authorization."]
        #[serde(rename = "verificationUrl")]
        verification_url: ::std::string::String,
    },
    #[serde(rename = "chatgptAuthTokens")]
    ChatgptAuthTokens,
}
#[doc = "`LogoutAccountResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"LogoutAccountResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct LogoutAccountResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for LogoutAccountResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<LogoutAccountResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: LogoutAccountResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for LogoutAccountResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`MarketplaceInterface`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"displayName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MarketplaceInterface {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for MarketplaceInterface {
    fn default() -> Self {
        Self {
            display_name: Default::default(),
        }
    }
}
#[doc = "`MarketplaceLoadErrorInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"marketplacePath\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"marketplacePath\": {"]
#[doc = "      \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MarketplaceLoadErrorInfo {
    #[serde(rename = "marketplacePath")]
    pub marketplace_path: AbsolutePathBuf,
    pub message: ::std::string::String,
}
#[doc = "`McpAuthStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"unsupported\","]
#[doc = "    \"notLoggedIn\","]
#[doc = "    \"bearerToken\","]
#[doc = "    \"oAuth\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpAuthStatus {
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "notLoggedIn")]
    NotLoggedIn,
    #[serde(rename = "bearerToken")]
    BearerToken,
    #[serde(rename = "oAuth")]
    OAuth,
}
impl ::std::fmt::Display for McpAuthStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unsupported => f.write_str("unsupported"),
            Self::NotLoggedIn => f.write_str("notLoggedIn"),
            Self::BearerToken => f.write_str("bearerToken"),
            Self::OAuth => f.write_str("oAuth"),
        }
    }
}
impl ::std::str::FromStr for McpAuthStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unsupported" => Ok(Self::Unsupported),
            "notLoggedIn" => Ok(Self::NotLoggedIn),
            "bearerToken" => Ok(Self::BearerToken),
            "oAuth" => Ok(Self::OAuth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpAuthStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for McpAuthStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpAuthStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`McpResourceReadParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpResourceReadParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"server\","]
#[doc = "    \"threadId\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"server\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpResourceReadParams {
    pub server: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    pub uri: ::std::string::String,
}
#[doc = "`McpResourceReadResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpResourceReadResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contents\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ResourceContent\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpResourceReadResponse {
    pub contents: ::std::vec::Vec<ResourceContent>,
}
#[doc = "`McpServerOauthLoginCompletedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpServerOauthLoginCompletedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"success\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"success\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpServerOauthLoginCompletedNotification {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub success: bool,
}
#[doc = "`McpServerOauthLoginParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpServerOauthLoginParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"timeoutSecs\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpServerOauthLoginParams {
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(
        rename = "timeoutSecs",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub timeout_secs: ::std::option::Option<i64>,
}
#[doc = "`McpServerOauthLoginResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpServerOauthLoginResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"authorizationUrl\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authorizationUrl\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpServerOauthLoginResponse {
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: ::std::string::String,
}
#[doc = "`McpServerRefreshResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpServerRefreshResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct McpServerRefreshResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for McpServerRefreshResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<McpServerRefreshResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: McpServerRefreshResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for McpServerRefreshResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`McpServerStartupState`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"starting\","]
#[doc = "    \"ready\","]
#[doc = "    \"failed\","]
#[doc = "    \"cancelled\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpServerStartupState {
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ::std::fmt::Display for McpServerStartupState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Starting => f.write_str("starting"),
            Self::Ready => f.write_str("ready"),
            Self::Failed => f.write_str("failed"),
            Self::Cancelled => f.write_str("cancelled"),
        }
    }
}
impl ::std::str::FromStr for McpServerStartupState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "starting" => Ok(Self::Starting),
            "ready" => Ok(Self::Ready),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpServerStartupState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for McpServerStartupState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpServerStartupState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`McpServerStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"authStatus\","]
#[doc = "    \"name\","]
#[doc = "    \"resourceTemplates\","]
#[doc = "    \"resources\","]
#[doc = "    \"tools\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authStatus\": {"]
#[doc = "      \"$ref\": \"#/definitions/McpAuthStatus\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resourceTemplates\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ResourceTemplate\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"resources\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Resource\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tools\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/Tool\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpServerStatus {
    #[serde(rename = "authStatus")]
    pub auth_status: McpAuthStatus,
    pub name: ::std::string::String,
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: ::std::vec::Vec<ResourceTemplate>,
    pub resources: ::std::vec::Vec<Resource>,
    pub tools: ::std::collections::HashMap<::std::string::String, Tool>,
}
#[doc = "`McpServerStatusDetail`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"full\","]
#[doc = "    \"toolsAndAuthOnly\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpServerStatusDetail {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "toolsAndAuthOnly")]
    ToolsAndAuthOnly,
}
impl ::std::fmt::Display for McpServerStatusDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Full => f.write_str("full"),
            Self::ToolsAndAuthOnly => f.write_str("toolsAndAuthOnly"),
        }
    }
}
impl ::std::str::FromStr for McpServerStatusDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "full" => Ok(Self::Full),
            "toolsAndAuthOnly" => Ok(Self::ToolsAndAuthOnly),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpServerStatusDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for McpServerStatusDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpServerStatusDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`McpServerStatusUpdatedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpServerStatusUpdatedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/McpServerStartupState\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpServerStatusUpdatedNotification {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub status: McpServerStartupState,
}
#[doc = "`McpServerToolCallParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpServerToolCallParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"server\","]
#[doc = "    \"threadId\","]
#[doc = "    \"tool\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": true,"]
#[doc = "    \"arguments\": true,"]
#[doc = "    \"server\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tool\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpServerToolCallParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub arguments: ::std::option::Option<::serde_json::Value>,
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<::serde_json::Value>,
    pub server: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    pub tool: ::std::string::String,
}
#[doc = "`McpServerToolCallResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpServerToolCallResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": true,"]
#[doc = "    \"content\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": true"]
#[doc = "    },"]
#[doc = "    \"isError\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"structuredContent\": true"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpServerToolCallResponse {
    pub content: ::std::vec::Vec<::serde_json::Value>,
    #[serde(
        rename = "isError",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_error: ::std::option::Option<bool>,
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<::serde_json::Value>,
    #[serde(
        rename = "structuredContent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub structured_content: ::std::option::Option<::serde_json::Value>,
}
#[doc = "`McpToolCallError`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpToolCallError {
    pub message: ::std::string::String,
}
#[doc = "`McpToolCallProgressNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"McpToolCallProgressNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"itemId\","]
#[doc = "    \"message\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpToolCallProgressNotification {
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    pub message: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`McpToolCallResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": true,"]
#[doc = "    \"content\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": true"]
#[doc = "    },"]
#[doc = "    \"structuredContent\": true"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct McpToolCallResult {
    pub content: ::std::vec::Vec<::serde_json::Value>,
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<::serde_json::Value>,
    #[serde(
        rename = "structuredContent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub structured_content: ::std::option::Option<::serde_json::Value>,
}
#[doc = "`McpToolCallStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inProgress\","]
#[doc = "    \"completed\","]
#[doc = "    \"failed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpToolCallStatus {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for McpToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("inProgress"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for McpToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inProgress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for McpToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MemoryCitation`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"entries\","]
#[doc = "    \"threadIds\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"entries\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/MemoryCitationEntry\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"threadIds\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MemoryCitation {
    pub entries: ::std::vec::Vec<MemoryCitationEntry>,
    #[serde(rename = "threadIds")]
    pub thread_ids: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`MemoryCitationEntry`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"lineEnd\","]
#[doc = "    \"lineStart\","]
#[doc = "    \"note\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"lineEnd\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"lineStart\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"note\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MemoryCitationEntry {
    #[serde(rename = "lineEnd")]
    pub line_end: u32,
    #[serde(rename = "lineStart")]
    pub line_start: u32,
    pub note: ::std::string::String,
    pub path: ::std::string::String,
}
#[doc = "`MergeStrategy`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"replace\","]
#[doc = "    \"upsert\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MergeStrategy {
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "upsert")]
    Upsert,
}
impl ::std::fmt::Display for MergeStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Replace => f.write_str("replace"),
            Self::Upsert => f.write_str("upsert"),
        }
    }
}
impl ::std::str::FromStr for MergeStrategy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "replace" => Ok(Self::Replace),
            "upsert" => Ok(Self::Upsert),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MergeStrategy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MergeStrategy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MergeStrategy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Classifies an assistant message as interim commentary or final answer text.\n\nProviders do not emit this consistently, so callers must treat `None` as \"phase unknown\" and keep compatibility behavior for legacy models."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Classifies an assistant message as interim commentary or final answer text.\\n\\nProviders do not emit this consistently, so callers must treat `None` as \\\"phase unknown\\\" and keep compatibility behavior for legacy models.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Mid-turn assistant text (for example preamble/progress narration).\\n\\nAdditional tool calls or assistant output may follow before turn completion.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"commentary\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"The assistant's terminal answer text for the current turn.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"final_answer\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessagePhase {
    #[doc = "Mid-turn assistant text (for example preamble/progress narration).\n\nAdditional tool calls or assistant output may follow before turn completion."]
    #[serde(rename = "commentary")]
    Commentary,
    #[doc = "The assistant's terminal answer text for the current turn."]
    #[serde(rename = "final_answer")]
    FinalAnswer,
}
impl ::std::fmt::Display for MessagePhase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Commentary => f.write_str("commentary"),
            Self::FinalAnswer => f.write_str("final_answer"),
        }
    }
}
impl ::std::str::FromStr for MessagePhase {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "commentary" => Ok(Self::Commentary),
            "final_answer" => Ok(Self::FinalAnswer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessagePhase {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MessagePhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessagePhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Initial collaboration mode to use when the TUI starts."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Initial collaboration mode to use when the TUI starts.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"plan\","]
#[doc = "    \"default\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ModeKind {
    #[serde(rename = "plan")]
    Plan,
    #[serde(rename = "default")]
    Default,
}
impl ::std::fmt::Display for ModeKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Plan => f.write_str("plan"),
            Self::Default => f.write_str("default"),
        }
    }
}
impl ::std::str::FromStr for ModeKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "plan" => Ok(Self::Plan),
            "default" => Ok(Self::Default),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModeKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ModeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Model`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"defaultReasoningEffort\","]
#[doc = "    \"description\","]
#[doc = "    \"displayName\","]
#[doc = "    \"hidden\","]
#[doc = "    \"id\","]
#[doc = "    \"isDefault\","]
#[doc = "    \"model\","]
#[doc = "    \"supportedReasoningEfforts\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"additionalSpeedTiers\": {"]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"availabilityNux\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ModelAvailabilityNux\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"defaultReasoningEffort\": {"]
#[doc = "      \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"displayName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"hidden\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"inputModalities\": {"]
#[doc = "      \"default\": ["]
#[doc = "        \"text\","]
#[doc = "        \"image\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/InputModality\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isDefault\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"supportedReasoningEfforts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ReasoningEffortOption\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"supportsPersonality\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"upgrade\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"upgradeInfo\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ModelUpgradeInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Model {
    #[serde(
        rename = "additionalSpeedTiers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub additional_speed_tiers: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "availabilityNux",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub availability_nux: ::std::option::Option<ModelAvailabilityNux>,
    #[serde(rename = "defaultReasoningEffort")]
    pub default_reasoning_effort: ReasoningEffort,
    pub description: ::std::string::String,
    #[serde(rename = "displayName")]
    pub display_name: ::std::string::String,
    pub hidden: bool,
    pub id: ::std::string::String,
    #[serde(
        rename = "inputModalities",
        default = "defaults::model_input_modalities"
    )]
    pub input_modalities: ::std::vec::Vec<InputModality>,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    pub model: ::std::string::String,
    #[serde(rename = "supportedReasoningEfforts")]
    pub supported_reasoning_efforts: ::std::vec::Vec<ReasoningEffortOption>,
    #[serde(rename = "supportsPersonality", default)]
    pub supports_personality: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub upgrade: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "upgradeInfo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub upgrade_info: ::std::option::Option<ModelUpgradeInfo>,
}
#[doc = "`ModelAvailabilityNux`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ModelAvailabilityNux {
    pub message: ::std::string::String,
}
#[doc = "`ModelListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ModelListParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"Opaque pagination cursor returned by a previous call.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"includeHidden\": {"]
#[doc = "      \"description\": \"When true, include models that are hidden from the default picker list.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limit\": {"]
#[doc = "      \"description\": \"Optional page size; defaults to a reasonable server-side value.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ModelListParams {
    #[doc = "Opaque pagination cursor returned by a previous call."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[doc = "When true, include models that are hidden from the default picker list."]
    #[serde(
        rename = "includeHidden",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_hidden: ::std::option::Option<bool>,
    #[doc = "Optional page size; defaults to a reasonable server-side value."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<u32>,
}
impl ::std::default::Default for ModelListParams {
    fn default() -> Self {
        Self {
            cursor: Default::default(),
            include_hidden: Default::default(),
            limit: Default::default(),
        }
    }
}
#[doc = "`ModelListResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ModelListResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Model\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"Opaque cursor to pass to the next call to continue after the last item. If None, there are no more items to return.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ModelListResponse {
    pub data: ::std::vec::Vec<Model>,
    #[doc = "Opaque cursor to pass to the next call to continue after the last item. If None, there are no more items to return."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
}
#[doc = "`ModelRerouteReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"highRiskCyberActivity\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ModelRerouteReason {
    #[serde(rename = "highRiskCyberActivity")]
    HighRiskCyberActivity,
}
impl ::std::fmt::Display for ModelRerouteReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::HighRiskCyberActivity => f.write_str("highRiskCyberActivity"),
        }
    }
}
impl ::std::str::FromStr for ModelRerouteReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "highRiskCyberActivity" => Ok(Self::HighRiskCyberActivity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelRerouteReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ModelRerouteReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelRerouteReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ModelReroutedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ModelReroutedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fromModel\","]
#[doc = "    \"reason\","]
#[doc = "    \"threadId\","]
#[doc = "    \"toModel\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fromModel\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"$ref\": \"#/definitions/ModelRerouteReason\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"toModel\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ModelReroutedNotification {
    #[serde(rename = "fromModel")]
    pub from_model: ::std::string::String,
    pub reason: ModelRerouteReason,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "toModel")]
    pub to_model: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`ModelUpgradeInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"model\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"migrationMarkdown\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modelLink\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"upgradeCopy\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ModelUpgradeInfo {
    #[serde(
        rename = "migrationMarkdown",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub migration_markdown: ::std::option::Option<::std::string::String>,
    pub model: ::std::string::String,
    #[serde(
        rename = "modelLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_link: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "upgradeCopy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub upgrade_copy: ::std::option::Option<::std::string::String>,
}
#[doc = "`NetworkAccess`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"restricted\","]
#[doc = "    \"enabled\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NetworkAccess {
    #[serde(rename = "restricted")]
    Restricted,
    #[serde(rename = "enabled")]
    Enabled,
}
impl ::std::fmt::Display for NetworkAccess {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Restricted => f.write_str("restricted"),
            Self::Enabled => f.write_str("enabled"),
        }
    }
}
impl ::std::str::FromStr for NetworkAccess {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "restricted" => Ok(Self::Restricted),
            "enabled" => Ok(Self::Enabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NetworkAccess {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NetworkAccess {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NetworkAccess {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`NetworkApprovalProtocol`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"http\","]
#[doc = "    \"https\","]
#[doc = "    \"socks5Tcp\","]
#[doc = "    \"socks5Udp\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NetworkApprovalProtocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "socks5Tcp")]
    Socks5Tcp,
    #[serde(rename = "socks5Udp")]
    Socks5Udp,
}
impl ::std::fmt::Display for NetworkApprovalProtocol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Http => f.write_str("http"),
            Self::Https => f.write_str("https"),
            Self::Socks5Tcp => f.write_str("socks5Tcp"),
            Self::Socks5Udp => f.write_str("socks5Udp"),
        }
    }
}
impl ::std::str::FromStr for NetworkApprovalProtocol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            "socks5Tcp" => Ok(Self::Socks5Tcp),
            "socks5Udp" => Ok(Self::Socks5Udp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NetworkApprovalProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NetworkApprovalProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NetworkApprovalProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`NetworkDomainPermission`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"allow\","]
#[doc = "    \"deny\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NetworkDomainPermission {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}
impl ::std::fmt::Display for NetworkDomainPermission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allow => f.write_str("allow"),
            Self::Deny => f.write_str("deny"),
        }
    }
}
impl ::std::str::FromStr for NetworkDomainPermission {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow" => Ok(Self::Allow),
            "deny" => Ok(Self::Deny),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NetworkDomainPermission {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NetworkDomainPermission {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NetworkDomainPermission {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`NetworkRequirements`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowLocalBinding\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"allowUnixSockets\": {"]
#[doc = "      \"description\": \"Legacy compatibility view derived from `unix_sockets`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"allowUpstreamProxy\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"allowedDomains\": {"]
#[doc = "      \"description\": \"Legacy compatibility view derived from `domains`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"dangerFullAccessDenylistOnly\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"dangerouslyAllowAllUnixSockets\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"dangerouslyAllowNonLoopbackProxy\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"deniedDomains\": {"]
#[doc = "      \"description\": \"Legacy compatibility view derived from `domains`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"domains\": {"]
#[doc = "      \"description\": \"Canonical network permission map for `experimental_network`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/NetworkDomainPermission\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"httpPort\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"managedAllowedDomainsOnly\": {"]
#[doc = "      \"description\": \"When true, only managed allowlist entries are respected while managed network enforcement is active.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"socksPort\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"unixSockets\": {"]
#[doc = "      \"description\": \"Canonical unix socket permission map for `experimental_network`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/NetworkUnixSocketPermission\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NetworkRequirements {
    #[serde(
        rename = "allowLocalBinding",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_local_binding: ::std::option::Option<bool>,
    #[doc = "Legacy compatibility view derived from `unix_sockets`."]
    #[serde(
        rename = "allowUnixSockets",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_unix_sockets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(
        rename = "allowUpstreamProxy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_upstream_proxy: ::std::option::Option<bool>,
    #[doc = "Legacy compatibility view derived from `domains`."]
    #[serde(
        rename = "allowedDomains",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allowed_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(
        rename = "dangerFullAccessDenylistOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub danger_full_access_denylist_only: ::std::option::Option<bool>,
    #[serde(
        rename = "dangerouslyAllowAllUnixSockets",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dangerously_allow_all_unix_sockets: ::std::option::Option<bool>,
    #[serde(
        rename = "dangerouslyAllowNonLoopbackProxy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dangerously_allow_non_loopback_proxy: ::std::option::Option<bool>,
    #[doc = "Legacy compatibility view derived from `domains`."]
    #[serde(
        rename = "deniedDomains",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub denied_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "Canonical network permission map for `experimental_network`."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub domains: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, NetworkDomainPermission>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "httpPort",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub http_port: ::std::option::Option<u16>,
    #[doc = "When true, only managed allowlist entries are respected while managed network enforcement is active."]
    #[serde(
        rename = "managedAllowedDomainsOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub managed_allowed_domains_only: ::std::option::Option<bool>,
    #[serde(
        rename = "socksPort",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub socks_port: ::std::option::Option<u16>,
    #[doc = "Canonical unix socket permission map for `experimental_network`."]
    #[serde(
        rename = "unixSockets",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub unix_sockets: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, NetworkUnixSocketPermission>,
    >,
}
impl ::std::default::Default for NetworkRequirements {
    fn default() -> Self {
        Self {
            allow_local_binding: Default::default(),
            allow_unix_sockets: Default::default(),
            allow_upstream_proxy: Default::default(),
            allowed_domains: Default::default(),
            danger_full_access_denylist_only: Default::default(),
            dangerously_allow_all_unix_sockets: Default::default(),
            dangerously_allow_non_loopback_proxy: Default::default(),
            denied_domains: Default::default(),
            domains: Default::default(),
            enabled: Default::default(),
            http_port: Default::default(),
            managed_allowed_domains_only: Default::default(),
            socks_port: Default::default(),
            unix_sockets: Default::default(),
        }
    }
}
#[doc = "`NetworkUnixSocketPermission`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"allow\","]
#[doc = "    \"none\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NetworkUnixSocketPermission {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "none")]
    None,
}
impl ::std::fmt::Display for NetworkUnixSocketPermission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allow => f.write_str("allow"),
            Self::None => f.write_str("none"),
        }
    }
}
impl ::std::str::FromStr for NetworkUnixSocketPermission {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow" => Ok(Self::Allow),
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NetworkUnixSocketPermission {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NetworkUnixSocketPermission {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NetworkUnixSocketPermission {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`NonSteerableTurnKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"review\","]
#[doc = "    \"compact\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NonSteerableTurnKind {
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "compact")]
    Compact,
}
impl ::std::fmt::Display for NonSteerableTurnKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Review => f.write_str("review"),
            Self::Compact => f.write_str("compact"),
        }
    }
}
impl ::std::str::FromStr for NonSteerableTurnKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "review" => Ok(Self::Review),
            "compact" => Ok(Self::Compact),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NonSteerableTurnKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NonSteerableTurnKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NonSteerableTurnKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`OverriddenMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"effectiveValue\","]
#[doc = "    \"message\","]
#[doc = "    \"overridingLayer\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"effectiveValue\": true,"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"overridingLayer\": {"]
#[doc = "      \"$ref\": \"#/definitions/ConfigLayerMetadata\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OverriddenMetadata {
    #[serde(rename = "effectiveValue")]
    pub effective_value: ::serde_json::Value,
    pub message: ::std::string::String,
    #[serde(rename = "overridingLayer")]
    pub overriding_layer: ConfigLayerMetadata,
}
#[doc = "`PatchApplyStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inProgress\","]
#[doc = "    \"completed\","]
#[doc = "    \"failed\","]
#[doc = "    \"declined\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PatchApplyStatus {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "declined")]
    Declined,
}
impl ::std::fmt::Display for PatchApplyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("inProgress"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
            Self::Declined => f.write_str("declined"),
        }
    }
}
impl ::std::str::FromStr for PatchApplyStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inProgress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "declined" => Ok(Self::Declined),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PatchApplyStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PatchApplyStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PatchApplyStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PatchChangeKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"AddPatchChangeKind\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"AddPatchChangeKindType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"add\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"DeletePatchChangeKind\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"DeletePatchChangeKindType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"delete\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"UpdatePatchChangeKind\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"move_path\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"UpdatePatchChangeKindType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"update\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum PatchChangeKind {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "delete")]
    Delete,
    #[doc = "UpdatePatchChangeKind"]
    #[serde(rename = "update")]
    Update {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        move_path: ::std::option::Option<::std::string::String>,
    },
}
#[doc = "`Personality`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"none\","]
#[doc = "    \"friendly\","]
#[doc = "    \"pragmatic\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Personality {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "friendly")]
    Friendly,
    #[serde(rename = "pragmatic")]
    Pragmatic,
}
impl ::std::fmt::Display for Personality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Friendly => f.write_str("friendly"),
            Self::Pragmatic => f.write_str("pragmatic"),
        }
    }
}
impl ::std::str::FromStr for Personality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "friendly" => Ok(Self::Friendly),
            "pragmatic" => Ok(Self::Pragmatic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Personality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Personality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Personality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "EXPERIMENTAL - proposed plan streaming deltas for plan items. Clients should not assume concatenated deltas match the completed plan item content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PlanDeltaNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - proposed plan streaming deltas for plan items. Clients should not assume concatenated deltas match the completed plan item content.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"delta\","]
#[doc = "    \"itemId\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"delta\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PlanDeltaNotification {
    pub delta: ::std::string::String,
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`PlanType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"free\","]
#[doc = "    \"go\","]
#[doc = "    \"plus\","]
#[doc = "    \"pro\","]
#[doc = "    \"prolite\","]
#[doc = "    \"team\","]
#[doc = "    \"self_serve_business_usage_based\","]
#[doc = "    \"business\","]
#[doc = "    \"enterprise_cbp_usage_based\","]
#[doc = "    \"enterprise\","]
#[doc = "    \"edu\","]
#[doc = "    \"unknown\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PlanType {
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "plus")]
    Plus,
    #[serde(rename = "pro")]
    Pro,
    #[serde(rename = "prolite")]
    Prolite,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "self_serve_business_usage_based")]
    SelfServeBusinessUsageBased,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "enterprise_cbp_usage_based")]
    EnterpriseCbpUsageBased,
    #[serde(rename = "enterprise")]
    Enterprise,
    #[serde(rename = "edu")]
    Edu,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for PlanType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Free => f.write_str("free"),
            Self::Go => f.write_str("go"),
            Self::Plus => f.write_str("plus"),
            Self::Pro => f.write_str("pro"),
            Self::Prolite => f.write_str("prolite"),
            Self::Team => f.write_str("team"),
            Self::SelfServeBusinessUsageBased => f.write_str("self_serve_business_usage_based"),
            Self::Business => f.write_str("business"),
            Self::EnterpriseCbpUsageBased => f.write_str("enterprise_cbp_usage_based"),
            Self::Enterprise => f.write_str("enterprise"),
            Self::Edu => f.write_str("edu"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for PlanType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "free" => Ok(Self::Free),
            "go" => Ok(Self::Go),
            "plus" => Ok(Self::Plus),
            "pro" => Ok(Self::Pro),
            "prolite" => Ok(Self::Prolite),
            "team" => Ok(Self::Team),
            "self_serve_business_usage_based" => Ok(Self::SelfServeBusinessUsageBased),
            "business" => Ok(Self::Business),
            "enterprise_cbp_usage_based" => Ok(Self::EnterpriseCbpUsageBased),
            "enterprise" => Ok(Self::Enterprise),
            "edu" => Ok(Self::Edu),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlanType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PlanType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlanType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PluginAuthPolicy`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ON_INSTALL\","]
#[doc = "    \"ON_USE\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PluginAuthPolicy {
    #[serde(rename = "ON_INSTALL")]
    OnInstall,
    #[serde(rename = "ON_USE")]
    OnUse,
}
impl ::std::fmt::Display for PluginAuthPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OnInstall => f.write_str("ON_INSTALL"),
            Self::OnUse => f.write_str("ON_USE"),
        }
    }
}
impl ::std::str::FromStr for PluginAuthPolicy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ON_INSTALL" => Ok(Self::OnInstall),
            "ON_USE" => Ok(Self::OnUse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginAuthPolicy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PluginAuthPolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginAuthPolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PluginDetail`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"apps\","]
#[doc = "    \"marketplaceName\","]
#[doc = "    \"marketplacePath\","]
#[doc = "    \"mcpServers\","]
#[doc = "    \"skills\","]
#[doc = "    \"summary\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"apps\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AppSummary\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"marketplaceName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"marketplacePath\": {"]
#[doc = "      \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "    },"]
#[doc = "    \"mcpServers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"skills\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SkillSummary\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"$ref\": \"#/definitions/PluginSummary\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginDetail {
    pub apps: ::std::vec::Vec<AppSummary>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marketplaceName")]
    pub marketplace_name: ::std::string::String,
    #[serde(rename = "marketplacePath")]
    pub marketplace_path: AbsolutePathBuf,
    #[serde(rename = "mcpServers")]
    pub mcp_servers: ::std::vec::Vec<::std::string::String>,
    pub skills: ::std::vec::Vec<SkillSummary>,
    pub summary: PluginSummary,
}
#[doc = "`PluginInstallParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PluginInstallParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"marketplacePath\","]
#[doc = "    \"pluginName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"forceRemoteSync\": {"]
#[doc = "      \"description\": \"When true, apply the remote plugin change before the local install flow.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"marketplacePath\": {"]
#[doc = "      \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "    },"]
#[doc = "    \"pluginName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginInstallParams {
    #[doc = "When true, apply the remote plugin change before the local install flow."]
    #[serde(
        rename = "forceRemoteSync",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub force_remote_sync: ::std::option::Option<bool>,
    #[serde(rename = "marketplacePath")]
    pub marketplace_path: AbsolutePathBuf,
    #[serde(rename = "pluginName")]
    pub plugin_name: ::std::string::String,
}
#[doc = "`PluginInstallPolicy`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"NOT_AVAILABLE\","]
#[doc = "    \"AVAILABLE\","]
#[doc = "    \"INSTALLED_BY_DEFAULT\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PluginInstallPolicy {
    #[serde(rename = "NOT_AVAILABLE")]
    NotAvailable,
    #[serde(rename = "AVAILABLE")]
    Available,
    #[serde(rename = "INSTALLED_BY_DEFAULT")]
    InstalledByDefault,
}
impl ::std::fmt::Display for PluginInstallPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NotAvailable => f.write_str("NOT_AVAILABLE"),
            Self::Available => f.write_str("AVAILABLE"),
            Self::InstalledByDefault => f.write_str("INSTALLED_BY_DEFAULT"),
        }
    }
}
impl ::std::str::FromStr for PluginInstallPolicy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NOT_AVAILABLE" => Ok(Self::NotAvailable),
            "AVAILABLE" => Ok(Self::Available),
            "INSTALLED_BY_DEFAULT" => Ok(Self::InstalledByDefault),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginInstallPolicy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PluginInstallPolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginInstallPolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PluginInstallResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PluginInstallResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"appsNeedingAuth\","]
#[doc = "    \"authPolicy\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"appsNeedingAuth\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AppSummary\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"authPolicy\": {"]
#[doc = "      \"$ref\": \"#/definitions/PluginAuthPolicy\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginInstallResponse {
    #[serde(rename = "appsNeedingAuth")]
    pub apps_needing_auth: ::std::vec::Vec<AppSummary>,
    #[serde(rename = "authPolicy")]
    pub auth_policy: PluginAuthPolicy,
}
#[doc = "`PluginInterface`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"screenshots\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"brandColor\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"category\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"composerIcon\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"defaultPrompt\": {"]
#[doc = "      \"description\": \"Starter prompts for the plugin. Capped at 3 entries with a maximum of 128 characters per entry.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"developerName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"displayName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"logo\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"longDescription\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"privacyPolicyUrl\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"screenshots\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"shortDescription\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"termsOfServiceUrl\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"websiteUrl\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginInterface {
    #[serde(
        rename = "brandColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub brand_color: ::std::option::Option<::std::string::String>,
    pub capabilities: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "composerIcon",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub composer_icon: ::std::option::Option<AbsolutePathBuf>,
    #[doc = "Starter prompts for the plugin. Capped at 3 entries with a maximum of 128 characters per entry."]
    #[serde(
        rename = "defaultPrompt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_prompt: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(
        rename = "developerName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub developer_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logo: ::std::option::Option<AbsolutePathBuf>,
    #[serde(
        rename = "longDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "privacyPolicyUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub privacy_policy_url: ::std::option::Option<::std::string::String>,
    pub screenshots: ::std::vec::Vec<AbsolutePathBuf>,
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "termsOfServiceUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terms_of_service_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "websiteUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub website_url: ::std::option::Option<::std::string::String>,
}
#[doc = "`PluginListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PluginListParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwds\": {"]
#[doc = "      \"description\": \"Optional working directories used to discover repo marketplaces. When omitted, only home-scoped marketplaces and the official curated marketplace are considered.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"forceRemoteSync\": {"]
#[doc = "      \"description\": \"When true, reconcile the official curated marketplace against the remote plugin state before listing marketplaces.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginListParams {
    #[doc = "Optional working directories used to discover repo marketplaces. When omitted, only home-scoped marketplaces and the official curated marketplace are considered."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwds: ::std::option::Option<::std::vec::Vec<AbsolutePathBuf>>,
    #[doc = "When true, reconcile the official curated marketplace against the remote plugin state before listing marketplaces."]
    #[serde(
        rename = "forceRemoteSync",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub force_remote_sync: ::std::option::Option<bool>,
}
impl ::std::default::Default for PluginListParams {
    fn default() -> Self {
        Self {
            cwds: Default::default(),
            force_remote_sync: Default::default(),
        }
    }
}
#[doc = "`PluginListResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PluginListResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"marketplaces\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"featuredPluginIds\": {"]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"marketplaceLoadErrors\": {"]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/MarketplaceLoadErrorInfo\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"marketplaces\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/PluginMarketplaceEntry\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"remoteSyncError\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginListResponse {
    #[serde(
        rename = "featuredPluginIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub featured_plugin_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "marketplaceLoadErrors",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub marketplace_load_errors: ::std::vec::Vec<MarketplaceLoadErrorInfo>,
    pub marketplaces: ::std::vec::Vec<PluginMarketplaceEntry>,
    #[serde(
        rename = "remoteSyncError",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub remote_sync_error: ::std::option::Option<::std::string::String>,
}
#[doc = "`PluginMarketplaceEntry`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"path\","]
#[doc = "    \"plugins\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"interface\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/MarketplaceInterface\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "    },"]
#[doc = "    \"plugins\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/PluginSummary\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginMarketplaceEntry {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub interface: ::std::option::Option<MarketplaceInterface>,
    pub name: ::std::string::String,
    pub path: AbsolutePathBuf,
    pub plugins: ::std::vec::Vec<PluginSummary>,
}
#[doc = "`PluginReadParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PluginReadParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"marketplacePath\","]
#[doc = "    \"pluginName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"marketplacePath\": {"]
#[doc = "      \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "    },"]
#[doc = "    \"pluginName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginReadParams {
    #[serde(rename = "marketplacePath")]
    pub marketplace_path: AbsolutePathBuf,
    #[serde(rename = "pluginName")]
    pub plugin_name: ::std::string::String,
}
#[doc = "`PluginReadResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PluginReadResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"plugin\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"plugin\": {"]
#[doc = "      \"$ref\": \"#/definitions/PluginDetail\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginReadResponse {
    pub plugin: PluginDetail,
}
#[doc = "`PluginSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"LocalPluginSource\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"path\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"path\": {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"LocalPluginSourceType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"local\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type", content = "path")]
pub enum PluginSource {
    #[doc = "LocalPluginSource"]
    #[serde(rename = "local")]
    Local(AbsolutePathBuf),
}
impl ::std::convert::From<AbsolutePathBuf> for PluginSource {
    fn from(value: AbsolutePathBuf) -> Self {
        Self::Local(value)
    }
}
#[doc = "`PluginSummary`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"authPolicy\","]
#[doc = "    \"enabled\","]
#[doc = "    \"id\","]
#[doc = "    \"installPolicy\","]
#[doc = "    \"installed\","]
#[doc = "    \"name\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authPolicy\": {"]
#[doc = "      \"$ref\": \"#/definitions/PluginAuthPolicy\""]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"installPolicy\": {"]
#[doc = "      \"$ref\": \"#/definitions/PluginInstallPolicy\""]
#[doc = "    },"]
#[doc = "    \"installed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"interface\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PluginInterface\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/definitions/PluginSource\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginSummary {
    #[serde(rename = "authPolicy")]
    pub auth_policy: PluginAuthPolicy,
    pub enabled: bool,
    pub id: ::std::string::String,
    #[serde(rename = "installPolicy")]
    pub install_policy: PluginInstallPolicy,
    pub installed: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub interface: ::std::option::Option<PluginInterface>,
    pub name: ::std::string::String,
    pub source: PluginSource,
}
#[doc = "`PluginUninstallParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PluginUninstallParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pluginId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"forceRemoteSync\": {"]
#[doc = "      \"description\": \"When true, apply the remote plugin change before the local uninstall flow.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"pluginId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PluginUninstallParams {
    #[doc = "When true, apply the remote plugin change before the local uninstall flow."]
    #[serde(
        rename = "forceRemoteSync",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub force_remote_sync: ::std::option::Option<bool>,
    #[serde(rename = "pluginId")]
    pub plugin_id: ::std::string::String,
}
#[doc = "`PluginUninstallResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PluginUninstallResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PluginUninstallResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for PluginUninstallResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<PluginUninstallResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: PluginUninstallResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for PluginUninstallResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`ProfileV2`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"approval_policy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"approvals_reviewer\": {"]
#[doc = "      \"description\": \"[UNSTABLE] Optional profile-level override for where approval requests are routed for review. If omitted, the enclosing config default is used.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"chatgpt_base_url\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model_provider\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model_reasoning_effort\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model_reasoning_summary\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningSummary\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model_verbosity\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Verbosity\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"service_tier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tools\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ToolsV2\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"web_search\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/WebSearchMode\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProfileV2 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub approval_policy: ::std::option::Option<AskForApproval>,
    #[doc = "[UNSTABLE] Optional profile-level override for where approval requests are routed for review. If omitted, the enclosing config default is used."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub approvals_reviewer: ::std::option::Option<ApprovalsReviewer>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chatgpt_base_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_provider: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_reasoning_effort: ::std::option::Option<ReasoningEffort>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_reasoning_summary: ::std::option::Option<ReasoningSummary>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model_verbosity: ::std::option::Option<Verbosity>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<ToolsV2>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub web_search: ::std::option::Option<WebSearchMode>,
}
impl ::std::default::Default for ProfileV2 {
    fn default() -> Self {
        Self {
            approval_policy: Default::default(),
            approvals_reviewer: Default::default(),
            chatgpt_base_url: Default::default(),
            model: Default::default(),
            model_provider: Default::default(),
            model_reasoning_effort: Default::default(),
            model_reasoning_summary: Default::default(),
            model_verbosity: Default::default(),
            service_tier: Default::default(),
            tools: Default::default(),
            web_search: Default::default(),
        }
    }
}
#[doc = "`RateLimitSnapshot`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"credits\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CreditsSnapshot\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limitId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limitName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"planType\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PlanType\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"primary\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/RateLimitWindow\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"secondary\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/RateLimitWindow\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RateLimitSnapshot {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credits: ::std::option::Option<CreditsSnapshot>,
    #[serde(
        rename = "limitId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub limit_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "limitName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub limit_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "planType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub plan_type: ::std::option::Option<PlanType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub primary: ::std::option::Option<RateLimitWindow>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub secondary: ::std::option::Option<RateLimitWindow>,
}
impl ::std::default::Default for RateLimitSnapshot {
    fn default() -> Self {
        Self {
            credits: Default::default(),
            limit_id: Default::default(),
            limit_name: Default::default(),
            plan_type: Default::default(),
            primary: Default::default(),
            secondary: Default::default(),
        }
    }
}
#[doc = "`RateLimitWindow`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"usedPercent\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"resetsAt\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"usedPercent\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"windowDurationMins\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RateLimitWindow {
    #[serde(
        rename = "resetsAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resets_at: ::std::option::Option<i64>,
    #[serde(rename = "usedPercent")]
    pub used_percent: i32,
    #[serde(
        rename = "windowDurationMins",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub window_duration_mins: ::std::option::Option<i64>,
}
#[doc = "`RawResponseItemCompletedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"RawResponseItemCompletedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"item\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"item\": {"]
#[doc = "      \"$ref\": \"#/definitions/ResponseItem\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RawResponseItemCompletedNotification {
    pub item: ResponseItem,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`ReadOnlyAccess`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"RestrictedReadOnlyAccess\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"includePlatformDefaults\": {"]
#[doc = "          \"default\": true,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"readableRoots\": {"]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"RestrictedReadOnlyAccessType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"restricted\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FullAccessReadOnlyAccess\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"FullAccessReadOnlyAccessType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fullAccess\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ReadOnlyAccess {
    #[doc = "RestrictedReadOnlyAccess"]
    #[serde(rename = "restricted")]
    Restricted {
        #[serde(
            rename = "includePlatformDefaults",
            default = "defaults::default_bool::<true>"
        )]
        include_platform_defaults: bool,
        #[serde(
            rename = "readableRoots",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        readable_roots: ::std::vec::Vec<AbsolutePathBuf>,
    },
    #[serde(rename = "fullAccess")]
    FullAccess,
}
#[doc = "`RealtimeConversationVersion`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"v1\","]
#[doc = "    \"v2\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RealtimeConversationVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}
impl ::std::fmt::Display for RealtimeConversationVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::V1 => f.write_str("v1"),
            Self::V2 => f.write_str("v2"),
        }
    }
}
impl ::std::str::FromStr for RealtimeConversationVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "v1" => Ok(Self::V1),
            "v2" => Ok(Self::V2),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeConversationVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RealtimeConversationVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeConversationVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeVoice`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"alloy\","]
#[doc = "    \"arbor\","]
#[doc = "    \"ash\","]
#[doc = "    \"ballad\","]
#[doc = "    \"breeze\","]
#[doc = "    \"cedar\","]
#[doc = "    \"coral\","]
#[doc = "    \"cove\","]
#[doc = "    \"echo\","]
#[doc = "    \"ember\","]
#[doc = "    \"juniper\","]
#[doc = "    \"maple\","]
#[doc = "    \"marin\","]
#[doc = "    \"sage\","]
#[doc = "    \"shimmer\","]
#[doc = "    \"sol\","]
#[doc = "    \"spruce\","]
#[doc = "    \"vale\","]
#[doc = "    \"verse\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RealtimeVoice {
    #[serde(rename = "alloy")]
    Alloy,
    #[serde(rename = "arbor")]
    Arbor,
    #[serde(rename = "ash")]
    Ash,
    #[serde(rename = "ballad")]
    Ballad,
    #[serde(rename = "breeze")]
    Breeze,
    #[serde(rename = "cedar")]
    Cedar,
    #[serde(rename = "coral")]
    Coral,
    #[serde(rename = "cove")]
    Cove,
    #[serde(rename = "echo")]
    Echo,
    #[serde(rename = "ember")]
    Ember,
    #[serde(rename = "juniper")]
    Juniper,
    #[serde(rename = "maple")]
    Maple,
    #[serde(rename = "marin")]
    Marin,
    #[serde(rename = "sage")]
    Sage,
    #[serde(rename = "shimmer")]
    Shimmer,
    #[serde(rename = "sol")]
    Sol,
    #[serde(rename = "spruce")]
    Spruce,
    #[serde(rename = "vale")]
    Vale,
    #[serde(rename = "verse")]
    Verse,
}
impl ::std::fmt::Display for RealtimeVoice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alloy => f.write_str("alloy"),
            Self::Arbor => f.write_str("arbor"),
            Self::Ash => f.write_str("ash"),
            Self::Ballad => f.write_str("ballad"),
            Self::Breeze => f.write_str("breeze"),
            Self::Cedar => f.write_str("cedar"),
            Self::Coral => f.write_str("coral"),
            Self::Cove => f.write_str("cove"),
            Self::Echo => f.write_str("echo"),
            Self::Ember => f.write_str("ember"),
            Self::Juniper => f.write_str("juniper"),
            Self::Maple => f.write_str("maple"),
            Self::Marin => f.write_str("marin"),
            Self::Sage => f.write_str("sage"),
            Self::Shimmer => f.write_str("shimmer"),
            Self::Sol => f.write_str("sol"),
            Self::Spruce => f.write_str("spruce"),
            Self::Vale => f.write_str("vale"),
            Self::Verse => f.write_str("verse"),
        }
    }
}
impl ::std::str::FromStr for RealtimeVoice {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "alloy" => Ok(Self::Alloy),
            "arbor" => Ok(Self::Arbor),
            "ash" => Ok(Self::Ash),
            "ballad" => Ok(Self::Ballad),
            "breeze" => Ok(Self::Breeze),
            "cedar" => Ok(Self::Cedar),
            "coral" => Ok(Self::Coral),
            "cove" => Ok(Self::Cove),
            "echo" => Ok(Self::Echo),
            "ember" => Ok(Self::Ember),
            "juniper" => Ok(Self::Juniper),
            "maple" => Ok(Self::Maple),
            "marin" => Ok(Self::Marin),
            "sage" => Ok(Self::Sage),
            "shimmer" => Ok(Self::Shimmer),
            "sol" => Ok(Self::Sol),
            "spruce" => Ok(Self::Spruce),
            "vale" => Ok(Self::Vale),
            "verse" => Ok(Self::Verse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RealtimeVoice {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RealtimeVoice {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RealtimeVoice {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RealtimeVoicesList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"defaultV1\","]
#[doc = "    \"defaultV2\","]
#[doc = "    \"v1\","]
#[doc = "    \"v2\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"defaultV1\": {"]
#[doc = "      \"$ref\": \"#/definitions/RealtimeVoice\""]
#[doc = "    },"]
#[doc = "    \"defaultV2\": {"]
#[doc = "      \"$ref\": \"#/definitions/RealtimeVoice\""]
#[doc = "    },"]
#[doc = "    \"v1\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/RealtimeVoice\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"v2\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/RealtimeVoice\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RealtimeVoicesList {
    #[serde(rename = "defaultV1")]
    pub default_v1: RealtimeVoice,
    #[serde(rename = "defaultV2")]
    pub default_v2: RealtimeVoice,
    pub v1: ::std::vec::Vec<RealtimeVoice>,
    pub v2: ::std::vec::Vec<RealtimeVoice>,
}
#[doc = "See https://platform.openai.com/docs/guides/reasoning?api-mode=responses#get-started-with-reasoning"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"See https://platform.openai.com/docs/guides/reasoning?api-mode=responses#get-started-with-reasoning\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"none\","]
#[doc = "    \"minimal\","]
#[doc = "    \"low\","]
#[doc = "    \"medium\","]
#[doc = "    \"high\","]
#[doc = "    \"xhigh\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningEffort {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
}
impl ::std::fmt::Display for ReasoningEffort {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Minimal => f.write_str("minimal"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
        }
    }
}
impl ::std::str::FromStr for ReasoningEffort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "minimal" => Ok(Self::Minimal),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ReasoningEffortOption`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"reasoningEffort\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reasoningEffort\": {"]
#[doc = "      \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReasoningEffortOption {
    pub description: ::std::string::String,
    #[serde(rename = "reasoningEffort")]
    pub reasoning_effort: ReasoningEffort,
}
#[doc = "`ReasoningItemContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"ReasoningTextReasoningItemContent\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ReasoningTextReasoningItemContentType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"reasoning_text\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"TextReasoningItemContent\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"TextReasoningItemContentType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"text\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type", content = "text")]
pub enum ReasoningItemContent {
    #[doc = "ReasoningTextReasoningItemContent"]
    #[serde(rename = "reasoning_text")]
    ReasoningText(::std::string::String),
    #[doc = "TextReasoningItemContent"]
    #[serde(rename = "text")]
    Text(::std::string::String),
}
#[doc = "`ReasoningItemReasoningSummary`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"SummaryTextReasoningItemReasoningSummary\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"SummaryTextReasoningItemReasoningSummaryType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"summary_text\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type", content = "text")]
pub enum ReasoningItemReasoningSummary {
    #[doc = "SummaryTextReasoningItemReasoningSummary"]
    #[serde(rename = "summary_text")]
    SummaryText(::std::string::String),
}
#[doc = "A summary of the reasoning performed by the model. This can be useful for debugging and understanding the model's reasoning process. See https://platform.openai.com/docs/guides/reasoning?api-mode=responses#reasoning-summaries"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A summary of the reasoning performed by the model. This can be useful for debugging and understanding the model's reasoning process. See https://platform.openai.com/docs/guides/reasoning?api-mode=responses#reasoning-summaries\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"auto\","]
#[doc = "        \"concise\","]
#[doc = "        \"detailed\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Option to disable reasoning summaries.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"none\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningSummary {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "concise")]
    Concise,
    #[serde(rename = "detailed")]
    Detailed,
    #[doc = "Option to disable reasoning summaries."]
    #[serde(rename = "none")]
    None,
}
impl ::std::fmt::Display for ReasoningSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Concise => f.write_str("concise"),
            Self::Detailed => f.write_str("detailed"),
            Self::None => f.write_str("none"),
        }
    }
}
impl ::std::str::FromStr for ReasoningSummary {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "concise" => Ok(Self::Concise),
            "detailed" => Ok(Self::Detailed),
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningSummary {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReasoningSummary {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningSummary {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ReasoningSummaryPartAddedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ReasoningSummaryPartAddedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"itemId\","]
#[doc = "    \"summaryIndex\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"summaryIndex\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReasoningSummaryPartAddedNotification {
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    #[serde(rename = "summaryIndex")]
    pub summary_index: i64,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`ReasoningSummaryTextDeltaNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ReasoningSummaryTextDeltaNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"delta\","]
#[doc = "    \"itemId\","]
#[doc = "    \"summaryIndex\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"delta\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"summaryIndex\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReasoningSummaryTextDeltaNotification {
    pub delta: ::std::string::String,
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    #[serde(rename = "summaryIndex")]
    pub summary_index: i64,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`ReasoningTextDeltaNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ReasoningTextDeltaNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contentIndex\","]
#[doc = "    \"delta\","]
#[doc = "    \"itemId\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contentIndex\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"delta\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReasoningTextDeltaNotification {
    #[serde(rename = "contentIndex")]
    pub content_index: i64,
    pub delta: ::std::string::String,
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`RequestId`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RequestId {
    String(::std::string::String),
    Int64(i64),
}
impl ::std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Int64(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for RequestId {
    fn from(value: i64) -> Self {
        Self::Int64(value)
    }
}
#[doc = "`ResidencyRequirement`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"us\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResidencyRequirement {
    #[serde(rename = "us")]
    Us,
}
impl ::std::fmt::Display for ResidencyRequirement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Us => f.write_str("us"),
        }
    }
}
impl ::std::str::FromStr for ResidencyRequirement {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "us" => Ok(Self::Us),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResidencyRequirement {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ResidencyRequirement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResidencyRequirement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A known resource that the server is capable of reading."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A known resource that the server is capable of reading.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": true,"]
#[doc = "    \"annotations\": true,"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"icons\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": true"]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub icons: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<::serde_json::Value>,
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    pub uri: ::std::string::String,
}
#[doc = "Contents returned when reading a resource from an MCP server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Contents returned when reading a resource from an MCP server.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": true,"]
#[doc = "        \"mimeType\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of this resource.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"blob\","]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": true,"]
#[doc = "        \"blob\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"mimeType\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of this resource.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ResourceContent {
    Variant0 {
        #[serde(
            rename = "_meta",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        meta: ::std::option::Option<::serde_json::Value>,
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        mime_type: ::std::option::Option<::std::string::String>,
        text: ::std::string::String,
        #[doc = "The URI of this resource."]
        uri: ::std::string::String,
    },
    Variant1 {
        blob: ::std::string::String,
        #[serde(
            rename = "_meta",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        meta: ::std::option::Option<::serde_json::Value>,
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        mime_type: ::std::option::Option<::std::string::String>,
        #[doc = "The URI of this resource."]
        uri: ::std::string::String,
    },
}
#[doc = "A template description for resources available on the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A template description for resources available on the server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"uriTemplate\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"annotations\": true,"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"uriTemplate\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceTemplate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uriTemplate")]
    pub uri_template: ::std::string::String,
}
#[doc = "`ResponseItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"MessageResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"content\","]
#[doc = "        \"role\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"content\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/ContentItem\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"end_turn\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"writeOnly\": true,"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"phase\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/MessagePhase\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"role\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"MessageResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"message\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ReasoningResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"summary\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"content\": {"]
#[doc = "          \"default\": null,"]
#[doc = "          \"type\": ["]
#[doc = "            \"array\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/ReasoningItemContent\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"encrypted_content\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"summary\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/ReasoningItemReasoningSummary\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ReasoningResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"reasoning\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"LocalShellCallResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"action\","]
#[doc = "        \"status\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"$ref\": \"#/definitions/LocalShellAction\""]
#[doc = "        },"]
#[doc = "        \"call_id\": {"]
#[doc = "          \"description\": \"Set when using the Responses API.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"Legacy id field retained for compatibility with older payloads.\","]
#[doc = "          \"writeOnly\": true,"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"$ref\": \"#/definitions/LocalShellStatus\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"LocalShellCallResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"local_shell_call\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FunctionCallResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"arguments\","]
#[doc = "        \"call_id\","]
#[doc = "        \"name\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"call_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"writeOnly\": true,"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"namespace\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"FunctionCallResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"function_call\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ToolSearchCallResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"arguments\","]
#[doc = "        \"execution\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": true,"]
#[doc = "        \"call_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"execution\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"writeOnly\": true,"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ToolSearchCallResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"tool_search_call\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FunctionCallOutputResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"call_id\","]
#[doc = "        \"output\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"call_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"output\": {"]
#[doc = "          \"$ref\": \"#/definitions/FunctionCallOutputBody\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"FunctionCallOutputResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"function_call_output\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CustomToolCallResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"call_id\","]
#[doc = "        \"input\","]
#[doc = "        \"name\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"call_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"writeOnly\": true,"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"input\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"CustomToolCallResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"custom_tool_call\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CustomToolCallOutputResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"call_id\","]
#[doc = "        \"output\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"call_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"output\": {"]
#[doc = "          \"$ref\": \"#/definitions/FunctionCallOutputBody\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"CustomToolCallOutputResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"custom_tool_call_output\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ToolSearchOutputResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"execution\","]
#[doc = "        \"status\","]
#[doc = "        \"tools\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"call_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"execution\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"tools\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": true"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ToolSearchOutputResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"tool_search_output\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"WebSearchCallResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ResponsesApiWebSearchAction\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"writeOnly\": true,"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"WebSearchCallResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"web_search_call\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ImageGenerationCallResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"result\","]
#[doc = "        \"status\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"result\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"revised_prompt\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ImageGenerationCallResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"image_generation_call\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"GhostSnapshotResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"ghost_commit\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ghost_commit\": {"]
#[doc = "          \"$ref\": \"#/definitions/GhostCommit\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"GhostSnapshotResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"ghost_snapshot\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CompactionResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"encrypted_content\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"encrypted_content\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"CompactionResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"compaction\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"OtherResponseItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"OtherResponseItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"other\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ResponseItem {
    #[doc = "MessageResponseItem"]
    #[serde(rename = "message")]
    Message {
        content: ::std::vec::Vec<ContentItem>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        end_turn: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        phase: ::std::option::Option<MessagePhase>,
        role: ::std::string::String,
    },
    #[doc = "ReasoningResponseItem"]
    #[serde(rename = "reasoning")]
    Reasoning {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        content: ::std::option::Option<::std::vec::Vec<ReasoningItemContent>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        encrypted_content: ::std::option::Option<::std::string::String>,
        summary: ::std::vec::Vec<ReasoningItemReasoningSummary>,
    },
    #[doc = "LocalShellCallResponseItem"]
    #[serde(rename = "local_shell_call")]
    LocalShellCall {
        action: LocalShellAction,
        #[doc = "Set when using the Responses API."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        call_id: ::std::option::Option<::std::string::String>,
        #[doc = "Legacy id field retained for compatibility with older payloads."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        id: ::std::option::Option<::std::string::String>,
        status: LocalShellStatus,
    },
    #[doc = "FunctionCallResponseItem"]
    #[serde(rename = "function_call")]
    FunctionCall {
        arguments: ::std::string::String,
        call_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        id: ::std::option::Option<::std::string::String>,
        name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        namespace: ::std::option::Option<::std::string::String>,
    },
    #[doc = "ToolSearchCallResponseItem"]
    #[serde(rename = "tool_search_call")]
    ToolSearchCall {
        arguments: ::serde_json::Value,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        call_id: ::std::option::Option<::std::string::String>,
        execution: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        status: ::std::option::Option<::std::string::String>,
    },
    #[doc = "FunctionCallOutputResponseItem"]
    #[serde(rename = "function_call_output")]
    FunctionCallOutput {
        call_id: ::std::string::String,
        output: FunctionCallOutputBody,
    },
    #[doc = "CustomToolCallResponseItem"]
    #[serde(rename = "custom_tool_call")]
    CustomToolCall {
        call_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        id: ::std::option::Option<::std::string::String>,
        input: ::std::string::String,
        name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        status: ::std::option::Option<::std::string::String>,
    },
    #[doc = "CustomToolCallOutputResponseItem"]
    #[serde(rename = "custom_tool_call_output")]
    CustomToolCallOutput {
        call_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        name: ::std::option::Option<::std::string::String>,
        output: FunctionCallOutputBody,
    },
    #[doc = "ToolSearchOutputResponseItem"]
    #[serde(rename = "tool_search_output")]
    ToolSearchOutput {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        call_id: ::std::option::Option<::std::string::String>,
        execution: ::std::string::String,
        status: ::std::string::String,
        tools: ::std::vec::Vec<::serde_json::Value>,
    },
    #[doc = "WebSearchCallResponseItem"]
    #[serde(rename = "web_search_call")]
    WebSearchCall {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        action: ::std::option::Option<ResponsesApiWebSearchAction>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        status: ::std::option::Option<::std::string::String>,
    },
    #[doc = "ImageGenerationCallResponseItem"]
    #[serde(rename = "image_generation_call")]
    ImageGenerationCall {
        id: ::std::string::String,
        result: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        revised_prompt: ::std::option::Option<::std::string::String>,
        status: ::std::string::String,
    },
    #[doc = "GhostSnapshotResponseItem"]
    #[serde(rename = "ghost_snapshot")]
    GhostSnapshot { ghost_commit: GhostCommit },
    #[doc = "CompactionResponseItem"]
    #[serde(rename = "compaction")]
    Compaction {
        encrypted_content: ::std::string::String,
    },
    #[serde(rename = "other")]
    Other,
}
#[doc = "`ResponsesApiWebSearchAction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"SearchResponsesApiWebSearchAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"queries\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"array\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"query\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"SearchResponsesApiWebSearchActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"search\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"OpenPageResponsesApiWebSearchAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"OpenPageResponsesApiWebSearchActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"open_page\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FindInPageResponsesApiWebSearchAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pattern\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"FindInPageResponsesApiWebSearchActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"find_in_page\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"OtherResponsesApiWebSearchAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"OtherResponsesApiWebSearchActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"other\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ResponsesApiWebSearchAction {
    #[doc = "SearchResponsesApiWebSearchAction"]
    #[serde(rename = "search")]
    Search {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        queries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        query: ::std::option::Option<::std::string::String>,
    },
    #[doc = "OpenPageResponsesApiWebSearchAction"]
    #[serde(rename = "open_page")]
    OpenPage {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        url: ::std::option::Option<::std::string::String>,
    },
    #[doc = "FindInPageResponsesApiWebSearchAction"]
    #[serde(rename = "find_in_page")]
    FindInPage {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pattern: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        url: ::std::option::Option<::std::string::String>,
    },
    #[serde(rename = "other")]
    Other,
}
#[doc = "`ReviewDelivery`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inline\","]
#[doc = "    \"detached\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReviewDelivery {
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "detached")]
    Detached,
}
impl ::std::fmt::Display for ReviewDelivery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inline => f.write_str("inline"),
            Self::Detached => f.write_str("detached"),
        }
    }
}
impl ::std::str::FromStr for ReviewDelivery {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inline" => Ok(Self::Inline),
            "detached" => Ok(Self::Detached),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReviewDelivery {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReviewDelivery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReviewDelivery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ReviewStartParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ReviewStartParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"target\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"delivery\": {"]
#[doc = "      \"description\": \"Where to run the review: inline (default) on the current thread or detached on a new thread (returned in `reviewThreadId`).\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReviewDelivery\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"$ref\": \"#/definitions/ReviewTarget\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReviewStartParams {
    #[doc = "Where to run the review: inline (default) on the current thread or detached on a new thread (returned in `reviewThreadId`)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub delivery: ::std::option::Option<ReviewDelivery>,
    pub target: ReviewTarget,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ReviewStartResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ReviewStartResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"reviewThreadId\","]
#[doc = "    \"turn\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"reviewThreadId\": {"]
#[doc = "      \"description\": \"Identifies the thread where the review runs.\\n\\nFor inline reviews, this is the original thread id. For detached reviews, this is the id of the new review thread.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turn\": {"]
#[doc = "      \"$ref\": \"#/definitions/Turn\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReviewStartResponse {
    #[doc = "Identifies the thread where the review runs.\n\nFor inline reviews, this is the original thread id. For detached reviews, this is the id of the new review thread."]
    #[serde(rename = "reviewThreadId")]
    pub review_thread_id: ::std::string::String,
    pub turn: Turn,
}
#[doc = "`ReviewTarget`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"UncommittedChangesReviewTarget\","]
#[doc = "      \"description\": \"Review the working tree: staged, unstaged, and untracked files.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"UncommittedChangesReviewTargetType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"uncommittedChanges\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"BaseBranchReviewTarget\","]
#[doc = "      \"description\": \"Review changes between the current branch and the given base branch.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"branch\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"branch\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"BaseBranchReviewTargetType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"baseBranch\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CommitReviewTarget\","]
#[doc = "      \"description\": \"Review the changes introduced by a specific commit.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"sha\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"sha\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"title\": {"]
#[doc = "          \"description\": \"Optional human-readable label (e.g., commit subject) for UIs.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"CommitReviewTargetType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"commit\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CustomReviewTarget\","]
#[doc = "      \"description\": \"Arbitrary instructions, equivalent to the old free-form prompt.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"instructions\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"instructions\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"CustomReviewTargetType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"custom\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ReviewTarget {
    #[serde(rename = "uncommittedChanges")]
    UncommittedChanges,
    #[doc = "BaseBranchReviewTarget\n\nReview changes between the current branch and the given base branch."]
    #[serde(rename = "baseBranch")]
    BaseBranch { branch: ::std::string::String },
    #[doc = "CommitReviewTarget\n\nReview the changes introduced by a specific commit."]
    #[serde(rename = "commit")]
    Commit {
        sha: ::std::string::String,
        #[doc = "Optional human-readable label (e.g., commit subject) for UIs."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        title: ::std::option::Option<::std::string::String>,
    },
    #[doc = "CustomReviewTarget\n\nArbitrary instructions, equivalent to the old free-form prompt."]
    #[serde(rename = "custom")]
    Custom { instructions: ::std::string::String },
}
#[doc = "`SandboxMode`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"read-only\","]
#[doc = "    \"workspace-write\","]
#[doc = "    \"danger-full-access\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SandboxMode {
    #[serde(rename = "read-only")]
    ReadOnly,
    #[serde(rename = "workspace-write")]
    WorkspaceWrite,
    #[serde(rename = "danger-full-access")]
    DangerFullAccess,
}
impl ::std::fmt::Display for SandboxMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ReadOnly => f.write_str("read-only"),
            Self::WorkspaceWrite => f.write_str("workspace-write"),
            Self::DangerFullAccess => f.write_str("danger-full-access"),
        }
    }
}
impl ::std::str::FromStr for SandboxMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "read-only" => Ok(Self::ReadOnly),
            "workspace-write" => Ok(Self::WorkspaceWrite),
            "danger-full-access" => Ok(Self::DangerFullAccess),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SandboxMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SandboxMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SandboxMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SandboxPolicy`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"DangerFullAccessSandboxPolicy\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"DangerFullAccessSandboxPolicyType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"dangerFullAccess\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ReadOnlySandboxPolicy\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"access\": {"]
#[doc = "          \"default\": {"]
#[doc = "            \"type\": \"fullAccess\""]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ReadOnlyAccess\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"networkAccess\": {"]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ReadOnlySandboxPolicyType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"readOnly\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ExternalSandboxSandboxPolicy\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"networkAccess\": {"]
#[doc = "          \"default\": \"restricted\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/NetworkAccess\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ExternalSandboxSandboxPolicyType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"externalSandbox\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"WorkspaceWriteSandboxPolicy\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"excludeSlashTmp\": {"]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"excludeTmpdirEnvVar\": {"]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"networkAccess\": {"]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"readOnlyAccess\": {"]
#[doc = "          \"default\": {"]
#[doc = "            \"type\": \"fullAccess\""]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ReadOnlyAccess\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"WorkspaceWriteSandboxPolicyType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"workspaceWrite\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"writableRoots\": {"]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum SandboxPolicy {
    #[serde(rename = "dangerFullAccess")]
    DangerFullAccess,
    #[doc = "ReadOnlySandboxPolicy"]
    #[serde(rename = "readOnly")]
    ReadOnly {
        #[serde(default = "defaults::sandbox_policy_read_only_access")]
        access: ReadOnlyAccess,
        #[serde(rename = "networkAccess", default)]
        network_access: bool,
    },
    #[doc = "ExternalSandboxSandboxPolicy"]
    #[serde(rename = "externalSandbox")]
    ExternalSandbox {
        #[serde(
            rename = "networkAccess",
            default = "defaults::sandbox_policy_external_sandbox_network_access"
        )]
        network_access: NetworkAccess,
    },
    #[doc = "WorkspaceWriteSandboxPolicy"]
    #[serde(rename = "workspaceWrite")]
    WorkspaceWrite {
        #[serde(rename = "excludeSlashTmp", default)]
        exclude_slash_tmp: bool,
        #[serde(rename = "excludeTmpdirEnvVar", default)]
        exclude_tmpdir_env_var: bool,
        #[serde(rename = "networkAccess", default)]
        network_access: bool,
        #[serde(
            rename = "readOnlyAccess",
            default = "defaults::sandbox_policy_workspace_write_read_only_access"
        )]
        read_only_access: ReadOnlyAccess,
        #[serde(
            rename = "writableRoots",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        writable_roots: ::std::vec::Vec<AbsolutePathBuf>,
    },
}
#[doc = "`SandboxWorkspaceWrite`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"exclude_slash_tmp\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"exclude_tmpdir_env_var\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"network_access\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"writable_roots\": {"]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SandboxWorkspaceWrite {
    #[serde(default)]
    pub exclude_slash_tmp: bool,
    #[serde(default)]
    pub exclude_tmpdir_env_var: bool,
    #[serde(default)]
    pub network_access: bool,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub writable_roots: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for SandboxWorkspaceWrite {
    fn default() -> Self {
        Self {
            exclude_slash_tmp: Default::default(),
            exclude_tmpdir_env_var: Default::default(),
            network_access: Default::default(),
            writable_roots: Default::default(),
        }
    }
}
#[doc = "Notification sent from the server to the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ServerNotification\","]
#[doc = "  \"description\": \"Notification sent from the server to the client.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"ErrorNotification\","]
#[doc = "      \"description\": \"NEW NOTIFICATIONS\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"ErrorNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"error\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ErrorNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/startedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/startedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/started\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadStartedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/status/changedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/status/changedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/status/changed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadStatusChangedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/archivedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/archivedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/archived\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadArchivedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/unarchivedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/unarchivedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/unarchived\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadUnarchivedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/closedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/closedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/closed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadClosedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Skills/changedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Skills/changedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"skills/changed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/SkillsChangedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/name/updatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/name/updatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/name/updated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadNameUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/tokenUsage/updatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/tokenUsage/updatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/tokenUsage/updated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadTokenUsageUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Turn/startedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Turn/startedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"turn/started\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/TurnStartedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Hook/startedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Hook/startedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"hook/started\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/HookStartedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Turn/completedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Turn/completedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"turn/completed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/TurnCompletedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Hook/completedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Hook/completedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"hook/completed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/HookCompletedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Turn/diff/updatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Turn/diff/updatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"turn/diff/updated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/TurnDiffUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Turn/plan/updatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Turn/plan/updatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"turn/plan/updated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/TurnPlanUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/startedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/startedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/started\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ItemStartedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/autoApprovalReview/startedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/autoApprovalReview/startedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/autoApprovalReview/started\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ItemGuardianApprovalReviewStartedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/autoApprovalReview/completedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/autoApprovalReview/completedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/autoApprovalReview/completed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ItemGuardianApprovalReviewCompletedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/completedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/completedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/completed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ItemCompletedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/agentMessage/deltaNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/agentMessage/deltaNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/agentMessage/delta\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/AgentMessageDeltaNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/plan/deltaNotification\","]
#[doc = "      \"description\": \"EXPERIMENTAL - proposed plan streaming deltas for plan items.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/plan/deltaNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/plan/delta\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/PlanDeltaNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Command/exec/outputDeltaNotification\","]
#[doc = "      \"description\": \"Stream base64-encoded stdout/stderr chunks for a running `command/exec` session.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Command/exec/outputDeltaNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"command/exec/outputDelta\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecOutputDeltaNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/commandExecution/outputDeltaNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/commandExecution/outputDeltaNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/commandExecution/outputDelta\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecutionOutputDeltaNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/commandExecution/terminalInteractionNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/commandExecution/terminalInteractionNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/commandExecution/terminalInteraction\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/TerminalInteractionNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/fileChange/outputDeltaNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/fileChange/outputDeltaNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/fileChange/outputDelta\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FileChangeOutputDeltaNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ServerRequest/resolvedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"ServerRequest/resolvedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"serverRequest/resolved\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ServerRequestResolvedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/mcpToolCall/progressNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/mcpToolCall/progressNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/mcpToolCall/progress\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/McpToolCallProgressNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"McpServer/oauthLogin/completedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"McpServer/oauthLogin/completedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mcpServer/oauthLogin/completed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/McpServerOauthLoginCompletedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"McpServer/startupStatus/updatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"McpServer/startupStatus/updatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mcpServer/startupStatus/updated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/McpServerStatusUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Account/updatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Account/updatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"account/updated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/AccountUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Account/rateLimits/updatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Account/rateLimits/updatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"account/rateLimits/updated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/AccountRateLimitsUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"App/list/updatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"App/list/updatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"app/list/updated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/AppListUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Fs/changedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Fs/changedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fs/changed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FsChangedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/reasoning/summaryTextDeltaNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/reasoning/summaryTextDeltaNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/reasoning/summaryTextDelta\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningSummaryTextDeltaNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/reasoning/summaryPartAddedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/reasoning/summaryPartAddedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/reasoning/summaryPartAdded\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningSummaryPartAddedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Item/reasoning/textDeltaNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Item/reasoning/textDeltaNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"item/reasoning/textDelta\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningTextDeltaNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/compactedNotification\","]
#[doc = "      \"description\": \"Deprecated: Use `ContextCompaction` item type instead.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/compactedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/compacted\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ContextCompactedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Model/reroutedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Model/reroutedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"model/rerouted\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ModelReroutedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"DeprecationNoticeNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"DeprecationNoticeNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"deprecationNotice\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/DeprecationNoticeNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ConfigWarningNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"ConfigWarningNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"configWarning\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigWarningNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FuzzyFileSearch/sessionUpdatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"FuzzyFileSearch/sessionUpdatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fuzzyFileSearch/sessionUpdated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FuzzyFileSearchSessionUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FuzzyFileSearch/sessionCompletedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"FuzzyFileSearch/sessionCompletedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fuzzyFileSearch/sessionCompleted\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/FuzzyFileSearchSessionCompletedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/realtime/startedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/realtime/startedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/realtime/started\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadRealtimeStartedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/realtime/itemAddedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/realtime/itemAddedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/realtime/itemAdded\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadRealtimeItemAddedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/realtime/transcriptUpdatedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/realtime/transcriptUpdatedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/realtime/transcriptUpdated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadRealtimeTranscriptUpdatedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/realtime/outputAudio/deltaNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/realtime/outputAudio/deltaNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/realtime/outputAudio/delta\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadRealtimeOutputAudioDeltaNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/realtime/sdpNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/realtime/sdpNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/realtime/sdp\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadRealtimeSdpNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/realtime/errorNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/realtime/errorNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/realtime/error\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadRealtimeErrorNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Thread/realtime/closedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Thread/realtime/closedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"thread/realtime/closed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadRealtimeClosedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Windows/worldWritableWarningNotification\","]
#[doc = "      \"description\": \"Notifies the user of world-writable directories on Windows, which cannot be protected by the sandbox.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Windows/worldWritableWarningNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"windows/worldWritableWarning\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/WindowsWorldWritableWarningNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"WindowsSandbox/setupCompletedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"WindowsSandbox/setupCompletedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"windowsSandbox/setupCompleted\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/WindowsSandboxSetupCompletedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Account/login/completedNotification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"title\": \"Account/login/completedNotificationMethod\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"account/login/completed\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/definitions/AccountLoginCompletedNotification\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "method", content = "params")]
pub enum ServerNotification {
    #[doc = "ErrorNotification\n\nNEW NOTIFICATIONS"]
    #[serde(rename = "error")]
    Error(ErrorNotification),
    #[doc = "Thread/startedNotification"]
    #[serde(rename = "thread/started")]
    ThreadStarted(ThreadStartedNotification),
    #[doc = "Thread/status/changedNotification"]
    #[serde(rename = "thread/status/changed")]
    ThreadStatusChanged(ThreadStatusChangedNotification),
    #[doc = "Thread/archivedNotification"]
    #[serde(rename = "thread/archived")]
    ThreadArchived(ThreadArchivedNotification),
    #[doc = "Thread/unarchivedNotification"]
    #[serde(rename = "thread/unarchived")]
    ThreadUnarchived(ThreadUnarchivedNotification),
    #[doc = "Thread/closedNotification"]
    #[serde(rename = "thread/closed")]
    ThreadClosed(ThreadClosedNotification),
    #[doc = "Skills/changedNotification"]
    #[serde(rename = "skills/changed")]
    SkillsChanged(SkillsChangedNotification),
    #[doc = "Thread/name/updatedNotification"]
    #[serde(rename = "thread/name/updated")]
    ThreadNameUpdated(ThreadNameUpdatedNotification),
    #[doc = "Thread/tokenUsage/updatedNotification"]
    #[serde(rename = "thread/tokenUsage/updated")]
    ThreadTokenUsageUpdated(ThreadTokenUsageUpdatedNotification),
    #[doc = "Turn/startedNotification"]
    #[serde(rename = "turn/started")]
    TurnStarted(TurnStartedNotification),
    #[doc = "Hook/startedNotification"]
    #[serde(rename = "hook/started")]
    HookStarted(HookStartedNotification),
    #[doc = "Turn/completedNotification"]
    #[serde(rename = "turn/completed")]
    TurnCompleted(TurnCompletedNotification),
    #[doc = "Hook/completedNotification"]
    #[serde(rename = "hook/completed")]
    HookCompleted(HookCompletedNotification),
    #[doc = "Turn/diff/updatedNotification"]
    #[serde(rename = "turn/diff/updated")]
    TurnDiffUpdated(TurnDiffUpdatedNotification),
    #[doc = "Turn/plan/updatedNotification"]
    #[serde(rename = "turn/plan/updated")]
    TurnPlanUpdated(TurnPlanUpdatedNotification),
    #[doc = "Item/startedNotification"]
    #[serde(rename = "item/started")]
    ItemStarted(ItemStartedNotification),
    #[doc = "Item/autoApprovalReview/startedNotification"]
    #[serde(rename = "item/autoApprovalReview/started")]
    ItemAutoApprovalReviewStarted(ItemGuardianApprovalReviewStartedNotification),
    #[doc = "Item/autoApprovalReview/completedNotification"]
    #[serde(rename = "item/autoApprovalReview/completed")]
    ItemAutoApprovalReviewCompleted(ItemGuardianApprovalReviewCompletedNotification),
    #[doc = "Item/completedNotification"]
    #[serde(rename = "item/completed")]
    ItemCompleted(ItemCompletedNotification),
    #[doc = "Item/agentMessage/deltaNotification"]
    #[serde(rename = "item/agentMessage/delta")]
    ItemAgentMessageDelta(AgentMessageDeltaNotification),
    #[doc = "Item/plan/deltaNotification\n\nEXPERIMENTAL - proposed plan streaming deltas for plan items."]
    #[serde(rename = "item/plan/delta")]
    ItemPlanDelta(PlanDeltaNotification),
    #[doc = "Command/exec/outputDeltaNotification\n\nStream base64-encoded stdout/stderr chunks for a running `command/exec` session."]
    #[serde(rename = "command/exec/outputDelta")]
    CommandExecOutputDelta(CommandExecOutputDeltaNotification),
    #[doc = "Item/commandExecution/outputDeltaNotification"]
    #[serde(rename = "item/commandExecution/outputDelta")]
    ItemCommandExecutionOutputDelta(CommandExecutionOutputDeltaNotification),
    #[doc = "Item/commandExecution/terminalInteractionNotification"]
    #[serde(rename = "item/commandExecution/terminalInteraction")]
    ItemCommandExecutionTerminalInteraction(TerminalInteractionNotification),
    #[doc = "Item/fileChange/outputDeltaNotification"]
    #[serde(rename = "item/fileChange/outputDelta")]
    ItemFileChangeOutputDelta(FileChangeOutputDeltaNotification),
    #[doc = "ServerRequest/resolvedNotification"]
    #[serde(rename = "serverRequest/resolved")]
    ServerRequestResolved(ServerRequestResolvedNotification),
    #[doc = "Item/mcpToolCall/progressNotification"]
    #[serde(rename = "item/mcpToolCall/progress")]
    ItemMcpToolCallProgress(McpToolCallProgressNotification),
    #[doc = "McpServer/oauthLogin/completedNotification"]
    #[serde(rename = "mcpServer/oauthLogin/completed")]
    McpServerOauthLoginCompleted(McpServerOauthLoginCompletedNotification),
    #[doc = "McpServer/startupStatus/updatedNotification"]
    #[serde(rename = "mcpServer/startupStatus/updated")]
    McpServerStartupStatusUpdated(McpServerStatusUpdatedNotification),
    #[doc = "Account/updatedNotification"]
    #[serde(rename = "account/updated")]
    AccountUpdated(AccountUpdatedNotification),
    #[doc = "Account/rateLimits/updatedNotification"]
    #[serde(rename = "account/rateLimits/updated")]
    AccountRateLimitsUpdated(AccountRateLimitsUpdatedNotification),
    #[doc = "App/list/updatedNotification"]
    #[serde(rename = "app/list/updated")]
    AppListUpdated(AppListUpdatedNotification),
    #[doc = "Fs/changedNotification"]
    #[serde(rename = "fs/changed")]
    FsChanged(FsChangedNotification),
    #[doc = "Item/reasoning/summaryTextDeltaNotification"]
    #[serde(rename = "item/reasoning/summaryTextDelta")]
    ItemReasoningSummaryTextDelta(ReasoningSummaryTextDeltaNotification),
    #[doc = "Item/reasoning/summaryPartAddedNotification"]
    #[serde(rename = "item/reasoning/summaryPartAdded")]
    ItemReasoningSummaryPartAdded(ReasoningSummaryPartAddedNotification),
    #[doc = "Item/reasoning/textDeltaNotification"]
    #[serde(rename = "item/reasoning/textDelta")]
    ItemReasoningTextDelta(ReasoningTextDeltaNotification),
    #[doc = "Thread/compactedNotification\n\nDeprecated: Use `ContextCompaction` item type instead."]
    #[serde(rename = "thread/compacted")]
    ThreadCompacted(ContextCompactedNotification),
    #[doc = "Model/reroutedNotification"]
    #[serde(rename = "model/rerouted")]
    ModelRerouted(ModelReroutedNotification),
    #[doc = "DeprecationNoticeNotification"]
    #[serde(rename = "deprecationNotice")]
    DeprecationNotice(DeprecationNoticeNotification),
    #[doc = "ConfigWarningNotification"]
    #[serde(rename = "configWarning")]
    ConfigWarning(ConfigWarningNotification),
    #[doc = "FuzzyFileSearch/sessionUpdatedNotification"]
    #[serde(rename = "fuzzyFileSearch/sessionUpdated")]
    FuzzyFileSearchSessionUpdated(FuzzyFileSearchSessionUpdatedNotification),
    #[doc = "FuzzyFileSearch/sessionCompletedNotification"]
    #[serde(rename = "fuzzyFileSearch/sessionCompleted")]
    FuzzyFileSearchSessionCompleted(FuzzyFileSearchSessionCompletedNotification),
    #[doc = "Thread/realtime/startedNotification"]
    #[serde(rename = "thread/realtime/started")]
    ThreadRealtimeStarted(ThreadRealtimeStartedNotification),
    #[doc = "Thread/realtime/itemAddedNotification"]
    #[serde(rename = "thread/realtime/itemAdded")]
    ThreadRealtimeItemAdded(ThreadRealtimeItemAddedNotification),
    #[doc = "Thread/realtime/transcriptUpdatedNotification"]
    #[serde(rename = "thread/realtime/transcriptUpdated")]
    ThreadRealtimeTranscriptUpdated(ThreadRealtimeTranscriptUpdatedNotification),
    #[doc = "Thread/realtime/outputAudio/deltaNotification"]
    #[serde(rename = "thread/realtime/outputAudio/delta")]
    ThreadRealtimeOutputAudioDelta(ThreadRealtimeOutputAudioDeltaNotification),
    #[doc = "Thread/realtime/sdpNotification"]
    #[serde(rename = "thread/realtime/sdp")]
    ThreadRealtimeSdp(ThreadRealtimeSdpNotification),
    #[doc = "Thread/realtime/errorNotification"]
    #[serde(rename = "thread/realtime/error")]
    ThreadRealtimeError(ThreadRealtimeErrorNotification),
    #[doc = "Thread/realtime/closedNotification"]
    #[serde(rename = "thread/realtime/closed")]
    ThreadRealtimeClosed(ThreadRealtimeClosedNotification),
    #[doc = "Windows/worldWritableWarningNotification\n\nNotifies the user of world-writable directories on Windows, which cannot be protected by the sandbox."]
    #[serde(rename = "windows/worldWritableWarning")]
    WindowsWorldWritableWarning(WindowsWorldWritableWarningNotification),
    #[doc = "WindowsSandbox/setupCompletedNotification"]
    #[serde(rename = "windowsSandbox/setupCompleted")]
    WindowsSandboxSetupCompleted(WindowsSandboxSetupCompletedNotification),
    #[doc = "Account/login/completedNotification"]
    #[serde(rename = "account/login/completed")]
    AccountLoginCompleted(AccountLoginCompletedNotification),
}
impl ::std::convert::From<ErrorNotification> for ServerNotification {
    fn from(value: ErrorNotification) -> Self {
        Self::Error(value)
    }
}
impl ::std::convert::From<ThreadStartedNotification> for ServerNotification {
    fn from(value: ThreadStartedNotification) -> Self {
        Self::ThreadStarted(value)
    }
}
impl ::std::convert::From<ThreadStatusChangedNotification> for ServerNotification {
    fn from(value: ThreadStatusChangedNotification) -> Self {
        Self::ThreadStatusChanged(value)
    }
}
impl ::std::convert::From<ThreadArchivedNotification> for ServerNotification {
    fn from(value: ThreadArchivedNotification) -> Self {
        Self::ThreadArchived(value)
    }
}
impl ::std::convert::From<ThreadUnarchivedNotification> for ServerNotification {
    fn from(value: ThreadUnarchivedNotification) -> Self {
        Self::ThreadUnarchived(value)
    }
}
impl ::std::convert::From<ThreadClosedNotification> for ServerNotification {
    fn from(value: ThreadClosedNotification) -> Self {
        Self::ThreadClosed(value)
    }
}
impl ::std::convert::From<SkillsChangedNotification> for ServerNotification {
    fn from(value: SkillsChangedNotification) -> Self {
        Self::SkillsChanged(value)
    }
}
impl ::std::convert::From<ThreadNameUpdatedNotification> for ServerNotification {
    fn from(value: ThreadNameUpdatedNotification) -> Self {
        Self::ThreadNameUpdated(value)
    }
}
impl ::std::convert::From<ThreadTokenUsageUpdatedNotification> for ServerNotification {
    fn from(value: ThreadTokenUsageUpdatedNotification) -> Self {
        Self::ThreadTokenUsageUpdated(value)
    }
}
impl ::std::convert::From<TurnStartedNotification> for ServerNotification {
    fn from(value: TurnStartedNotification) -> Self {
        Self::TurnStarted(value)
    }
}
impl ::std::convert::From<HookStartedNotification> for ServerNotification {
    fn from(value: HookStartedNotification) -> Self {
        Self::HookStarted(value)
    }
}
impl ::std::convert::From<TurnCompletedNotification> for ServerNotification {
    fn from(value: TurnCompletedNotification) -> Self {
        Self::TurnCompleted(value)
    }
}
impl ::std::convert::From<HookCompletedNotification> for ServerNotification {
    fn from(value: HookCompletedNotification) -> Self {
        Self::HookCompleted(value)
    }
}
impl ::std::convert::From<TurnDiffUpdatedNotification> for ServerNotification {
    fn from(value: TurnDiffUpdatedNotification) -> Self {
        Self::TurnDiffUpdated(value)
    }
}
impl ::std::convert::From<TurnPlanUpdatedNotification> for ServerNotification {
    fn from(value: TurnPlanUpdatedNotification) -> Self {
        Self::TurnPlanUpdated(value)
    }
}
impl ::std::convert::From<ItemStartedNotification> for ServerNotification {
    fn from(value: ItemStartedNotification) -> Self {
        Self::ItemStarted(value)
    }
}
impl ::std::convert::From<ItemGuardianApprovalReviewStartedNotification> for ServerNotification {
    fn from(value: ItemGuardianApprovalReviewStartedNotification) -> Self {
        Self::ItemAutoApprovalReviewStarted(value)
    }
}
impl ::std::convert::From<ItemGuardianApprovalReviewCompletedNotification> for ServerNotification {
    fn from(value: ItemGuardianApprovalReviewCompletedNotification) -> Self {
        Self::ItemAutoApprovalReviewCompleted(value)
    }
}
impl ::std::convert::From<ItemCompletedNotification> for ServerNotification {
    fn from(value: ItemCompletedNotification) -> Self {
        Self::ItemCompleted(value)
    }
}
impl ::std::convert::From<AgentMessageDeltaNotification> for ServerNotification {
    fn from(value: AgentMessageDeltaNotification) -> Self {
        Self::ItemAgentMessageDelta(value)
    }
}
impl ::std::convert::From<PlanDeltaNotification> for ServerNotification {
    fn from(value: PlanDeltaNotification) -> Self {
        Self::ItemPlanDelta(value)
    }
}
impl ::std::convert::From<CommandExecOutputDeltaNotification> for ServerNotification {
    fn from(value: CommandExecOutputDeltaNotification) -> Self {
        Self::CommandExecOutputDelta(value)
    }
}
impl ::std::convert::From<CommandExecutionOutputDeltaNotification> for ServerNotification {
    fn from(value: CommandExecutionOutputDeltaNotification) -> Self {
        Self::ItemCommandExecutionOutputDelta(value)
    }
}
impl ::std::convert::From<TerminalInteractionNotification> for ServerNotification {
    fn from(value: TerminalInteractionNotification) -> Self {
        Self::ItemCommandExecutionTerminalInteraction(value)
    }
}
impl ::std::convert::From<FileChangeOutputDeltaNotification> for ServerNotification {
    fn from(value: FileChangeOutputDeltaNotification) -> Self {
        Self::ItemFileChangeOutputDelta(value)
    }
}
impl ::std::convert::From<ServerRequestResolvedNotification> for ServerNotification {
    fn from(value: ServerRequestResolvedNotification) -> Self {
        Self::ServerRequestResolved(value)
    }
}
impl ::std::convert::From<McpToolCallProgressNotification> for ServerNotification {
    fn from(value: McpToolCallProgressNotification) -> Self {
        Self::ItemMcpToolCallProgress(value)
    }
}
impl ::std::convert::From<McpServerOauthLoginCompletedNotification> for ServerNotification {
    fn from(value: McpServerOauthLoginCompletedNotification) -> Self {
        Self::McpServerOauthLoginCompleted(value)
    }
}
impl ::std::convert::From<McpServerStatusUpdatedNotification> for ServerNotification {
    fn from(value: McpServerStatusUpdatedNotification) -> Self {
        Self::McpServerStartupStatusUpdated(value)
    }
}
impl ::std::convert::From<AccountUpdatedNotification> for ServerNotification {
    fn from(value: AccountUpdatedNotification) -> Self {
        Self::AccountUpdated(value)
    }
}
impl ::std::convert::From<AccountRateLimitsUpdatedNotification> for ServerNotification {
    fn from(value: AccountRateLimitsUpdatedNotification) -> Self {
        Self::AccountRateLimitsUpdated(value)
    }
}
impl ::std::convert::From<AppListUpdatedNotification> for ServerNotification {
    fn from(value: AppListUpdatedNotification) -> Self {
        Self::AppListUpdated(value)
    }
}
impl ::std::convert::From<FsChangedNotification> for ServerNotification {
    fn from(value: FsChangedNotification) -> Self {
        Self::FsChanged(value)
    }
}
impl ::std::convert::From<ReasoningSummaryTextDeltaNotification> for ServerNotification {
    fn from(value: ReasoningSummaryTextDeltaNotification) -> Self {
        Self::ItemReasoningSummaryTextDelta(value)
    }
}
impl ::std::convert::From<ReasoningSummaryPartAddedNotification> for ServerNotification {
    fn from(value: ReasoningSummaryPartAddedNotification) -> Self {
        Self::ItemReasoningSummaryPartAdded(value)
    }
}
impl ::std::convert::From<ReasoningTextDeltaNotification> for ServerNotification {
    fn from(value: ReasoningTextDeltaNotification) -> Self {
        Self::ItemReasoningTextDelta(value)
    }
}
impl ::std::convert::From<ContextCompactedNotification> for ServerNotification {
    fn from(value: ContextCompactedNotification) -> Self {
        Self::ThreadCompacted(value)
    }
}
impl ::std::convert::From<ModelReroutedNotification> for ServerNotification {
    fn from(value: ModelReroutedNotification) -> Self {
        Self::ModelRerouted(value)
    }
}
impl ::std::convert::From<DeprecationNoticeNotification> for ServerNotification {
    fn from(value: DeprecationNoticeNotification) -> Self {
        Self::DeprecationNotice(value)
    }
}
impl ::std::convert::From<ConfigWarningNotification> for ServerNotification {
    fn from(value: ConfigWarningNotification) -> Self {
        Self::ConfigWarning(value)
    }
}
impl ::std::convert::From<FuzzyFileSearchSessionUpdatedNotification> for ServerNotification {
    fn from(value: FuzzyFileSearchSessionUpdatedNotification) -> Self {
        Self::FuzzyFileSearchSessionUpdated(value)
    }
}
impl ::std::convert::From<FuzzyFileSearchSessionCompletedNotification> for ServerNotification {
    fn from(value: FuzzyFileSearchSessionCompletedNotification) -> Self {
        Self::FuzzyFileSearchSessionCompleted(value)
    }
}
impl ::std::convert::From<ThreadRealtimeStartedNotification> for ServerNotification {
    fn from(value: ThreadRealtimeStartedNotification) -> Self {
        Self::ThreadRealtimeStarted(value)
    }
}
impl ::std::convert::From<ThreadRealtimeItemAddedNotification> for ServerNotification {
    fn from(value: ThreadRealtimeItemAddedNotification) -> Self {
        Self::ThreadRealtimeItemAdded(value)
    }
}
impl ::std::convert::From<ThreadRealtimeTranscriptUpdatedNotification> for ServerNotification {
    fn from(value: ThreadRealtimeTranscriptUpdatedNotification) -> Self {
        Self::ThreadRealtimeTranscriptUpdated(value)
    }
}
impl ::std::convert::From<ThreadRealtimeOutputAudioDeltaNotification> for ServerNotification {
    fn from(value: ThreadRealtimeOutputAudioDeltaNotification) -> Self {
        Self::ThreadRealtimeOutputAudioDelta(value)
    }
}
impl ::std::convert::From<ThreadRealtimeSdpNotification> for ServerNotification {
    fn from(value: ThreadRealtimeSdpNotification) -> Self {
        Self::ThreadRealtimeSdp(value)
    }
}
impl ::std::convert::From<ThreadRealtimeErrorNotification> for ServerNotification {
    fn from(value: ThreadRealtimeErrorNotification) -> Self {
        Self::ThreadRealtimeError(value)
    }
}
impl ::std::convert::From<ThreadRealtimeClosedNotification> for ServerNotification {
    fn from(value: ThreadRealtimeClosedNotification) -> Self {
        Self::ThreadRealtimeClosed(value)
    }
}
impl ::std::convert::From<WindowsWorldWritableWarningNotification> for ServerNotification {
    fn from(value: WindowsWorldWritableWarningNotification) -> Self {
        Self::WindowsWorldWritableWarning(value)
    }
}
impl ::std::convert::From<WindowsSandboxSetupCompletedNotification> for ServerNotification {
    fn from(value: WindowsSandboxSetupCompletedNotification) -> Self {
        Self::WindowsSandboxSetupCompleted(value)
    }
}
impl ::std::convert::From<AccountLoginCompletedNotification> for ServerNotification {
    fn from(value: AccountLoginCompletedNotification) -> Self {
        Self::AccountLoginCompleted(value)
    }
}
#[doc = "`ServerRequestResolvedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ServerRequestResolvedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"requestId\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"requestId\": {"]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ServerRequestResolvedNotification {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ServiceTier`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"fast\","]
#[doc = "    \"flex\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ServiceTier {
    #[serde(rename = "fast")]
    Fast,
    #[serde(rename = "flex")]
    Flex,
}
impl ::std::fmt::Display for ServiceTier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Fast => f.write_str("fast"),
            Self::Flex => f.write_str("flex"),
        }
    }
}
impl ::std::str::FromStr for ServiceTier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fast" => Ok(Self::Fast),
            "flex" => Ok(Self::Flex),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ServiceTier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ServiceTier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ServiceTier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SessionSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"cli\","]
#[doc = "        \"vscode\","]
#[doc = "        \"exec\","]
#[doc = "        \"appServer\","]
#[doc = "        \"unknown\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CustomSessionSource\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"custom\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"custom\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"SubAgentSessionSource\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"subAgent\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"subAgent\": {"]
#[doc = "          \"$ref\": \"#/definitions/SubAgentSource\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum SessionSource {
    #[serde(rename = "cli")]
    Cli,
    #[serde(rename = "vscode")]
    Vscode,
    #[serde(rename = "exec")]
    Exec,
    #[serde(rename = "appServer")]
    AppServer,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "custom")]
    Custom(::std::string::String),
    #[serde(rename = "subAgent")]
    SubAgent(SubAgentSource),
}
impl ::std::convert::From<SubAgentSource> for SessionSource {
    fn from(value: SubAgentSource) -> Self {
        Self::SubAgent(value)
    }
}
#[doc = "Settings for a collaboration mode."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Settings for a collaboration mode.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"model\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"developer_instructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reasoning_effort\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Settings {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub developer_instructions: ::std::option::Option<::std::string::String>,
    pub model: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<ReasoningEffort>,
}
#[doc = "`SkillDependencies`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"tools\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"tools\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SkillToolDependency\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillDependencies {
    pub tools: ::std::vec::Vec<SkillToolDependency>,
}
#[doc = "`SkillErrorInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillErrorInfo {
    pub message: ::std::string::String,
    pub path: ::std::string::String,
}
#[doc = "`SkillInterface`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"brandColor\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"defaultPrompt\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"displayName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"iconLarge\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"iconSmall\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"shortDescription\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillInterface {
    #[serde(
        rename = "brandColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub brand_color: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "defaultPrompt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_prompt: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "iconLarge",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_large: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "iconSmall",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_small: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_description: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for SkillInterface {
    fn default() -> Self {
        Self {
            brand_color: Default::default(),
            default_prompt: Default::default(),
            display_name: Default::default(),
            icon_large: Default::default(),
            icon_small: Default::default(),
            short_description: Default::default(),
        }
    }
}
#[doc = "`SkillMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"enabled\","]
#[doc = "    \"name\","]
#[doc = "    \"path\","]
#[doc = "    \"scope\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dependencies\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SkillDependencies\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"interface\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SkillInterface\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"$ref\": \"#/definitions/SkillScope\""]
#[doc = "    },"]
#[doc = "    \"shortDescription\": {"]
#[doc = "      \"description\": \"Legacy short_description from SKILL.md. Prefer SKILL.json interface.short_description.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dependencies: ::std::option::Option<SkillDependencies>,
    pub description: ::std::string::String,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub interface: ::std::option::Option<SkillInterface>,
    pub name: ::std::string::String,
    pub path: ::std::string::String,
    pub scope: SkillScope,
    #[doc = "Legacy short_description from SKILL.md. Prefer SKILL.json interface.short_description."]
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_description: ::std::option::Option<::std::string::String>,
}
#[doc = "`SkillScope`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"user\","]
#[doc = "    \"repo\","]
#[doc = "    \"system\","]
#[doc = "    \"admin\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SkillScope {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "repo")]
    Repo,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "admin")]
    Admin,
}
impl ::std::fmt::Display for SkillScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Repo => f.write_str("repo"),
            Self::System => f.write_str("system"),
            Self::Admin => f.write_str("admin"),
        }
    }
}
impl ::std::str::FromStr for SkillScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "repo" => Ok(Self::Repo),
            "system" => Ok(Self::System),
            "admin" => Ok(Self::Admin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SkillScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SkillScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SkillScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SkillSummary`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"enabled\","]
#[doc = "    \"name\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"interface\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SkillInterface\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"shortDescription\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillSummary {
    pub description: ::std::string::String,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub interface: ::std::option::Option<SkillInterface>,
    pub name: ::std::string::String,
    pub path: ::std::string::String,
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_description: ::std::option::Option<::std::string::String>,
}
#[doc = "`SkillToolDependency`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"transport\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillToolDependency {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub command: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transport: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    pub value: ::std::string::String,
}
#[doc = "Notification emitted when watched local skill files change.\n\nTreat this as an invalidation signal and re-run `skills/list` with the client's current parameters when refreshed skill metadata is needed."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SkillsChangedNotification\","]
#[doc = "  \"description\": \"Notification emitted when watched local skill files change.\\n\\nTreat this as an invalidation signal and re-run `skills/list` with the client's current parameters when refreshed skill metadata is needed.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SkillsChangedNotification(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for SkillsChangedNotification {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<SkillsChangedNotification>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: SkillsChangedNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for SkillsChangedNotification
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`SkillsConfigWriteParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SkillsConfigWriteParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"enabled\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Name-based selector.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"Path-based selector.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillsConfigWriteParams {
    pub enabled: bool,
    #[doc = "Name-based selector."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Path-based selector."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<AbsolutePathBuf>,
}
#[doc = "`SkillsConfigWriteResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SkillsConfigWriteResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"effectiveEnabled\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"effectiveEnabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillsConfigWriteResponse {
    #[serde(rename = "effectiveEnabled")]
    pub effective_enabled: bool,
}
#[doc = "`SkillsListEntry`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cwd\","]
#[doc = "    \"errors\","]
#[doc = "    \"skills\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"errors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SkillErrorInfo\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"skills\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SkillMetadata\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillsListEntry {
    pub cwd: ::std::string::String,
    pub errors: ::std::vec::Vec<SkillErrorInfo>,
    pub skills: ::std::vec::Vec<SkillMetadata>,
}
#[doc = "`SkillsListExtraRootsForCwd`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cwd\","]
#[doc = "    \"extraUserRoots\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extraUserRoots\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillsListExtraRootsForCwd {
    pub cwd: ::std::string::String,
    #[serde(rename = "extraUserRoots")]
    pub extra_user_roots: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`SkillsListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SkillsListParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwds\": {"]
#[doc = "      \"description\": \"When empty, defaults to the current session working directory.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"forceReload\": {"]
#[doc = "      \"description\": \"When true, bypass the skills cache and re-scan skills from disk.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"perCwdExtraUserRoots\": {"]
#[doc = "      \"description\": \"Optional per-cwd extra roots to scan as user-scoped skills.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SkillsListExtraRootsForCwd\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillsListParams {
    #[doc = "When empty, defaults to the current session working directory."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub cwds: ::std::vec::Vec<::std::string::String>,
    #[doc = "When true, bypass the skills cache and re-scan skills from disk."]
    #[serde(
        rename = "forceReload",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub force_reload: ::std::option::Option<bool>,
    #[doc = "Optional per-cwd extra roots to scan as user-scoped skills."]
    #[serde(
        rename = "perCwdExtraUserRoots",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub per_cwd_extra_user_roots:
        ::std::option::Option<::std::vec::Vec<SkillsListExtraRootsForCwd>>,
}
impl ::std::default::Default for SkillsListParams {
    fn default() -> Self {
        Self {
            cwds: Default::default(),
            force_reload: Default::default(),
            per_cwd_extra_user_roots: Default::default(),
        }
    }
}
#[doc = "`SkillsListResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SkillsListResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SkillsListEntry\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SkillsListResponse {
    pub data: ::std::vec::Vec<SkillsListEntry>,
}
#[doc = "`SubAgentSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"review\","]
#[doc = "        \"compact\","]
#[doc = "        \"memory_consolidation\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ThreadSpawnSubAgentSource\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"thread_spawn\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"thread_spawn\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"depth\","]
#[doc = "            \"parent_thread_id\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"agent_nickname\": {"]
#[doc = "              \"default\": null,"]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"agent_path\": {"]
#[doc = "              \"default\": null,"]
#[doc = "              \"anyOf\": ["]
#[doc = "                {"]
#[doc = "                  \"$ref\": \"#/definitions/AgentPath\""]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"type\": \"null\""]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"agent_role\": {"]
#[doc = "              \"default\": null,"]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"depth\": {"]
#[doc = "              \"type\": \"integer\","]
#[doc = "              \"format\": \"int32\""]
#[doc = "            },"]
#[doc = "            \"parent_thread_id\": {"]
#[doc = "              \"$ref\": \"#/definitions/ThreadId\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"OtherSubAgentSource\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"other\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"other\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum SubAgentSource {
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "memory_consolidation")]
    MemoryConsolidation,
    #[serde(rename = "thread_spawn")]
    ThreadSpawn {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        agent_nickname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        agent_path: ::std::option::Option<AgentPath>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        agent_role: ::std::option::Option<::std::string::String>,
        depth: i32,
        parent_thread_id: ThreadId,
    },
    #[serde(rename = "other")]
    Other(::std::string::String),
}
#[doc = "`TerminalInteractionNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TerminalInteractionNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"itemId\","]
#[doc = "    \"processId\","]
#[doc = "    \"stdin\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"processId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stdin\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TerminalInteractionNotification {
    #[serde(rename = "itemId")]
    pub item_id: ::std::string::String,
    #[serde(rename = "processId")]
    pub process_id: ::std::string::String,
    pub stdin: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`TextElement`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"byteRange\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"byteRange\": {"]
#[doc = "      \"description\": \"Byte range in the parent `text` buffer that this element occupies.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ByteRange\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"placeholder\": {"]
#[doc = "      \"description\": \"Optional human-readable placeholder for the element, displayed in the UI.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextElement {
    #[doc = "Byte range in the parent `text` buffer that this element occupies."]
    #[serde(rename = "byteRange")]
    pub byte_range: ByteRange,
    #[doc = "Optional human-readable placeholder for the element, displayed in the UI."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub placeholder: ::std::option::Option<::std::string::String>,
}
#[doc = "`TextPosition`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"column\","]
#[doc = "    \"line\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"column\": {"]
#[doc = "      \"description\": \"1-based column number (in Unicode scalar values).\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"line\": {"]
#[doc = "      \"description\": \"1-based line number.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextPosition {
    #[doc = "1-based column number (in Unicode scalar values)."]
    pub column: u32,
    #[doc = "1-based line number."]
    pub line: u32,
}
#[doc = "`TextRange`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"end\","]
#[doc = "    \"start\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/definitions/TextPosition\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/definitions/TextPosition\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextRange {
    pub end: TextPosition,
    pub start: TextPosition,
}
#[doc = "`Thread`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cliVersion\","]
#[doc = "    \"createdAt\","]
#[doc = "    \"cwd\","]
#[doc = "    \"ephemeral\","]
#[doc = "    \"id\","]
#[doc = "    \"modelProvider\","]
#[doc = "    \"preview\","]
#[doc = "    \"source\","]
#[doc = "    \"status\","]
#[doc = "    \"turns\","]
#[doc = "    \"updatedAt\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agentNickname\": {"]
#[doc = "      \"description\": \"Optional random unique nickname assigned to an AgentControl-spawned sub-agent.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agentRole\": {"]
#[doc = "      \"description\": \"Optional role (agent_role) assigned to an AgentControl-spawned sub-agent.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cliVersion\": {"]
#[doc = "      \"description\": \"Version of the CLI that created the thread.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"createdAt\": {"]
#[doc = "      \"description\": \"Unix timestamp (in seconds) when the thread was created.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"description\": \"Working directory captured for the thread.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ephemeral\": {"]
#[doc = "      \"description\": \"Whether the thread is ephemeral and should not be materialized on disk.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"forkedFromId\": {"]
#[doc = "      \"description\": \"Source thread id when this thread was created by forking another thread.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"gitInfo\": {"]
#[doc = "      \"description\": \"Optional Git metadata captured when the thread was created.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/GitInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modelProvider\": {"]
#[doc = "      \"description\": \"Model provider used for this thread (for example, 'openai').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Optional user-facing thread title.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"[UNSTABLE] Path to the thread on disk.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"preview\": {"]
#[doc = "      \"description\": \"Usually the first user message in the thread, if available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"description\": \"Origin of the thread (CLI, VSCode, codex exec, codex app-server, etc.).\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SessionSource\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Current runtime status for the thread.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadStatus\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"turns\": {"]
#[doc = "      \"description\": \"Only populated on `thread/resume`, `thread/rollback`, `thread/fork`, and `thread/read` (when `includeTurns` is true) responses. For all other responses and notifications returning a Thread, the turns field will be an empty list.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Turn\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"updatedAt\": {"]
#[doc = "      \"description\": \"Unix timestamp (in seconds) when the thread was last updated.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Thread {
    #[doc = "Optional random unique nickname assigned to an AgentControl-spawned sub-agent."]
    #[serde(
        rename = "agentNickname",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_nickname: ::std::option::Option<::std::string::String>,
    #[doc = "Optional role (agent_role) assigned to an AgentControl-spawned sub-agent."]
    #[serde(
        rename = "agentRole",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_role: ::std::option::Option<::std::string::String>,
    #[doc = "Version of the CLI that created the thread."]
    #[serde(rename = "cliVersion")]
    pub cli_version: ::std::string::String,
    #[doc = "Unix timestamp (in seconds) when the thread was created."]
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[doc = "Working directory captured for the thread."]
    pub cwd: ::std::string::String,
    #[doc = "Whether the thread is ephemeral and should not be materialized on disk."]
    pub ephemeral: bool,
    #[doc = "Source thread id when this thread was created by forking another thread."]
    #[serde(
        rename = "forkedFromId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub forked_from_id: ::std::option::Option<::std::string::String>,
    #[doc = "Optional Git metadata captured when the thread was created."]
    #[serde(
        rename = "gitInfo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub git_info: ::std::option::Option<GitInfo>,
    pub id: ::std::string::String,
    #[doc = "Model provider used for this thread (for example, 'openai')."]
    #[serde(rename = "modelProvider")]
    pub model_provider: ::std::string::String,
    #[doc = "Optional user-facing thread title."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "[UNSTABLE] Path to the thread on disk."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[doc = "Usually the first user message in the thread, if available."]
    pub preview: ::std::string::String,
    #[doc = "Origin of the thread (CLI, VSCode, codex exec, codex app-server, etc.)."]
    pub source: SessionSource,
    #[doc = "Current runtime status for the thread."]
    pub status: ThreadStatus,
    #[doc = "Only populated on `thread/resume`, `thread/rollback`, `thread/fork`, and `thread/read` (when `includeTurns` is true) responses. For all other responses and notifications returning a Thread, the turns field will be an empty list."]
    pub turns: ::std::vec::Vec<Turn>,
    #[doc = "Unix timestamp (in seconds) when the thread was last updated."]
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}
#[doc = "`ThreadActiveFlag`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"waitingOnApproval\","]
#[doc = "    \"waitingOnUserInput\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadActiveFlag {
    #[serde(rename = "waitingOnApproval")]
    WaitingOnApproval,
    #[serde(rename = "waitingOnUserInput")]
    WaitingOnUserInput,
}
impl ::std::fmt::Display for ThreadActiveFlag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WaitingOnApproval => f.write_str("waitingOnApproval"),
            Self::WaitingOnUserInput => f.write_str("waitingOnUserInput"),
        }
    }
}
impl ::std::str::FromStr for ThreadActiveFlag {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "waitingOnApproval" => Ok(Self::WaitingOnApproval),
            "waitingOnUserInput" => Ok(Self::WaitingOnUserInput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadActiveFlag {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadActiveFlag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadActiveFlag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ThreadArchiveParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadArchiveParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadArchiveParams {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadArchiveResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadArchiveResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ThreadArchiveResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for ThreadArchiveResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ThreadArchiveResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: ThreadArchiveResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for ThreadArchiveResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`ThreadArchivedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadArchivedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadArchivedNotification {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadClosedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadClosedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadClosedNotification {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadCompactStartParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadCompactStartParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadCompactStartParams {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadCompactStartResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadCompactStartResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ThreadCompactStartResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for ThreadCompactStartResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ThreadCompactStartResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: ThreadCompactStartResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for ThreadCompactStartResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "There are two ways to fork a thread: 1. By thread_id: load the thread from disk by thread_id and fork it into a new thread. 2. By path: load the thread from disk by path and fork it into a new thread.\n\nIf using path, the thread_id param will be ignored.\n\nPrefer using thread_id whenever possible."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadForkParams\","]
#[doc = "  \"description\": \"There are two ways to fork a thread: 1. By thread_id: load the thread from disk by thread_id and fork it into a new thread. 2. By path: load the thread from disk by path and fork it into a new thread.\\n\\nIf using path, the thread_id param will be ignored.\\n\\nPrefer using thread_id whenever possible.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvalPolicy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"approvalsReviewer\": {"]
#[doc = "      \"description\": \"Override where approval requests are routed for review on this thread and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"baseInstructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"config\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"developerInstructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ephemeral\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"description\": \"Configuration overrides for the forked thread, if any.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modelProvider\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandbox\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SandboxMode\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"serviceTier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadForkParams {
    #[serde(
        rename = "approvalPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approval_policy: ::std::option::Option<AskForApproval>,
    #[doc = "Override where approval requests are routed for review on this thread and subsequent turns."]
    #[serde(
        rename = "approvalsReviewer",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approvals_reviewer: ::std::option::Option<ApprovalsReviewer>,
    #[serde(
        rename = "baseInstructions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_instructions: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub config:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "developerInstructions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub developer_instructions: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ephemeral: ::std::option::Option<bool>,
    #[doc = "Configuration overrides for the forked thread, if any."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modelProvider",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_provider: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sandbox: ::std::option::Option<SandboxMode>,
    #[serde(
        rename = "serviceTier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadForkResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadForkResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"approvalPolicy\","]
#[doc = "    \"approvalsReviewer\","]
#[doc = "    \"cwd\","]
#[doc = "    \"model\","]
#[doc = "    \"modelProvider\","]
#[doc = "    \"sandbox\","]
#[doc = "    \"thread\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvalPolicy\": {"]
#[doc = "      \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "    },"]
#[doc = "    \"approvalsReviewer\": {"]
#[doc = "      \"description\": \"Reviewer currently used for approval requests on this thread.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modelProvider\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reasoningEffort\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandbox\": {"]
#[doc = "      \"$ref\": \"#/definitions/SandboxPolicy\""]
#[doc = "    },"]
#[doc = "    \"serviceTier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"thread\": {"]
#[doc = "      \"$ref\": \"#/definitions/Thread\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadForkResponse {
    #[serde(rename = "approvalPolicy")]
    pub approval_policy: AskForApproval,
    #[doc = "Reviewer currently used for approval requests on this thread."]
    #[serde(rename = "approvalsReviewer")]
    pub approvals_reviewer: ApprovalsReviewer,
    pub cwd: ::std::string::String,
    pub model: ::std::string::String,
    #[serde(rename = "modelProvider")]
    pub model_provider: ::std::string::String,
    #[serde(
        rename = "reasoningEffort",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reasoning_effort: ::std::option::Option<ReasoningEffort>,
    pub sandbox: SandboxPolicy,
    #[serde(
        rename = "serviceTier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_tier: ::std::option::Option<ServiceTier>,
    pub thread: Thread,
}
#[doc = "`ThreadId`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct ThreadId(pub ::std::string::String);
impl ::std::ops::Deref for ThreadId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ThreadId> for ::std::string::String {
    fn from(value: ThreadId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ThreadId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ThreadId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for ThreadId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`ThreadItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"UserMessageThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"content\","]
#[doc = "        \"id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"content\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/UserInput\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"UserMessageThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"userMessage\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"HookPromptThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"fragments\","]
#[doc = "        \"id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"fragments\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/HookPromptFragment\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"HookPromptThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"hookPrompt\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"AgentMessageThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"memoryCitation\": {"]
#[doc = "          \"default\": null,"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/MemoryCitation\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"phase\": {"]
#[doc = "          \"default\": null,"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/MessagePhase\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"AgentMessageThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"agentMessage\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"PlanThreadItem\","]
#[doc = "      \"description\": \"EXPERIMENTAL - proposed plan item content. The completed plan item is authoritative and may not match the concatenation of `PlanDelta` text.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"PlanThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"plan\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ReasoningThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"content\": {"]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"summary\": {"]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ReasoningThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"reasoning\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CommandExecutionThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"command\","]
#[doc = "        \"commandActions\","]
#[doc = "        \"cwd\","]
#[doc = "        \"id\","]
#[doc = "        \"status\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"aggregatedOutput\": {"]
#[doc = "          \"description\": \"The command's output, aggregated from stdout and stderr.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"command\": {"]
#[doc = "          \"description\": \"The command to be executed.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"commandActions\": {"]
#[doc = "          \"description\": \"A best-effort parsing of the command to understand the action(s) it will perform. This returns a list of CommandAction objects because a single shell command may be composed of many commands piped together.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/CommandAction\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"cwd\": {"]
#[doc = "          \"description\": \"The command's working directory.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"durationMs\": {"]
#[doc = "          \"description\": \"The duration of the command execution in milliseconds.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"int64\""]
#[doc = "        },"]
#[doc = "        \"exitCode\": {"]
#[doc = "          \"description\": \"The command's exit code.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"int32\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"processId\": {"]
#[doc = "          \"description\": \"Identifier for the underlying PTY process (when available).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"default\": \"agent\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/CommandExecutionSource\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"$ref\": \"#/definitions/CommandExecutionStatus\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"CommandExecutionThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"commandExecution\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FileChangeThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"changes\","]
#[doc = "        \"id\","]
#[doc = "        \"status\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"changes\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/FileUpdateChange\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"$ref\": \"#/definitions/PatchApplyStatus\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"FileChangeThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"fileChange\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"McpToolCallThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"arguments\","]
#[doc = "        \"id\","]
#[doc = "        \"server\","]
#[doc = "        \"status\","]
#[doc = "        \"tool\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": true,"]
#[doc = "        \"durationMs\": {"]
#[doc = "          \"description\": \"The duration of the MCP tool call in milliseconds.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"int64\""]
#[doc = "        },"]
#[doc = "        \"error\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/McpToolCallError\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"result\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/McpToolCallResult\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"server\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"$ref\": \"#/definitions/McpToolCallStatus\""]
#[doc = "        },"]
#[doc = "        \"tool\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"McpToolCallThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mcpToolCall\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"DynamicToolCallThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"arguments\","]
#[doc = "        \"id\","]
#[doc = "        \"status\","]
#[doc = "        \"tool\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": true,"]
#[doc = "        \"contentItems\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"array\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/DynamicToolCallOutputContentItem\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"durationMs\": {"]
#[doc = "          \"description\": \"The duration of the dynamic tool call in milliseconds.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"int64\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"$ref\": \"#/definitions/DynamicToolCallStatus\""]
#[doc = "        },"]
#[doc = "        \"success\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tool\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"DynamicToolCallThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"dynamicToolCall\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"CollabAgentToolCallThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"agentsStates\","]
#[doc = "        \"id\","]
#[doc = "        \"receiverThreadIds\","]
#[doc = "        \"senderThreadId\","]
#[doc = "        \"status\","]
#[doc = "        \"tool\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agentsStates\": {"]
#[doc = "          \"description\": \"Last known status of the target agents, when available.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"$ref\": \"#/definitions/CollabAgentState\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"Unique identifier for this collab tool call.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"model\": {"]
#[doc = "          \"description\": \"Model requested for the spawned agent, when applicable.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"prompt\": {"]
#[doc = "          \"description\": \"Prompt text sent as part of the collab tool call, when available.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"reasoningEffort\": {"]
#[doc = "          \"description\": \"Reasoning effort requested for the spawned agent, when applicable.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"receiverThreadIds\": {"]
#[doc = "          \"description\": \"Thread ID of the receiving agent, when applicable. In case of spawn operation, this corresponds to the newly spawned agent.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"senderThreadId\": {"]
#[doc = "          \"description\": \"Thread ID of the agent issuing the collab request.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"description\": \"Current status of the collab tool call.\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/CollabAgentToolCallStatus\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tool\": {"]
#[doc = "          \"description\": \"Name of the collab tool that was invoked.\","]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/CollabAgentTool\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"CollabAgentToolCallThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"collabAgentToolCall\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"WebSearchThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"query\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/WebSearchAction\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"query\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"WebSearchThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"webSearch\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ImageViewThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"path\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ImageViewThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"imageView\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ImageGenerationThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"result\","]
#[doc = "        \"status\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"result\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"revisedPrompt\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"savedPath\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ImageGenerationThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"imageGeneration\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"EnteredReviewModeThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"review\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"review\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"EnteredReviewModeThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"enteredReviewMode\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ExitedReviewModeThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"review\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"review\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ExitedReviewModeThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"exitedReviewMode\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ContextCompactionThreadItem\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ContextCompactionThreadItemType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"contextCompaction\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ThreadItem {
    #[doc = "UserMessageThreadItem"]
    #[serde(rename = "userMessage")]
    UserMessage {
        content: ::std::vec::Vec<UserInput>,
        id: ::std::string::String,
    },
    #[doc = "HookPromptThreadItem"]
    #[serde(rename = "hookPrompt")]
    HookPrompt {
        fragments: ::std::vec::Vec<HookPromptFragment>,
        id: ::std::string::String,
    },
    #[doc = "AgentMessageThreadItem"]
    #[serde(rename = "agentMessage")]
    AgentMessage {
        id: ::std::string::String,
        #[serde(
            rename = "memoryCitation",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        memory_citation: ::std::option::Option<MemoryCitation>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        phase: ::std::option::Option<MessagePhase>,
        text: ::std::string::String,
    },
    #[doc = "PlanThreadItem\n\nEXPERIMENTAL - proposed plan item content. The completed plan item is authoritative and may not match the concatenation of `PlanDelta` text."]
    #[serde(rename = "plan")]
    Plan {
        id: ::std::string::String,
        text: ::std::string::String,
    },
    #[doc = "ReasoningThreadItem"]
    #[serde(rename = "reasoning")]
    Reasoning {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        content: ::std::vec::Vec<::std::string::String>,
        id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        summary: ::std::vec::Vec<::std::string::String>,
    },
    #[doc = "CommandExecutionThreadItem"]
    #[serde(rename = "commandExecution")]
    CommandExecution {
        #[doc = "The command's output, aggregated from stdout and stderr."]
        #[serde(
            rename = "aggregatedOutput",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        aggregated_output: ::std::option::Option<::std::string::String>,
        #[doc = "The command to be executed."]
        command: ::std::string::String,
        #[doc = "A best-effort parsing of the command to understand the action(s) it will perform. This returns a list of CommandAction objects because a single shell command may be composed of many commands piped together."]
        #[serde(rename = "commandActions")]
        command_actions: ::std::vec::Vec<CommandAction>,
        #[doc = "The command's working directory."]
        cwd: ::std::string::String,
        #[doc = "The duration of the command execution in milliseconds."]
        #[serde(
            rename = "durationMs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        duration_ms: ::std::option::Option<i64>,
        #[doc = "The command's exit code."]
        #[serde(
            rename = "exitCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        exit_code: ::std::option::Option<i32>,
        id: ::std::string::String,
        #[doc = "Identifier for the underlying PTY process (when available)."]
        #[serde(
            rename = "processId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        process_id: ::std::option::Option<::std::string::String>,
        #[serde(default = "defaults::thread_item_command_execution_source")]
        source: CommandExecutionSource,
        status: CommandExecutionStatus,
    },
    #[doc = "FileChangeThreadItem"]
    #[serde(rename = "fileChange")]
    FileChange {
        changes: ::std::vec::Vec<FileUpdateChange>,
        id: ::std::string::String,
        status: PatchApplyStatus,
    },
    #[doc = "McpToolCallThreadItem"]
    #[serde(rename = "mcpToolCall")]
    McpToolCall {
        arguments: ::serde_json::Value,
        #[doc = "The duration of the MCP tool call in milliseconds."]
        #[serde(
            rename = "durationMs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        duration_ms: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        error: ::std::option::Option<McpToolCallError>,
        id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        result: ::std::option::Option<McpToolCallResult>,
        server: ::std::string::String,
        status: McpToolCallStatus,
        tool: ::std::string::String,
    },
    #[doc = "DynamicToolCallThreadItem"]
    #[serde(rename = "dynamicToolCall")]
    DynamicToolCall {
        arguments: ::serde_json::Value,
        #[serde(
            rename = "contentItems",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        content_items: ::std::option::Option<::std::vec::Vec<DynamicToolCallOutputContentItem>>,
        #[doc = "The duration of the dynamic tool call in milliseconds."]
        #[serde(
            rename = "durationMs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        duration_ms: ::std::option::Option<i64>,
        id: ::std::string::String,
        status: DynamicToolCallStatus,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        success: ::std::option::Option<bool>,
        tool: ::std::string::String,
    },
    #[doc = "CollabAgentToolCallThreadItem"]
    #[serde(rename = "collabAgentToolCall")]
    CollabAgentToolCall {
        #[doc = "Last known status of the target agents, when available."]
        #[serde(rename = "agentsStates")]
        agents_states: ::std::collections::HashMap<::std::string::String, CollabAgentState>,
        #[doc = "Unique identifier for this collab tool call."]
        id: ::std::string::String,
        #[doc = "Model requested for the spawned agent, when applicable."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        model: ::std::option::Option<::std::string::String>,
        #[doc = "Prompt text sent as part of the collab tool call, when available."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        prompt: ::std::option::Option<::std::string::String>,
        #[doc = "Reasoning effort requested for the spawned agent, when applicable."]
        #[serde(
            rename = "reasoningEffort",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        reasoning_effort: ::std::option::Option<ReasoningEffort>,
        #[doc = "Thread ID of the receiving agent, when applicable. In case of spawn operation, this corresponds to the newly spawned agent."]
        #[serde(rename = "receiverThreadIds")]
        receiver_thread_ids: ::std::vec::Vec<::std::string::String>,
        #[doc = "Thread ID of the agent issuing the collab request."]
        #[serde(rename = "senderThreadId")]
        sender_thread_id: ::std::string::String,
        #[doc = "Current status of the collab tool call."]
        status: CollabAgentToolCallStatus,
        #[doc = "Name of the collab tool that was invoked."]
        tool: CollabAgentTool,
    },
    #[doc = "WebSearchThreadItem"]
    #[serde(rename = "webSearch")]
    WebSearch {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        action: ::std::option::Option<WebSearchAction>,
        id: ::std::string::String,
        query: ::std::string::String,
    },
    #[doc = "ImageViewThreadItem"]
    #[serde(rename = "imageView")]
    ImageView {
        id: ::std::string::String,
        path: ::std::string::String,
    },
    #[doc = "ImageGenerationThreadItem"]
    #[serde(rename = "imageGeneration")]
    ImageGeneration {
        id: ::std::string::String,
        result: ::std::string::String,
        #[serde(
            rename = "revisedPrompt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        revised_prompt: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "savedPath",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        saved_path: ::std::option::Option<::std::string::String>,
        status: ::std::string::String,
    },
    #[doc = "EnteredReviewModeThreadItem"]
    #[serde(rename = "enteredReviewMode")]
    EnteredReviewMode {
        id: ::std::string::String,
        review: ::std::string::String,
    },
    #[doc = "ExitedReviewModeThreadItem"]
    #[serde(rename = "exitedReviewMode")]
    ExitedReviewMode {
        id: ::std::string::String,
        review: ::std::string::String,
    },
    #[doc = "ContextCompactionThreadItem"]
    #[serde(rename = "contextCompaction")]
    ContextCompaction { id: ::std::string::String },
}
#[doc = "`ThreadListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadListParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"archived\": {"]
#[doc = "      \"description\": \"Optional archived filter; when set to true, only archived threads are returned. If false or null, only non-archived threads are returned.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"Opaque pagination cursor returned by a previous call.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"description\": \"Optional cwd filter; when set, only threads whose session cwd exactly matches this path are returned.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limit\": {"]
#[doc = "      \"description\": \"Optional page size; defaults to a reasonable server-side value.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"modelProviders\": {"]
#[doc = "      \"description\": \"Optional provider filter; when set, only sessions recorded under these providers are returned. When present but empty, includes all providers.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"searchTerm\": {"]
#[doc = "      \"description\": \"Optional substring filter for the extracted thread title.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sortKey\": {"]
#[doc = "      \"description\": \"Optional sort key; defaults to created_at.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadSortKey\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sourceKinds\": {"]
#[doc = "      \"description\": \"Optional source filter; when set, only sessions from these source kinds are returned. When omitted or empty, defaults to interactive sources.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ThreadSourceKind\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadListParams {
    #[doc = "Optional archived filter; when set to true, only archived threads are returned. If false or null, only non-archived threads are returned."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub archived: ::std::option::Option<bool>,
    #[doc = "Opaque pagination cursor returned by a previous call."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[doc = "Optional cwd filter; when set, only threads whose session cwd exactly matches this path are returned."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[doc = "Optional page size; defaults to a reasonable server-side value."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<u32>,
    #[doc = "Optional provider filter; when set, only sessions recorded under these providers are returned. When present but empty, includes all providers."]
    #[serde(
        rename = "modelProviders",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_providers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "Optional substring filter for the extracted thread title."]
    #[serde(
        rename = "searchTerm",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub search_term: ::std::option::Option<::std::string::String>,
    #[doc = "Optional sort key; defaults to created_at."]
    #[serde(
        rename = "sortKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sort_key: ::std::option::Option<ThreadSortKey>,
    #[doc = "Optional source filter; when set, only sessions from these source kinds are returned. When omitted or empty, defaults to interactive sources."]
    #[serde(
        rename = "sourceKinds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_kinds: ::std::option::Option<::std::vec::Vec<ThreadSourceKind>>,
}
impl ::std::default::Default for ThreadListParams {
    fn default() -> Self {
        Self {
            archived: Default::default(),
            cursor: Default::default(),
            cwd: Default::default(),
            limit: Default::default(),
            model_providers: Default::default(),
            search_term: Default::default(),
            sort_key: Default::default(),
            source_kinds: Default::default(),
        }
    }
}
#[doc = "`ThreadListResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadListResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Thread\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"Opaque cursor to pass to the next call to continue after the last item. if None, there are no more items to return.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadListResponse {
    pub data: ::std::vec::Vec<Thread>,
    #[doc = "Opaque cursor to pass to the next call to continue after the last item. if None, there are no more items to return."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
}
#[doc = "`ThreadLoadedListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadLoadedListParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"Opaque pagination cursor returned by a previous call.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limit\": {"]
#[doc = "      \"description\": \"Optional page size; defaults to no limit.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadLoadedListParams {
    #[doc = "Opaque pagination cursor returned by a previous call."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[doc = "Optional page size; defaults to no limit."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<u32>,
}
impl ::std::default::Default for ThreadLoadedListParams {
    fn default() -> Self {
        Self {
            cursor: Default::default(),
            limit: Default::default(),
        }
    }
}
#[doc = "`ThreadLoadedListResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadLoadedListResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"Thread ids for sessions currently loaded in memory.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"Opaque cursor to pass to the next call to continue after the last item. if None, there are no more items to return.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadLoadedListResponse {
    #[doc = "Thread ids for sessions currently loaded in memory."]
    pub data: ::std::vec::Vec<::std::string::String>,
    #[doc = "Opaque cursor to pass to the next call to continue after the last item. if None, there are no more items to return."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
}
#[doc = "`ThreadMetadataGitInfoUpdateParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"branch\": {"]
#[doc = "      \"description\": \"Omit to leave the stored branch unchanged, set to `null` to clear it, or provide a non-empty string to replace it.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"originUrl\": {"]
#[doc = "      \"description\": \"Omit to leave the stored origin URL unchanged, set to `null` to clear it, or provide a non-empty string to replace it.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sha\": {"]
#[doc = "      \"description\": \"Omit to leave the stored commit unchanged, set to `null` to clear it, or provide a non-empty string to replace it.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadMetadataGitInfoUpdateParams {
    #[doc = "Omit to leave the stored branch unchanged, set to `null` to clear it, or provide a non-empty string to replace it."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<::std::string::String>,
    #[doc = "Omit to leave the stored origin URL unchanged, set to `null` to clear it, or provide a non-empty string to replace it."]
    #[serde(
        rename = "originUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub origin_url: ::std::option::Option<::std::string::String>,
    #[doc = "Omit to leave the stored commit unchanged, set to `null` to clear it, or provide a non-empty string to replace it."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sha: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ThreadMetadataGitInfoUpdateParams {
    fn default() -> Self {
        Self {
            branch: Default::default(),
            origin_url: Default::default(),
            sha: Default::default(),
        }
    }
}
#[doc = "`ThreadMetadataUpdateParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadMetadataUpdateParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"gitInfo\": {"]
#[doc = "      \"description\": \"Patch the stored Git metadata for this thread. Omit a field to leave it unchanged, set it to `null` to clear it, or provide a string to replace the stored value.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadMetadataGitInfoUpdateParams\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadMetadataUpdateParams {
    #[doc = "Patch the stored Git metadata for this thread. Omit a field to leave it unchanged, set it to `null` to clear it, or provide a string to replace the stored value."]
    #[serde(
        rename = "gitInfo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub git_info: ::std::option::Option<ThreadMetadataGitInfoUpdateParams>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadMetadataUpdateResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadMetadataUpdateResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"thread\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"thread\": {"]
#[doc = "      \"$ref\": \"#/definitions/Thread\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadMetadataUpdateResponse {
    pub thread: Thread,
}
#[doc = "`ThreadNameUpdatedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadNameUpdatedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadNameUpdatedNotification {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(
        rename = "threadName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub thread_name: ::std::option::Option<::std::string::String>,
}
#[doc = "`ThreadReadParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadReadParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"includeTurns\": {"]
#[doc = "      \"description\": \"When true, include turns and their items from rollout history.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadReadParams {
    #[doc = "When true, include turns and their items from rollout history."]
    #[serde(rename = "includeTurns", default)]
    pub include_turns: bool,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadReadResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadReadResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"thread\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"thread\": {"]
#[doc = "      \"$ref\": \"#/definitions/Thread\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadReadResponse {
    pub thread: Thread,
}
#[doc = "EXPERIMENTAL - thread realtime audio chunk."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"EXPERIMENTAL - thread realtime audio chunk.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"numChannels\","]
#[doc = "    \"sampleRate\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"numChannels\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"sampleRate\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"samplesPerChannel\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRealtimeAudioChunk {
    pub data: ::std::string::String,
    #[serde(
        rename = "itemId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numChannels")]
    pub num_channels: u16,
    #[serde(rename = "sampleRate")]
    pub sample_rate: u32,
    #[serde(
        rename = "samplesPerChannel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub samples_per_channel: ::std::option::Option<u32>,
}
#[doc = "EXPERIMENTAL - emitted when thread realtime transport closes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRealtimeClosedNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - emitted when thread realtime transport closes.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"reason\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRealtimeClosedNotification {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "EXPERIMENTAL - emitted when thread realtime encounters an error."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRealtimeErrorNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - emitted when thread realtime encounters an error.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRealtimeErrorNotification {
    pub message: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "EXPERIMENTAL - raw non-audio thread realtime item emitted by the backend."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRealtimeItemAddedNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - raw non-audio thread realtime item emitted by the backend.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"item\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"item\": true,"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRealtimeItemAddedNotification {
    pub item: ::serde_json::Value,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "EXPERIMENTAL - streamed output audio emitted by thread realtime."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRealtimeOutputAudioDeltaNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - streamed output audio emitted by thread realtime.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"audio\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"audio\": {"]
#[doc = "      \"$ref\": \"#/definitions/ThreadRealtimeAudioChunk\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRealtimeOutputAudioDeltaNotification {
    pub audio: ThreadRealtimeAudioChunk,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "EXPERIMENTAL - emitted with the remote SDP for a WebRTC realtime session."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRealtimeSdpNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - emitted with the remote SDP for a WebRTC realtime session.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"sdp\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"sdp\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRealtimeSdpNotification {
    pub sdp: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "EXPERIMENTAL - transport used by thread realtime."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"EXPERIMENTAL - transport used by thread realtime.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"WebsocketThreadRealtimeStartTransport\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"WebsocketThreadRealtimeStartTransportType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"websocket\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"WebrtcThreadRealtimeStartTransport\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"sdp\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"sdp\": {"]
#[doc = "          \"description\": \"SDP offer generated by a WebRTC RTCPeerConnection after configuring audio and the realtime events data channel.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"WebrtcThreadRealtimeStartTransportType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"webrtc\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type", content = "sdp")]
pub enum ThreadRealtimeStartTransport {
    #[serde(rename = "websocket")]
    Websocket,
    #[doc = "WebrtcThreadRealtimeStartTransport"]
    #[serde(rename = "webrtc")]
    Webrtc(::std::string::String),
}
#[doc = "EXPERIMENTAL - emitted when thread realtime startup is accepted."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRealtimeStartedNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - emitted when thread realtime startup is accepted.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"sessionId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"$ref\": \"#/definitions/RealtimeConversationVersion\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRealtimeStartedNotification {
    #[serde(
        rename = "sessionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    pub version: RealtimeConversationVersion,
}
#[doc = "EXPERIMENTAL - flat transcript delta emitted whenever realtime transcript text changes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRealtimeTranscriptUpdatedNotification\","]
#[doc = "  \"description\": \"EXPERIMENTAL - flat transcript delta emitted whenever realtime transcript text changes.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"role\","]
#[doc = "    \"text\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"role\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRealtimeTranscriptUpdatedNotification {
    pub role: ::std::string::String,
    pub text: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "There are three ways to resume a thread: 1. By thread_id: load the thread from disk by thread_id and resume it. 2. By history: instantiate the thread from memory and resume it. 3. By path: load the thread from disk by path and resume it.\n\nThe precedence is: history > path > thread_id. If using history or path, the thread_id param will be ignored.\n\nPrefer using thread_id whenever possible."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadResumeParams\","]
#[doc = "  \"description\": \"There are three ways to resume a thread: 1. By thread_id: load the thread from disk by thread_id and resume it. 2. By history: instantiate the thread from memory and resume it. 3. By path: load the thread from disk by path and resume it.\\n\\nThe precedence is: history > path > thread_id. If using history or path, the thread_id param will be ignored.\\n\\nPrefer using thread_id whenever possible.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvalPolicy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"approvalsReviewer\": {"]
#[doc = "      \"description\": \"Override where approval requests are routed for review on this thread and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"baseInstructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"config\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"developerInstructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"description\": \"Configuration overrides for the resumed thread, if any.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modelProvider\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"personality\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Personality\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandbox\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SandboxMode\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"serviceTier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadResumeParams {
    #[serde(
        rename = "approvalPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approval_policy: ::std::option::Option<AskForApproval>,
    #[doc = "Override where approval requests are routed for review on this thread and subsequent turns."]
    #[serde(
        rename = "approvalsReviewer",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approvals_reviewer: ::std::option::Option<ApprovalsReviewer>,
    #[serde(
        rename = "baseInstructions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_instructions: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub config:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "developerInstructions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub developer_instructions: ::std::option::Option<::std::string::String>,
    #[doc = "Configuration overrides for the resumed thread, if any."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modelProvider",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_provider: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub personality: ::std::option::Option<Personality>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sandbox: ::std::option::Option<SandboxMode>,
    #[serde(
        rename = "serviceTier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadResumeResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadResumeResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"approvalPolicy\","]
#[doc = "    \"approvalsReviewer\","]
#[doc = "    \"cwd\","]
#[doc = "    \"model\","]
#[doc = "    \"modelProvider\","]
#[doc = "    \"sandbox\","]
#[doc = "    \"thread\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvalPolicy\": {"]
#[doc = "      \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "    },"]
#[doc = "    \"approvalsReviewer\": {"]
#[doc = "      \"description\": \"Reviewer currently used for approval requests on this thread.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modelProvider\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reasoningEffort\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandbox\": {"]
#[doc = "      \"$ref\": \"#/definitions/SandboxPolicy\""]
#[doc = "    },"]
#[doc = "    \"serviceTier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"thread\": {"]
#[doc = "      \"$ref\": \"#/definitions/Thread\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadResumeResponse {
    #[serde(rename = "approvalPolicy")]
    pub approval_policy: AskForApproval,
    #[doc = "Reviewer currently used for approval requests on this thread."]
    #[serde(rename = "approvalsReviewer")]
    pub approvals_reviewer: ApprovalsReviewer,
    pub cwd: ::std::string::String,
    pub model: ::std::string::String,
    #[serde(rename = "modelProvider")]
    pub model_provider: ::std::string::String,
    #[serde(
        rename = "reasoningEffort",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reasoning_effort: ::std::option::Option<ReasoningEffort>,
    pub sandbox: SandboxPolicy,
    #[serde(
        rename = "serviceTier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_tier: ::std::option::Option<ServiceTier>,
    pub thread: Thread,
}
#[doc = "`ThreadRollbackParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRollbackParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"numTurns\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"numTurns\": {"]
#[doc = "      \"description\": \"The number of turns to drop from the end of the thread. Must be >= 1.\\n\\nThis only modifies the thread's history and does not revert local file changes that have been made by the agent. Clients are responsible for reverting these changes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRollbackParams {
    #[doc = "The number of turns to drop from the end of the thread. Must be >= 1.\n\nThis only modifies the thread's history and does not revert local file changes that have been made by the agent. Clients are responsible for reverting these changes."]
    #[serde(rename = "numTurns")]
    pub num_turns: u32,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadRollbackResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadRollbackResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"thread\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"thread\": {"]
#[doc = "      \"description\": \"The updated thread after applying the rollback, with `turns` populated.\\n\\nThe ThreadItems stored in each Turn are lossy since we explicitly do not persist all agent interactions, such as command executions. This is the same behavior as `thread/resume`.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Thread\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadRollbackResponse {
    #[doc = "The updated thread after applying the rollback, with `turns` populated.\n\nThe ThreadItems stored in each Turn are lossy since we explicitly do not persist all agent interactions, such as command executions. This is the same behavior as `thread/resume`."]
    pub thread: Thread,
}
#[doc = "`ThreadSetNameParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadSetNameParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadSetNameParams {
    pub name: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadSetNameResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadSetNameResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ThreadSetNameResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for ThreadSetNameResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ThreadSetNameResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: ThreadSetNameResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for ThreadSetNameResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`ThreadShellCommandParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadShellCommandParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command\": {"]
#[doc = "      \"description\": \"Shell command string evaluated by the thread's configured shell. Unlike `command/exec`, this intentionally preserves shell syntax such as pipes, redirects, and quoting. This runs unsandboxed with full access rather than inheriting the thread sandbox policy.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadShellCommandParams {
    #[doc = "Shell command string evaluated by the thread's configured shell. Unlike `command/exec`, this intentionally preserves shell syntax such as pipes, redirects, and quoting. This runs unsandboxed with full access rather than inheriting the thread sandbox policy."]
    pub command: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadShellCommandResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadShellCommandResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ThreadShellCommandResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for ThreadShellCommandResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ThreadShellCommandResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: ThreadShellCommandResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for ThreadShellCommandResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`ThreadSortKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"created_at\","]
#[doc = "    \"updated_at\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadSortKey {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}
impl ::std::fmt::Display for ThreadSortKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreatedAt => f.write_str("created_at"),
            Self::UpdatedAt => f.write_str("updated_at"),
        }
    }
}
impl ::std::str::FromStr for ThreadSortKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "created_at" => Ok(Self::CreatedAt),
            "updated_at" => Ok(Self::UpdatedAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadSortKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadSortKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadSortKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ThreadSourceKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"cli\","]
#[doc = "    \"vscode\","]
#[doc = "    \"exec\","]
#[doc = "    \"appServer\","]
#[doc = "    \"subAgent\","]
#[doc = "    \"subAgentReview\","]
#[doc = "    \"subAgentCompact\","]
#[doc = "    \"subAgentThreadSpawn\","]
#[doc = "    \"subAgentOther\","]
#[doc = "    \"unknown\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadSourceKind {
    #[serde(rename = "cli")]
    Cli,
    #[serde(rename = "vscode")]
    Vscode,
    #[serde(rename = "exec")]
    Exec,
    #[serde(rename = "appServer")]
    AppServer,
    #[serde(rename = "subAgent")]
    SubAgent,
    #[serde(rename = "subAgentReview")]
    SubAgentReview,
    #[serde(rename = "subAgentCompact")]
    SubAgentCompact,
    #[serde(rename = "subAgentThreadSpawn")]
    SubAgentThreadSpawn,
    #[serde(rename = "subAgentOther")]
    SubAgentOther,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for ThreadSourceKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cli => f.write_str("cli"),
            Self::Vscode => f.write_str("vscode"),
            Self::Exec => f.write_str("exec"),
            Self::AppServer => f.write_str("appServer"),
            Self::SubAgent => f.write_str("subAgent"),
            Self::SubAgentReview => f.write_str("subAgentReview"),
            Self::SubAgentCompact => f.write_str("subAgentCompact"),
            Self::SubAgentThreadSpawn => f.write_str("subAgentThreadSpawn"),
            Self::SubAgentOther => f.write_str("subAgentOther"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for ThreadSourceKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cli" => Ok(Self::Cli),
            "vscode" => Ok(Self::Vscode),
            "exec" => Ok(Self::Exec),
            "appServer" => Ok(Self::AppServer),
            "subAgent" => Ok(Self::SubAgent),
            "subAgentReview" => Ok(Self::SubAgentReview),
            "subAgentCompact" => Ok(Self::SubAgentCompact),
            "subAgentThreadSpawn" => Ok(Self::SubAgentThreadSpawn),
            "subAgentOther" => Ok(Self::SubAgentOther),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadSourceKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadSourceKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadSourceKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ThreadStartParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadStartParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvalPolicy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"approvalsReviewer\": {"]
#[doc = "      \"description\": \"Override where approval requests are routed for review on this thread and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"baseInstructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"config\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"developerInstructions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ephemeral\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modelProvider\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"personality\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Personality\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandbox\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SandboxMode\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"serviceName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"serviceTier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sessionStartSource\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ThreadStartSource\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadStartParams {
    #[serde(
        rename = "approvalPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approval_policy: ::std::option::Option<AskForApproval>,
    #[doc = "Override where approval requests are routed for review on this thread and subsequent turns."]
    #[serde(
        rename = "approvalsReviewer",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approvals_reviewer: ::std::option::Option<ApprovalsReviewer>,
    #[serde(
        rename = "baseInstructions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_instructions: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub config:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "developerInstructions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub developer_instructions: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ephemeral: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modelProvider",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_provider: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub personality: ::std::option::Option<Personality>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sandbox: ::std::option::Option<SandboxMode>,
    #[serde(
        rename = "serviceName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "serviceTier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[serde(
        rename = "sessionStartSource",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_start_source: ::std::option::Option<ThreadStartSource>,
}
impl ::std::default::Default for ThreadStartParams {
    fn default() -> Self {
        Self {
            approval_policy: Default::default(),
            approvals_reviewer: Default::default(),
            base_instructions: Default::default(),
            config: Default::default(),
            cwd: Default::default(),
            developer_instructions: Default::default(),
            ephemeral: Default::default(),
            model: Default::default(),
            model_provider: Default::default(),
            personality: Default::default(),
            sandbox: Default::default(),
            service_name: Default::default(),
            service_tier: Default::default(),
            session_start_source: Default::default(),
        }
    }
}
#[doc = "`ThreadStartResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadStartResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"approvalPolicy\","]
#[doc = "    \"approvalsReviewer\","]
#[doc = "    \"cwd\","]
#[doc = "    \"model\","]
#[doc = "    \"modelProvider\","]
#[doc = "    \"sandbox\","]
#[doc = "    \"thread\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvalPolicy\": {"]
#[doc = "      \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "    },"]
#[doc = "    \"approvalsReviewer\": {"]
#[doc = "      \"description\": \"Reviewer currently used for approval requests on this thread.\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modelProvider\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reasoningEffort\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandbox\": {"]
#[doc = "      \"$ref\": \"#/definitions/SandboxPolicy\""]
#[doc = "    },"]
#[doc = "    \"serviceTier\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"thread\": {"]
#[doc = "      \"$ref\": \"#/definitions/Thread\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadStartResponse {
    #[serde(rename = "approvalPolicy")]
    pub approval_policy: AskForApproval,
    #[doc = "Reviewer currently used for approval requests on this thread."]
    #[serde(rename = "approvalsReviewer")]
    pub approvals_reviewer: ApprovalsReviewer,
    pub cwd: ::std::string::String,
    pub model: ::std::string::String,
    #[serde(rename = "modelProvider")]
    pub model_provider: ::std::string::String,
    #[serde(
        rename = "reasoningEffort",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reasoning_effort: ::std::option::Option<ReasoningEffort>,
    pub sandbox: SandboxPolicy,
    #[serde(
        rename = "serviceTier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_tier: ::std::option::Option<ServiceTier>,
    pub thread: Thread,
}
#[doc = "`ThreadStartSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"startup\","]
#[doc = "    \"clear\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadStartSource {
    #[serde(rename = "startup")]
    Startup,
    #[serde(rename = "clear")]
    Clear,
}
impl ::std::fmt::Display for ThreadStartSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Startup => f.write_str("startup"),
            Self::Clear => f.write_str("clear"),
        }
    }
}
impl ::std::str::FromStr for ThreadStartSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "startup" => Ok(Self::Startup),
            "clear" => Ok(Self::Clear),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadStartSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadStartSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadStartSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ThreadStartedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadStartedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"thread\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"thread\": {"]
#[doc = "      \"$ref\": \"#/definitions/Thread\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadStartedNotification {
    pub thread: Thread,
}
#[doc = "`ThreadStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"NotLoadedThreadStatus\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"NotLoadedThreadStatusType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"notLoaded\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"IdleThreadStatus\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"IdleThreadStatusType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"idle\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"SystemErrorThreadStatus\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"SystemErrorThreadStatusType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"systemError\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ActiveThreadStatus\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"activeFlags\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"activeFlags\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/ThreadActiveFlag\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ActiveThreadStatusType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"active\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type", content = "activeFlags")]
pub enum ThreadStatus {
    #[serde(rename = "notLoaded")]
    NotLoaded,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "systemError")]
    SystemError,
    #[doc = "ActiveThreadStatus"]
    #[serde(rename = "active")]
    Active(::std::vec::Vec<ThreadActiveFlag>),
}
impl ::std::convert::From<::std::vec::Vec<ThreadActiveFlag>> for ThreadStatus {
    fn from(value: ::std::vec::Vec<ThreadActiveFlag>) -> Self {
        Self::Active(value)
    }
}
#[doc = "`ThreadStatusChangedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadStatusChangedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/ThreadStatus\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadStatusChangedNotification {
    pub status: ThreadStatus,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadTokenUsage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"last\","]
#[doc = "    \"total\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"last\": {"]
#[doc = "      \"$ref\": \"#/definitions/TokenUsageBreakdown\""]
#[doc = "    },"]
#[doc = "    \"modelContextWindow\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"total\": {"]
#[doc = "      \"$ref\": \"#/definitions/TokenUsageBreakdown\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadTokenUsage {
    pub last: TokenUsageBreakdown,
    #[serde(
        rename = "modelContextWindow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_context_window: ::std::option::Option<i64>,
    pub total: TokenUsageBreakdown,
}
#[doc = "`ThreadTokenUsageUpdatedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadTokenUsageUpdatedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\","]
#[doc = "    \"tokenUsage\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tokenUsage\": {"]
#[doc = "      \"$ref\": \"#/definitions/ThreadTokenUsage\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadTokenUsageUpdatedNotification {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "tokenUsage")]
    pub token_usage: ThreadTokenUsage,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`ThreadUnarchiveParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadUnarchiveParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadUnarchiveParams {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadUnarchiveResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadUnarchiveResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"thread\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"thread\": {"]
#[doc = "      \"$ref\": \"#/definitions/Thread\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadUnarchiveResponse {
    pub thread: Thread,
}
#[doc = "`ThreadUnarchivedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadUnarchivedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadUnarchivedNotification {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadUnsubscribeParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadUnsubscribeParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadUnsubscribeParams {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`ThreadUnsubscribeResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadUnsubscribeResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/ThreadUnsubscribeStatus\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadUnsubscribeResponse {
    pub status: ThreadUnsubscribeStatus,
}
#[doc = "`ThreadUnsubscribeStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"notLoaded\","]
#[doc = "    \"notSubscribed\","]
#[doc = "    \"unsubscribed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadUnsubscribeStatus {
    #[serde(rename = "notLoaded")]
    NotLoaded,
    #[serde(rename = "notSubscribed")]
    NotSubscribed,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}
impl ::std::fmt::Display for ThreadUnsubscribeStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NotLoaded => f.write_str("notLoaded"),
            Self::NotSubscribed => f.write_str("notSubscribed"),
            Self::Unsubscribed => f.write_str("unsubscribed"),
        }
    }
}
impl ::std::str::FromStr for ThreadUnsubscribeStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "notLoaded" => Ok(Self::NotLoaded),
            "notSubscribed" => Ok(Self::NotSubscribed),
            "unsubscribed" => Ok(Self::Unsubscribed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadUnsubscribeStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadUnsubscribeStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadUnsubscribeStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`TokenUsageBreakdown`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cachedInputTokens\","]
#[doc = "    \"inputTokens\","]
#[doc = "    \"outputTokens\","]
#[doc = "    \"reasoningOutputTokens\","]
#[doc = "    \"totalTokens\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cachedInputTokens\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"inputTokens\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"outputTokens\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"reasoningOutputTokens\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"totalTokens\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TokenUsageBreakdown {
    #[serde(rename = "cachedInputTokens")]
    pub cached_input_tokens: i64,
    #[serde(rename = "inputTokens")]
    pub input_tokens: i64,
    #[serde(rename = "outputTokens")]
    pub output_tokens: i64,
    #[serde(rename = "reasoningOutputTokens")]
    pub reasoning_output_tokens: i64,
    #[serde(rename = "totalTokens")]
    pub total_tokens: i64,
}
#[doc = "Definition for a tool the client can call."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Definition for a tool the client can call.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"inputSchema\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": true,"]
#[doc = "    \"annotations\": true,"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"icons\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": true"]
#[doc = "    },"]
#[doc = "    \"inputSchema\": true,"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"outputSchema\": true,"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Tool {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub icons: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "inputSchema")]
    pub input_schema: ::serde_json::Value,
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<::serde_json::Value>,
    pub name: ::std::string::String,
    #[serde(
        rename = "outputSchema",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub output_schema: ::std::option::Option<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
#[doc = "`ToolsV2`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"view_image\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"web_search\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/WebSearchToolConfig\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolsV2 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub view_image: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub web_search: ::std::option::Option<WebSearchToolConfig>,
}
impl ::std::default::Default for ToolsV2 {
    fn default() -> Self {
        Self {
            view_image: Default::default(),
            web_search: Default::default(),
        }
    }
}
#[doc = "`Turn`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"items\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"completedAt\": {"]
#[doc = "      \"description\": \"Unix timestamp (in seconds) when the turn completed.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"durationMs\": {"]
#[doc = "      \"description\": \"Duration between turn start and completion in milliseconds, if known.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"error\": {"]
#[doc = "      \"description\": \"Only populated when the Turn's status is failed.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TurnError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"items\": {"]
#[doc = "      \"description\": \"Only populated on a `thread/resume` or `thread/fork` response. For all other responses and notifications returning a Turn, the items field will be an empty list.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ThreadItem\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"startedAt\": {"]
#[doc = "      \"description\": \"Unix timestamp (in seconds) when the turn started.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/TurnStatus\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Turn {
    #[doc = "Unix timestamp (in seconds) when the turn completed."]
    #[serde(
        rename = "completedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub completed_at: ::std::option::Option<i64>,
    #[doc = "Duration between turn start and completion in milliseconds, if known."]
    #[serde(
        rename = "durationMs",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub duration_ms: ::std::option::Option<i64>,
    #[doc = "Only populated when the Turn's status is failed."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<TurnError>,
    pub id: ::std::string::String,
    #[doc = "Only populated on a `thread/resume` or `thread/fork` response. For all other responses and notifications returning a Turn, the items field will be an empty list."]
    pub items: ::std::vec::Vec<ThreadItem>,
    #[doc = "Unix timestamp (in seconds) when the turn started."]
    #[serde(
        rename = "startedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub started_at: ::std::option::Option<i64>,
    pub status: TurnStatus,
}
#[doc = "`TurnCompletedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnCompletedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\","]
#[doc = "    \"turn\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turn\": {"]
#[doc = "      \"$ref\": \"#/definitions/Turn\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnCompletedNotification {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    pub turn: Turn,
}
#[doc = "Notification that the turn-level unified diff has changed. Contains the latest aggregated diff across all file changes in the turn."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnDiffUpdatedNotification\","]
#[doc = "  \"description\": \"Notification that the turn-level unified diff has changed. Contains the latest aggregated diff across all file changes in the turn.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"diff\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"diff\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnDiffUpdatedNotification {
    pub diff: ::std::string::String,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`TurnError`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"additionalDetails\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"codexErrorInfo\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CodexErrorInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnError {
    #[serde(
        rename = "additionalDetails",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub additional_details: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "codexErrorInfo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub codex_error_info: ::std::option::Option<CodexErrorInfo>,
    pub message: ::std::string::String,
}
#[doc = "`TurnInterruptParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnInterruptParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnInterruptParams {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`TurnInterruptResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnInterruptResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TurnInterruptResponse(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for TurnInterruptResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<TurnInterruptResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: TurnInterruptResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for TurnInterruptResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`TurnPlanStep`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"status\","]
#[doc = "    \"step\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/TurnPlanStepStatus\""]
#[doc = "    },"]
#[doc = "    \"step\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnPlanStep {
    pub status: TurnPlanStepStatus,
    pub step: ::std::string::String,
}
#[doc = "`TurnPlanStepStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"pending\","]
#[doc = "    \"inProgress\","]
#[doc = "    \"completed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TurnPlanStepStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for TurnPlanStepStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
            Self::InProgress => f.write_str("inProgress"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for TurnPlanStepStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            "inProgress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TurnPlanStepStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TurnPlanStepStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TurnPlanStepStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`TurnPlanUpdatedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnPlanUpdatedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"plan\","]
#[doc = "    \"threadId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"explanation\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"plan\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/TurnPlanStep\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnPlanUpdatedNotification {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub explanation: ::std::option::Option<::std::string::String>,
    pub plan: ::std::vec::Vec<TurnPlanStep>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`TurnStartParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnStartParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"input\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"approvalPolicy\": {"]
#[doc = "      \"description\": \"Override the approval policy for this turn and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AskForApproval\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"approvalsReviewer\": {"]
#[doc = "      \"description\": \"Override where approval requests are routed for review on this turn and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ApprovalsReviewer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"description\": \"Override the working directory for this turn and subsequent turns.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"effort\": {"]
#[doc = "      \"description\": \"Override the reasoning effort for this turn and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningEffort\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"input\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/UserInput\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"description\": \"Override the model for this turn and subsequent turns.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"outputSchema\": {"]
#[doc = "      \"description\": \"Optional JSON Schema used to constrain the final assistant message for this turn.\""]
#[doc = "    },"]
#[doc = "    \"personality\": {"]
#[doc = "      \"description\": \"Override the personality for this turn and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Personality\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sandboxPolicy\": {"]
#[doc = "      \"description\": \"Override the sandbox policy for this turn and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SandboxPolicy\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"serviceTier\": {"]
#[doc = "      \"description\": \"Override the service tier for this turn and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ServiceTier\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"Override the reasoning summary for this turn and subsequent turns.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReasoningSummary\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnStartParams {
    #[doc = "Override the approval policy for this turn and subsequent turns."]
    #[serde(
        rename = "approvalPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approval_policy: ::std::option::Option<AskForApproval>,
    #[doc = "Override where approval requests are routed for review on this turn and subsequent turns."]
    #[serde(
        rename = "approvalsReviewer",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub approvals_reviewer: ::std::option::Option<ApprovalsReviewer>,
    #[doc = "Override the working directory for this turn and subsequent turns."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[doc = "Override the reasoning effort for this turn and subsequent turns."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub effort: ::std::option::Option<ReasoningEffort>,
    pub input: ::std::vec::Vec<UserInput>,
    #[doc = "Override the model for this turn and subsequent turns."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "Optional JSON Schema used to constrain the final assistant message for this turn."]
    #[serde(
        rename = "outputSchema",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub output_schema: ::std::option::Option<::serde_json::Value>,
    #[doc = "Override the personality for this turn and subsequent turns."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub personality: ::std::option::Option<Personality>,
    #[doc = "Override the sandbox policy for this turn and subsequent turns."]
    #[serde(
        rename = "sandboxPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sandbox_policy: ::std::option::Option<SandboxPolicy>,
    #[doc = "Override the service tier for this turn and subsequent turns."]
    #[serde(
        rename = "serviceTier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[doc = "Override the reasoning summary for this turn and subsequent turns."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<ReasoningSummary>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`TurnStartResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnStartResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"turn\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"turn\": {"]
#[doc = "      \"$ref\": \"#/definitions/Turn\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnStartResponse {
    pub turn: Turn,
}
#[doc = "`TurnStartedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnStartedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"threadId\","]
#[doc = "    \"turn\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"turn\": {"]
#[doc = "      \"$ref\": \"#/definitions/Turn\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnStartedNotification {
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
    pub turn: Turn,
}
#[doc = "`TurnStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"completed\","]
#[doc = "    \"interrupted\","]
#[doc = "    \"failed\","]
#[doc = "    \"inProgress\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TurnStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "interrupted")]
    Interrupted,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "inProgress")]
    InProgress,
}
impl ::std::fmt::Display for TurnStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Interrupted => f.write_str("interrupted"),
            Self::Failed => f.write_str("failed"),
            Self::InProgress => f.write_str("inProgress"),
        }
    }
}
impl ::std::str::FromStr for TurnStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "interrupted" => Ok(Self::Interrupted),
            "failed" => Ok(Self::Failed),
            "inProgress" => Ok(Self::InProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TurnStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TurnStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TurnStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`TurnSteerParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnSteerParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expectedTurnId\","]
#[doc = "    \"input\","]
#[doc = "    \"threadId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expectedTurnId\": {"]
#[doc = "      \"description\": \"Required active turn id precondition. The request fails when it does not match the currently active turn.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"input\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/UserInput\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"threadId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnSteerParams {
    #[doc = "Required active turn id precondition. The request fails when it does not match the currently active turn."]
    #[serde(rename = "expectedTurnId")]
    pub expected_turn_id: ::std::string::String,
    pub input: ::std::vec::Vec<UserInput>,
    #[serde(rename = "threadId")]
    pub thread_id: ::std::string::String,
}
#[doc = "`TurnSteerResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TurnSteerResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TurnSteerResponse {
    #[serde(rename = "turnId")]
    pub turn_id: ::std::string::String,
}
#[doc = "`UserInput`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"TextUserInput\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"text_elements\": {"]
#[doc = "          \"description\": \"UI-defined spans within `text` used to render or persist special elements.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/TextElement\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"TextUserInputType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"text\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"ImageUserInput\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"url\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"ImageUserInputType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"image\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"LocalImageUserInput\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"path\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"path\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"LocalImageUserInputType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"localImage\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"SkillUserInput\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\","]
#[doc = "        \"path\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"SkillUserInputType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"skill\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"MentionUserInput\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\","]
#[doc = "        \"path\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"MentionUserInputType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"mention\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum UserInput {
    #[doc = "TextUserInput"]
    #[serde(rename = "text")]
    Text {
        text: ::std::string::String,
        #[doc = "UI-defined spans within `text` used to render or persist special elements."]
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        text_elements: ::std::vec::Vec<TextElement>,
    },
    #[doc = "ImageUserInput"]
    #[serde(rename = "image")]
    Image { url: ::std::string::String },
    #[doc = "LocalImageUserInput"]
    #[serde(rename = "localImage")]
    LocalImage { path: ::std::string::String },
    #[doc = "SkillUserInput"]
    #[serde(rename = "skill")]
    Skill {
        name: ::std::string::String,
        path: ::std::string::String,
    },
    #[doc = "MentionUserInput"]
    #[serde(rename = "mention")]
    Mention {
        name: ::std::string::String,
        path: ::std::string::String,
    },
}
#[doc = "Controls output length/detail on GPT-5 models via the Responses API. Serialized with lowercase values to match the OpenAI API."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls output length/detail on GPT-5 models via the Responses API. Serialized with lowercase values to match the OpenAI API.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"low\","]
#[doc = "    \"medium\","]
#[doc = "    \"high\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Verbosity {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for Verbosity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Verbosity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Verbosity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Verbosity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebSearchAction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"SearchWebSearchAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"queries\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"array\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"query\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"SearchWebSearchActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"search\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"OpenPageWebSearchAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"OpenPageWebSearchActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"openPage\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"FindInPageWebSearchAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pattern\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"FindInPageWebSearchActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"findInPage\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"OtherWebSearchAction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"OtherWebSearchActionType\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"other\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum WebSearchAction {
    #[doc = "SearchWebSearchAction"]
    #[serde(rename = "search")]
    Search {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        queries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        query: ::std::option::Option<::std::string::String>,
    },
    #[doc = "OpenPageWebSearchAction"]
    #[serde(rename = "openPage")]
    OpenPage {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        url: ::std::option::Option<::std::string::String>,
    },
    #[doc = "FindInPageWebSearchAction"]
    #[serde(rename = "findInPage")]
    FindInPage {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pattern: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        url: ::std::option::Option<::std::string::String>,
    },
    #[serde(rename = "other")]
    Other,
}
#[doc = "`WebSearchContextSize`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"low\","]
#[doc = "    \"medium\","]
#[doc = "    \"high\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchContextSize {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for WebSearchContextSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for WebSearchContextSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WebSearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebSearchLocation`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"city\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"country\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"region\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"timezone\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WebSearchLocation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WebSearchLocation {
    fn default() -> Self {
        Self {
            city: Default::default(),
            country: Default::default(),
            region: Default::default(),
            timezone: Default::default(),
        }
    }
}
#[doc = "`WebSearchMode`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"disabled\","]
#[doc = "    \"cached\","]
#[doc = "    \"live\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchMode {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "cached")]
    Cached,
    #[serde(rename = "live")]
    Live,
}
impl ::std::fmt::Display for WebSearchMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Disabled => f.write_str("disabled"),
            Self::Cached => f.write_str("cached"),
            Self::Live => f.write_str("live"),
        }
    }
}
impl ::std::str::FromStr for WebSearchMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "disabled" => Ok(Self::Disabled),
            "cached" => Ok(Self::Cached),
            "live" => Ok(Self::Live),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WebSearchMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebSearchToolConfig`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowed_domains\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"context_size\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/WebSearchContextSize\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/WebSearchLocation\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WebSearchToolConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub context_size: ::std::option::Option<WebSearchContextSize>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<WebSearchLocation>,
}
impl ::std::default::Default for WebSearchToolConfig {
    fn default() -> Self {
        Self {
            allowed_domains: Default::default(),
            context_size: Default::default(),
            location: Default::default(),
        }
    }
}
#[doc = "`WindowsSandboxSetupCompletedNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"WindowsSandboxSetupCompletedNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"mode\","]
#[doc = "    \"success\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"mode\": {"]
#[doc = "      \"$ref\": \"#/definitions/WindowsSandboxSetupMode\""]
#[doc = "    },"]
#[doc = "    \"success\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WindowsSandboxSetupCompletedNotification {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    pub mode: WindowsSandboxSetupMode,
    pub success: bool,
}
#[doc = "`WindowsSandboxSetupMode`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"elevated\","]
#[doc = "    \"unelevated\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WindowsSandboxSetupMode {
    #[serde(rename = "elevated")]
    Elevated,
    #[serde(rename = "unelevated")]
    Unelevated,
}
impl ::std::fmt::Display for WindowsSandboxSetupMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Elevated => f.write_str("elevated"),
            Self::Unelevated => f.write_str("unelevated"),
        }
    }
}
impl ::std::str::FromStr for WindowsSandboxSetupMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "elevated" => Ok(Self::Elevated),
            "unelevated" => Ok(Self::Unelevated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WindowsSandboxSetupMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WindowsSandboxSetupMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WindowsSandboxSetupMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WindowsSandboxSetupStartParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"WindowsSandboxSetupStartParams\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"mode\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/AbsolutePathBuf\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"mode\": {"]
#[doc = "      \"$ref\": \"#/definitions/WindowsSandboxSetupMode\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WindowsSandboxSetupStartParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<AbsolutePathBuf>,
    pub mode: WindowsSandboxSetupMode,
}
#[doc = "`WindowsSandboxSetupStartResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"WindowsSandboxSetupStartResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"started\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"started\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WindowsSandboxSetupStartResponse {
    pub started: bool,
}
#[doc = "`WindowsWorldWritableWarningNotification`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"WindowsWorldWritableWarningNotification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"extraCount\","]
#[doc = "    \"failedScan\","]
#[doc = "    \"samplePaths\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extraCount\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"failedScan\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"samplePaths\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"http://json-schema.org/draft-07/schema#\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WindowsWorldWritableWarningNotification {
    #[serde(rename = "extraCount")]
    pub extra_count: u32,
    #[serde(rename = "failedScan")]
    pub failed_scan: bool,
    #[serde(rename = "samplePaths")]
    pub sample_paths: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`WriteStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ok\","]
#[doc = "    \"okOverridden\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WriteStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "okOverridden")]
    OkOverridden,
}
impl ::std::fmt::Display for WriteStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
            Self::OkOverridden => f.write_str("okOverridden"),
        }
    }
}
impl ::std::str::FromStr for WriteStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            "okOverridden" => Ok(Self::OkOverridden),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WriteStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WriteStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WriteStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn model_input_modalities() -> ::std::vec::Vec<super::InputModality> {
        vec![super::InputModality::Text, super::InputModality::Image]
    }
    pub(super) fn sandbox_policy_external_sandbox_network_access() -> super::NetworkAccess {
        super::NetworkAccess::Restricted
    }
    pub(super) fn sandbox_policy_read_only_access() -> super::ReadOnlyAccess {
        super::ReadOnlyAccess::FullAccess
    }
    pub(super) fn sandbox_policy_workspace_write_read_only_access() -> super::ReadOnlyAccess {
        super::ReadOnlyAccess::FullAccess
    }
    pub(super) fn thread_item_command_execution_source() -> super::CommandExecutionSource {
        super::CommandExecutionSource::Agent
    }
}
