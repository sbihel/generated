pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Agent {
        #[doc = "Optional. API version displayed in Dialogflow console. If not specified,\nV2 API is assumed. Clients are free to query different service endpoints\nfor different API versions. However, bots connectors and webhook calls will\nfollow the specified API version."]
        #[serde(rename = "apiVersion", default)]
        pub api_version:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2AgentApiVersion>,
        #[doc = "Optional. The URI of the agent's avatar.\nAvatars are used throughout the Dialogflow console and in the self-hosted\n[Web\nDemo](https://cloud.google.com/dialogflow/docs/integrations/web-demo)\nintegration."]
        #[serde(rename = "avatarUri", default)]
        pub avatar_uri: ::std::option::Option<String>,
        #[doc = "Optional. To filter out false positive results and still get variety in\nmatched natural language inputs for your agent, you can tune the machine\nlearning classification threshold. If the returned score value is less than\nthe threshold value, then a fallback intent will be triggered or, if there\nare no fallback intents defined, no intent will be triggered. The score\nvalues range from 0.0 (completely uncertain) to 1.0 (completely certain).\nIf set to 0.0, the default of 0.3 is used."]
        #[serde(rename = "classificationThreshold", default)]
        pub classification_threshold: ::std::option::Option<f32>,
        #[doc = "Required. The default language of the agent as a language tag. See\n[Language\nSupport](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be\nset by the `Update` method."]
        #[serde(rename = "defaultLanguageCode", default)]
        pub default_language_code: ::std::option::Option<String>,
        #[doc = "Optional. The description of this agent.\nThe maximum length is 500 characters. If exceeded, the request is rejected."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. The name of this agent."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. Determines whether this agent should log conversation queries."]
        #[serde(rename = "enableLogging", default)]
        pub enable_logging: ::std::option::Option<bool>,
        #[doc = "Optional. Determines how intents are detected from user queries."]
        #[serde(rename = "matchMode", default)]
        pub match_mode:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2AgentMatchMode>,
        #[doc = "Required. The project of this agent.\nFormat: `projects/<Project ID>`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
        #[doc = "Optional. The list of all languages supported by this agent (except for the\n`default_language_code`)."]
        #[serde(rename = "supportedLanguageCodes", default)]
        pub supported_language_codes: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The agent tier. If not specified, TIER_STANDARD is assumed."]
        #[serde(rename = "tier", default)]
        pub tier: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2AgentTier>,
        #[doc = "Required. The time zone of this agent from the\n[time zone database](https://www.iana.org/time-zones), e.g.,\nAmerica/New_York, Europe/Paris."]
        #[serde(rename = "timeZone", default)]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Agent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Agent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2AgentApiVersion {
        #[doc = "Not specified."]
        ApiVersionUnspecified,
        #[doc = "Legacy V1 API."]
        ApiVersionV1,
        #[doc = "V2 API."]
        ApiVersionV2,
        #[doc = "V2beta1 API."]
        ApiVersionV2Beta1,
    }
    impl GoogleCloudDialogflowV2AgentApiVersion {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2AgentApiVersion::ApiVersionUnspecified => {
                    "API_VERSION_UNSPECIFIED"
                }
                GoogleCloudDialogflowV2AgentApiVersion::ApiVersionV1 => "API_VERSION_V1",
                GoogleCloudDialogflowV2AgentApiVersion::ApiVersionV2 => "API_VERSION_V2",
                GoogleCloudDialogflowV2AgentApiVersion::ApiVersionV2Beta1 => {
                    "API_VERSION_V2_BETA_1"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2AgentApiVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2AgentApiVersion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2AgentApiVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "API_VERSION_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2AgentApiVersion::ApiVersionUnspecified
                }
                "API_VERSION_V1" => GoogleCloudDialogflowV2AgentApiVersion::ApiVersionV1,
                "API_VERSION_V2" => GoogleCloudDialogflowV2AgentApiVersion::ApiVersionV2,
                "API_VERSION_V2_BETA_1" => {
                    GoogleCloudDialogflowV2AgentApiVersion::ApiVersionV2Beta1
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2AgentApiVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2AgentApiVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2AgentMatchMode {
        #[doc = "Best for agents with a small number of examples in intents and/or wide\nuse of templates syntax and composite entities."]
        MatchModeHybrid,
        #[doc = "Can be used for agents with a large number of examples in intents,\nespecially the ones using @sys.any or very large developer entities."]
        MatchModeMlOnly,
        #[doc = "Not specified."]
        MatchModeUnspecified,
    }
    impl GoogleCloudDialogflowV2AgentMatchMode {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2AgentMatchMode::MatchModeHybrid => "MATCH_MODE_HYBRID",
                GoogleCloudDialogflowV2AgentMatchMode::MatchModeMlOnly => "MATCH_MODE_ML_ONLY",
                GoogleCloudDialogflowV2AgentMatchMode::MatchModeUnspecified => {
                    "MATCH_MODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2AgentMatchMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2AgentMatchMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2AgentMatchMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MATCH_MODE_HYBRID" => GoogleCloudDialogflowV2AgentMatchMode::MatchModeHybrid,
                "MATCH_MODE_ML_ONLY" => GoogleCloudDialogflowV2AgentMatchMode::MatchModeMlOnly,
                "MATCH_MODE_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2AgentMatchMode::MatchModeUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2AgentMatchMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2AgentMatchMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2AgentTier {
        #[doc = "Enterprise tier (Essentials)."]
        TierEnterprise,
        #[doc = "Enterprise tier (Plus)."]
        TierEnterprisePlus,
        #[doc = "Standard tier."]
        TierStandard,
        #[doc = "Not specified. This value should never be used."]
        TierUnspecified,
    }
    impl GoogleCloudDialogflowV2AgentTier {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2AgentTier::TierEnterprise => "TIER_ENTERPRISE",
                GoogleCloudDialogflowV2AgentTier::TierEnterprisePlus => "TIER_ENTERPRISE_PLUS",
                GoogleCloudDialogflowV2AgentTier::TierStandard => "TIER_STANDARD",
                GoogleCloudDialogflowV2AgentTier::TierUnspecified => "TIER_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2AgentTier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2AgentTier {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2AgentTier {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TIER_ENTERPRISE" => GoogleCloudDialogflowV2AgentTier::TierEnterprise,
                "TIER_ENTERPRISE_PLUS" => GoogleCloudDialogflowV2AgentTier::TierEnterprisePlus,
                "TIER_STANDARD" => GoogleCloudDialogflowV2AgentTier::TierStandard,
                "TIER_UNSPECIFIED" => GoogleCloudDialogflowV2AgentTier::TierUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2AgentTier {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2AgentTier {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2BatchCreateEntitiesRequest {
        #[doc = "Required. The entities to create."]
        #[serde(rename = "entities", default)]
        pub entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2EntityTypeEntity>>,
        #[doc = "Optional. The language of entity synonyms defined in `entities`. If not\nspecified, the agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2BatchCreateEntitiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2BatchCreateEntitiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2BatchDeleteEntitiesRequest {
        #[doc = "Required. The canonical `values` of the entities to delete. Note that\nthese are not fully-qualified names, i.e. they don't start with\n`projects/<Project ID>`."]
        #[serde(rename = "entityValues", default)]
        pub entity_values: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The language of entity synonyms defined in `entities`. If not\nspecified, the agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2BatchDeleteEntitiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2BatchDeleteEntitiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2BatchDeleteEntityTypesRequest {
        #[doc = "Required. The names entity types to delete. All names must point to the\nsame agent as `parent`."]
        #[serde(rename = "entityTypeNames", default)]
        pub entity_type_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2BatchDeleteEntityTypesRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2BatchDeleteEntityTypesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2BatchDeleteIntentsRequest {
        #[doc = "Required. The collection of intents to delete. Only intent `name` must be\nfilled in."]
        #[serde(rename = "intents", default)]
        pub intents: ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Intent>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2BatchDeleteIntentsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2BatchDeleteIntentsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2BatchUpdateEntitiesRequest {
        #[doc = "Required. The entities to update or create."]
        #[serde(rename = "entities", default)]
        pub entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2EntityTypeEntity>>,
        #[doc = "Optional. The language of entity synonyms defined in `entities`. If not\nspecified, the agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. The mask to control which fields get updated."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2BatchUpdateEntitiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2BatchUpdateEntitiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2BatchUpdateEntityTypesRequest {
        #[doc = "The collection of entity types to update or create."]
        #[serde(rename = "entityTypeBatchInline", default)]
        pub entity_type_batch_inline:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2EntityTypeBatch>,
        #[doc = "The URI to a Google Cloud Storage file containing entity types to update\nor create. The file format can either be a serialized proto (of\nEntityBatch type) or a JSON object. Note: The URI must start with\n\"gs://\"."]
        #[serde(rename = "entityTypeBatchUri", default)]
        pub entity_type_batch_uri: ::std::option::Option<String>,
        #[doc = "Optional. The language of entity synonyms defined in `entity_types`. If not\nspecified, the agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. The mask to control which fields get updated."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2BatchUpdateEntityTypesRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2BatchUpdateEntityTypesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2BatchUpdateEntityTypesResponse {
        #[doc = "The collection of updated or created entity types."]
        #[serde(rename = "entityTypes", default)]
        pub entity_types:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2EntityType>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2BatchUpdateEntityTypesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2BatchUpdateEntityTypesResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2BatchUpdateIntentsRequest {
        #[doc = "The collection of intents to update or create."]
        #[serde(rename = "intentBatchInline", default)]
        pub intent_batch_inline:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentBatch>,
        #[doc = "The URI to a Google Cloud Storage file containing intents to update or\ncreate. The file format can either be a serialized proto (of IntentBatch\ntype) or JSON object. Note: The URI must start with \"gs://\"."]
        #[serde(rename = "intentBatchUri", default)]
        pub intent_batch_uri: ::std::option::Option<String>,
        #[doc = "Optional. The resource view to apply to the returned intent."]
        #[serde(rename = "intentView", default)]
        pub intent_view: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView,
        >,
        #[doc = "Optional. The language of training phrases, parameters and rich messages\ndefined in `intents`. If not specified, the agent's default language is\nused. [Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. The mask to control which fields get updated."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2BatchUpdateIntentsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2BatchUpdateIntentsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView {
        #[doc = "All fields are populated."]
        IntentViewFull,
        #[doc = "Training phrases field is not populated in the response."]
        IntentViewUnspecified,
    }
    impl GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView :: IntentViewFull => "INTENT_VIEW_FULL" , GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView :: IntentViewUnspecified => "INTENT_VIEW_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "INTENT_VIEW_FULL" => GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView :: IntentViewFull , "INTENT_VIEW_UNSPECIFIED" => GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView :: IntentViewUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentView
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2BatchUpdateIntentsResponse {
        #[doc = "The collection of updated or created intents."]
        #[serde(rename = "intents", default)]
        pub intents: ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Intent>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2BatchUpdateIntentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2BatchUpdateIntentsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1AnnotatedConversationDataset {
        #[doc = "Output only. Number of examples that have annotations in the annotated\nconversation dataset."]
        #[serde(rename = "completedExampleCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub completed_example_count: ::std::option::Option<i64>,
        #[doc = "Output only. Creation time of this annotated conversation dataset."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Optional. The description of the annotated conversation dataset.\nMaximum of 10000 bytes."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. The display name of the annotated conversation dataset.\nIt's specified when user starts an annotation task. Maximum of 64 bytes."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. Number of examples in the annotated conversation dataset."]
        #[serde(rename = "exampleCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub example_count: ::std::option::Option<i64>,
        #[doc = "Output only. AnnotatedConversationDataset resource name. Format:\n`projects/<Project ID>/conversationDatasets/<Conversation Dataset ID>/annotatedConversationDatasets/<Annotated Conversation Dataset ID>`"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Question type name that identifies a labeling task.\nA question is a single task that a worker answers. A question type is set\nof related questions. Each question belongs to a particular question type.\nIt can be used in CrowdCompute UI to filter and manage labeling tasks."]
        #[serde(rename = "questionTypeName", default)]
        pub question_type_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1AnnotatedConversationDataset
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1AnnotatedConversationDataset
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1ArticleSuggestionModelMetadata {
        #[doc = "Optional. Type of the article suggestion model. The available values are:\n\n* `article-suggestion-gbt-1` - (default) Article Suggestion Gbt model."]
        #[serde(rename = "modelType", default)]
        pub model_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1ArticleSuggestionModelMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1ArticleSuggestionModelMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1BatchUpdateEntityTypesResponse {
        #[doc = "The collection of updated or created entity types."]
        #[serde(rename = "entityTypes", default)]
        pub entity_types:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1EntityType>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1BatchUpdateEntityTypesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1BatchUpdateEntityTypesResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1BatchUpdateIntentsResponse {
        #[doc = "The collection of updated or created intents."]
        #[serde(rename = "intents", default)]
        pub intents: ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1Intent>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1BatchUpdateIntentsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1BatchUpdateIntentsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1Context {
        #[doc = "Optional. The number of conversational query requests after which the\ncontext expires. If set to `0` (the default) the context expires\nimmediately. Contexts expire automatically after 20 minutes if there\nare no matching queries."]
        #[serde(rename = "lifespanCount", default)]
        pub lifespan_count: ::std::option::Option<i32>,
        #[doc = "Required. The unique identifier of the context. Format:\n`projects/<Project ID>/agent/sessions/<Session ID>/contexts/<Context ID>`,\nor `projects/<Project ID>/agent/environments/<Environment ID>/users/<User ID>/sessions/<Session ID>/contexts/<Context ID>`.\n\nThe `Context ID` is always converted to lowercase, may only contain\ncharacters in a-zA-Z0-9_-% and may be at most 250 bytes long.\n\nIf `Environment ID` is not specified, we assume default 'draft'\nenvironment. If `User ID` is not specified, we assume default '-' user."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of parameters associated with this context.\nRefer to [this\ndoc](https://cloud.google.com/dialogflow/docs/intents-actions-parameters)\nfor syntax."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1Context {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1Context {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1ConversationModel {
        #[doc = "Metadata for article suggestion models."]
        #[serde(rename = "articleSuggestionModelMetadata", default)]
        pub article_suggestion_model_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1ArticleSuggestionModelMetadata,
        >,
        #[doc = "Output only. Creation time of this model."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Required. Datasets used to create model."]
        #[serde(rename = "datasets", default)]
        pub datasets:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1InputDataset>>,
        #[doc = "Required. The display name of the model. At most 64 bytes long."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. ConversationModel resource name. Format:\n`projects/<Project ID>/conversationModels/<Conversation Model ID>`"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. State of the model. A model can only serve prediction requests\nafter it gets deployed."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1ConversationModelState,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1ConversationModel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1ConversationModel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1ConversationModelState {
        #[doc = "Model is creating."]
        Creating,
        #[doc = "Model is deleting."]
        Deleting,
        #[doc = "Model is deployed and ready to use."]
        Deployed,
        #[doc = "Model is deploying."]
        Deploying,
        #[doc = "Model is in error state. Not ready to deploy and use."]
        Failed,
        #[doc = "Should not be used, an un-set enum has this value by default."]
        StateUnspecified,
        #[doc = "Model is not deployed but ready to deploy."]
        Undeployed,
        #[doc = "Model is undeploying."]
        Undeploying,
    }
    impl GoogleCloudDialogflowV2Beta1ConversationModelState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2Beta1ConversationModelState::Creating => "CREATING",
                GoogleCloudDialogflowV2Beta1ConversationModelState::Deleting => "DELETING",
                GoogleCloudDialogflowV2Beta1ConversationModelState::Deployed => "DEPLOYED",
                GoogleCloudDialogflowV2Beta1ConversationModelState::Deploying => "DEPLOYING",
                GoogleCloudDialogflowV2Beta1ConversationModelState::Failed => "FAILED",
                GoogleCloudDialogflowV2Beta1ConversationModelState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudDialogflowV2Beta1ConversationModelState::Undeployed => "UNDEPLOYED",
                GoogleCloudDialogflowV2Beta1ConversationModelState::Undeploying => "UNDEPLOYING",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1ConversationModelState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1ConversationModelState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2Beta1ConversationModelState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => GoogleCloudDialogflowV2Beta1ConversationModelState::Creating,
                "DELETING" => GoogleCloudDialogflowV2Beta1ConversationModelState::Deleting,
                "DEPLOYED" => GoogleCloudDialogflowV2Beta1ConversationModelState::Deployed,
                "DEPLOYING" => GoogleCloudDialogflowV2Beta1ConversationModelState::Deploying,
                "FAILED" => GoogleCloudDialogflowV2Beta1ConversationModelState::Failed,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2Beta1ConversationModelState::StateUnspecified
                }
                "UNDEPLOYED" => GoogleCloudDialogflowV2Beta1ConversationModelState::Undeployed,
                "UNDEPLOYING" => GoogleCloudDialogflowV2Beta1ConversationModelState::Undeploying,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1ConversationModelState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1ConversationModelState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1EntityType {
        #[doc = "Optional. Indicates whether the entity type can be automatically\nexpanded."]
        #[serde(rename = "autoExpansionMode", default)]
        pub auto_expansion_mode: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode,
        >,
        #[doc = "Required. The name of the entity type."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of entity entries associated with the entity type."]
        #[serde(rename = "entities", default)]
        pub entities: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1EntityTypeEntity>,
        >,
        #[doc = "Required. Indicates the kind of entity type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1EntityTypeKind>,
        #[doc = "The unique identifier of the entity type.\nRequired for EntityTypes.UpdateEntityType and\nEntityTypes.BatchUpdateEntityTypes methods.\nFormat: `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1EntityType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1EntityType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode {
        #[doc = "Allows an agent to recognize values that have not been explicitly\nlisted in the entity."]
        AutoExpansionModeDefault,
        #[doc = "Auto expansion disabled for the entity."]
        AutoExpansionModeUnspecified,
    }
    impl GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode :: AutoExpansionModeDefault => "AUTO_EXPANSION_MODE_DEFAULT" , GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode :: AutoExpansionModeUnspecified => "AUTO_EXPANSION_MODE_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "AUTO_EXPANSION_MODE_DEFAULT" => GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode :: AutoExpansionModeDefault , "AUTO_EXPANSION_MODE_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode :: AutoExpansionModeUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1EntityTypeAutoExpansionMode
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1EntityTypeKind {
        #[doc = "List entity types contain a set of entries that do not map to canonical\nvalues. However, list entity types can contain references to other entity\ntypes (with or without aliases)."]
        KindList,
        #[doc = "Map entity types allow mapping of a group of synonyms to a canonical\nvalue."]
        KindMap,
        #[doc = "Not specified. This value should be never used."]
        KindUnspecified,
    }
    impl GoogleCloudDialogflowV2Beta1EntityTypeKind {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2Beta1EntityTypeKind::KindList => "KIND_LIST",
                GoogleCloudDialogflowV2Beta1EntityTypeKind::KindMap => "KIND_MAP",
                GoogleCloudDialogflowV2Beta1EntityTypeKind::KindUnspecified => "KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1EntityTypeKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1EntityTypeKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2Beta1EntityTypeKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "KIND_LIST" => GoogleCloudDialogflowV2Beta1EntityTypeKind::KindList,
                "KIND_MAP" => GoogleCloudDialogflowV2Beta1EntityTypeKind::KindMap,
                "KIND_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1EntityTypeKind::KindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1EntityTypeKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1EntityTypeKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1EntityTypeEntity {
        #[doc = "Required. A collection of value synonyms. For example, if the entity type\nis *vegetable*, and `value` is *scallions*, a synonym could be *green\nonions*.\n\nFor `KIND_LIST` entity types:\n\n* This collection must contain exactly one synonym equal to `value`."]
        #[serde(rename = "synonyms", default)]
        pub synonyms: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The primary value associated with this entity entry.\nFor example, if the entity type is *vegetable*, the value could be\n*scallions*.\n\nFor `KIND_MAP` entity types:\n\n* A canonical value to be used in place of synonyms.\n\nFor `KIND_LIST` entity types:\n\n* A string that can contain references to other entity types (with or\n  without aliases)."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1EntityTypeEntity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1EntityTypeEntity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1EventInput {
        #[doc = "Required. The language of this query. See [Language\nSupport](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes. Note that queries in\nthe same session do not necessarily need to specify the same language."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Required. The unique identifier of the event."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of parameters associated with the event."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1EventInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1EventInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1ExportAgentResponse {
        #[doc = "The exported agent.\n\nExample for how to export an agent to a zip file via a command line:\n\n<pre>curl \\\n  'https://dialogflow.googleapis.com/v2beta1/projects/&lt;project_name&gt;/agent:export'\\\n  -X POST \\\n  -H 'Authorization: Bearer '$(gcloud auth application-default\n  print-access-token) \\\n  -H 'Accept: application/json' \\\n  -H 'Content-Type: application/json' \\\n  --compressed \\\n  --data-binary '{}' \\\n| grep agentContent | sed -e 's/.*\"agentContent\": \"\\([^\"]*\\)\".*/\\1/' \\\n| base64 --decode > &lt;agent zip file&gt;</pre>"]
        #[serde(rename = "agentContent", default)]
        pub agent_content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The URI to a file containing the exported agent. This field is populated\nonly if `agent_uri` is specified in `ExportAgentRequest`."]
        #[serde(rename = "agentUri", default)]
        pub agent_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1ExportAgentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1ExportAgentResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1InputDataset {
        #[doc = "Required. ConversationDataset resource name. Format:\n`projects/<Project ID>/conversationDatasets/<Conversation Dataset ID>`\nor\n`projects/<Project ID>/conversationDatasets/<Conversation Dataset ID>/annotatedConversationDatasets/<Annotated Conversation Dataset ID>`"]
        #[serde(rename = "dataset", default)]
        pub dataset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1InputDataset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1InputDataset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1Intent {
        #[doc = "Optional. The name of the action associated with the intent.\nNote: The action name must not contain whitespaces."]
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<String>,
        #[doc = "Optional. The list of platforms for which the first responses will be\ncopied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform)."]
        #[serde(rename = "defaultResponsePlatforms", default)]
        pub default_response_platforms: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems>,
        >,
        #[doc = "Required. The name of this intent."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. Indicates that this intent ends an interaction. Some integrations\n(e.g., Actions on Google or Dialogflow phone gateway) use this information\nto close interaction with an end user. Default is false."]
        #[serde(rename = "endInteraction", default)]
        pub end_interaction: ::std::option::Option<bool>,
        #[doc = "Optional. The collection of event names that trigger the intent.\nIf the collection of input contexts is not empty, all of the contexts must\nbe present in the active user session for an event to trigger this intent."]
        #[serde(rename = "events", default)]
        pub events: ::std::option::Option<Vec<String>>,
        #[doc = "Read-only. Information about all followup intents that have this intent as\na direct or indirect parent. We populate this field only in the output."]
        #[serde(rename = "followupIntentInfo", default)]
        pub followup_intent_info: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentFollowupIntentInfo>,
        >,
        #[doc = "Optional. The list of context names required for this intent to be\ntriggered.\nFormat: `projects/<Project ID>/agent/sessions/-/contexts/<Context ID>`."]
        #[serde(rename = "inputContextNames", default)]
        pub input_context_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Indicates whether this is a fallback intent."]
        #[serde(rename = "isFallback", default)]
        pub is_fallback: ::std::option::Option<bool>,
        #[doc = "Optional. The collection of rich messages corresponding to the\n`Response` field in the Dialogflow console."]
        #[serde(rename = "messages", default)]
        pub messages:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessage>>,
        #[doc = "Optional. Indicates whether Machine Learning is disabled for the intent.\nNote: If `ml_disabled` setting is set to true, then this intent is not\ntaken into account during inference in `ML ONLY` match mode. Also,\nauto-markup in the UI is turned off."]
        #[serde(rename = "mlDisabled", default)]
        pub ml_disabled: ::std::option::Option<bool>,
        #[doc = "Optional. Indicates whether Machine Learning is enabled for the intent.\nNote: If `ml_enabled` setting is set to false, then this intent is not\ntaken into account during inference in `ML ONLY` match mode. Also,\nauto-markup in the UI is turned off.\nDEPRECATED! Please use `ml_disabled` field instead.\nNOTE: If both `ml_enabled` and `ml_disabled` are either not set or false,\nthen the default value is determined as follows:\n\n* Before April 15th, 2018 the default is:\n  ml_enabled = false / ml_disabled = true.\n* After April 15th, 2018 the default is:\n  ml_enabled = true / ml_disabled = false."]
        #[serde(rename = "mlEnabled", default)]
        pub ml_enabled: ::std::option::Option<bool>,
        #[doc = "The unique identifier of this intent.\nRequired for Intents.UpdateIntent and Intents.BatchUpdateIntents\nmethods.\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of contexts that are activated when the intent\nis matched. Context messages in this collection should not set the\nparameters field. Setting the `lifespan_count` to 0 will reset the context\nwhen the intent is matched.\nFormat: `projects/<Project ID>/agent/sessions/-/contexts/<Context ID>`."]
        #[serde(rename = "outputContexts", default)]
        pub output_contexts:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1Context>>,
        #[doc = "Optional. The collection of parameters associated with the intent."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentParameter>>,
        #[doc = "Read-only after creation. The unique identifier of the parent intent in the\nchain of followup intents. You can set this field when creating an intent,\nfor example with CreateIntent or BatchUpdateIntents, in order to\nmake this intent a followup intent.\n\nIt identifies the parent followup intent.\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "parentFollowupIntentName", default)]
        pub parent_followup_intent_name: ::std::option::Option<String>,
        #[doc = "Optional. The priority of this intent. Higher numbers represent higher\npriorities. If this is zero or unspecified, we use the default\npriority 500000.\n\nNegative numbers mean that the intent is disabled."]
        #[serde(rename = "priority", default)]
        pub priority: ::std::option::Option<i32>,
        #[doc = "Optional. Indicates whether to delete all contexts in the current\nsession when this intent is matched."]
        #[serde(rename = "resetContexts", default)]
        pub reset_contexts: ::std::option::Option<bool>,
        #[doc = "Read-only. The unique identifier of the root intent in the chain of\nfollowup intents. It identifies the correct followup intents chain for\nthis intent. We populate this field only in the output.\n\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "rootFollowupIntentName", default)]
        pub root_followup_intent_name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of examples that the agent is\ntrained on."]
        #[serde(rename = "trainingPhrases", default)]
        pub training_phrases: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentTrainingPhrase>,
        >,
        #[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
        #[serde(rename = "webhookState", default)]
        pub webhook_state:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1IntentWebhookState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1Intent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1Intent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems {
        ActionsOnGoogle,
        Facebook,
        GoogleHangouts,
        Kik,
        Line,
        PlatformUnspecified,
        Skype,
        Slack,
        Telegram,
        Telephony,
        Viber,
    }
    impl GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: ActionsOnGoogle => "ACTIONS_ON_GOOGLE" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Facebook => "FACEBOOK" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: GoogleHangouts => "GOOGLE_HANGOUTS" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Kik => "KIK" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Line => "LINE" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: PlatformUnspecified => "PLATFORM_UNSPECIFIED" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Skype => "SKYPE" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Slack => "SLACK" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Telegram => "TELEGRAM" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Telephony => "TELEPHONY" , GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Viber => "VIBER" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ACTIONS_ON_GOOGLE" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: ActionsOnGoogle , "FACEBOOK" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Facebook , "GOOGLE_HANGOUTS" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: GoogleHangouts , "KIK" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Kik , "LINE" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Line , "PLATFORM_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: PlatformUnspecified , "SKYPE" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Skype , "SLACK" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Slack , "TELEGRAM" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Telegram , "TELEPHONY" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Telephony , "VIBER" => GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems :: Viber , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentDefaultResponsePlatformsItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1IntentWebhookState {
        #[doc = "Webhook is enabled in the agent and in the intent."]
        WebhookStateEnabled,
        #[doc = "Webhook is enabled in the agent and in the intent. Also, each slot\nfilling prompt is forwarded to the webhook."]
        WebhookStateEnabledForSlotFilling,
        #[doc = "Webhook is disabled in the agent and in the intent."]
        WebhookStateUnspecified,
    }
    impl GoogleCloudDialogflowV2Beta1IntentWebhookState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2Beta1IntentWebhookState :: WebhookStateEnabled => "WEBHOOK_STATE_ENABLED" , GoogleCloudDialogflowV2Beta1IntentWebhookState :: WebhookStateEnabledForSlotFilling => "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING" , GoogleCloudDialogflowV2Beta1IntentWebhookState :: WebhookStateUnspecified => "WEBHOOK_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1IntentWebhookState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1IntentWebhookState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2Beta1IntentWebhookState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "WEBHOOK_STATE_ENABLED" => GoogleCloudDialogflowV2Beta1IntentWebhookState :: WebhookStateEnabled , "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING" => GoogleCloudDialogflowV2Beta1IntentWebhookState :: WebhookStateEnabledForSlotFilling , "WEBHOOK_STATE_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1IntentWebhookState :: WebhookStateUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentWebhookState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentWebhookState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentFollowupIntentInfo {
        #[doc = "The unique identifier of the followup intent.\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "followupIntentName", default)]
        pub followup_intent_name: ::std::option::Option<String>,
        #[doc = "The unique identifier of the followup intent's parent.\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "parentFollowupIntentName", default)]
        pub parent_followup_intent_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentFollowupIntentInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentFollowupIntentInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessage {
        #[doc = "Displays a basic card for Actions on Google."]
        #[serde(rename = "basicCard", default)]
        pub basic_card: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageBasicCard,
        >,
        #[doc = "Displays a card."]
        #[serde(rename = "card", default)]
        pub card:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageCard>,
        #[doc = "Displays a carousel card for Actions on Google."]
        #[serde(rename = "carouselSelect", default)]
        pub carousel_select: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageCarouselSelect,
        >,
        #[doc = "Displays an image."]
        #[serde(rename = "image", default)]
        pub image:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageImage>,
        #[doc = "Displays a link out suggestion chip for Actions on Google."]
        #[serde(rename = "linkOutSuggestion", default)]
        pub link_out_suggestion: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageLinkOutSuggestion,
        >,
        #[doc = "Displays a list card for Actions on Google."]
        #[serde(rename = "listSelect", default)]
        pub list_select: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageListSelect,
        >,
        #[doc = "Returns a response containing a custom, platform-specific payload.\nSee the Intent.Message.Platform type for a description of the\nstructure that may be required for your platform."]
        #[serde(rename = "payload", default)]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Optional. The platform that this message is intended for."]
        #[serde(rename = "platform", default)]
        pub platform: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessagePlatform,
        >,
        #[doc = "Displays quick replies."]
        #[serde(rename = "quickReplies", default)]
        pub quick_replies: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageQuickReplies,
        >,
        #[doc = "Rich Business Messaging (RBM) carousel rich card response."]
        #[serde(rename = "rbmCarouselRichCard", default)]
        pub rbm_carousel_rich_card: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCard,
        >,
        #[doc = "Standalone Rich Business Messaging (RBM) rich card response."]
        #[serde(rename = "rbmStandaloneRichCard", default)]
        pub rbm_standalone_rich_card: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCard,
        >,
        #[doc = "Rich Business Messaging (RBM) text response.\n\nRBM allows businesses to send enriched and branded versions of SMS. See\nhttps://jibe.google.com/business-messaging."]
        #[serde(rename = "rbmText", default)]
        pub rbm_text:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmText>,
        #[doc = "Returns a voice or text-only response for Actions on Google."]
        #[serde(rename = "simpleResponses", default)]
        pub simple_responses: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageSimpleResponses,
        >,
        #[doc = "Displays suggestion chips for Actions on Google."]
        #[serde(rename = "suggestions", default)]
        pub suggestions: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageSuggestions,
        >,
        #[doc = "Plays audio from a file in Telephony Gateway."]
        #[serde(rename = "telephonyPlayAudio", default)]
        pub telephony_play_audio: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageTelephonyPlayAudio,
        >,
        #[doc = "Synthesizes speech in Telephony Gateway."]
        #[serde(rename = "telephonySynthesizeSpeech", default)]
        pub telephony_synthesize_speech: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageTelephonySynthesizeSpeech,
        >,
        #[doc = "Transfers the call in Telephony Gateway."]
        #[serde(rename = "telephonyTransferCall", default)]
        pub telephony_transfer_call: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageTelephonyTransferCall,
        >,
        #[doc = "Returns a text response."]
        #[serde(rename = "text", default)]
        pub text:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageText>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1IntentMessagePlatform {
        #[doc = "Actions on Google.\nWhen using Actions on Google, you can choose one of the specific\nIntent.Message types that mention support for Actions on Google,\nor you can use the advanced Intent.Message.payload field.\nThe payload field provides access to AoG features not available in the\nspecific message types.\nIf using the Intent.Message.payload field, it should have a structure\nsimilar to the JSON message shown here. For more information, see\n[Actions on Google Webhook\nFormat](https://developers.google.com/actions/dialogflow/webhook)\n\n<pre>{\n  \"expectUserResponse\": true,\n  \"isSsml\": false,\n  \"noInputPrompts\": [],\n  \"richResponse\": {\n    \"items\": [\n      {\n        \"simpleResponse\": {\n          \"displayText\": \"hi\",\n          \"textToSpeech\": \"hello\"\n        }\n      }\n    ],\n    \"suggestions\": [\n      {\n        \"title\": \"Say this\"\n      },\n      {\n        \"title\": \"or this\"\n      }\n    ]\n  },\n  \"systemIntent\": {\n    \"data\": {\n      \"@type\": \"type.googleapis.com/google.actions.v2.OptionValueSpec\",\n      \"listSelect\": {\n        \"items\": [\n          {\n            \"optionInfo\": {\n              \"key\": \"key1\",\n              \"synonyms\": [\n                \"key one\"\n              ]\n            },\n            \"title\": \"must not be empty, but unique\"\n          },\n          {\n            \"optionInfo\": {\n              \"key\": \"key2\",\n              \"synonyms\": [\n                \"key two\"\n              ]\n            },\n            \"title\": \"must not be empty, but unique\"\n          }\n        ]\n      }\n    },\n    \"intent\": \"actions.intent.OPTION\"\n  }\n}</pre>"]
        ActionsOnGoogle,
        #[doc = "Facebook."]
        Facebook,
        #[doc = "Google Hangouts."]
        GoogleHangouts,
        #[doc = "Kik."]
        Kik,
        #[doc = "Line."]
        Line,
        #[doc = "Not specified."]
        PlatformUnspecified,
        #[doc = "Skype."]
        Skype,
        #[doc = "Slack."]
        Slack,
        #[doc = "Telegram."]
        Telegram,
        #[doc = "Telephony Gateway."]
        Telephony,
        #[doc = "Viber."]
        Viber,
    }
    impl GoogleCloudDialogflowV2Beta1IntentMessagePlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::ActionsOnGoogle => {
                    "ACTIONS_ON_GOOGLE"
                }
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Facebook => "FACEBOOK",
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::GoogleHangouts => {
                    "GOOGLE_HANGOUTS"
                }
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Kik => "KIK",
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Line => "LINE",
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::PlatformUnspecified => {
                    "PLATFORM_UNSPECIFIED"
                }
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Skype => "SKYPE",
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Slack => "SLACK",
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Telegram => "TELEGRAM",
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Telephony => "TELEPHONY",
                GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Viber => "VIBER",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1IntentMessagePlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1IntentMessagePlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2Beta1IntentMessagePlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIONS_ON_GOOGLE" => {
                    GoogleCloudDialogflowV2Beta1IntentMessagePlatform::ActionsOnGoogle
                }
                "FACEBOOK" => GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Facebook,
                "GOOGLE_HANGOUTS" => {
                    GoogleCloudDialogflowV2Beta1IntentMessagePlatform::GoogleHangouts
                }
                "KIK" => GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Kik,
                "LINE" => GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Line,
                "PLATFORM_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2Beta1IntentMessagePlatform::PlatformUnspecified
                }
                "SKYPE" => GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Skype,
                "SLACK" => GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Slack,
                "TELEGRAM" => GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Telegram,
                "TELEPHONY" => GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Telephony,
                "VIBER" => GoogleCloudDialogflowV2Beta1IntentMessagePlatform::Viber,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentMessagePlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessagePlatform {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageBasicCard {
        #[doc = "Optional. The collection of card buttons."]
        #[serde(rename = "buttons", default)]
        pub buttons: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageBasicCardButton>,
        >,
        #[doc = "Required, unless image is present. The body text of the card."]
        #[serde(rename = "formattedText", default)]
        pub formatted_text: ::std::option::Option<String>,
        #[doc = "Optional. The image for the card."]
        #[serde(rename = "image", default)]
        pub image:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageImage>,
        #[doc = "Optional. The subtitle of the card."]
        #[serde(rename = "subtitle", default)]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "Optional. The title of the card."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentMessageBasicCard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageBasicCard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageBasicCardButton {
        #[doc = "Required. Action to take when a user taps on the button."]
        #[serde(rename = "openUriAction", default)]
        pub open_uri_action: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageBasicCardButtonOpenUriAction,
        >,
        #[doc = "Required. The title of the button."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageBasicCardButton
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageBasicCardButton
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageBasicCardButtonOpenUriAction {
        #[doc = "Required. The HTTP or HTTPS scheme URI."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageBasicCardButtonOpenUriAction
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageBasicCardButtonOpenUriAction
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageCard {
        #[doc = "Optional. The collection of card buttons."]
        #[serde(rename = "buttons", default)]
        pub buttons: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageCardButton>,
        >,
        #[doc = "Optional. The public URI to an image file for the card."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
        #[doc = "Optional. The subtitle of the card."]
        #[serde(rename = "subtitle", default)]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "Optional. The title of the card."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentMessageCard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageCard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageCardButton {
        #[doc = "Optional. The text to send back to the Dialogflow API or a URI to\nopen."]
        #[serde(rename = "postback", default)]
        pub postback: ::std::option::Option<String>,
        #[doc = "Optional. The text to show on the button."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageCardButton
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageCardButton {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageCarouselSelect {
        #[doc = "Required. Carousel items."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageCarouselSelectItem>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageCarouselSelect
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageCarouselSelect
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageCarouselSelectItem {
        #[doc = "Optional. The body text of the card."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. The image to display."]
        #[serde(rename = "image", default)]
        pub image:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageImage>,
        #[doc = "Required. Additional info about the option item."]
        #[serde(rename = "info", default)]
        pub info: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageSelectItemInfo,
        >,
        #[doc = "Required. Title of the carousel item."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageCarouselSelectItem
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageCarouselSelectItem
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageImage {
        #[doc = "A text description of the image to be used for accessibility,\ne.g., screen readers. Required if image_uri is set for CarouselSelect."]
        #[serde(rename = "accessibilityText", default)]
        pub accessibility_text: ::std::option::Option<String>,
        #[doc = "Optional. The public URI to an image file."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentMessageImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageImage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageLinkOutSuggestion {
        #[doc = "Required. The name of the app or site this chip is linking to."]
        #[serde(rename = "destinationName", default)]
        pub destination_name: ::std::option::Option<String>,
        #[doc = "Required. The URI of the app or site to open when the user taps the\nsuggestion chip."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageLinkOutSuggestion
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageLinkOutSuggestion
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageListSelect {
        #[doc = "Required. List items."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageListSelectItem>,
        >,
        #[doc = "Optional. The overall title of the list."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageListSelect
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageListSelect {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageListSelectItem {
        #[doc = "Optional. The main text describing the item."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. The image to display."]
        #[serde(rename = "image", default)]
        pub image:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageImage>,
        #[doc = "Required. Additional information about this option."]
        #[serde(rename = "info", default)]
        pub info: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageSelectItemInfo,
        >,
        #[doc = "Required. The title of the list item."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageListSelectItem
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageListSelectItem
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageQuickReplies {
        #[doc = "Optional. The collection of quick replies."]
        #[serde(rename = "quickReplies", default)]
        pub quick_replies: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The title of the collection of quick replies."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageQuickReplies
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageQuickReplies
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContent {
        #[doc = "Optional. Description of the card (at most 2000 bytes).\n\nAt least one of the title, description or media must be set."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. However at least one of the title, description or media must\nbe set. Media (image, GIF or a video) to include in the card."]
        #[serde(rename = "media", default)]
        pub media: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMedia,
        >,
        #[doc = "Optional. List of suggestions to include in the card."]
        #[serde(rename = "suggestions", default)]
        pub suggestions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestion>,
        >,
        #[doc = "Optional. Title of the card (at most 200 bytes).\n\nAt least one of the title, description or media must be set."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContent
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContent
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMedia {
        #[doc = "Required. Publicly reachable URI of the file. The RBM platform\ndetermines the MIME type of the file from the content-type field in\nthe HTTP headers when the platform fetches the file. The content-type\nfield must be present and accurate in the HTTP response from the URL."]
        #[serde(rename = "fileUri", default)]
        pub file_uri: ::std::option::Option<String>,
        #[doc = "Required for cards with vertical orientation. The height of the media\nwithin a rich card with a vertical layout. (https://goo.gl/NeFCjz).\nFor a standalone card with horizontal layout, height is not\ncustomizable, and this field is ignored."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight,
        >,
        #[doc = "Optional. Publicly reachable URI of the thumbnail.If you don't\nprovide a thumbnail URI, the RBM platform displays a blank\nplaceholder thumbnail until the user's device downloads the file.\nDepending on the user's setting, the file may not download\nautomatically and may require the user to tap a download button."]
        #[serde(rename = "thumbnailUri", default)]
        pub thumbnail_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMedia
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMedia
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight {
        #[doc = "Not specified."]
        HeightUnspecified,
        #[doc = "168 DP."]
        Medium,
        #[doc = "112 DP."]
        Short,
        #[doc = "264 DP. Not available for rich card carousels when the card width\nis set to small."]
        Tall,
    }
    impl GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight :: HeightUnspecified => "HEIGHT_UNSPECIFIED" , GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight :: Medium => "MEDIUM" , GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight :: Short => "SHORT" , GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight :: Tall => "TALL" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "HEIGHT_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight :: HeightUnspecified , "MEDIUM" => GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight :: Medium , "SHORT" => GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight :: Short , "TALL" => GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight :: Tall , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContentRbmMediaHeight
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCard {
        #[doc = "Required. The cards in the carousel. A carousel must have at least\n2 cards and at most 10."]
        #[serde(rename = "cardContents", default)]
        pub card_contents: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContent>,
        >,
        #[doc = "Required. The width of the cards in the carousel."]
        #[serde(rename = "cardWidth", default)]
        pub card_width: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCard
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCard
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth {
        #[doc = "Not specified."]
        CardWidthUnspecified,
        #[doc = "232 DP."]
        Medium,
        #[doc = "120 DP. Note that tall media cannot be used."]
        Small,
    }
    impl GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth :: CardWidthUnspecified => "CARD_WIDTH_UNSPECIFIED" , GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth :: Medium => "MEDIUM" , GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth :: Small => "SMALL" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "CARD_WIDTH_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth :: CardWidthUnspecified , "MEDIUM" => GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth :: Medium , "SMALL" => GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth :: Small , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmCarouselCardCardWidth
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCard { # [ doc = "Required. Card content." ] # [ serde ( rename = "cardContent" , default ) ] pub card_content : :: std :: option :: Option < crate :: schemas :: GoogleCloudDialogflowV2Beta1IntentMessageRbmCardContent > , # [ doc = "Required. Orientation of the card." ] # [ serde ( rename = "cardOrientation" , default ) ] pub card_orientation : :: std :: option :: Option < crate :: schemas :: GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation > , # [ doc = "Required if orientation is horizontal.\nImage preview alignment for standalone cards with horizontal layout." ] # [ serde ( rename = "thumbnailImageAlignment" , default ) ] pub thumbnail_image_alignment : :: std :: option :: Option < crate :: schemas :: GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCard
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCard
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation {
        #[doc = "Not specified."]
        CardOrientationUnspecified,
        #[doc = "Horizontal layout."]
        Horizontal,
        #[doc = "Vertical layout."]
        Vertical,
    }
    impl GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation :: CardOrientationUnspecified => "CARD_ORIENTATION_UNSPECIFIED" , GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation :: Horizontal => "HORIZONTAL" , GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation :: Vertical => "VERTICAL" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "CARD_ORIENTATION_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation :: CardOrientationUnspecified , "HORIZONTAL" => GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation :: Horizontal , "VERTICAL" => GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation :: Vertical , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardCardOrientation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment {
        #[doc = "Thumbnail preview is left-aligned."]
        Left,
        #[doc = "Thumbnail preview is right-aligned."]
        Right,
        #[doc = "Not specified."]
        ThumbnailImageAlignmentUnspecified,
    }
    impl GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment :: Left => "LEFT" , GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment :: Right => "RIGHT" , GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment :: ThumbnailImageAlignmentUnspecified => "THUMBNAIL_IMAGE_ALIGNMENT_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "LEFT" => GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment :: Left , "RIGHT" => GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment :: Right , "THUMBNAIL_IMAGE_ALIGNMENT_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment :: ThumbnailImageAlignmentUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmStandaloneCardThumbnailImageAlignment
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedAction { # [ doc = "Suggested client side action: Dial a phone number" ] # [ serde ( rename = "dial" , default ) ] pub dial : :: std :: option :: Option < crate :: schemas :: GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial > , # [ doc = "Suggested client side action: Open a URI on device" ] # [ serde ( rename = "openUrl" , default ) ] pub open_url : :: std :: option :: Option < crate :: schemas :: GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri > , # [ doc = "Opaque payload that the Dialogflow receives in a user event\nwhen the user taps the suggested action. This data will be also\nforwarded to webhook to allow performing custom business logic." ] # [ serde ( rename = "postbackData" , default ) ] pub postback_data : :: std :: option :: Option < String > , # [ doc = "Suggested client side action: Share user location" ] # [ serde ( rename = "shareLocation" , default ) ] pub share_location : :: std :: option :: Option < crate :: schemas :: GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation > , # [ doc = "Text to display alongside the action." ] # [ serde ( rename = "text" , default ) ] pub text : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedAction
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedAction
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial {
        #[doc = "Required. The phone number to fill in the default dialer app.\nThis field should be in [E.164](https://en.wikipedia.org/wiki/E.164)\nformat. An example of a correctly formatted phone number:\n+15556767888."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri {
        #[doc = "Required. The uri to open on the user device"]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation;
    impl :: google_field_selector :: FieldSelector for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation { fn fields ( ) -> Vec < :: google_field_selector :: Field > { Vec :: new ( ) } }
    impl :: google_field_selector :: ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation { fn field_type ( ) -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedReply {
        #[doc = "Opaque payload that the Dialogflow receives in a user event\nwhen the user taps the suggested reply. This data will be also\nforwarded to webhook to allow performing custom business logic."]
        #[serde(rename = "postbackData", default)]
        pub postback_data: ::std::option::Option<String>,
        #[doc = "Suggested reply text."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedReply
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedReply
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestion {
        #[doc = "Predefined client side actions that user can choose"]
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedAction,
        >,
        #[doc = "Predefined replies for user to select instead of typing"]
        #[serde(rename = "reply", default)]
        pub reply: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestedReply,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestion
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestion
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageRbmText {
        #[doc = "Optional. One or more suggestions to show to the user."]
        #[serde(rename = "rbmSuggestion", default)]
        pub rbm_suggestion: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageRbmSuggestion>,
        >,
        #[doc = "Required. Text sent and displayed to the user."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentMessageRbmText {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageRbmText {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageSelectItemInfo {
        #[doc = "Required. A unique key that will be sent back to the agent if this\nresponse is given."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "Optional. A list of synonyms that can also be used to trigger this\nitem in dialog."]
        #[serde(rename = "synonyms", default)]
        pub synonyms: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageSelectItemInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageSelectItemInfo
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageSimpleResponse {
        #[doc = "Optional. The text to display."]
        #[serde(rename = "displayText", default)]
        pub display_text: ::std::option::Option<String>,
        #[doc = "One of text_to_speech or ssml must be provided. Structured spoken\nresponse to the user in the SSML format. Mutually exclusive with\ntext_to_speech."]
        #[serde(rename = "ssml", default)]
        pub ssml: ::std::option::Option<String>,
        #[doc = "One of text_to_speech or ssml must be provided. The plain text of the\nspeech output. Mutually exclusive with ssml."]
        #[serde(rename = "textToSpeech", default)]
        pub text_to_speech: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageSimpleResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageSimpleResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageSimpleResponses {
        #[doc = "Required. The list of simple responses."]
        #[serde(rename = "simpleResponses", default)]
        pub simple_responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageSimpleResponse>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageSimpleResponses
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageSimpleResponses
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageSuggestion {
        #[doc = "Required. The text shown the in the suggestion chip."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageSuggestion
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageSuggestion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageSuggestions {
        #[doc = "Required. The list of suggested replies."]
        #[serde(rename = "suggestions", default)]
        pub suggestions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessageSuggestion>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageSuggestions
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageSuggestions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageTelephonyPlayAudio {
        #[doc = "Required. URI to a Google Cloud Storage object containing the audio to\nplay, e.g., \"gs://bucket/object\". The object must contain a single\nchannel (mono) of linear PCM audio (2 bytes / sample) at 8kHz.\n\nThis object must be readable by the `service-<Project Number>@gcp-sa-dialogflow.iam.gserviceaccount.com` service account\nwhere <Project Number> is the number of the Telephony Gateway project\n(usually the same as the Dialogflow agent project). If the Google Cloud\nStorage bucket is in the Telephony Gateway project, this permission is\nadded by default when enabling the Dialogflow V2 API.\n\nFor audio from other sources, consider using the\n`TelephonySynthesizeSpeech` message with SSML."]
        #[serde(rename = "audioUri", default)]
        pub audio_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageTelephonyPlayAudio
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageTelephonyPlayAudio
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageTelephonySynthesizeSpeech {
        #[doc = "The SSML to be synthesized. For more information, see\n[SSML](https://developers.google.com/actions/reference/ssml)."]
        #[serde(rename = "ssml", default)]
        pub ssml: ::std::option::Option<String>,
        #[doc = "The raw text to be synthesized."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageTelephonySynthesizeSpeech
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageTelephonySynthesizeSpeech
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageTelephonyTransferCall {
        #[doc = "Required. The phone number to transfer the call to\nin [E.164 format](https://en.wikipedia.org/wiki/E.164).\n\nWe currently only allow transferring to US numbers (+1xxxyyyzzzz)."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentMessageTelephonyTransferCall
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1IntentMessageTelephonyTransferCall
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentMessageText {
        #[doc = "Optional. The collection of the agent's responses."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentMessageText {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentMessageText {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentParameter {
        #[doc = "Optional. The default value to use when the `value` yields an empty\nresult.\nDefault values can be extracted from contexts by using the following\nsyntax: `#context_name.parameter_name`."]
        #[serde(rename = "defaultValue", default)]
        pub default_value: ::std::option::Option<String>,
        #[doc = "Required. The name of the parameter."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. The name of the entity type, prefixed with `@`, that\ndescribes values of the parameter. If the parameter is\nrequired, this must be provided."]
        #[serde(rename = "entityTypeDisplayName", default)]
        pub entity_type_display_name: ::std::option::Option<String>,
        #[doc = "Optional. Indicates whether the parameter represents a list of values."]
        #[serde(rename = "isList", default)]
        pub is_list: ::std::option::Option<bool>,
        #[doc = "Optional. Indicates whether the parameter is required. That is,\nwhether the intent cannot be completed without collecting the parameter\nvalue."]
        #[serde(rename = "mandatory", default)]
        pub mandatory: ::std::option::Option<bool>,
        #[doc = "The unique identifier of this parameter."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of prompts that the agent can present to the\nuser in order to collect a value for the parameter."]
        #[serde(rename = "prompts", default)]
        pub prompts: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The definition of the parameter value. It can be:\n\n* a constant string,\n* a parameter value defined as `$parameter_name`,\n* an original parameter value defined as `$parameter_name.original`,\n* a parameter value from some context defined as\n  `#context_name.parameter_name`."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentParameter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentTrainingPhrase {
        #[doc = "Output only. The unique identifier of this training phrase."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The ordered list of training phrase parts.\nThe parts are concatenated in order to form the training phrase.\n\nNote: The API does not automatically annotate training phrases like the\nDialogflow Console does.\n\nNote: Do not forget to include whitespace at part boundaries,\nso the training phrase is well formatted when the parts are concatenated.\n\nIf the training phrase does not need to be annotated with parameters,\nyou just need a single part with only the Part.text field set.\n\nIf you want to annotate the training phrase, you must create multiple\nparts, where the fields of each part are populated in one of two ways:\n\n* `Part.text` is set to a part of the phrase that has no parameters.\n* `Part.text` is set to a part of the phrase that you want to annotate,\n  and the `entity_type`, `alias`, and `user_defined` fields are all\n  set."]
        #[serde(rename = "parts", default)]
        pub parts: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentTrainingPhrasePart>,
        >,
        #[doc = "Required. The type of the training phrase."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType,
        >,
        #[doc = "Optional. Indicates how many times this example was added to\nthe intent. Each time a developer adds an existing sample by editing an\nintent or training, this counter is increased."]
        #[serde(rename = "timesAddedCount", default)]
        pub times_added_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1IntentTrainingPhrase {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentTrainingPhrase {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType {
        #[doc = "Examples do not contain @-prefixed entity type names, but example parts\ncan be annotated with entity types."]
        Example,
        #[doc = "Templates are not annotated with entity types, but they can contain\n@-prefixed entity type names as substrings.\nTemplate mode has been deprecated. Example mode is the only supported\nway to create new training phrases. If you have existing training\nphrases that you've created in template mode, those will continue to\nwork."]
        Template,
        #[doc = "Not specified. This value should never be used."]
        TypeUnspecified,
    }
    impl GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType::Example => "EXAMPLE",
                GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType::Template => "TEMPLATE",
                GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType::TypeUnspecified => {
                    "TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXAMPLE" => GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType::Example,
                "TEMPLATE" => GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType::Template,
                "TYPE_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType::TypeUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentTrainingPhraseType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1IntentTrainingPhrasePart {
        #[doc = "Optional. The parameter name for the value extracted from the\nannotated part of the example.\nThis field is required for annotated parts of the training phrase."]
        #[serde(rename = "alias", default)]
        pub alias: ::std::option::Option<String>,
        #[doc = "Optional. The entity type name prefixed with `@`.\nThis field is required for annotated parts of the training phrase."]
        #[serde(rename = "entityType", default)]
        pub entity_type: ::std::option::Option<String>,
        #[doc = "Required. The text for this part."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
        #[doc = "Optional. Indicates whether the text was manually annotated.\nThis field is set to true when the Dialogflow Console is used to\nmanually annotate the part. When creating an annotated part with the\nAPI, you must set this to true."]
        #[serde(rename = "userDefined", default)]
        pub user_defined: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1IntentTrainingPhrasePart
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1IntentTrainingPhrasePart {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1KnowledgeAnswers {
        #[doc = "A list of answers from Knowledge Connector."]
        #[serde(rename = "answers", default)]
        pub answers: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswer>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1KnowledgeAnswers {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1KnowledgeAnswers {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswer {
        #[doc = "The piece of text from the `source` knowledge base document that answers\nthis conversational query."]
        #[serde(rename = "answer", default)]
        pub answer: ::std::option::Option<String>,
        #[doc = "The corresponding FAQ question if the answer was extracted from a FAQ\nDocument, empty otherwise."]
        #[serde(rename = "faqQuestion", default)]
        pub faq_question: ::std::option::Option<String>,
        #[doc = "The system's confidence score that this Knowledge answer is a good match\nfor this conversational query.\nThe range is from 0.0 (completely uncertain) to 1.0 (completely certain).\nNote: The confidence score is likely to vary somewhat (possibly even for\nidentical requests), as the underlying model is under constant\nimprovement. It may be deprecated in the future. We recommend using\n`match_confidence_level` which should be generally more stable."]
        #[serde(rename = "matchConfidence", default)]
        pub match_confidence: ::std::option::Option<f32>,
        #[doc = "The system's confidence level that this knowledge answer is a good match\nfor this conversational query.\nNOTE: The confidence level for a given `<query, answer>` pair may change\nwithout notice, as it depends on models that are constantly being\nimproved. However, it will change less frequently than the confidence\nscore below, and should be preferred for referencing the quality of an\nanswer."]
        #[serde(rename = "matchConfidenceLevel", default)]
        pub match_confidence_level: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel,
        >,
        #[doc = "Indicates which Knowledge Document this answer was extracted from.\nFormat: `projects/<Project ID>/knowledgeBases/<Knowledge Base ID>/documents/<Document ID>`."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel {
        #[doc = "Indicates our confidence is high."]
        High,
        #[doc = "Indicates that the confidence is low."]
        Low,
        #[doc = "Not specified."]
        MatchConfidenceLevelUnspecified,
        #[doc = "Indicates our confidence is medium."]
        Medium,
    }
    impl GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel :: High => "HIGH" , GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel :: Low => "LOW" , GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel :: MatchConfidenceLevelUnspecified => "MATCH_CONFIDENCE_LEVEL_UNSPECIFIED" , GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel :: Medium => "MEDIUM" , }
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "HIGH" => GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel :: High , "LOW" => GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel :: Low , "MATCH_CONFIDENCE_LEVEL_UNSPECIFIED" => GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel :: MatchConfidenceLevelUnspecified , "MEDIUM" => GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel :: Medium , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1KnowledgeAnswersAnswerMatchConfidenceLevel
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadata {
        #[doc = "Required. The current state of this operation."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState {
        #[doc = "The operation is done, either cancelled or completed."]
        Done,
        #[doc = "The operation has been created."]
        Pending,
        #[doc = "The operation is currently running."]
        Running,
        #[doc = "State unspecified."]
        StateUnspecified,
    }
    impl GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState::Done => "DONE",
                GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState::Pending => "PENDING",
                GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState::Running => "RUNNING",
                GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DONE" => GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState::Done,
                "PENDING" => GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState::Pending,
                "RUNNING" => GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState::StateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1KnowledgeOperationMetadataState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1LabelConversationResponse {
        #[doc = "New annotated conversation dataset created by the labeling task."]
        #[serde(rename = "annotatedConversationDataset", default)]
        pub annotated_conversation_dataset: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1AnnotatedConversationDataset,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1LabelConversationResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1LabelConversationResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1OriginalDetectIntentRequest {
        #[doc = "Optional. This field is set to the value of the `QueryParameters.payload`\nfield passed in the request. Some integrations that query a Dialogflow\nagent may provide additional information in the payload.\n\nIn particular for the Telephony Gateway this field has the form:\n\n<pre>{\n \"telephony\": {\n   \"caller_id\": \"+18558363987\"\n }\n}</pre>\n\nNote: The caller ID field (`caller_id`) will be redacted for Standard\nEdition agents and populated with the caller ID in [E.164\nformat](https://en.wikipedia.org/wiki/E.164) for Enterprise Edition agents."]
        #[serde(rename = "payload", default)]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The source of this request, e.g., `google`, `facebook`, `slack`. It is set\nby Dialogflow-owned servers."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<String>,
        #[doc = "Optional. The version of the protocol used for this request.\nThis field is AoG-specific."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1OriginalDetectIntentRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2Beta1OriginalDetectIntentRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1QueryResult {
        #[doc = "The action name from the matched intent."]
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<String>,
        #[doc = "This field is set to:\n\n* `false` if the matched intent has required parameters and not all of\n  the required parameter values have been collected.\n* `true` if all required parameter values have been collected, or if the\n  matched intent doesn't contain any required parameters."]
        #[serde(rename = "allRequiredParamsPresent", default)]
        pub all_required_params_present: ::std::option::Option<bool>,
        #[doc = "The free-form diagnostic info. For example, this field could contain\nwebhook call latency. The string keys of the Struct's fields map can change\nwithout notice."]
        #[serde(rename = "diagnosticInfo", default)]
        pub diagnostic_info:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The collection of rich messages to present to the user."]
        #[serde(rename = "fulfillmentMessages", default)]
        pub fulfillment_messages:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessage>>,
        #[doc = "The text to be pronounced to the user or shown on the screen.\nNote: This is a legacy field, `fulfillment_messages` should be preferred."]
        #[serde(rename = "fulfillmentText", default)]
        pub fulfillment_text: ::std::option::Option<String>,
        #[doc = "The intent that matched the conversational query. Some, not\nall fields are filled in this message, including but not limited to:\n`name`, `display_name`, `end_interaction` and `is_fallback`."]
        #[serde(rename = "intent", default)]
        pub intent: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1Intent>,
        #[doc = "The intent detection confidence. Values range from 0.0\n(completely uncertain) to 1.0 (completely certain).\nThis value is for informational purpose only and is only used to\nhelp match the best intent within the classification threshold.\nThis value may change for the same end-user expression at any time due to a\nmodel retraining or change in implementation.\nIf there are `multiple knowledge_answers` messages, this value is set to\nthe greatest `knowledgeAnswers.match_confidence` value in the list."]
        #[serde(rename = "intentDetectionConfidence", default)]
        pub intent_detection_confidence: ::std::option::Option<f32>,
        #[doc = "The result from Knowledge Connector (if any), ordered by decreasing\n`KnowledgeAnswers.match_confidence`."]
        #[serde(rename = "knowledgeAnswers", default)]
        pub knowledge_answers:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1KnowledgeAnswers>,
        #[doc = "The language that was triggered during intent detection.\nSee [Language\nSupport](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "The collection of output contexts. If applicable,\n`output_contexts.parameters` contains entries with name\n`<parameter name>.original` containing the original parameter values\nbefore the query."]
        #[serde(rename = "outputContexts", default)]
        pub output_contexts:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1Context>>,
        #[doc = "The collection of extracted parameters."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The original conversational query text:\n\n* If natural language text was provided as input, `query_text` contains\n  a copy of the input.\n* If natural language speech audio was provided as input, `query_text`\n  contains the speech recognition result. If speech recognizer produced\n  multiple alternatives, a particular one is picked.\n* If automatic spell correction is enabled, `query_text` will contain the\n  corrected user input."]
        #[serde(rename = "queryText", default)]
        pub query_text: ::std::option::Option<String>,
        #[doc = "The sentiment analysis result, which depends on the\n`sentiment_analysis_request_config` specified in the request."]
        #[serde(rename = "sentimentAnalysisResult", default)]
        pub sentiment_analysis_result: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1SentimentAnalysisResult,
        >,
        #[doc = "The Speech recognition confidence between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. The default of 0.0 is a sentinel value indicating that confidence\nwas not set.\n\nThis field is not guaranteed to be accurate or set. In particular this\nfield isn't set for StreamingDetectIntent since the streaming endpoint has\nseparate confidence estimates per portion of the audio in\nStreamingRecognitionResult."]
        #[serde(rename = "speechRecognitionConfidence", default)]
        pub speech_recognition_confidence: ::std::option::Option<f32>,
        #[doc = "If the query was fulfilled by a webhook call, this field is set to the\nvalue of the `payload` field returned in the webhook response."]
        #[serde(rename = "webhookPayload", default)]
        pub webhook_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "If the query was fulfilled by a webhook call, this field is set to the\nvalue of the `source` field returned in the webhook response."]
        #[serde(rename = "webhookSource", default)]
        pub webhook_source: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1QueryResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1QueryResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1Sentiment {
        #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute\nmagnitude of sentiment, regardless of score (positive or negative)."]
        #[serde(rename = "magnitude", default)]
        pub magnitude: ::std::option::Option<f32>,
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive\nsentiment)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1Sentiment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1Sentiment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Beta1SentimentAnalysisResult {
        #[doc = "The sentiment analysis result for `query_text`."]
        #[serde(rename = "queryTextSentiment", default)]
        pub query_text_sentiment:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1Sentiment>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2Beta1SentimentAnalysisResult
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1SentimentAnalysisResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1WebhookRequest {
        #[doc = "Alternative query results from KnowledgeService."]
        #[serde(rename = "alternativeQueryResults", default)]
        pub alternative_query_results:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1QueryResult>>,
        #[doc = "Optional. The contents of the original request that was passed to\n`[Streaming]DetectIntent` call."]
        #[serde(rename = "originalDetectIntentRequest", default)]
        pub original_detect_intent_request: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2Beta1OriginalDetectIntentRequest,
        >,
        #[doc = "The result of the conversational query or event processing. Contains the\nsame value as `[Streaming]DetectIntentResponse.query_result`."]
        #[serde(rename = "queryResult", default)]
        pub query_result:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1QueryResult>,
        #[doc = "The unique identifier of the response. Contains the same value as\n`[Streaming]DetectIntentResponse.response_id`."]
        #[serde(rename = "responseId", default)]
        pub response_id: ::std::option::Option<String>,
        #[doc = "The unique identifier of detectIntent request session.\nCan be used to identify end-user inside webhook implementation.\nFormat: `projects/<Project ID>/agent/sessions/<Session ID>`, or\n`projects/<Project ID>/agent/environments/<Environment ID>/users/<User ID>/sessions/<Session ID>`."]
        #[serde(rename = "session", default)]
        pub session: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1WebhookRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1WebhookRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Beta1WebhookResponse {
        #[doc = "Optional. Indicates that this intent ends an interaction. Some integrations\n(e.g., Actions on Google or Dialogflow phone gateway) use this information\nto close interaction with an end user. Default is false."]
        #[serde(rename = "endInteraction", default)]
        pub end_interaction: ::std::option::Option<bool>,
        #[doc = "Optional. Makes the platform immediately invoke another `DetectIntent` call\ninternally with the specified event as input.\nWhen this field is set, Dialogflow ignores the `fulfillment_text`,\n`fulfillment_messages`, and `payload` fields."]
        #[serde(rename = "followupEventInput", default)]
        pub followup_event_input:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Beta1EventInput>,
        #[doc = "Optional. The collection of rich messages to present to the user. This\nvalue is passed directly to `QueryResult.fulfillment_messages`."]
        #[serde(rename = "fulfillmentMessages", default)]
        pub fulfillment_messages:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1IntentMessage>>,
        #[doc = "Optional. The text to be shown on the screen. This value is passed directly\nto `QueryResult.fulfillment_text`."]
        #[serde(rename = "fulfillmentText", default)]
        pub fulfillment_text: ::std::option::Option<String>,
        #[doc = "Optional. The collection of output contexts. This value is passed directly\nto `QueryResult.output_contexts`."]
        #[serde(rename = "outputContexts", default)]
        pub output_contexts:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Beta1Context>>,
        #[doc = "Optional. This value is passed directly to `QueryResult.webhook_payload`.\nSee the related `fulfillment_messages[i].payload field`, which may be used\nas an alternative to this field.\n\nThis field can be used for Actions on Google responses.\nIt should have a structure similar to the JSON message shown here. For more\ninformation, see\n[Actions on Google Webhook\nFormat](https://developers.google.com/actions/dialogflow/webhook)\n\n<pre>{\n  \"google\": {\n    \"expectUserResponse\": true,\n    \"richResponse\": {\n      \"items\": [\n        {\n          \"simpleResponse\": {\n            \"textToSpeech\": \"this is a simple response\"\n          }\n        }\n      ]\n    }\n  }\n}</pre>"]
        #[serde(rename = "payload", default)]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Optional. This value is passed directly to `QueryResult.webhook_source`."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Beta1WebhookResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Beta1WebhookResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Context {
        #[doc = "Optional. The number of conversational query requests after which the\ncontext expires. If set to `0` (the default) the context expires\nimmediately. Contexts expire automatically after 20 minutes if there\nare no matching queries."]
        #[serde(rename = "lifespanCount", default)]
        pub lifespan_count: ::std::option::Option<i32>,
        #[doc = "Required. The unique identifier of the context. Format:\n`projects/<Project ID>/agent/sessions/<Session ID>/contexts/<Context ID>`.\n\nThe `Context ID` is always converted to lowercase, may only contain\ncharacters in [a-zA-Z0-9_-%] and may be at most 250 bytes long."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of parameters associated with this context.\nRefer to [this\ndoc](https://cloud.google.com/dialogflow/docs/intents-actions-parameters)\nfor syntax."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Context {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Context {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2DetectIntentRequest {
        #[doc = "Optional. The natural language speech audio to be processed. This field\nshould be populated iff `query_input` is set to an input audio config.\nA single request can contain up to 1 minute of speech audio data."]
        #[serde(rename = "inputAudio", default)]
        pub input_audio: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Optional. Instructs the speech synthesizer how to generate the output\naudio. If this field is not set and agent-level speech synthesizer is not\nconfigured, no output audio is generated."]
        #[serde(rename = "outputAudioConfig", default)]
        pub output_audio_config:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2OutputAudioConfig>,
        #[doc = "Required. The input specification. It can be set to:\n\n1. an audio config\n   which instructs the speech recognizer how to process the speech audio,\n\n1. a conversational query in the form of text, or\n\n1. an event that specifies which intent to trigger."]
        #[serde(rename = "queryInput", default)]
        pub query_input: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2QueryInput>,
        #[doc = "Optional. The parameters of this query."]
        #[serde(rename = "queryParams", default)]
        pub query_params:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2QueryParameters>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2DetectIntentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2DetectIntentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2DetectIntentResponse {
        #[doc = "The audio data bytes encoded as specified in the request.\nNote: The output audio is generated based on the values of default platform\ntext responses found in the `query_result.fulfillment_messages` field. If\nmultiple default text responses exist, they will be concatenated when\ngenerating audio. If no default platform text responses exist, the\ngenerated audio content will be empty."]
        #[serde(rename = "outputAudio", default)]
        pub output_audio: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The config used by the speech synthesizer to generate the output audio."]
        #[serde(rename = "outputAudioConfig", default)]
        pub output_audio_config:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2OutputAudioConfig>,
        #[doc = "The selected results of the conversational query or event processing.\nSee `alternative_query_results` for additional potential results."]
        #[serde(rename = "queryResult", default)]
        pub query_result: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2QueryResult>,
        #[doc = "The unique identifier of the response. It can be used to\nlocate a response in the training example set or for reporting issues."]
        #[serde(rename = "responseId", default)]
        pub response_id: ::std::option::Option<String>,
        #[doc = "Specifies the status of the webhook request."]
        #[serde(rename = "webhookStatus", default)]
        pub webhook_status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2DetectIntentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2DetectIntentResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2EntityType {
        #[doc = "Optional. Indicates whether the entity type can be automatically\nexpanded."]
        #[serde(rename = "autoExpansionMode", default)]
        pub auto_expansion_mode: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2EntityTypeAutoExpansionMode,
        >,
        #[doc = "Required. The name of the entity type."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of entity entries associated with the entity type."]
        #[serde(rename = "entities", default)]
        pub entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2EntityTypeEntity>>,
        #[doc = "Required. Indicates the kind of entity type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2EntityTypeKind>,
        #[doc = "The unique identifier of the entity type.\nRequired for EntityTypes.UpdateEntityType and\nEntityTypes.BatchUpdateEntityTypes methods.\nFormat: `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2EntityType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2EntityType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2EntityTypeAutoExpansionMode {
        #[doc = "Allows an agent to recognize values that have not been explicitly\nlisted in the entity."]
        AutoExpansionModeDefault,
        #[doc = "Auto expansion disabled for the entity."]
        AutoExpansionModeUnspecified,
    }
    impl GoogleCloudDialogflowV2EntityTypeAutoExpansionMode {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2EntityTypeAutoExpansionMode :: AutoExpansionModeDefault => "AUTO_EXPANSION_MODE_DEFAULT" , GoogleCloudDialogflowV2EntityTypeAutoExpansionMode :: AutoExpansionModeUnspecified => "AUTO_EXPANSION_MODE_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2EntityTypeAutoExpansionMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2EntityTypeAutoExpansionMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2EntityTypeAutoExpansionMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTO_EXPANSION_MODE_DEFAULT" => {
                    GoogleCloudDialogflowV2EntityTypeAutoExpansionMode::AutoExpansionModeDefault
                }
                "AUTO_EXPANSION_MODE_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2EntityTypeAutoExpansionMode::AutoExpansionModeUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2EntityTypeAutoExpansionMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2EntityTypeAutoExpansionMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2EntityTypeKind {
        #[doc = "List entity types contain a set of entries that do not map to canonical\nvalues. However, list entity types can contain references to other entity\ntypes (with or without aliases)."]
        KindList,
        #[doc = "Map entity types allow mapping of a group of synonyms to a canonical\nvalue."]
        KindMap,
        #[doc = "Not specified. This value should be never used."]
        KindUnspecified,
    }
    impl GoogleCloudDialogflowV2EntityTypeKind {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2EntityTypeKind::KindList => "KIND_LIST",
                GoogleCloudDialogflowV2EntityTypeKind::KindMap => "KIND_MAP",
                GoogleCloudDialogflowV2EntityTypeKind::KindUnspecified => "KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2EntityTypeKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2EntityTypeKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2EntityTypeKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "KIND_LIST" => GoogleCloudDialogflowV2EntityTypeKind::KindList,
                "KIND_MAP" => GoogleCloudDialogflowV2EntityTypeKind::KindMap,
                "KIND_UNSPECIFIED" => GoogleCloudDialogflowV2EntityTypeKind::KindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2EntityTypeKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2EntityTypeKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2EntityTypeBatch {
        #[doc = "A collection of entity types."]
        #[serde(rename = "entityTypes", default)]
        pub entity_types:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2EntityType>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2EntityTypeBatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2EntityTypeBatch {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2EntityTypeEntity {
        #[doc = "Required. A collection of value synonyms. For example, if the entity type\nis *vegetable*, and `value` is *scallions*, a synonym could be *green\nonions*.\n\nFor `KIND_LIST` entity types:\n\n* This collection must contain exactly one synonym equal to `value`."]
        #[serde(rename = "synonyms", default)]
        pub synonyms: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The primary value associated with this entity entry.\nFor example, if the entity type is *vegetable*, the value could be\n*scallions*.\n\nFor `KIND_MAP` entity types:\n\n* A canonical value to be used in place of synonyms.\n\nFor `KIND_LIST` entity types:\n\n* A string that can contain references to other entity types (with or\n  without aliases)."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2EntityTypeEntity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2EntityTypeEntity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2EventInput {
        #[doc = "Required. The language of this query. See [Language\nSupport](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes. Note that queries in\nthe same session do not necessarily need to specify the same language."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Required. The unique identifier of the event."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of parameters associated with the event."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2EventInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2EventInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2ExportAgentRequest {
        #[doc = "Optional. The\n[Google Cloud Storage](https://cloud.google.com/storage/docs/)\nURI to export the agent to.\nThe format of this URI must be `gs://<bucket-name>/<object-name>`.\nIf left unspecified, the serialized agent is returned inline."]
        #[serde(rename = "agentUri", default)]
        pub agent_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2ExportAgentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2ExportAgentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2ExportAgentResponse {
        #[doc = "The exported agent.\n\nExample for how to export an agent to a zip file via a command line:\n\n<pre>curl \\\n  'https://dialogflow.googleapis.com/v2/projects/&lt;project_name&gt;/agent:export'\\\n  -X POST \\\n  -H 'Authorization: Bearer '$(gcloud auth application-default\n  print-access-token) \\\n  -H 'Accept: application/json' \\\n  -H 'Content-Type: application/json' \\\n  --compressed \\\n  --data-binary '{}' \\\n| grep agentContent | sed -e 's/.*\"agentContent\": \"\\([^\"]*\\)\".*/\\1/' \\\n| base64 --decode > &lt;agent zip file&gt;</pre>"]
        #[serde(rename = "agentContent", default)]
        pub agent_content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The URI to a file containing the exported agent. This field is populated\nonly if `agent_uri` is specified in `ExportAgentRequest`."]
        #[serde(rename = "agentUri", default)]
        pub agent_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2ExportAgentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2ExportAgentResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2ImportAgentRequest {
        #[doc = "The agent to import.\n\nExample for how to import an agent via the command line:\n\n<pre>curl \\\n  'https://dialogflow.googleapis.com/v2/projects/&lt;project_name&gt;/agent:import\\\n   -X POST \\\n   -H 'Authorization: Bearer '$(gcloud auth application-default\n   print-access-token) \\\n   -H 'Accept: application/json' \\\n   -H 'Content-Type: application/json' \\\n   --compressed \\\n   --data-binary \"{\n      'agentContent': '$(cat &lt;agent zip file&gt; | base64 -w 0)'\n   }\"</pre>"]
        #[serde(rename = "agentContent", default)]
        pub agent_content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The URI to a Google Cloud Storage file containing the agent to import.\nNote: The URI must start with \"gs://\"."]
        #[serde(rename = "agentUri", default)]
        pub agent_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2ImportAgentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2ImportAgentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2InputAudioConfig {
        #[doc = "Required. Audio encoding of the audio content to process."]
        #[serde(rename = "audioEncoding", default)]
        pub audio_encoding: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2InputAudioConfigAudioEncoding,
        >,
        #[doc = "Required. The language of the supplied audio. Dialogflow does not do\ntranslations. See [Language\nSupport](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes. Note that queries in\nthe same session do not necessarily need to specify the same language."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. Which variant of the Speech model to use."]
        #[serde(rename = "modelVariant", default)]
        pub model_variant: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2InputAudioConfigModelVariant,
        >,
        #[doc = "Optional. A list of strings containing words and phrases that the speech\nrecognizer should recognize with higher likelihood.\n\nSee [the Cloud Speech\ndocumentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints)\nfor more details."]
        #[serde(rename = "phraseHints", default)]
        pub phrase_hints: ::std::option::Option<Vec<String>>,
        #[doc = "Required. Sample rate (in Hertz) of the audio content sent in the query.\nRefer to\n[Cloud Speech API\ndocumentation](https://cloud.google.com/speech-to-text/docs/basics) for\nmore details."]
        #[serde(rename = "sampleRateHertz", default)]
        pub sample_rate_hertz: ::std::option::Option<i32>,
        #[doc = "Optional. If `false` (default), recognition does not cease until the\nclient closes the stream.\nIf `true`, the recognizer will detect a single spoken utterance in input\naudio. Recognition ceases when it detects the audio's voice has\nstopped or paused. In this case, once a detected intent is received, the\nclient should close the stream and start a new request with a new stream as\nneeded.\nNote: This setting is relevant only for streaming methods.\nNote: When specified, InputAudioConfig.single_utterance takes precedence\nover StreamingDetectIntentRequest.single_utterance."]
        #[serde(rename = "singleUtterance", default)]
        pub single_utterance: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2InputAudioConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2InputAudioConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2InputAudioConfigAudioEncoding {
        #[doc = "Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000."]
        AudioEncodingAmr,
        #[doc = "Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000."]
        AudioEncodingAmrWb,
        #[doc = "[`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio\nCodec) is the recommended encoding because it is lossless (therefore\nrecognition is not compromised) and requires only about half the\nbandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and\n24-bit samples, however, not all fields in `STREAMINFO` are supported."]
        AudioEncodingFlac,
        #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM)."]
        AudioEncodingLinear16,
        #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law."]
        AudioEncodingMulaw,
        #[doc = "Opus encoded audio frames in Ogg container\n([OggOpus](https://wiki.xiph.org/OggOpus)).\n`sample_rate_hertz` must be 16000."]
        AudioEncodingOggOpus,
        #[doc = "Although the use of lossy encodings is not recommended, if a very low\nbitrate encoding is required, `OGG_OPUS` is highly preferred over\nSpeex encoding. The [Speex](https://speex.org/) encoding supported by\nDialogflow API has a header byte in each block, as in MIME type\n`audio/x-speex-with-header-byte`.\nIt is a variant of the RTP Speex encoding defined in\n[RFC 5574](https://tools.ietf.org/html/rfc5574).\nThe stream is a sequence of blocks, one block per RTP packet. Each block\nstarts with a byte containing the length of the block, in bytes, followed\nby one or more frames of Speex data, padded to an integral number of\nbytes (octets) as specified in RFC 5574. In other words, each RTP header\nis replaced with a single byte containing the block length. Only Speex\nwideband is supported. `sample_rate_hertz` must be 16000."]
        AudioEncodingSpeexWithHeaderByte,
        #[doc = "Not specified."]
        AudioEncodingUnspecified,
    }
    impl GoogleCloudDialogflowV2InputAudioConfigAudioEncoding {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingAmr => "AUDIO_ENCODING_AMR" , GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingAmrWb => "AUDIO_ENCODING_AMR_WB" , GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingFlac => "AUDIO_ENCODING_FLAC" , GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingLinear16 => "AUDIO_ENCODING_LINEAR_16" , GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingMulaw => "AUDIO_ENCODING_MULAW" , GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingOggOpus => "AUDIO_ENCODING_OGG_OPUS" , GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingSpeexWithHeaderByte => "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE" , GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingUnspecified => "AUDIO_ENCODING_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2InputAudioConfigAudioEncoding {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2InputAudioConfigAudioEncoding {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2InputAudioConfigAudioEncoding {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "AUDIO_ENCODING_AMR" => GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingAmr , "AUDIO_ENCODING_AMR_WB" => GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingAmrWb , "AUDIO_ENCODING_FLAC" => GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingFlac , "AUDIO_ENCODING_LINEAR_16" => GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingLinear16 , "AUDIO_ENCODING_MULAW" => GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingMulaw , "AUDIO_ENCODING_OGG_OPUS" => GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingOggOpus , "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE" => GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingSpeexWithHeaderByte , "AUDIO_ENCODING_UNSPECIFIED" => GoogleCloudDialogflowV2InputAudioConfigAudioEncoding :: AudioEncodingUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2InputAudioConfigAudioEncoding
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2InputAudioConfigAudioEncoding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2InputAudioConfigModelVariant {
        #[doc = "No model variant specified. In this case Dialogflow defaults to\nUSE_BEST_AVAILABLE."]
        SpeechModelVariantUnspecified,
        #[doc = "Use the best available variant of the Speech\nmodel that the caller is eligible for.\n\nPlease see the [Dialogflow\ndocs](https://cloud.google.com/dialogflow/docs/data-logging) for\nhow to make your project eligible for enhanced models."]
        UseBestAvailable,
        #[doc = "Use an enhanced model variant:\n\n* If an enhanced variant does not exist for the given\n  model and request language, Dialogflow falls\n  back to the standard variant.\n  \n  The [Cloud Speech\n  documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)\n  describes which models have enhanced variants.\n\n* If the API caller isn't eligible for enhanced models, Dialogflow returns\n  an error. Please see the [Dialogflow\n  docs](https://cloud.google.com/dialogflow/docs/data-logging)\n  for how to make your project eligible."]
        UseEnhanced,
        #[doc = "Use standard model variant even if an enhanced model is available.  See the\n[Cloud Speech\ndocumentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)\nfor details about enhanced models."]
        UseStandard,
    }
    impl GoogleCloudDialogflowV2InputAudioConfigModelVariant {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2InputAudioConfigModelVariant :: SpeechModelVariantUnspecified => "SPEECH_MODEL_VARIANT_UNSPECIFIED" , GoogleCloudDialogflowV2InputAudioConfigModelVariant :: UseBestAvailable => "USE_BEST_AVAILABLE" , GoogleCloudDialogflowV2InputAudioConfigModelVariant :: UseEnhanced => "USE_ENHANCED" , GoogleCloudDialogflowV2InputAudioConfigModelVariant :: UseStandard => "USE_STANDARD" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2InputAudioConfigModelVariant {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2InputAudioConfigModelVariant {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2InputAudioConfigModelVariant {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "SPEECH_MODEL_VARIANT_UNSPECIFIED" => GoogleCloudDialogflowV2InputAudioConfigModelVariant :: SpeechModelVariantUnspecified , "USE_BEST_AVAILABLE" => GoogleCloudDialogflowV2InputAudioConfigModelVariant :: UseBestAvailable , "USE_ENHANCED" => GoogleCloudDialogflowV2InputAudioConfigModelVariant :: UseEnhanced , "USE_STANDARD" => GoogleCloudDialogflowV2InputAudioConfigModelVariant :: UseStandard , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2InputAudioConfigModelVariant
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2InputAudioConfigModelVariant {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2Intent {
        #[doc = "Optional. The name of the action associated with the intent.\nNote: The action name must not contain whitespaces."]
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<String>,
        #[doc = "Optional. The list of platforms for which the first responses will be\ncopied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform)."]
        #[serde(rename = "defaultResponsePlatforms", default)]
        pub default_response_platforms: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems>,
        >,
        #[doc = "Required. The name of this intent."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of event names that trigger the intent.\nIf the collection of input contexts is not empty, all of the contexts must\nbe present in the active user session for an event to trigger this intent."]
        #[serde(rename = "events", default)]
        pub events: ::std::option::Option<Vec<String>>,
        #[doc = "Read-only. Information about all followup intents that have this intent as\na direct or indirect parent. We populate this field only in the output."]
        #[serde(rename = "followupIntentInfo", default)]
        pub followup_intent_info: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentFollowupIntentInfo>,
        >,
        #[doc = "Optional. The list of context names required for this intent to be\ntriggered.\nFormat: `projects/<Project ID>/agent/sessions/-/contexts/<Context ID>`."]
        #[serde(rename = "inputContextNames", default)]
        pub input_context_names: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Indicates whether this is a fallback intent."]
        #[serde(rename = "isFallback", default)]
        pub is_fallback: ::std::option::Option<bool>,
        #[doc = "Optional. The collection of rich messages corresponding to the\n`Response` field in the Dialogflow console."]
        #[serde(rename = "messages", default)]
        pub messages:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessage>>,
        #[doc = "Optional. Indicates whether Machine Learning is disabled for the intent.\nNote: If `ml_diabled` setting is set to true, then this intent is not\ntaken into account during inference in `ML ONLY` match mode. Also,\nauto-markup in the UI is turned off."]
        #[serde(rename = "mlDisabled", default)]
        pub ml_disabled: ::std::option::Option<bool>,
        #[doc = "The unique identifier of this intent.\nRequired for Intents.UpdateIntent and Intents.BatchUpdateIntents\nmethods.\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of contexts that are activated when the intent\nis matched. Context messages in this collection should not set the\nparameters field. Setting the `lifespan_count` to 0 will reset the context\nwhen the intent is matched.\nFormat: `projects/<Project ID>/agent/sessions/-/contexts/<Context ID>`."]
        #[serde(rename = "outputContexts", default)]
        pub output_contexts:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Context>>,
        #[doc = "Optional. The collection of parameters associated with the intent."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2IntentParameter>>,
        #[doc = "Read-only after creation. The unique identifier of the parent intent in the\nchain of followup intents. You can set this field when creating an intent,\nfor example with CreateIntent or BatchUpdateIntents, in order to\nmake this intent a followup intent.\n\nIt identifies the parent followup intent.\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "parentFollowupIntentName", default)]
        pub parent_followup_intent_name: ::std::option::Option<String>,
        #[doc = "Optional. The priority of this intent. Higher numbers represent higher\npriorities. If this is zero or unspecified, we use the default\npriority 500000.\n\nNegative numbers mean that the intent is disabled."]
        #[serde(rename = "priority", default)]
        pub priority: ::std::option::Option<i32>,
        #[doc = "Optional. Indicates whether to delete all contexts in the current\nsession when this intent is matched."]
        #[serde(rename = "resetContexts", default)]
        pub reset_contexts: ::std::option::Option<bool>,
        #[doc = "Read-only. The unique identifier of the root intent in the chain of\nfollowup intents. It identifies the correct followup intents chain for\nthis intent. We populate this field only in the output.\n\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "rootFollowupIntentName", default)]
        pub root_followup_intent_name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of examples that the agent is\ntrained on."]
        #[serde(rename = "trainingPhrases", default)]
        pub training_phrases:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2IntentTrainingPhrase>>,
        #[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
        #[serde(rename = "webhookState", default)]
        pub webhook_state:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentWebhookState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Intent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Intent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems {
        ActionsOnGoogle,
        Facebook,
        GoogleHangouts,
        Kik,
        Line,
        PlatformUnspecified,
        Skype,
        Slack,
        Telegram,
        Viber,
    }
    impl GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::ActionsOnGoogle => {
                    "ACTIONS_ON_GOOGLE"
                }
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Facebook => "FACEBOOK",
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::GoogleHangouts => {
                    "GOOGLE_HANGOUTS"
                }
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Kik => "KIK",
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Line => "LINE",
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::PlatformUnspecified => {
                    "PLATFORM_UNSPECIFIED"
                }
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Skype => "SKYPE",
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Slack => "SLACK",
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Telegram => "TELEGRAM",
                GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Viber => "VIBER",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIONS_ON_GOOGLE" => {
                    GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::ActionsOnGoogle
                }
                "FACEBOOK" => GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Facebook,
                "GOOGLE_HANGOUTS" => {
                    GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::GoogleHangouts
                }
                "KIK" => GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Kik,
                "LINE" => GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Line,
                "PLATFORM_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::PlatformUnspecified
                }
                "SKYPE" => GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Skype,
                "SLACK" => GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Slack,
                "TELEGRAM" => GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Telegram,
                "VIBER" => GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems::Viber,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2IntentDefaultResponsePlatformsItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2IntentWebhookState {
        #[doc = "Webhook is enabled in the agent and in the intent."]
        WebhookStateEnabled,
        #[doc = "Webhook is enabled in the agent and in the intent. Also, each slot\nfilling prompt is forwarded to the webhook."]
        WebhookStateEnabledForSlotFilling,
        #[doc = "Webhook is disabled in the agent and in the intent."]
        WebhookStateUnspecified,
    }
    impl GoogleCloudDialogflowV2IntentWebhookState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2IntentWebhookState::WebhookStateEnabled => {
                    "WEBHOOK_STATE_ENABLED"
                }
                GoogleCloudDialogflowV2IntentWebhookState::WebhookStateEnabledForSlotFilling => {
                    "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING"
                }
                GoogleCloudDialogflowV2IntentWebhookState::WebhookStateUnspecified => {
                    "WEBHOOK_STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2IntentWebhookState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2IntentWebhookState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2IntentWebhookState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "WEBHOOK_STATE_ENABLED" => {
                    GoogleCloudDialogflowV2IntentWebhookState::WebhookStateEnabled
                }
                "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING" => {
                    GoogleCloudDialogflowV2IntentWebhookState::WebhookStateEnabledForSlotFilling
                }
                "WEBHOOK_STATE_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2IntentWebhookState::WebhookStateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentWebhookState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentWebhookState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2IntentBatch {
        #[doc = "A collection of intents."]
        #[serde(rename = "intents", default)]
        pub intents: ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Intent>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentBatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentBatch {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentFollowupIntentInfo {
        #[doc = "The unique identifier of the followup intent.\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "followupIntentName", default)]
        pub followup_intent_name: ::std::option::Option<String>,
        #[doc = "The unique identifier of the followup intent's parent.\nFormat: `projects/<Project ID>/agent/intents/<Intent ID>`."]
        #[serde(rename = "parentFollowupIntentName", default)]
        pub parent_followup_intent_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentFollowupIntentInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentFollowupIntentInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2IntentMessage {
        #[doc = "The basic card response for Actions on Google."]
        #[serde(rename = "basicCard", default)]
        pub basic_card:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageBasicCard>,
        #[doc = "The card response."]
        #[serde(rename = "card", default)]
        pub card: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageCard>,
        #[doc = "The carousel card response for Actions on Google."]
        #[serde(rename = "carouselSelect", default)]
        pub carousel_select: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2IntentMessageCarouselSelect,
        >,
        #[doc = "The image response."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageImage>,
        #[doc = "The link out suggestion chip for Actions on Google."]
        #[serde(rename = "linkOutSuggestion", default)]
        pub link_out_suggestion: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion,
        >,
        #[doc = "The list card response for Actions on Google."]
        #[serde(rename = "listSelect", default)]
        pub list_select:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageListSelect>,
        #[doc = "Returns a response containing a custom, platform-specific payload.\nSee the Intent.Message.Platform type for a description of the\nstructure that may be required for your platform."]
        #[serde(rename = "payload", default)]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Optional. The platform that this message is intended for."]
        #[serde(rename = "platform", default)]
        pub platform:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessagePlatform>,
        #[doc = "The quick replies response."]
        #[serde(rename = "quickReplies", default)]
        pub quick_replies:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageQuickReplies>,
        #[doc = "The voice and text-only responses for Actions on Google."]
        #[serde(rename = "simpleResponses", default)]
        pub simple_responses: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2IntentMessageSimpleResponses,
        >,
        #[doc = "The suggestion chips for Actions on Google."]
        #[serde(rename = "suggestions", default)]
        pub suggestions:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageSuggestions>,
        #[doc = "The text response."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageText>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2IntentMessagePlatform {
        #[doc = "Actions on Google.\nWhen using Actions on Google, you can choose one of the specific\nIntent.Message types that mention support for Actions on Google,\nor you can use the advanced Intent.Message.payload field.\nThe payload field provides access to AoG features not available in the\nspecific message types.\nIf using the Intent.Message.payload field, it should have a structure\nsimilar to the JSON message shown here. For more information, see\n[Actions on Google Webhook\nFormat](https://developers.google.com/actions/dialogflow/webhook)\n\n<pre>{\n  \"expectUserResponse\": true,\n  \"isSsml\": false,\n  \"noInputPrompts\": [],\n  \"richResponse\": {\n    \"items\": [\n      {\n        \"simpleResponse\": {\n          \"displayText\": \"hi\",\n          \"textToSpeech\": \"hello\"\n        }\n      }\n    ],\n    \"suggestions\": [\n      {\n        \"title\": \"Say this\"\n      },\n      {\n        \"title\": \"or this\"\n      }\n    ]\n  },\n  \"systemIntent\": {\n    \"data\": {\n      \"@type\": \"type.googleapis.com/google.actions.v2.OptionValueSpec\",\n      \"listSelect\": {\n        \"items\": [\n          {\n            \"optionInfo\": {\n              \"key\": \"key1\",\n              \"synonyms\": [\n                \"key one\"\n              ]\n            },\n            \"title\": \"must not be empty, but unique\"\n          },\n          {\n            \"optionInfo\": {\n              \"key\": \"key2\",\n              \"synonyms\": [\n                \"key two\"\n              ]\n            },\n            \"title\": \"must not be empty, but unique\"\n          }\n        ]\n      }\n    },\n    \"intent\": \"actions.intent.OPTION\"\n  }\n}</pre>"]
        ActionsOnGoogle,
        #[doc = "Facebook."]
        Facebook,
        #[doc = "Google Hangouts."]
        GoogleHangouts,
        #[doc = "Kik."]
        Kik,
        #[doc = "Line."]
        Line,
        #[doc = "Not specified."]
        PlatformUnspecified,
        #[doc = "Skype."]
        Skype,
        #[doc = "Slack."]
        Slack,
        #[doc = "Telegram."]
        Telegram,
        #[doc = "Viber."]
        Viber,
    }
    impl GoogleCloudDialogflowV2IntentMessagePlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2IntentMessagePlatform::ActionsOnGoogle => {
                    "ACTIONS_ON_GOOGLE"
                }
                GoogleCloudDialogflowV2IntentMessagePlatform::Facebook => "FACEBOOK",
                GoogleCloudDialogflowV2IntentMessagePlatform::GoogleHangouts => "GOOGLE_HANGOUTS",
                GoogleCloudDialogflowV2IntentMessagePlatform::Kik => "KIK",
                GoogleCloudDialogflowV2IntentMessagePlatform::Line => "LINE",
                GoogleCloudDialogflowV2IntentMessagePlatform::PlatformUnspecified => {
                    "PLATFORM_UNSPECIFIED"
                }
                GoogleCloudDialogflowV2IntentMessagePlatform::Skype => "SKYPE",
                GoogleCloudDialogflowV2IntentMessagePlatform::Slack => "SLACK",
                GoogleCloudDialogflowV2IntentMessagePlatform::Telegram => "TELEGRAM",
                GoogleCloudDialogflowV2IntentMessagePlatform::Viber => "VIBER",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2IntentMessagePlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2IntentMessagePlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2IntentMessagePlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIONS_ON_GOOGLE" => {
                    GoogleCloudDialogflowV2IntentMessagePlatform::ActionsOnGoogle
                }
                "FACEBOOK" => GoogleCloudDialogflowV2IntentMessagePlatform::Facebook,
                "GOOGLE_HANGOUTS" => GoogleCloudDialogflowV2IntentMessagePlatform::GoogleHangouts,
                "KIK" => GoogleCloudDialogflowV2IntentMessagePlatform::Kik,
                "LINE" => GoogleCloudDialogflowV2IntentMessagePlatform::Line,
                "PLATFORM_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2IntentMessagePlatform::PlatformUnspecified
                }
                "SKYPE" => GoogleCloudDialogflowV2IntentMessagePlatform::Skype,
                "SLACK" => GoogleCloudDialogflowV2IntentMessagePlatform::Slack,
                "TELEGRAM" => GoogleCloudDialogflowV2IntentMessagePlatform::Telegram,
                "VIBER" => GoogleCloudDialogflowV2IntentMessagePlatform::Viber,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessagePlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessagePlatform {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageBasicCard {
        #[doc = "Optional. The collection of card buttons."]
        #[serde(rename = "buttons", default)]
        pub buttons: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessageBasicCardButton>,
        >,
        #[doc = "Required, unless image is present. The body text of the card."]
        #[serde(rename = "formattedText", default)]
        pub formatted_text: ::std::option::Option<String>,
        #[doc = "Optional. The image for the card."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageImage>,
        #[doc = "Optional. The subtitle of the card."]
        #[serde(rename = "subtitle", default)]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "Optional. The title of the card."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageBasicCard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageBasicCard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageBasicCardButton {
        #[doc = "Required. Action to take when a user taps on the button."]
        #[serde(rename = "openUriAction", default)]
        pub open_uri_action: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction,
        >,
        #[doc = "Required. The title of the button."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2IntentMessageBasicCardButton
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageBasicCardButton {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction {
        #[doc = "Required. The HTTP or HTTPS scheme URI."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageCard {
        #[doc = "Optional. The collection of card buttons."]
        #[serde(rename = "buttons", default)]
        pub buttons: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessageCardButton>,
        >,
        #[doc = "Optional. The public URI to an image file for the card."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
        #[doc = "Optional. The subtitle of the card."]
        #[serde(rename = "subtitle", default)]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "Optional. The title of the card."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageCard {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageCard {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageCardButton {
        #[doc = "Optional. The text to send back to the Dialogflow API or a URI to\nopen."]
        #[serde(rename = "postback", default)]
        pub postback: ::std::option::Option<String>,
        #[doc = "Optional. The text to show on the button."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageCardButton {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageCardButton {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageCarouselSelect {
        #[doc = "Required. Carousel items."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessageCarouselSelectItem>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageCarouselSelect {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageCarouselSelect {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageCarouselSelectItem {
        #[doc = "Optional. The body text of the card."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. The image to display."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageImage>,
        #[doc = "Required. Additional info about the option item."]
        #[serde(rename = "info", default)]
        pub info: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2IntentMessageSelectItemInfo,
        >,
        #[doc = "Required. Title of the carousel item."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2IntentMessageCarouselSelectItem
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2IntentMessageCarouselSelectItem
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageImage {
        #[doc = "Optional. A text description of the image to be used for accessibility,\ne.g., screen readers."]
        #[serde(rename = "accessibilityText", default)]
        pub accessibility_text: ::std::option::Option<String>,
        #[doc = "Optional. The public URI to an image file."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageImage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion {
        #[doc = "Required. The name of the app or site this chip is linking to."]
        #[serde(rename = "destinationName", default)]
        pub destination_name: ::std::option::Option<String>,
        #[doc = "Required. The URI of the app or site to open when the user taps the\nsuggestion chip."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageListSelect {
        #[doc = "Required. List items."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessageListSelectItem>,
        >,
        #[doc = "Optional. The overall title of the list."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageListSelect {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageListSelect {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageListSelectItem {
        #[doc = "Optional. The main text describing the item."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. The image to display."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentMessageImage>,
        #[doc = "Required. Additional information about this option."]
        #[serde(rename = "info", default)]
        pub info: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2IntentMessageSelectItemInfo,
        >,
        #[doc = "Required. The title of the list item."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageListSelectItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageListSelectItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageQuickReplies {
        #[doc = "Optional. The collection of quick replies."]
        #[serde(rename = "quickReplies", default)]
        pub quick_replies: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The title of the collection of quick replies."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageQuickReplies {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageQuickReplies {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageSelectItemInfo {
        #[doc = "Required. A unique key that will be sent back to the agent if this\nresponse is given."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "Optional. A list of synonyms that can also be used to trigger this\nitem in dialog."]
        #[serde(rename = "synonyms", default)]
        pub synonyms: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageSelectItemInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageSelectItemInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageSimpleResponse {
        #[doc = "Optional. The text to display."]
        #[serde(rename = "displayText", default)]
        pub display_text: ::std::option::Option<String>,
        #[doc = "One of text_to_speech or ssml must be provided. Structured spoken\nresponse to the user in the SSML format. Mutually exclusive with\ntext_to_speech."]
        #[serde(rename = "ssml", default)]
        pub ssml: ::std::option::Option<String>,
        #[doc = "One of text_to_speech or ssml must be provided. The plain text of the\nspeech output. Mutually exclusive with ssml."]
        #[serde(rename = "textToSpeech", default)]
        pub text_to_speech: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageSimpleResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageSimpleResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageSimpleResponses {
        #[doc = "Required. The list of simple responses."]
        #[serde(rename = "simpleResponses", default)]
        pub simple_responses: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessageSimpleResponse>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2IntentMessageSimpleResponses
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageSimpleResponses {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageSuggestion {
        #[doc = "Required. The text shown the in the suggestion chip."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageSuggestion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageSuggestion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageSuggestions {
        #[doc = "Required. The list of suggested replies."]
        #[serde(rename = "suggestions", default)]
        pub suggestions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessageSuggestion>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageSuggestions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageSuggestions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentMessageText {
        #[doc = "Optional. The collection of the agent's responses."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentMessageText {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentMessageText {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentParameter {
        #[doc = "Optional. The default value to use when the `value` yields an empty\nresult.\nDefault values can be extracted from contexts by using the following\nsyntax: `#context_name.parameter_name`."]
        #[serde(rename = "defaultValue", default)]
        pub default_value: ::std::option::Option<String>,
        #[doc = "Required. The name of the parameter."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Optional. The name of the entity type, prefixed with `@`, that\ndescribes values of the parameter. If the parameter is\nrequired, this must be provided."]
        #[serde(rename = "entityTypeDisplayName", default)]
        pub entity_type_display_name: ::std::option::Option<String>,
        #[doc = "Optional. Indicates whether the parameter represents a list of values."]
        #[serde(rename = "isList", default)]
        pub is_list: ::std::option::Option<bool>,
        #[doc = "Optional. Indicates whether the parameter is required. That is,\nwhether the intent cannot be completed without collecting the parameter\nvalue."]
        #[serde(rename = "mandatory", default)]
        pub mandatory: ::std::option::Option<bool>,
        #[doc = "The unique identifier of this parameter."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The collection of prompts that the agent can present to the\nuser in order to collect a value for the parameter."]
        #[serde(rename = "prompts", default)]
        pub prompts: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The definition of the parameter value. It can be:\n\n* a constant string,\n* a parameter value defined as `$parameter_name`,\n* an original parameter value defined as `$parameter_name.original`,\n* a parameter value from some context defined as\n  `#context_name.parameter_name`."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentParameter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentTrainingPhrase {
        #[doc = "Output only. The unique identifier of this training phrase."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The ordered list of training phrase parts.\nThe parts are concatenated in order to form the training phrase.\n\nNote: The API does not automatically annotate training phrases like the\nDialogflow Console does.\n\nNote: Do not forget to include whitespace at part boundaries,\nso the training phrase is well formatted when the parts are concatenated.\n\nIf the training phrase does not need to be annotated with parameters,\nyou just need a single part with only the Part.text field set.\n\nIf you want to annotate the training phrase, you must create multiple\nparts, where the fields of each part are populated in one of two ways:\n\n* `Part.text` is set to a part of the phrase that has no parameters.\n* `Part.text` is set to a part of the phrase that you want to annotate,\n  and the `entity_type`, `alias`, and `user_defined` fields are all\n  set."]
        #[serde(rename = "parts", default)]
        pub parts: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudDialogflowV2IntentTrainingPhrasePart>,
        >,
        #[doc = "Required. The type of the training phrase."]
        #[serde(rename = "type", default)]
        pub r#type:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2IntentTrainingPhraseType>,
        #[doc = "Optional. Indicates how many times this example was added to\nthe intent. Each time a developer adds an existing sample by editing an\nintent or training, this counter is increased."]
        #[serde(rename = "timesAddedCount", default)]
        pub times_added_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentTrainingPhrase {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentTrainingPhrase {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2IntentTrainingPhraseType {
        #[doc = "Examples do not contain @-prefixed entity type names, but example parts\ncan be annotated with entity types."]
        Example,
        #[doc = "Templates are not annotated with entity types, but they can contain\n@-prefixed entity type names as substrings.\nTemplate mode has been deprecated. Example mode is the only supported\nway to create new training phrases. If you have existing training\nphrases that you've created in template mode, those will continue to\nwork."]
        Template,
        #[doc = "Not specified. This value should never be used."]
        TypeUnspecified,
    }
    impl GoogleCloudDialogflowV2IntentTrainingPhraseType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudDialogflowV2IntentTrainingPhraseType::Example => "EXAMPLE",
                GoogleCloudDialogflowV2IntentTrainingPhraseType::Template => "TEMPLATE",
                GoogleCloudDialogflowV2IntentTrainingPhraseType::TypeUnspecified => {
                    "TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2IntentTrainingPhraseType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2IntentTrainingPhraseType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2IntentTrainingPhraseType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXAMPLE" => GoogleCloudDialogflowV2IntentTrainingPhraseType::Example,
                "TEMPLATE" => GoogleCloudDialogflowV2IntentTrainingPhraseType::Template,
                "TYPE_UNSPECIFIED" => {
                    GoogleCloudDialogflowV2IntentTrainingPhraseType::TypeUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentTrainingPhraseType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentTrainingPhraseType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2IntentTrainingPhrasePart {
        #[doc = "Optional. The parameter name for the value extracted from the\nannotated part of the example.\nThis field is required for annotated parts of the training phrase."]
        #[serde(rename = "alias", default)]
        pub alias: ::std::option::Option<String>,
        #[doc = "Optional. The entity type name prefixed with `@`.\nThis field is required for annotated parts of the training phrase."]
        #[serde(rename = "entityType", default)]
        pub entity_type: ::std::option::Option<String>,
        #[doc = "Required. The text for this part."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
        #[doc = "Optional. Indicates whether the text was manually annotated.\nThis field is set to true when the Dialogflow Console is used to\nmanually annotate the part. When creating an annotated part with the\nAPI, you must set this to true."]
        #[serde(rename = "userDefined", default)]
        pub user_defined: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2IntentTrainingPhrasePart {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2IntentTrainingPhrasePart {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2ListContextsResponse {
        #[doc = "The list of contexts. There will be a maximum number of items\nreturned based on the page_size field in the request."]
        #[serde(rename = "contexts", default)]
        pub contexts: ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Context>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2ListContextsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2ListContextsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2ListEntityTypesResponse {
        #[doc = "The list of agent entity types. There will be a maximum number of items\nreturned based on the page_size field in the request."]
        #[serde(rename = "entityTypes", default)]
        pub entity_types:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2EntityType>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2ListEntityTypesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2ListEntityTypesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2ListIntentsResponse {
        #[doc = "The list of agent intents. There will be a maximum number of items\nreturned based on the page_size field in the request."]
        #[serde(rename = "intents", default)]
        pub intents: ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Intent>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2ListIntentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2ListIntentsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2ListSessionEntityTypesResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of session entity types. There will be a maximum number of items\nreturned based on the page_size field in the request."]
        #[serde(rename = "sessionEntityTypes", default)]
        pub session_entity_types:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2SessionEntityType>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2ListSessionEntityTypesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2ListSessionEntityTypesResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2OriginalDetectIntentRequest {
        #[doc = "Optional. This field is set to the value of the `QueryParameters.payload`\nfield passed in the request. Some integrations that query a Dialogflow\nagent may provide additional information in the payload.\n\nIn particular for the Telephony Gateway this field has the form:\n\n<pre>{\n \"telephony\": {\n   \"caller_id\": \"+18558363987\"\n }\n}</pre>\n\nNote: The caller ID field (`caller_id`) will be redacted for Standard\nEdition agents and populated with the caller ID in [E.164\nformat](https://en.wikipedia.org/wiki/E.164) for Enterprise Edition agents."]
        #[serde(rename = "payload", default)]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The source of this request, e.g., `google`, `facebook`, `slack`. It is set\nby Dialogflow-owned servers."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<String>,
        #[doc = "Optional. The version of the protocol used for this request.\nThis field is AoG-specific."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2OriginalDetectIntentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2OriginalDetectIntentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2OutputAudioConfig {
        #[doc = "Required. Audio encoding of the synthesized audio content."]
        #[serde(rename = "audioEncoding", default)]
        pub audio_encoding: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding,
        >,
        #[doc = "Optional. The synthesis sample rate (in hertz) for this audio. If not\nprovided, then the synthesizer will use the default sample rate based on\nthe audio encoding. If this is different from the voice's natural sample\nrate, then the synthesizer will honor this request by converting to the\ndesired sample rate (which might result in worse audio quality)."]
        #[serde(rename = "sampleRateHertz", default)]
        pub sample_rate_hertz: ::std::option::Option<i32>,
        #[doc = "Optional. Configuration of how speech should be synthesized."]
        #[serde(rename = "synthesizeSpeechConfig", default)]
        pub synthesize_speech_config:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2SynthesizeSpeechConfig>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2OutputAudioConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2OutputAudioConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding {
        #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM).\nAudio content returned as LINEAR16 also contains a WAV header."]
        OutputAudioEncodingLinear16,
        #[doc = "MP3 audio."]
        OutputAudioEncodingMp3,
        #[doc = "Opus encoded audio wrapped in an ogg container. The result will be a\nfile which can be played natively on Android, and in browsers (at least\nChrome and Firefox). The quality of the encoding is considerably higher\nthan MP3 while using approximately the same bitrate."]
        OutputAudioEncodingOggOpus,
        #[doc = "Not specified."]
        OutputAudioEncodingUnspecified,
    }
    impl GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding :: OutputAudioEncodingLinear16 => "OUTPUT_AUDIO_ENCODING_LINEAR_16" , GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding :: OutputAudioEncodingMp3 => "OUTPUT_AUDIO_ENCODING_MP3" , GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding :: OutputAudioEncodingOggOpus => "OUTPUT_AUDIO_ENCODING_OGG_OPUS" , GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding :: OutputAudioEncodingUnspecified => "OUTPUT_AUDIO_ENCODING_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "OUTPUT_AUDIO_ENCODING_LINEAR_16" => GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding :: OutputAudioEncodingLinear16 , "OUTPUT_AUDIO_ENCODING_MP3" => GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding :: OutputAudioEncodingMp3 , "OUTPUT_AUDIO_ENCODING_OGG_OPUS" => GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding :: OutputAudioEncodingOggOpus , "OUTPUT_AUDIO_ENCODING_UNSPECIFIED" => GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding :: OutputAudioEncodingUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2OutputAudioConfigAudioEncoding
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2QueryInput {
        #[doc = "Instructs the speech recognizer how to process the speech audio."]
        #[serde(rename = "audioConfig", default)]
        pub audio_config:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2InputAudioConfig>,
        #[doc = "The event to be processed."]
        #[serde(rename = "event", default)]
        pub event: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2EventInput>,
        #[doc = "The natural language text to be processed."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2TextInput>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2QueryInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2QueryInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2QueryParameters {
        #[doc = "Optional. The collection of contexts to be activated before this query is\nexecuted."]
        #[serde(rename = "contexts", default)]
        pub contexts: ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Context>>,
        #[doc = "Optional. The geo location of this conversational query."]
        #[serde(rename = "geoLocation", default)]
        pub geo_location: ::std::option::Option<crate::schemas::GoogleTypeLatLng>,
        #[doc = "Optional. This field can be used to pass custom data into the webhook\nassociated with the agent. Arbitrary JSON objects are supported."]
        #[serde(rename = "payload", default)]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Optional. Specifies whether to delete all contexts in the current session\nbefore the new ones are activated."]
        #[serde(rename = "resetContexts", default)]
        pub reset_contexts: ::std::option::Option<bool>,
        #[doc = "Optional. Configures the type of sentiment analysis to perform. If not\nprovided, sentiment analysis is not performed."]
        #[serde(rename = "sentimentAnalysisRequestConfig", default)]
        pub sentiment_analysis_request_config: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2SentimentAnalysisRequestConfig,
        >,
        #[doc = "Optional. Additional session entity types to replace or extend developer\nentity types with. The entity synonyms apply to all languages and persist\nfor the session of this query."]
        #[serde(rename = "sessionEntityTypes", default)]
        pub session_entity_types:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2SessionEntityType>>,
        #[doc = "Optional. The time zone of this conversational query from the\n[time zone database](https://www.iana.org/time-zones), e.g.,\nAmerica/New_York, Europe/Paris. If not provided, the time zone specified in\nagent settings is used."]
        #[serde(rename = "timeZone", default)]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2QueryParameters {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2QueryParameters {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2QueryResult {
        #[doc = "The action name from the matched intent."]
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<String>,
        #[doc = "This field is set to:\n\n* `false` if the matched intent has required parameters and not all of\n  the required parameter values have been collected.\n* `true` if all required parameter values have been collected, or if the\n  matched intent doesn't contain any required parameters."]
        #[serde(rename = "allRequiredParamsPresent", default)]
        pub all_required_params_present: ::std::option::Option<bool>,
        #[doc = "The free-form diagnostic info. For example, this field could contain\nwebhook call latency. The string keys of the Struct's fields map can change\nwithout notice."]
        #[serde(rename = "diagnosticInfo", default)]
        pub diagnostic_info:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The collection of rich messages to present to the user."]
        #[serde(rename = "fulfillmentMessages", default)]
        pub fulfillment_messages:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessage>>,
        #[doc = "The text to be pronounced to the user or shown on the screen.\nNote: This is a legacy field, `fulfillment_messages` should be preferred."]
        #[serde(rename = "fulfillmentText", default)]
        pub fulfillment_text: ::std::option::Option<String>,
        #[doc = "The intent that matched the conversational query. Some, not\nall fields are filled in this message, including but not limited to:\n`name`, `display_name`, `end_interaction` and `is_fallback`."]
        #[serde(rename = "intent", default)]
        pub intent: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Intent>,
        #[doc = "The intent detection confidence. Values range from 0.0\n(completely uncertain) to 1.0 (completely certain).\nThis value is for informational purpose only and is only used to\nhelp match the best intent within the classification threshold.\nThis value may change for the same end-user expression at any time due to a\nmodel retraining or change in implementation.\nIf there are `multiple knowledge_answers` messages, this value is set to\nthe greatest `knowledgeAnswers.match_confidence` value in the list."]
        #[serde(rename = "intentDetectionConfidence", default)]
        pub intent_detection_confidence: ::std::option::Option<f32>,
        #[doc = "The language that was triggered during intent detection.\nSee [Language\nSupport](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "The collection of output contexts. If applicable,\n`output_contexts.parameters` contains entries with name\n`<parameter name>.original` containing the original parameter values\nbefore the query."]
        #[serde(rename = "outputContexts", default)]
        pub output_contexts:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Context>>,
        #[doc = "The collection of extracted parameters."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The original conversational query text:\n\n* If natural language text was provided as input, `query_text` contains\n  a copy of the input.\n* If natural language speech audio was provided as input, `query_text`\n  contains the speech recognition result. If speech recognizer produced\n  multiple alternatives, a particular one is picked.\n* If automatic spell correction is enabled, `query_text` will contain the\n  corrected user input."]
        #[serde(rename = "queryText", default)]
        pub query_text: ::std::option::Option<String>,
        #[doc = "The sentiment analysis result, which depends on the\n`sentiment_analysis_request_config` specified in the request."]
        #[serde(rename = "sentimentAnalysisResult", default)]
        pub sentiment_analysis_result:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2SentimentAnalysisResult>,
        #[doc = "The Speech recognition confidence between 0.0 and 1.0. A higher number\nindicates an estimated greater likelihood that the recognized words are\ncorrect. The default of 0.0 is a sentinel value indicating that confidence\nwas not set.\n\nThis field is not guaranteed to be accurate or set. In particular this\nfield isn't set for StreamingDetectIntent since the streaming endpoint has\nseparate confidence estimates per portion of the audio in\nStreamingRecognitionResult."]
        #[serde(rename = "speechRecognitionConfidence", default)]
        pub speech_recognition_confidence: ::std::option::Option<f32>,
        #[doc = "If the query was fulfilled by a webhook call, this field is set to the\nvalue of the `payload` field returned in the webhook response."]
        #[serde(rename = "webhookPayload", default)]
        pub webhook_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "If the query was fulfilled by a webhook call, this field is set to the\nvalue of the `source` field returned in the webhook response."]
        #[serde(rename = "webhookSource", default)]
        pub webhook_source: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2QueryResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2QueryResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2RestoreAgentRequest {
        #[doc = "The agent to restore.\n\nExample for how to restore an agent via the command line:\n\n<pre>curl \\\n  'https://dialogflow.googleapis.com/v2/projects/&lt;project_name&gt;/agent:restore\\\n   -X POST \\\n   -H 'Authorization: Bearer '$(gcloud auth application-default\n   print-access-token) \\\n   -H 'Accept: application/json' \\\n   -H 'Content-Type: application/json' \\\n   --compressed \\\n   --data-binary \"{\n       'agentContent': '$(cat &lt;agent zip file&gt; | base64 -w 0)'\n   }\"</pre>"]
        #[serde(rename = "agentContent", default)]
        pub agent_content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The URI to a Google Cloud Storage file containing the agent to restore.\nNote: The URI must start with \"gs://\"."]
        #[serde(rename = "agentUri", default)]
        pub agent_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2RestoreAgentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2RestoreAgentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2SearchAgentsResponse {
        #[doc = "The list of agents. There will be a maximum number of items returned based\non the page_size field in the request."]
        #[serde(rename = "agents", default)]
        pub agents: ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Agent>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2SearchAgentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2SearchAgentsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2Sentiment {
        #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute\nmagnitude of sentiment, regardless of score (positive or negative)."]
        #[serde(rename = "magnitude", default)]
        pub magnitude: ::std::option::Option<f32>,
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive\nsentiment)."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2Sentiment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2Sentiment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2SentimentAnalysisRequestConfig {
        #[doc = "Optional. Instructs the service to perform sentiment analysis on\n`query_text`. If not provided, sentiment analysis is not performed on\n`query_text`."]
        #[serde(rename = "analyzeQueryTextSentiment", default)]
        pub analyze_query_text_sentiment: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2SentimentAnalysisRequestConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2SentimentAnalysisRequestConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2SentimentAnalysisResult {
        #[doc = "The sentiment analysis result for `query_text`."]
        #[serde(rename = "queryTextSentiment", default)]
        pub query_text_sentiment:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2Sentiment>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2SentimentAnalysisResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2SentimentAnalysisResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2SessionEntityType {
        #[doc = "Required. The collection of entities associated with this session entity\ntype."]
        #[serde(rename = "entities", default)]
        pub entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2EntityTypeEntity>>,
        #[doc = "Required. Indicates whether the additional data should override or\nsupplement the developer entity type definition."]
        #[serde(rename = "entityOverrideMode", default)]
        pub entity_override_mode: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode,
        >,
        #[doc = "Required. The unique identifier of this session entity type. Format:\n`projects/<Project ID>/agent/sessions/<Session ID>/entityTypes/<Entity Type Display Name>`.\n\n`<Entity Type Display Name>` must be the display name of an existing entity\ntype in the same agent that will be overridden or supplemented."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2SessionEntityType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2SessionEntityType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode {
        #[doc = "The collection of session entities overrides the collection of entities\nin the corresponding developer entity type."]
        EntityOverrideModeOverride,
        #[doc = "The collection of session entities extends the collection of entities in\nthe corresponding developer entity type.\n\nNote: Even in this override mode calls to `ListSessionEntityTypes`,\n`GetSessionEntityType`, `CreateSessionEntityType` and\n`UpdateSessionEntityType` only return the additional entities added in\nthis session entity type. If you want to get the supplemented list,\nplease call EntityTypes.GetEntityType on the developer entity type\nand merge."]
        EntityOverrideModeSupplement,
        #[doc = "Not specified. This value should be never used."]
        EntityOverrideModeUnspecified,
    }
    impl GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode :: EntityOverrideModeOverride => "ENTITY_OVERRIDE_MODE_OVERRIDE" , GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode :: EntityOverrideModeSupplement => "ENTITY_OVERRIDE_MODE_SUPPLEMENT" , GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode :: EntityOverrideModeUnspecified => "ENTITY_OVERRIDE_MODE_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ENTITY_OVERRIDE_MODE_OVERRIDE" => GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode :: EntityOverrideModeOverride , "ENTITY_OVERRIDE_MODE_SUPPLEMENT" => GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode :: EntityOverrideModeSupplement , "ENTITY_OVERRIDE_MODE_UNSPECIFIED" => GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode :: EntityOverrideModeUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideMode
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2SynthesizeSpeechConfig {
        #[doc = "Optional. An identifier which selects 'audio effects' profiles that are\napplied on (post synthesized) text to speech. Effects are applied on top of\neach other in the order they are given."]
        #[serde(rename = "effectsProfileId", default)]
        pub effects_profile_id: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20\nsemitones from the original pitch. -20 means decrease 20 semitones from the\noriginal pitch."]
        #[serde(rename = "pitch", default)]
        pub pitch: ::std::option::Option<f64>,
        #[doc = "Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal\nnative speed supported by the specific voice. 2.0 is twice as fast, and\n0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any\nother values < 0.25 or > 4.0 will return an error."]
        #[serde(rename = "speakingRate", default)]
        pub speaking_rate: ::std::option::Option<f64>,
        #[doc = "Optional. The desired voice of the synthesized audio."]
        #[serde(rename = "voice", default)]
        pub voice:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2VoiceSelectionParams>,
        #[doc = "Optional. Volume gain (in dB) of the normal native volume supported by the\nspecific voice, in the range [-96.0, 16.0]. If unset, or set to a value of\n0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)\nwill play at approximately half the amplitude of the normal native signal\namplitude. A value of +6.0 (dB) will play at approximately twice the\namplitude of the normal native signal amplitude. We strongly recommend not\nto exceed +10 (dB) as there's usually no effective increase in loudness for\nany value greater than that."]
        #[serde(rename = "volumeGainDb", default)]
        pub volume_gain_db: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2SynthesizeSpeechConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2SynthesizeSpeechConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2TextInput {
        #[doc = "Required. The language of this conversational query. See [Language\nSupport](https://cloud.google.com/dialogflow/docs/reference/language)\nfor a list of the currently supported language codes. Note that queries in\nthe same session do not necessarily need to specify the same language."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Required. The UTF-8 encoded natural language text to be processed.\nText length must not exceed 256 characters."]
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2TextInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2TextInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2TrainAgentRequest;
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2TrainAgentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2TrainAgentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudDialogflowV2VoiceSelectionParams {
        #[doc = "Optional. The name of the voice. If not set, the service will choose a\nvoice based on the other parameters such as language_code and gender."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. The preferred gender of the voice. If not set, the service will\nchoose a voice based on the other parameters such as language_code and\nname. Note that this is only a preference, not requirement. If a\nvoice of the appropriate gender is not available, the synthesizer should\nsubstitute a voice with a different gender rather than failing the request."]
        #[serde(rename = "ssmlGender", default)]
        pub ssml_gender: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2VoiceSelectionParams {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2VoiceSelectionParams {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender {
        #[doc = "A female voice."]
        SsmlVoiceGenderFemale,
        #[doc = "A male voice."]
        SsmlVoiceGenderMale,
        #[doc = "A gender-neutral voice."]
        SsmlVoiceGenderNeutral,
        #[doc = "An unspecified gender, which means that the client doesn't care which\ngender the selected voice will have."]
        SsmlVoiceGenderUnspecified,
    }
    impl GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender :: SsmlVoiceGenderFemale => "SSML_VOICE_GENDER_FEMALE" , GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender :: SsmlVoiceGenderMale => "SSML_VOICE_GENDER_MALE" , GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender :: SsmlVoiceGenderNeutral => "SSML_VOICE_GENDER_NEUTRAL" , GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender :: SsmlVoiceGenderUnspecified => "SSML_VOICE_GENDER_UNSPECIFIED" , }
        }
    }
    impl ::std::fmt::Display for GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "SSML_VOICE_GENDER_FEMALE" => GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender :: SsmlVoiceGenderFemale , "SSML_VOICE_GENDER_MALE" => GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender :: SsmlVoiceGenderMale , "SSML_VOICE_GENDER_NEUTRAL" => GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender :: SsmlVoiceGenderNeutral , "SSML_VOICE_GENDER_UNSPECIFIED" => GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender :: SsmlVoiceGenderUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudDialogflowV2VoiceSelectionParamsSsmlGender
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2WebhookRequest {
        #[doc = "Optional. The contents of the original request that was passed to\n`[Streaming]DetectIntent` call."]
        #[serde(rename = "originalDetectIntentRequest", default)]
        pub original_detect_intent_request: ::std::option::Option<
            crate::schemas::GoogleCloudDialogflowV2OriginalDetectIntentRequest,
        >,
        #[doc = "The result of the conversational query or event processing. Contains the\nsame value as `[Streaming]DetectIntentResponse.query_result`."]
        #[serde(rename = "queryResult", default)]
        pub query_result: ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2QueryResult>,
        #[doc = "The unique identifier of the response. Contains the same value as\n`[Streaming]DetectIntentResponse.response_id`."]
        #[serde(rename = "responseId", default)]
        pub response_id: ::std::option::Option<String>,
        #[doc = "The unique identifier of detectIntent request session.\nCan be used to identify end-user inside webhook implementation.\nFormat: `projects/<Project ID>/agent/sessions/<Session ID>`, or\n`projects/<Project ID>/agent/environments/<Environment ID>/users/<User ID>/sessions/<Session ID>`."]
        #[serde(rename = "session", default)]
        pub session: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2WebhookRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2WebhookRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudDialogflowV2WebhookResponse {
        #[doc = "Optional. Makes the platform immediately invoke another `DetectIntent` call\ninternally with the specified event as input.\nWhen this field is set, Dialogflow ignores the `fulfillment_text`,\n`fulfillment_messages`, and `payload` fields."]
        #[serde(rename = "followupEventInput", default)]
        pub followup_event_input:
            ::std::option::Option<crate::schemas::GoogleCloudDialogflowV2EventInput>,
        #[doc = "Optional. The collection of rich messages to present to the user. This\nvalue is passed directly to `QueryResult.fulfillment_messages`."]
        #[serde(rename = "fulfillmentMessages", default)]
        pub fulfillment_messages:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2IntentMessage>>,
        #[doc = "Optional. The text to be shown on the screen. This value is passed directly\nto `QueryResult.fulfillment_text`."]
        #[serde(rename = "fulfillmentText", default)]
        pub fulfillment_text: ::std::option::Option<String>,
        #[doc = "Optional. The collection of output contexts. This value is passed directly\nto `QueryResult.output_contexts`."]
        #[serde(rename = "outputContexts", default)]
        pub output_contexts:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudDialogflowV2Context>>,
        #[doc = "Optional. This value is passed directly to `QueryResult.webhook_payload`.\nSee the related `fulfillment_messages[i].payload field`, which may be used\nas an alternative to this field.\n\nThis field can be used for Actions on Google responses.\nIt should have a structure similar to the JSON message shown here. For more\ninformation, see\n[Actions on Google Webhook\nFormat](https://developers.google.com/actions/dialogflow/webhook)\n\n<pre>{\n  \"google\": {\n    \"expectUserResponse\": true,\n    \"richResponse\": {\n      \"items\": [\n        {\n          \"simpleResponse\": {\n            \"textToSpeech\": \"this is a simple response\"\n          }\n        }\n      ]\n    }\n  }\n}</pre>"]
        #[serde(rename = "payload", default)]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Optional. This value is passed directly to `QueryResult.webhook_source`."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudDialogflowV2WebhookResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudDialogflowV2WebhookResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(rename = "operations", default)]
        pub operations: ::std::option::Option<Vec<crate::schemas::GoogleLongrunningOperation>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningListOperationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningOperation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningOperation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleProtobufEmpty;
    impl ::google_field_selector::FieldSelector for GoogleProtobufEmpty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleProtobufEmpty {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleRpcStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRpcStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleTypeLatLng {
        #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeLatLng {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeLatLng {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates/updates the specified agent."]
            pub fn agent_method(
                &self,
                request: crate::schemas::GoogleCloudDialogflowV2Agent,
                parent: impl Into<String>,
            ) -> AgentMethodRequestBuilder {
                AgentMethodRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    parent: parent.into(),
                    update_mask: None,
                }
            }
            #[doc = "Deletes the specified agent."]
            pub fn delete_agent(&self, parent: impl Into<String>) -> DeleteAgentRequestBuilder {
                DeleteAgentRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    parent: parent.into(),
                }
            }
            #[doc = "Retrieves the specified agent."]
            pub fn get_agent(&self, parent: impl Into<String>) -> GetAgentRequestBuilder {
                GetAgentRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    parent: parent.into(),
                }
            }
            #[doc = "Actions that can be performed on the agent resource"]
            pub fn agent(&self) -> crate::resources::projects::agent::AgentActions {
                crate::resources::projects::agent::AgentActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(&self) -> crate::resources::projects::operations::OperationsActions {
                crate::resources::projects::operations::OperationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct AgentMethodRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudDialogflowV2Agent,
            parent: String,
            update_mask: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> AgentMethodRequestBuilder<'a> {
            #[doc = "Optional. The mask to control which fields get updated."]
            pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                self.update_mask = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudDialogflowV2Agent, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudDialogflowV2Agent, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://dialogflow.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/agent");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("updateMask", &self.update_mask)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteAgentRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            parent: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> DeleteAgentRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://dialogflow.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/agent");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetAgentRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            parent: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetAgentRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudDialogflowV2Agent, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudDialogflowV2Agent, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://dialogflow.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/agent");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        pub mod agent {
            pub mod params {}
            pub struct AgentActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AgentActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Exports the specified agent to a ZIP file.\n\nOperation <response: ExportAgentResponse>"]
                pub fn export(
                    &self,
                    request: crate::schemas::GoogleCloudDialogflowV2ExportAgentRequest,
                    parent: impl Into<String>,
                ) -> ExportRequestBuilder {
                    ExportRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Imports the specified agent from a ZIP file.\n\nUploads new intents and entity types without deleting the existing ones.\nIntents and entity types with the same name are replaced with the new\nversions from ImportAgentRequest.\n\nOperation <response: google.protobuf.Empty>"]
                pub fn import(
                    &self,
                    request: crate::schemas::GoogleCloudDialogflowV2ImportAgentRequest,
                    parent: impl Into<String>,
                ) -> ImportRequestBuilder {
                    ImportRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Restores the specified agent from a ZIP file.\n\nReplaces the current agent version with a new one. All the intents and\nentity types in the older version are deleted.\n\nOperation <response: google.protobuf.Empty>"]
                pub fn restore(
                    &self,
                    request: crate::schemas::GoogleCloudDialogflowV2RestoreAgentRequest,
                    parent: impl Into<String>,
                ) -> RestoreRequestBuilder {
                    RestoreRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Returns the list of agents.\n\nSince there is at most one conversational agent per project, this method is\nuseful primarily for listing all agents across projects the caller has\naccess to. One can achieve that with a wildcard project collection id \"-\".\nRefer to [List\nSub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections)."]
                pub fn search(&self, parent: impl Into<String>) -> SearchRequestBuilder {
                    SearchRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Trains the specified agent.\n\nOperation <response: google.protobuf.Empty>"]
                pub fn train(
                    &self,
                    request: crate::schemas::GoogleCloudDialogflowV2TrainAgentRequest,
                    parent: impl Into<String>,
                ) -> TrainRequestBuilder {
                    TrainRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Actions that can be performed on the entity_types resource"]
                pub fn entity_types(
                    &self,
                ) -> crate::resources::projects::agent::entity_types::EntityTypesActions
                {
                    crate::resources::projects::agent::entity_types::EntityTypesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the intents resource"]
                pub fn intents(
                    &self,
                ) -> crate::resources::projects::agent::intents::IntentsActions {
                    crate::resources::projects::agent::intents::IntentsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the sessions resource"]
                pub fn sessions(
                    &self,
                ) -> crate::resources::projects::agent::sessions::SessionsActions {
                    crate::resources::projects::agent::sessions::SessionsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ExportRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudDialogflowV2ExportAgentRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ExportRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dialogflow.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/agent:export");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ImportRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudDialogflowV2ImportAgentRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ImportRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dialogflow.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/agent:import");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct RestoreRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudDialogflowV2RestoreAgentRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> RestoreRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dialogflow.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/agent:restore");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct SearchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> SearchRequestBuilder<'a> {
                #[doc = "Optional. The maximum number of items to return in a single page. By\ndefault 100 and at most 1000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. The next_page_token value returned from a previous list request."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_agents<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_agents_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_agents_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudDialogflowV2Agent>
                {
                    self.iter_agents_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_agents_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudDialogflowV2Agent>
                {
                    self.iter_agents_with_fields(Some("*"))
                }
                pub fn iter_agents_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "agents").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "agents")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleCloudDialogflowV2SearchAgentsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleCloudDialogflowV2SearchAgentsResponse,
                > {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudDialogflowV2SearchAgentsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudDialogflowV2SearchAgentsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dialogflow.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/agent:search");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for SearchRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[derive(Debug, Clone)]
            pub struct TrainRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudDialogflowV2TrainAgentRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> TrainRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dialogflow.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/agent:train");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            pub mod entity_types {
                pub mod params {}
                pub struct EntityTypesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> EntityTypesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Deletes entity types in the specified agent.\n\nOperation <response: google.protobuf.Empty>"]
                    pub fn batch_delete(
                        &self,
                        request : crate :: schemas :: GoogleCloudDialogflowV2BatchDeleteEntityTypesRequest,
                        parent: impl Into<String>,
                    ) -> BatchDeleteRequestBuilder {
                        BatchDeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Updates/Creates multiple entity types in the specified agent.\n\nOperation <response: BatchUpdateEntityTypesResponse>"]
                    pub fn batch_update(
                        &self,
                        request : crate :: schemas :: GoogleCloudDialogflowV2BatchUpdateEntityTypesRequest,
                        parent: impl Into<String>,
                    ) -> BatchUpdateRequestBuilder {
                        BatchUpdateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Creates an entity type in the specified agent."]
                    pub fn create(
                        &self,
                        request: crate::schemas::GoogleCloudDialogflowV2EntityType,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            language_code: None,
                        }
                    }
                    #[doc = "Deletes the specified entity type."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Retrieves the specified entity type."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            language_code: None,
                        }
                    }
                    #[doc = "Returns the list of all entity types in the specified agent."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            language_code: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates the specified entity type."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::GoogleCloudDialogflowV2EntityType,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder {
                        PatchRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            language_code: None,
                            update_mask: None,
                        }
                    }
                    #[doc = "Actions that can be performed on the entities resource"]
                    pub fn entities(
                        &self,
                    ) -> crate::resources::projects::agent::entity_types::entities::EntitiesActions
                    {
                        crate::resources::projects::agent::entity_types::entities::EntitiesActions {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct BatchDeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2BatchDeleteEntityTypesRequest,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> BatchDeleteRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/entityTypes:batchDelete");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct BatchUpdateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2BatchUpdateEntityTypesRequest,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> BatchUpdateRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/entityTypes:batchUpdate");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2EntityType,
                    parent: String,
                    language_code: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> CreateRequestBuilder<'a> {
                    #[doc = "Optional. The language of entity synonyms defined in `entity_type`. If not\nspecified, the agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2EntityType, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2EntityType, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/entityTypes");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("languageCode", &self.language_code)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DeleteRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    language_code: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> GetRequestBuilder<'a> {
                    #[doc = "Optional. The language to retrieve entity synonyms for. If not specified,\nthe agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2EntityType, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2EntityType, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("languageCode", &self.language_code)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    language_code: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "Optional. The language to list entity synonyms for. If not specified,\nthe agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "Optional. The maximum number of items to return in a single page. By\ndefault 100 and at most 1000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. The next_page_token value returned from a previous list request."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_entity_types<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_entity_types_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_entity_types_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<
                        Self,
                        crate::schemas::GoogleCloudDialogflowV2EntityType,
                    > {
                        self.iter_entity_types_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_entity_types_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<
                        Self,
                        crate::schemas::GoogleCloudDialogflowV2EntityType,
                    > {
                        self.iter_entity_types_with_fields(Some("*"))
                    }
                    pub fn iter_entity_types_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "entityTypes").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "entityTypes")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudDialogflowV2ListEntityTypesResponse,
                    > {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudDialogflowV2ListEntityTypesResponse,
                    > {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudDialogflowV2ListEntityTypesResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudDialogflowV2ListEntityTypesResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/entityTypes");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("languageCode", &self.language_code)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2EntityType,
                    name: String,
                    language_code: Option<String>,
                    update_mask: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> PatchRequestBuilder<'a> {
                    #[doc = "Optional. The language of entity synonyms defined in `entity_type`. If not\nspecified, the agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "Optional. The mask to control which fields get updated."]
                    pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                        self.update_mask = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2EntityType, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2EntityType, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        let req = req.query(&[("languageCode", &self.language_code)]);
                        let req = req.query(&[("updateMask", &self.update_mask)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                pub mod entities {
                    pub mod params {}
                    pub struct EntitiesActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> EntitiesActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Creates multiple new entities in the specified entity type.\n\nOperation <response: google.protobuf.Empty>"]
                        pub fn batch_create(
                            &self,
                            request : crate :: schemas :: GoogleCloudDialogflowV2BatchCreateEntitiesRequest,
                            parent: impl Into<String>,
                        ) -> BatchCreateRequestBuilder {
                            BatchCreateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Deletes entities in the specified entity type.\n\nOperation <response: google.protobuf.Empty>"]
                        pub fn batch_delete(
                            &self,
                            request : crate :: schemas :: GoogleCloudDialogflowV2BatchDeleteEntitiesRequest,
                            parent: impl Into<String>,
                        ) -> BatchDeleteRequestBuilder {
                            BatchDeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Updates or creates multiple entities in the specified entity type. This\nmethod does not affect entities in the entity type that aren't explicitly\nspecified in the request.\n\nOperation <response: google.protobuf.Empty>"]
                        pub fn batch_update(
                            &self,
                            request : crate :: schemas :: GoogleCloudDialogflowV2BatchUpdateEntitiesRequest,
                            parent: impl Into<String>,
                        ) -> BatchUpdateRequestBuilder {
                            BatchUpdateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct BatchCreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDialogflowV2BatchCreateEntitiesRequest,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> BatchCreateRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/entities:batchCreate");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct BatchDeleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDialogflowV2BatchDeleteEntitiesRequest,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> BatchDeleteRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/entities:batchDelete");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct BatchUpdateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDialogflowV2BatchUpdateEntitiesRequest,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> BatchUpdateRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/entities:batchUpdate");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                }
            }
            pub mod intents {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum CreateIntentView {
                        IntentViewFull,
                        IntentViewUnspecified,
                    }
                    impl CreateIntentView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                CreateIntentView::IntentViewFull => "INTENT_VIEW_FULL",
                                CreateIntentView::IntentViewUnspecified => {
                                    "INTENT_VIEW_UNSPECIFIED"
                                }
                            }
                        }
                    }
                    impl ::std::fmt::Display for CreateIntentView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for CreateIntentView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for CreateIntentView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "INTENT_VIEW_FULL" => CreateIntentView::IntentViewFull,
                                "INTENT_VIEW_UNSPECIFIED" => {
                                    CreateIntentView::IntentViewUnspecified
                                }
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for CreateIntentView {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for CreateIntentView {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum GetIntentView {
                        IntentViewFull,
                        IntentViewUnspecified,
                    }
                    impl GetIntentView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                GetIntentView::IntentViewFull => "INTENT_VIEW_FULL",
                                GetIntentView::IntentViewUnspecified => "INTENT_VIEW_UNSPECIFIED",
                            }
                        }
                    }
                    impl ::std::fmt::Display for GetIntentView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for GetIntentView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for GetIntentView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "INTENT_VIEW_FULL" => GetIntentView::IntentViewFull,
                                "INTENT_VIEW_UNSPECIFIED" => GetIntentView::IntentViewUnspecified,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for GetIntentView {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for GetIntentView {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListIntentView {
                        IntentViewFull,
                        IntentViewUnspecified,
                    }
                    impl ListIntentView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                ListIntentView::IntentViewFull => "INTENT_VIEW_FULL",
                                ListIntentView::IntentViewUnspecified => "INTENT_VIEW_UNSPECIFIED",
                            }
                        }
                    }
                    impl ::std::fmt::Display for ListIntentView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListIntentView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for ListIntentView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "INTENT_VIEW_FULL" => ListIntentView::IntentViewFull,
                                "INTENT_VIEW_UNSPECIFIED" => ListIntentView::IntentViewUnspecified,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for ListIntentView {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for ListIntentView {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum PatchIntentView {
                        IntentViewFull,
                        IntentViewUnspecified,
                    }
                    impl PatchIntentView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                PatchIntentView::IntentViewFull => "INTENT_VIEW_FULL",
                                PatchIntentView::IntentViewUnspecified => "INTENT_VIEW_UNSPECIFIED",
                            }
                        }
                    }
                    impl ::std::fmt::Display for PatchIntentView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for PatchIntentView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for PatchIntentView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "INTENT_VIEW_FULL" => PatchIntentView::IntentViewFull,
                                "INTENT_VIEW_UNSPECIFIED" => PatchIntentView::IntentViewUnspecified,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for PatchIntentView {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for PatchIntentView {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct IntentsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> IntentsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Deletes intents in the specified agent.\n\nOperation <response: google.protobuf.Empty>"]
                    pub fn batch_delete(
                        &self,
                        request: crate::schemas::GoogleCloudDialogflowV2BatchDeleteIntentsRequest,
                        parent: impl Into<String>,
                    ) -> BatchDeleteRequestBuilder {
                        BatchDeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Updates/Creates multiple intents in the specified agent.\n\nOperation <response: BatchUpdateIntentsResponse>"]
                    pub fn batch_update(
                        &self,
                        request: crate::schemas::GoogleCloudDialogflowV2BatchUpdateIntentsRequest,
                        parent: impl Into<String>,
                    ) -> BatchUpdateRequestBuilder {
                        BatchUpdateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Creates an intent in the specified agent."]
                    pub fn create(
                        &self,
                        request: crate::schemas::GoogleCloudDialogflowV2Intent,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            intent_view: None,
                            language_code: None,
                        }
                    }
                    #[doc = "Deletes the specified intent and its direct or indirect followup intents."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                        }
                    }
                    #[doc = "Retrieves the specified intent."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            intent_view: None,
                            language_code: None,
                        }
                    }
                    #[doc = "Returns the list of all intents in the specified agent."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            intent_view: None,
                            language_code: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates the specified intent."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::GoogleCloudDialogflowV2Intent,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder {
                        PatchRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            intent_view: None,
                            language_code: None,
                            update_mask: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct BatchDeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2BatchDeleteIntentsRequest,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> BatchDeleteRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/intents:batchDelete");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct BatchUpdateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2BatchUpdateIntentsRequest,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> BatchUpdateRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/intents:batchUpdate");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2Intent,
                    parent: String,
                    intent_view: Option<
                        crate::resources::projects::agent::intents::params::CreateIntentView,
                    >,
                    language_code: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> CreateRequestBuilder<'a> {
                    #[doc = "Optional. The resource view to apply to the returned intent."]
                    pub fn intent_view(
                        mut self,
                        value: crate::resources::projects::agent::intents::params::CreateIntentView,
                    ) -> Self {
                        self.intent_view = Some(value);
                        self
                    }
                    #[doc = "Optional. The language of training phrases, parameters and rich messages\ndefined in `intent`. If not specified, the agent's default language is\nused. [Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2Intent, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2Intent, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/intents");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("intentView", &self.intent_view)]);
                        let req = req.query(&[("languageCode", &self.language_code)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DeleteRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    intent_view:
                        Option<crate::resources::projects::agent::intents::params::GetIntentView>,
                    language_code: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> GetRequestBuilder<'a> {
                    #[doc = "Optional. The resource view to apply to the returned intent."]
                    pub fn intent_view(
                        mut self,
                        value: crate::resources::projects::agent::intents::params::GetIntentView,
                    ) -> Self {
                        self.intent_view = Some(value);
                        self
                    }
                    #[doc = "Optional. The language to retrieve training phrases, parameters and rich\nmessages for. If not specified, the agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2Intent, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2Intent, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("intentView", &self.intent_view)]);
                        let req = req.query(&[("languageCode", &self.language_code)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    intent_view:
                        Option<crate::resources::projects::agent::intents::params::ListIntentView>,
                    language_code: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "Optional. The resource view to apply to the returned intent."]
                    pub fn intent_view(
                        mut self,
                        value: crate::resources::projects::agent::intents::params::ListIntentView,
                    ) -> Self {
                        self.intent_view = Some(value);
                        self
                    }
                    #[doc = "Optional. The language to list training phrases, parameters and rich\nmessages for. If not specified, the agent's default language is used.\n[Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "Optional. The maximum number of items to return in a single page. By\ndefault 100 and at most 1000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. The next_page_token value returned from a previous list request."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_intents<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_intents_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_intents_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<
                        Self,
                        crate::schemas::GoogleCloudDialogflowV2Intent,
                    > {
                        self.iter_intents_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_intents_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<
                        Self,
                        crate::schemas::GoogleCloudDialogflowV2Intent,
                    > {
                        self.iter_intents_with_fields(Some("*"))
                    }
                    pub fn iter_intents_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "intents").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "intents")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudDialogflowV2ListIntentsResponse,
                    > {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudDialogflowV2ListIntentsResponse,
                    > {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudDialogflowV2ListIntentsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudDialogflowV2ListIntentsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/intents");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("intentView", &self.intent_view)]);
                        let req = req.query(&[("languageCode", &self.language_code)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2Intent,
                    name: String,
                    intent_view:
                        Option<crate::resources::projects::agent::intents::params::PatchIntentView>,
                    language_code: Option<String>,
                    update_mask: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> PatchRequestBuilder<'a> {
                    #[doc = "Optional. The resource view to apply to the returned intent."]
                    pub fn intent_view(
                        mut self,
                        value: crate::resources::projects::agent::intents::params::PatchIntentView,
                    ) -> Self {
                        self.intent_view = Some(value);
                        self
                    }
                    #[doc = "Optional. The language of training phrases, parameters and rich messages\ndefined in `intent`. If not specified, the agent's default language is\nused. [Many\nlanguages](https://cloud.google.com/dialogflow/docs/reference/language)\nare supported. Note: languages must be enabled in the agent before they can\nbe used."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "Optional. The mask to control which fields get updated."]
                    pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                        self.update_mask = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2Intent, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudDialogflowV2Intent, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        let req = req.query(&[("intentView", &self.intent_view)]);
                        let req = req.query(&[("languageCode", &self.language_code)]);
                        let req = req.query(&[("updateMask", &self.update_mask)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
            }
            pub mod sessions {
                pub mod params {}
                pub struct SessionsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> SessionsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Deletes all active contexts in the specified session."]
                    pub fn delete_contexts(
                        &self,
                        parent: impl Into<String>,
                    ) -> DeleteContextsRequestBuilder {
                        DeleteContextsRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Processes a natural language query and returns structured, actionable data\nas a result. This method is not idempotent, because it may cause contexts\nand session entity types to be updated, which in turn might affect\nresults of future queries."]
                    pub fn detect_intent(
                        &self,
                        request: crate::schemas::GoogleCloudDialogflowV2DetectIntentRequest,
                        session: impl Into<String>,
                    ) -> DetectIntentRequestBuilder {
                        DetectIntentRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            session: session.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the contexts resource"]
                    pub fn contexts(
                        &self,
                    ) -> crate::resources::projects::agent::sessions::contexts::ContextsActions
                    {
                        crate::resources::projects::agent::sessions::contexts::ContextsActions {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                        }
                    }
                    #[doc = "Actions that can be performed on the entity_types resource"]
                    pub fn entity_types(
                        &self,
                    ) -> crate::resources::projects::agent::sessions::entity_types::EntityTypesActions
                    {
                        crate :: resources :: projects :: agent :: sessions :: entity_types :: EntityTypesActions { reqwest : & self . reqwest , auth : self . auth_ref ( ) , }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DeleteContextsRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DeleteContextsRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/contexts");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DetectIntentRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudDialogflowV2DetectIntentRequest,
                    session: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DetectIntentRequestBuilder<'a> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudDialogflowV2DetectIntentResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudDialogflowV2DetectIntentResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dialogflow.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.session;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":detectIntent");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                pub mod contexts {
                    pub mod params {}
                    pub struct ContextsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> ContextsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Creates a context.\n\nIf the specified context already exists, overrides the context."]
                        pub fn create(
                            &self,
                            request: crate::schemas::GoogleCloudDialogflowV2Context,
                            parent: impl Into<String>,
                        ) -> CreateRequestBuilder {
                            CreateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Deletes the specified context."]
                        pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                            DeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Retrieves the specified context."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Returns the list of all contexts in the specified session."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                page_size: None,
                                page_token: None,
                            }
                        }
                        #[doc = "Updates the specified context."]
                        pub fn patch(
                            &self,
                            request: crate::schemas::GoogleCloudDialogflowV2Context,
                            name: impl Into<String>,
                        ) -> PatchRequestBuilder {
                            PatchRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                                update_mask: None,
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDialogflowV2Context,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> CreateRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudDialogflowV2Context, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudDialogflowV2Context, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/contexts");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> DeleteRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> GetRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudDialogflowV2Context, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudDialogflowV2Context, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "Optional. The maximum number of items to return in a single page. By\ndefault 100 and at most 1000."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Optional. The next_page_token value returned from a previous list request."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                        #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                        #[doc = r" populated fields in the yielded items will be determined by the"]
                        #[doc = r" `FieldSelector` implementation."]
                        pub fn iter_contexts<T>(self) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.iter_contexts_with_fields(fields)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be the default fields populated by"]
                        #[doc = r" the server."]
                        pub fn iter_contexts_with_default_fields(
                            self,
                        ) -> crate::iter::PageItemIter<
                            Self,
                            crate::schemas::GoogleCloudDialogflowV2Context,
                        > {
                            self.iter_contexts_with_fields(None::<String>)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be all fields available. This should"]
                        #[doc = r" primarily be used during developement and debugging as fetching"]
                        #[doc = r" all fields can be expensive both in bandwidth and server"]
                        #[doc = r" resources."]
                        pub fn iter_contexts_with_all_fields(
                            self,
                        ) -> crate::iter::PageItemIter<
                            Self,
                            crate::schemas::GoogleCloudDialogflowV2Context,
                        > {
                            self.iter_contexts_with_fields(Some("*"))
                        }
                        pub fn iter_contexts_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: AsRef<str>,
                        {
                            self.fields = Some({
                                let mut selector = concat!("nextPageToken,", "contexts").to_owned();
                                let items_fields =
                                    fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                                if !items_fields.is_empty() {
                                    selector.push_str("(");
                                    selector.push_str(items_fields);
                                    selector.push_str(")");
                                }
                                selector
                            });
                            crate::iter::PageItemIter::new(self, "contexts")
                        }
                        pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.iter_with_fields(fields)
                        }
                        pub fn iter_with_default_fields(
                            self,
                        ) -> crate::iter::PageIter<
                            Self,
                            crate::schemas::GoogleCloudDialogflowV2ListContextsResponse,
                        > {
                            self.iter_with_fields(None::<&str>)
                        }
                        pub fn iter_with_all_fields(
                            self,
                        ) -> crate::iter::PageIter<
                            Self,
                            crate::schemas::GoogleCloudDialogflowV2ListContextsResponse,
                        > {
                            self.iter_with_fields(Some("*"))
                        }
                        pub fn iter_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> crate::iter::PageIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: AsRef<str>,
                        {
                            let mut fields =
                                fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                            if !fields.is_empty() {
                                match fields.chars().rev().nth(0) {
                                    Some(',') | None => {}
                                    _ => fields.push_str(","),
                                }
                                fields.push_str("nextPageToken");
                                self.fields = Some(fields);
                            }
                            crate::iter::PageIter::new(self)
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2ListContextsResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2ListContextsResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/contexts");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("pageSize", &self.page_size)]);
                            let req = req.query(&[("pageToken", &self.page_token)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                        fn set_page_token(&mut self, value: String) {
                            self.page_token = value.into();
                        }
                        fn execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            self._execute()
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDialogflowV2Context,
                        name: String,
                        update_mask: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> PatchRequestBuilder<'a> {
                        #[doc = "Optional. The mask to control which fields get updated."]
                        pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                            self.update_mask = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudDialogflowV2Context, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleCloudDialogflowV2Context, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                            let req = req.query(&[("updateMask", &self.update_mask)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                }
                pub mod entity_types {
                    pub mod params {}
                    pub struct EntityTypesActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> EntityTypesActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Creates a session entity type.\n\nIf the specified session entity type already exists, overrides the session\nentity type.\n\nThis method doesn't work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration."]
                        pub fn create(
                            &self,
                            request: crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                            parent: impl Into<String>,
                        ) -> CreateRequestBuilder {
                            CreateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Deletes the specified session entity type.\n\nThis method doesn't work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration."]
                        pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                            DeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Retrieves the specified session entity type.\n\nThis method doesn't work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Returns the list of all session entity types in the specified session.\n\nThis method doesn't work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                page_size: None,
                                page_token: None,
                            }
                        }
                        #[doc = "Updates the specified session entity type.\n\nThis method doesn't work with Google Assistant integration.\nContact Dialogflow support if you need to use session entities\nwith Google Assistant integration."]
                        pub fn patch(
                            &self,
                            request: crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                            name: impl Into<String>,
                        ) -> PatchRequestBuilder {
                            PatchRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                                update_mask: None,
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> CreateRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/entityTypes");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> DeleteRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> GetRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "Optional. The maximum number of items to return in a single page. By\ndefault 100 and at most 1000."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Optional. The next_page_token value returned from a previous list request."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                        #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                        #[doc = r" populated fields in the yielded items will be determined by the"]
                        #[doc = r" `FieldSelector` implementation."]
                        pub fn iter_session_entity_types<T>(
                            self,
                        ) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.iter_session_entity_types_with_fields(fields)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be the default fields populated by"]
                        #[doc = r" the server."]
                        pub fn iter_session_entity_types_with_default_fields(
                            self,
                        ) -> crate::iter::PageItemIter<
                            Self,
                            crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                        > {
                            self.iter_session_entity_types_with_fields(None::<String>)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be all fields available. This should"]
                        #[doc = r" primarily be used during developement and debugging as fetching"]
                        #[doc = r" all fields can be expensive both in bandwidth and server"]
                        #[doc = r" resources."]
                        pub fn iter_session_entity_types_with_all_fields(
                            self,
                        ) -> crate::iter::PageItemIter<
                            Self,
                            crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                        > {
                            self.iter_session_entity_types_with_fields(Some("*"))
                        }
                        pub fn iter_session_entity_types_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: AsRef<str>,
                        {
                            self.fields = Some({
                                let mut selector =
                                    concat!("nextPageToken,", "sessionEntityTypes").to_owned();
                                let items_fields =
                                    fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                                if !items_fields.is_empty() {
                                    selector.push_str("(");
                                    selector.push_str(items_fields);
                                    selector.push_str(")");
                                }
                                selector
                            });
                            crate::iter::PageItemIter::new(self, "sessionEntityTypes")
                        }
                        pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.iter_with_fields(fields)
                        }
                        pub fn iter_with_default_fields(
                            self,
                        ) -> crate::iter::PageIter<
                            Self,
                            crate::schemas::GoogleCloudDialogflowV2ListSessionEntityTypesResponse,
                        > {
                            self.iter_with_fields(None::<&str>)
                        }
                        pub fn iter_with_all_fields(
                            self,
                        ) -> crate::iter::PageIter<
                            Self,
                            crate::schemas::GoogleCloudDialogflowV2ListSessionEntityTypesResponse,
                        > {
                            self.iter_with_fields(Some("*"))
                        }
                        pub fn iter_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> crate::iter::PageIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: AsRef<str>,
                        {
                            let mut fields =
                                fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                            if !fields.is_empty() {
                                match fields.chars().rev().nth(0) {
                                    Some(',') | None => {}
                                    _ => fields.push_str(","),
                                }
                                fields.push_str("nextPageToken");
                                self.fields = Some(fields);
                            }
                            crate::iter::PageIter::new(self)
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2ListSessionEntityTypesResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2ListSessionEntityTypesResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/entityTypes");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("pageSize", &self.page_size)]);
                            let req = req.query(&[("pageToken", &self.page_token)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                    impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                        fn set_page_token(&mut self, value: String) {
                            self.page_token = value.into();
                        }
                        fn execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            self._execute()
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                        name: String,
                        update_mask: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> PatchRequestBuilder<'a> {
                        #[doc = "Optional. The mask to control which fields get updated."]
                        pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                            self.update_mask = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudDialogflowV2SessionEntityType,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path())?;
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dialogflow.googleapis.com/".to_owned();
                            output.push_str("v2/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                            let req = req.query(&[("updateMask", &self.update_mask)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let req = req.bearer_auth(
                                self.auth
                                    .access_token()
                                    .map_err(|err| crate::Error::OAuth2(err))?,
                            );
                            Ok(req)
                        }
                    }
                }
            }
        }
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OperationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                    }
                }
                #[doc = "Lists operations that match the specified filter in the request. If the\nserver doesn't support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        name: name.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dialogflow.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The standard list filter."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The standard list page size."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The standard list page token."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                pub fn key(mut self, value: impl Into<String>) -> Self {
                    self.key = Some(value.into());
                    self
                }
                #[doc = "OAuth 2.0 token for the current user."]
                pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                    self.oauth_token = Some(value.into());
                    self
                }
                #[doc = "Returns response with indentations and line breaks."]
                pub fn pretty_print(mut self, value: bool) -> Self {
                    self.pretty_print = Some(value);
                    self
                }
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_operations<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_operations_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_operations_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleLongrunningOperation>
                {
                    self.iter_operations_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_operations_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleLongrunningOperation>
                {
                    self.iter_operations_with_fields(Some("*"))
                }
                pub fn iter_operations_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "operations").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "operations")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleLongrunningListOperationsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleLongrunningListOperationsResponse,
                > {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningListOperationsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningListOperationsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dialogflow.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/operations");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
        }
    }
}
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error>),
    JSON(::serde_json::Error),
    Reqwest(::reqwest::Error),
    Other(Box<dyn ::std::error::Error>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest(err) => err
                .get_ref()
                .and_then(|err| err.downcast_ref::<::serde_json::Error>()),
            Error::Other(_) => None,
        }
    }
}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
} // Bytes in google apis are represented as urlsafe base64 encoded strings.
  // This defines a Bytes type that is a simple wrapper around a Vec<u8> used
  // internally to handle byte fields in google apis.
pub mod bytes {
    use radix64::URL_SAFE as BASE64_CFG;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Bytes(Vec<u8>);

    impl ::std::convert::From<Vec<u8>> for Bytes {
        fn from(x: Vec<u8>) -> Bytes {
            Bytes(x)
        }
    }

    impl ::std::fmt::Display for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            ::radix64::Display::new(BASE64_CFG, &self.0).fmt(f)
        }
    }

    impl ::serde::Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            let encoded = BASE64_CFG.encode(&self.0);
            encoded.serialize(serializer)
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bytes, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            let encoded = String::deserialize(deserializer)?;
            let decoded = BASE64_CFG
                .decode(&encoded)
                .map_err(|_| ::serde::de::Error::custom("invalid base64 input"))?;
            Ok(Bytes(decoded))
        }
    }
}
