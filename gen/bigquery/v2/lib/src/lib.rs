pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AggregateClassificationMetrics {
        #[doc = "Accuracy is the fraction of predictions given the correct label. For\nmulticlass this is a micro-averaged metric."]
        #[serde(rename = "accuracy", default)]
        pub accuracy: Option<f64>,
        #[doc = "The F1 score is an average of recall and precision. For multiclass\nthis is a macro-averaged metric."]
        #[serde(rename = "f1Score", default)]
        pub f_1_score: Option<f64>,
        #[doc = "Logarithmic Loss. For multiclass this is a macro-averaged metric."]
        #[serde(rename = "logLoss", default)]
        pub log_loss: Option<f64>,
        #[doc = "Precision is the fraction of actual positive predictions that had\npositive actual labels. For multiclass this is a macro-averaged\nmetric treating each class as a binary classifier."]
        #[serde(rename = "precision", default)]
        pub precision: Option<f64>,
        #[doc = "Recall is the fraction of actual positive labels that were given a\npositive prediction. For multiclass this is a macro-averaged metric."]
        #[serde(rename = "recall", default)]
        pub recall: Option<f64>,
        #[doc = "Area Under a ROC Curve. For multiclass this is a macro-averaged\nmetric."]
        #[serde(rename = "rocAuc", default)]
        pub roc_auc: Option<f64>,
        #[doc = "Threshold at which the metrics are computed. For binary\nclassification models this is the positive class threshold.\nFor multi-class classfication models this is the confidence\nthreshold."]
        #[serde(rename = "threshold", default)]
        pub threshold: Option<f64>,
    }
    impl ::field_selector::FieldSelector for AggregateClassificationMetrics {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ArgumentArgumentKind {
        ArgumentKindUnspecified,
        #[doc = "The argument is a variable with fully specified type, which can be a\nstruct or an array, but not a table."]
        FixedType,
        #[doc = "The argument is any type, including struct or array, but not a table.\nTo be added: FIXED_TABLE, ANY_TABLE"]
        AnyType,
    }
    impl ArgumentArgumentKind {
        pub fn as_str(self) -> &'static str {
            match self {
                ArgumentArgumentKind::ArgumentKindUnspecified => "ARGUMENT_KIND_UNSPECIFIED",
                ArgumentArgumentKind::FixedType => "FIXED_TYPE",
                ArgumentArgumentKind::AnyType => "ANY_TYPE",
            }
        }
    }
    impl ::std::fmt::Display for ArgumentArgumentKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ArgumentArgumentKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArgumentArgumentKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARGUMENT_KIND_UNSPECIFIED" => ArgumentArgumentKind::ArgumentKindUnspecified,
                "FIXED_TYPE" => ArgumentArgumentKind::FixedType,
                "ANY_TYPE" => ArgumentArgumentKind::AnyType,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ArgumentMode {
        ModeUnspecified,
        #[doc = "The argument is input-only."]
        In,
        #[doc = "The argument is output-only."]
        Out,
        #[doc = "The argument is both an input and an output."]
        Inout,
    }
    impl ArgumentMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ArgumentMode::ModeUnspecified => "MODE_UNSPECIFIED",
                ArgumentMode::In => "IN",
                ArgumentMode::Out => "OUT",
                ArgumentMode::Inout => "INOUT",
            }
        }
    }
    impl ::std::fmt::Display for ArgumentMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ArgumentMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArgumentMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MODE_UNSPECIFIED" => ArgumentMode::ModeUnspecified,
                "IN" => ArgumentMode::In,
                "OUT" => ArgumentMode::Out,
                "INOUT" => ArgumentMode::Inout,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
    pub struct Argument {
        #[doc = "Optional. Defaults to FIXED_TYPE."]
        #[serde(rename = "argumentKind", default)]
        pub argument_kind: Option<crate::schemas::ArgumentArgumentKind>,
        #[doc = "Required unless argument_kind = ANY_TYPE."]
        #[serde(rename = "dataType", default)]
        pub data_type: Option<crate::schemas::StandardSqlDataType>,
        #[doc = "Optional. Specifies whether the argument is input or output.\nCan be set for procedures only."]
        #[serde(rename = "mode", default)]
        pub mode: Option<crate::schemas::ArgumentMode>,
        #[doc = "Optional. The name of this argument. Can be absent for function return\nargument."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Argument {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct BigQueryModelTraining {
        #[doc = "[Output-only, Beta] Index of current ML training iteration. Updated during create model query job to show job progress."]
        #[serde(rename = "currentIteration", default)]
        pub current_iteration: Option<i32>,
        #[doc = "[Output-only, Beta] Expected number of iterations for the create model query job specified as num_iterations in the input query. The actual total number of iterations may be less than this number due to early stop."]
        #[serde(rename = "expectedTotalIterations", default)]
        #[serde(with = "crate::parsed_string")]
        pub expected_total_iterations: Option<i64>,
    }
    impl ::field_selector::FieldSelector for BigQueryModelTraining {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct BigtableColumn {
        #[doc = "[Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels."]
        #[serde(rename = "encoding", default)]
        pub encoding: Option<String>,
        #[doc = "[Optional] If the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as the column field name and is used as field name in queries."]
        #[serde(rename = "fieldName", default)]
        pub field_name: Option<String>,
        #[doc = "[Optional] If this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels."]
        #[serde(rename = "onlyReadLatest", default)]
        pub only_read_latest: Option<bool>,
        #[doc = "[Required] Qualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifier_string field. Otherwise, a base-64 encoded value must be set to qualifier_encoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as field_name."]
        #[serde(rename = "qualifierEncoded", default)]
        pub qualifier_encoded: Option<Vec<u8>>,
        #[serde(rename = "qualifierString", default)]
        pub qualifier_string: Option<String>,
        #[doc = "[Optional] The type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
    }
    impl ::field_selector::FieldSelector for BigtableColumn {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct BigtableColumnFamily {
        #[doc = "[Optional] Lists of columns that should be exposed as individual fields as opposed to a list of (column name, value) pairs. All columns whose qualifier matches a qualifier in this list can be accessed as .. Other columns can be accessed as a list through .Column field."]
        #[serde(rename = "columns", default)]
        pub columns: Option<Vec<crate::schemas::BigtableColumn>>,
        #[doc = "[Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it."]
        #[serde(rename = "encoding", default)]
        pub encoding: Option<String>,
        #[doc = "Identifier of the column family."]
        #[serde(rename = "familyId", default)]
        pub family_id: Option<String>,
        #[doc = "[Optional] If this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column."]
        #[serde(rename = "onlyReadLatest", default)]
        pub only_read_latest: Option<bool>,
        #[doc = "[Optional] The type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
    }
    impl ::field_selector::FieldSelector for BigtableColumnFamily {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct BigtableOptions {
        #[doc = "[Optional] List of column families to expose in the table schema along with their types. This list restricts the column families that can be referenced in queries and specifies their value types. You can use this list to do type conversions - see the 'type' field for more details. If you leave this list empty, all column families are present in the table schema and their values are read as BYTES. During a query only the column families referenced in that query are read from Bigtable."]
        #[serde(rename = "columnFamilies", default)]
        pub column_families: Option<Vec<crate::schemas::BigtableColumnFamily>>,
        #[doc = "[Optional] If field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false."]
        #[serde(rename = "ignoreUnspecifiedColumnFamilies", default)]
        pub ignore_unspecified_column_families: Option<bool>,
        #[doc = "[Optional] If field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false."]
        #[serde(rename = "readRowkeyAsString", default)]
        pub read_rowkey_as_string: Option<bool>,
    }
    impl ::field_selector::FieldSelector for BigtableOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BinaryClassificationMetrics {
        #[doc = "Aggregate classification metrics."]
        #[serde(rename = "aggregateClassificationMetrics", default)]
        pub aggregate_classification_metrics:
            Option<crate::schemas::AggregateClassificationMetrics>,
        #[doc = "Binary confusion matrix at multiple thresholds."]
        #[serde(rename = "binaryConfusionMatrixList", default)]
        pub binary_confusion_matrix_list: Option<Vec<crate::schemas::BinaryConfusionMatrix>>,
        #[doc = "Label representing the negative class."]
        #[serde(rename = "negativeLabel", default)]
        pub negative_label: Option<String>,
        #[doc = "Label representing the positive class."]
        #[serde(rename = "positiveLabel", default)]
        pub positive_label: Option<String>,
    }
    impl ::field_selector::FieldSelector for BinaryClassificationMetrics {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BinaryConfusionMatrix {
        #[doc = "The fraction of predictions given the correct label."]
        #[serde(rename = "accuracy", default)]
        pub accuracy: Option<f64>,
        #[doc = "The equally weighted average of recall and precision."]
        #[serde(rename = "f1Score", default)]
        pub f_1_score: Option<f64>,
        #[doc = "Number of false samples predicted as false."]
        #[serde(rename = "falseNegatives", default)]
        #[serde(with = "crate::parsed_string")]
        pub false_negatives: Option<i64>,
        #[doc = "Number of false samples predicted as true."]
        #[serde(rename = "falsePositives", default)]
        #[serde(with = "crate::parsed_string")]
        pub false_positives: Option<i64>,
        #[doc = "Threshold value used when computing each of the following metric."]
        #[serde(rename = "positiveClassThreshold", default)]
        pub positive_class_threshold: Option<f64>,
        #[doc = "The fraction of actual positive predictions that had positive actual\nlabels."]
        #[serde(rename = "precision", default)]
        pub precision: Option<f64>,
        #[doc = "The fraction of actual positive labels that were given a positive\nprediction."]
        #[serde(rename = "recall", default)]
        pub recall: Option<f64>,
        #[doc = "Number of true samples predicted as false."]
        #[serde(rename = "trueNegatives", default)]
        #[serde(with = "crate::parsed_string")]
        pub true_negatives: Option<i64>,
        #[doc = "Number of true samples predicted as true."]
        #[serde(rename = "truePositives", default)]
        #[serde(with = "crate::parsed_string")]
        pub true_positives: Option<i64>,
    }
    impl ::field_selector::FieldSelector for BinaryConfusionMatrix {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BqmlIterationResult {
        #[doc = "[Output-only, Beta] Time taken to run the training iteration in milliseconds."]
        #[serde(rename = "durationMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub duration_ms: Option<i64>,
        #[doc = "[Output-only, Beta] Eval loss computed on the eval data at the end of the iteration. The eval loss is used for early stopping to avoid overfitting. No eval loss if eval_split_method option is specified as no_split or auto_split with input data size less than 500 rows."]
        #[serde(rename = "evalLoss", default)]
        pub eval_loss: Option<f64>,
        #[doc = "[Output-only, Beta] Index of the ML training iteration, starting from zero for each training run."]
        #[serde(rename = "index", default)]
        pub index: Option<i32>,
        #[doc = "[Output-only, Beta] Learning rate used for this iteration, it varies for different training iterations if learn_rate_strategy option is not constant."]
        #[serde(rename = "learnRate", default)]
        pub learn_rate: Option<f64>,
        #[doc = "[Output-only, Beta] Training loss computed on the training data at the end of the iteration. The training loss function is defined by model type."]
        #[serde(rename = "trainingLoss", default)]
        pub training_loss: Option<f64>,
    }
    impl ::field_selector::FieldSelector for BqmlIterationResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BqmlTrainingRunTrainingOptions {
        #[serde(rename = "earlyStop", default)]
        pub early_stop: Option<bool>,
        #[serde(rename = "l1Reg", default)]
        pub l_1_reg: Option<f64>,
        #[serde(rename = "l2Reg", default)]
        pub l_2_reg: Option<f64>,
        #[serde(rename = "learnRate", default)]
        pub learn_rate: Option<f64>,
        #[serde(rename = "learnRateStrategy", default)]
        pub learn_rate_strategy: Option<String>,
        #[serde(rename = "lineSearchInitLearnRate", default)]
        pub line_search_init_learn_rate: Option<f64>,
        #[serde(rename = "maxIteration", default)]
        #[serde(with = "crate::parsed_string")]
        pub max_iteration: Option<i64>,
        #[serde(rename = "minRelProgress", default)]
        pub min_rel_progress: Option<f64>,
        #[serde(rename = "warmStart", default)]
        pub warm_start: Option<bool>,
    }
    impl ::field_selector::FieldSelector for BqmlTrainingRunTrainingOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BqmlTrainingRun {
        #[doc = "[Output-only, Beta] List of each iteration results."]
        #[serde(rename = "iterationResults", default)]
        pub iteration_results: Option<Vec<crate::schemas::BqmlIterationResult>>,
        #[doc = "[Output-only, Beta] Training run start time in milliseconds since the epoch."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "[Output-only, Beta] Different state applicable for a training run. IN PROGRESS: Training run is in progress. FAILED: Training run ended due to a non-retryable failure. SUCCEEDED: Training run successfully completed. CANCELLED: Training run cancelled by the user."]
        #[serde(rename = "state", default)]
        pub state: Option<String>,
        #[doc = "[Output-only, Beta] Training options used by this training run. These options are mutable for subsequent training runs. Default values are explicitly stored for options not specified in the input query of the first training run. For subsequent training runs, any option not explicitly specified in the input query will be copied from the previous training run."]
        #[serde(rename = "trainingOptions", default)]
        pub training_options: Option<crate::schemas::BqmlTrainingRunTrainingOptions>,
    }
    impl ::field_selector::FieldSelector for BqmlTrainingRun {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct CategoricalValue {
        #[doc = "Counts of all categories for the categorical feature. If there are\nmore than ten categories, we return top ten (by count) and return\none more CategoryCount with category \"_OTHER_\" and count as\naggregate counts of remaining categories."]
        #[serde(rename = "categoryCounts", default)]
        pub category_counts: Option<Vec<crate::schemas::CategoryCount>>,
    }
    impl ::field_selector::FieldSelector for CategoricalValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct CategoryCount {
        #[doc = "The name of category."]
        #[serde(rename = "category", default)]
        pub category: Option<String>,
        #[doc = "The count of training samples matching the category within the\ncluster."]
        #[serde(rename = "count", default)]
        #[serde(with = "crate::parsed_string")]
        pub count: Option<i64>,
    }
    impl ::field_selector::FieldSelector for CategoryCount {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cluster {
        #[doc = "Centroid id."]
        #[serde(rename = "centroidId", default)]
        #[serde(with = "crate::parsed_string")]
        pub centroid_id: Option<i64>,
        #[doc = "Count of training data rows that were assigned to this cluster."]
        #[serde(rename = "count", default)]
        #[serde(with = "crate::parsed_string")]
        pub count: Option<i64>,
        #[doc = "Values of highly variant features for this cluster."]
        #[serde(rename = "featureValues", default)]
        pub feature_values: Option<Vec<crate::schemas::FeatureValue>>,
    }
    impl ::field_selector::FieldSelector for Cluster {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ClusterInfo {
        #[doc = "Centroid id."]
        #[serde(rename = "centroidId", default)]
        #[serde(with = "crate::parsed_string")]
        pub centroid_id: Option<i64>,
        #[doc = "Cluster radius, the average distance from centroid\nto each point assigned to the cluster."]
        #[serde(rename = "clusterRadius", default)]
        pub cluster_radius: Option<f64>,
        #[doc = "Cluster size, the total number of points assigned to the cluster."]
        #[serde(rename = "clusterSize", default)]
        #[serde(with = "crate::parsed_string")]
        pub cluster_size: Option<i64>,
    }
    impl ::field_selector::FieldSelector for ClusterInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct Clustering {
        #[doc = "[Repeated] One or more fields on which data should be clustered. Only top-level, non-repeated, simple-type fields are supported. When you cluster a table using multiple columns, the order of columns you specify is important. The order of the specified columns determines the sort order of the data."]
        #[serde(rename = "fields", default)]
        pub fields: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for Clustering {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ClusteringMetrics {
        #[doc = "[Beta] Information for all clusters."]
        #[serde(rename = "clusters", default)]
        pub clusters: Option<Vec<crate::schemas::Cluster>>,
        #[doc = "Davies-Bouldin index."]
        #[serde(rename = "daviesBouldinIndex", default)]
        pub davies_bouldin_index: Option<f64>,
        #[doc = "Mean of squared distances between each sample to its cluster centroid."]
        #[serde(rename = "meanSquaredDistance", default)]
        pub mean_squared_distance: Option<f64>,
    }
    impl ::field_selector::FieldSelector for ClusteringMetrics {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ConfusionMatrix {
        #[doc = "Confidence threshold used when computing the entries of the\nconfusion matrix."]
        #[serde(rename = "confidenceThreshold", default)]
        pub confidence_threshold: Option<f64>,
        #[doc = "One row per actual label."]
        #[serde(rename = "rows", default)]
        pub rows: Option<Vec<crate::schemas::Row>>,
    }
    impl ::field_selector::FieldSelector for ConfusionMatrix {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct CsvOptions {
        #[doc = "[Optional] Indicates if BigQuery should accept rows that are missing trailing optional columns. If true, BigQuery treats missing trailing columns as null values. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false."]
        #[serde(rename = "allowJaggedRows", default)]
        pub allow_jagged_rows: Option<bool>,
        #[doc = "[Optional] Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
        #[serde(rename = "allowQuotedNewlines", default)]
        pub allow_quoted_newlines: Option<bool>,
        #[doc = "[Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties."]
        #[serde(rename = "encoding", default)]
        pub encoding: Option<String>,
        #[doc = "[Optional] The separator for fields in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',')."]
        #[serde(rename = "fieldDelimiter", default)]
        pub field_delimiter: Option<String>,
        #[doc = "[Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
        #[serde(rename = "quote", default)]
        pub quote: Option<String>,
        #[doc = "[Optional] The number of rows at the top of a CSV file that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped."]
        #[serde(rename = "skipLeadingRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub skip_leading_rows: Option<i64>,
    }
    impl ::field_selector::FieldSelector for CsvOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct DatasetAccessItems {
        #[doc = "[Pick one] A domain to grant access to. Any users signed in with the domain specified will be granted the specified access. Example: \"example.com\". Maps to IAM policy member \"domain:DOMAIN\"."]
        #[serde(rename = "domain", default)]
        pub domain: Option<String>,
        #[doc = "[Pick one] An email address of a Google Group to grant access to. Maps to IAM policy member \"group:GROUP\"."]
        #[serde(rename = "groupByEmail", default)]
        pub group_by_email: Option<String>,
        #[doc = "[Pick one] Some other type of member that appears in the IAM Policy but isn't a user, group, domain, or special group."]
        #[serde(rename = "iamMember", default)]
        pub iam_member: Option<String>,
        #[doc = "[Required] An IAM role ID that should be granted to the user, group, or domain specified in this access entry. The following legacy mappings will be applied: OWNER  roles/bigquery.dataOwner WRITER  roles/bigquery.dataEditor READER  roles/bigquery.dataViewer This field will accept any of the above formats, but will return only the legacy format. For example, if you set this field to \"roles/bigquery.dataOwner\", it will be returned back as \"OWNER\"."]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
        #[doc = "[Pick one] A special group to grant access to. Possible values include: projectOwners: Owners of the enclosing project. projectReaders: Readers of the enclosing project. projectWriters: Writers of the enclosing project. allAuthenticatedUsers: All authenticated BigQuery users. Maps to similarly-named IAM members."]
        #[serde(rename = "specialGroup", default)]
        pub special_group: Option<String>,
        #[doc = "[Pick one] An email address of a user to grant access to. For example: fred@example.com. Maps to IAM policy member \"user:EMAIL\" or \"serviceAccount:EMAIL\"."]
        #[serde(rename = "userByEmail", default)]
        pub user_by_email: Option<String>,
        #[doc = "[Pick one] A view from a different dataset to grant access to. Queries executed against that view will have read access to tables in this dataset. The role field is not required when this field is set. If that view is updated by any user, access to the view needs to be granted again via an update operation."]
        #[serde(rename = "view", default)]
        pub view: Option<crate::schemas::TableReference>,
    }
    impl ::field_selector::FieldSelector for DatasetAccessItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct Dataset {
        #[doc = "[Optional] An array of objects that define dataset access for one or more entities. You can set this property when inserting or updating a dataset in order to control who is allowed to access the data. If unspecified at dataset creation time, BigQuery adds default dataset access for the following entities: access.specialGroup: projectReaders; access.role: READER; access.specialGroup: projectWriters; access.role: WRITER; access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset creator email]; access.role: OWNER;"]
        #[serde(rename = "access", default)]
        pub access: Option<Vec<crate::schemas::DatasetAccessItems>>,
        #[doc = "[Output-only] The time when this dataset was created, in milliseconds since the epoch."]
        #[serde(rename = "creationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: Option<i64>,
        #[doc = "[Required] A reference that identifies the dataset."]
        #[serde(rename = "datasetReference", default)]
        pub dataset_reference: Option<crate::schemas::DatasetReference>,
        #[serde(rename = "defaultEncryptionConfiguration", default)]
        pub default_encryption_configuration: Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Optional] The default partition expiration for all partitioned tables in the dataset, in milliseconds. Once this property is set, all newly-created partitioned tables in the dataset will have an expirationMs property in the timePartitioning settings set to this value, and changing the value will only affect new tables, not existing ones. The storage in a partition will have an expiration time of its partition time plus this value. Setting this property overrides the use of defaultTableExpirationMs for partitioned tables: only one of defaultTableExpirationMs and defaultPartitionExpirationMs will be used for any new partitioned table. If you provide an explicit timePartitioning.expirationMs when creating or updating a partitioned table, that value takes precedence over the default partition expiration time indicated by this property."]
        #[serde(rename = "defaultPartitionExpirationMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub default_partition_expiration_ms: Option<i64>,
        #[doc = "[Optional] The default lifetime of all tables in the dataset, in milliseconds. The minimum value is 3600000 milliseconds (one hour). Once this property is set, all newly-created tables in the dataset will have an expirationTime property set to the creation time plus the value in this property, and changing the value will only affect new tables, not existing ones. When the expirationTime for a given table is reached, that table will be deleted automatically. If a table's expirationTime is modified or removed before the table expires, or if you provide an explicit expirationTime when creating a table, that value takes precedence over the default expiration time indicated by this property."]
        #[serde(rename = "defaultTableExpirationMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub default_table_expiration_ms: Option<i64>,
        #[doc = "[Optional] A user-friendly description of the dataset."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "[Output-only] A hash of the resource."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "[Optional] A descriptive name for the dataset."]
        #[serde(rename = "friendlyName", default)]
        pub friendly_name: Option<String>,
        #[doc = "[Output-only] The fully-qualified unique name of the dataset in the format projectId:datasetId. The dataset name without the project name is given in the datasetId field. When creating a new dataset, leave this field blank, and instead specify the datasetId field."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "[Output-only] The resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The labels associated with this dataset. You can use these to organize and group your datasets. You can set this property when inserting or updating a dataset. See Creating and Updating Dataset Labels for more information."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "[Output-only] The date when this dataset or any of its tables was last modified, in milliseconds since the epoch."]
        #[serde(rename = "lastModifiedTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_time: Option<i64>,
        #[doc = "The geographic location where the dataset should reside. The default value is US. See details at https://cloud.google.com/bigquery/docs/locations."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "[Output-only] A URL that can be used to access the resource again. You can use this URL in Get or Update requests to the resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
    }
    impl ::field_selector::FieldSelector for Dataset {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct DatasetListDatasetsItems {
        #[doc = "The dataset reference. Use this property to access specific parts of the dataset's ID, such as project ID or dataset ID."]
        #[serde(rename = "datasetReference", default)]
        pub dataset_reference: Option<crate::schemas::DatasetReference>,
        #[doc = "A descriptive name for the dataset, if one exists."]
        #[serde(rename = "friendlyName", default)]
        pub friendly_name: Option<String>,
        #[doc = "The fully-qualified, unique, opaque ID of the dataset."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The resource type. This property always returns the value \"bigquery#dataset\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The labels associated with this dataset. You can use these to organize and group your datasets."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The geographic location where the data resides."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
    }
    impl ::field_selector::FieldSelector for DatasetListDatasetsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct DatasetList {
        #[doc = "An array of the dataset resources in the project. Each resource contains basic information. For full information about a particular dataset resource, use the Datasets: get method. This property is omitted when there are no datasets in the project."]
        #[serde(rename = "datasets", default)]
        pub datasets: Option<Vec<crate::schemas::DatasetListDatasetsItems>>,
        #[doc = "A hash value of the results page. You can use this property to determine if the page has changed since the last request."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The list type. This property always returns the value \"bigquery#datasetList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "A token that can be used to request the next results page. This property is omitted on the final results page."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for DatasetList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct DatasetReference {
        #[doc = "[Required] A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
        #[serde(rename = "datasetId", default)]
        pub dataset_id: Option<String>,
        #[doc = "[Optional] The ID of the project containing this dataset."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for DatasetReference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct DestinationTableProperties {
        #[doc = "[Optional] The description for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current description is provided, the job will fail."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "[Optional] The friendly name for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current friendly name is provided, the job will fail."]
        #[serde(rename = "friendlyName", default)]
        pub friendly_name: Option<String>,
        #[doc = "[Optional] The labels associated with this table. You can use these to organize and group your tables. This will only be used if the destination table is newly created. If the table already exists and labels are different than the current labels are provided, the job will fail."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for DestinationTableProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct EncryptionConfiguration {
        #[doc = "[Optional] Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key."]
        #[serde(rename = "kmsKeyName", default)]
        pub kms_key_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for EncryptionConfiguration {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct Entry {
        #[doc = "Number of items being predicted as this label."]
        #[serde(rename = "itemCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub item_count: Option<i64>,
        #[doc = "The predicted label. For confidence_threshold > 0, we will\nalso add an entry indicating the number of items under the\nconfidence threshold."]
        #[serde(rename = "predictedLabel", default)]
        pub predicted_label: Option<String>,
    }
    impl ::field_selector::FieldSelector for Entry {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ErrorProto {
        #[doc = "Debugging information. This property is internal to Google and should not be used."]
        #[serde(rename = "debugInfo", default)]
        pub debug_info: Option<String>,
        #[doc = "Specifies where the error occurred, if present."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "A human-readable description of the error."]
        #[serde(rename = "message", default)]
        pub message: Option<String>,
        #[doc = "A short error code that summarizes the error."]
        #[serde(rename = "reason", default)]
        pub reason: Option<String>,
    }
    impl ::field_selector::FieldSelector for ErrorProto {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EvaluationMetrics {
        #[doc = "Populated for binary classification/classifier models."]
        #[serde(rename = "binaryClassificationMetrics", default)]
        pub binary_classification_metrics: Option<crate::schemas::BinaryClassificationMetrics>,
        #[doc = "Populated for clustering models."]
        #[serde(rename = "clusteringMetrics", default)]
        pub clustering_metrics: Option<crate::schemas::ClusteringMetrics>,
        #[doc = "Populated for multi-class classification/classifier models."]
        #[serde(rename = "multiClassClassificationMetrics", default)]
        pub multi_class_classification_metrics:
            Option<crate::schemas::MultiClassClassificationMetrics>,
        #[doc = "Populated for regression models and explicit feedback type matrix\nfactorization models."]
        #[serde(rename = "regressionMetrics", default)]
        pub regression_metrics: Option<crate::schemas::RegressionMetrics>,
    }
    impl ::field_selector::FieldSelector for EvaluationMetrics {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ExplainQueryStage {
        #[doc = "Number of parallel input segments completed."]
        #[serde(rename = "completedParallelInputs", default)]
        #[serde(with = "crate::parsed_string")]
        pub completed_parallel_inputs: Option<i64>,
        #[doc = "Milliseconds the average shard spent on CPU-bound tasks."]
        #[serde(rename = "computeMsAvg", default)]
        #[serde(with = "crate::parsed_string")]
        pub compute_ms_avg: Option<i64>,
        #[doc = "Milliseconds the slowest shard spent on CPU-bound tasks."]
        #[serde(rename = "computeMsMax", default)]
        #[serde(with = "crate::parsed_string")]
        pub compute_ms_max: Option<i64>,
        #[doc = "Relative amount of time the average shard spent on CPU-bound tasks."]
        #[serde(rename = "computeRatioAvg", default)]
        pub compute_ratio_avg: Option<f64>,
        #[doc = "Relative amount of time the slowest shard spent on CPU-bound tasks."]
        #[serde(rename = "computeRatioMax", default)]
        pub compute_ratio_max: Option<f64>,
        #[doc = "Stage end time represented as milliseconds since epoch."]
        #[serde(rename = "endMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_ms: Option<i64>,
        #[doc = "Unique ID for stage within plan."]
        #[serde(rename = "id", default)]
        #[serde(with = "crate::parsed_string")]
        pub id: Option<i64>,
        #[doc = "IDs for stages that are inputs to this stage."]
        #[serde(rename = "inputStages", default)]
        pub input_stages: Option<Vec<i64>>,
        #[doc = "Human-readable name for stage."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Number of parallel input segments to be processed."]
        #[serde(rename = "parallelInputs", default)]
        #[serde(with = "crate::parsed_string")]
        pub parallel_inputs: Option<i64>,
        #[doc = "Milliseconds the average shard spent reading input."]
        #[serde(rename = "readMsAvg", default)]
        #[serde(with = "crate::parsed_string")]
        pub read_ms_avg: Option<i64>,
        #[doc = "Milliseconds the slowest shard spent reading input."]
        #[serde(rename = "readMsMax", default)]
        #[serde(with = "crate::parsed_string")]
        pub read_ms_max: Option<i64>,
        #[doc = "Relative amount of time the average shard spent reading input."]
        #[serde(rename = "readRatioAvg", default)]
        pub read_ratio_avg: Option<f64>,
        #[doc = "Relative amount of time the slowest shard spent reading input."]
        #[serde(rename = "readRatioMax", default)]
        pub read_ratio_max: Option<f64>,
        #[doc = "Number of records read into the stage."]
        #[serde(rename = "recordsRead", default)]
        #[serde(with = "crate::parsed_string")]
        pub records_read: Option<i64>,
        #[doc = "Number of records written by the stage."]
        #[serde(rename = "recordsWritten", default)]
        #[serde(with = "crate::parsed_string")]
        pub records_written: Option<i64>,
        #[doc = "Total number of bytes written to shuffle."]
        #[serde(rename = "shuffleOutputBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub shuffle_output_bytes: Option<i64>,
        #[doc = "Total number of bytes written to shuffle and spilled to disk."]
        #[serde(rename = "shuffleOutputBytesSpilled", default)]
        #[serde(with = "crate::parsed_string")]
        pub shuffle_output_bytes_spilled: Option<i64>,
        #[doc = "Stage start time represented as milliseconds since epoch."]
        #[serde(rename = "startMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_ms: Option<i64>,
        #[doc = "Current status for the stage."]
        #[serde(rename = "status", default)]
        pub status: Option<String>,
        #[doc = "List of operations within the stage in dependency order (approximately chronological)."]
        #[serde(rename = "steps", default)]
        pub steps: Option<Vec<crate::schemas::ExplainQueryStep>>,
        #[doc = "Milliseconds the average shard spent waiting to be scheduled."]
        #[serde(rename = "waitMsAvg", default)]
        #[serde(with = "crate::parsed_string")]
        pub wait_ms_avg: Option<i64>,
        #[doc = "Milliseconds the slowest shard spent waiting to be scheduled."]
        #[serde(rename = "waitMsMax", default)]
        #[serde(with = "crate::parsed_string")]
        pub wait_ms_max: Option<i64>,
        #[doc = "Relative amount of time the average shard spent waiting to be scheduled."]
        #[serde(rename = "waitRatioAvg", default)]
        pub wait_ratio_avg: Option<f64>,
        #[doc = "Relative amount of time the slowest shard spent waiting to be scheduled."]
        #[serde(rename = "waitRatioMax", default)]
        pub wait_ratio_max: Option<f64>,
        #[doc = "Milliseconds the average shard spent on writing output."]
        #[serde(rename = "writeMsAvg", default)]
        #[serde(with = "crate::parsed_string")]
        pub write_ms_avg: Option<i64>,
        #[doc = "Milliseconds the slowest shard spent on writing output."]
        #[serde(rename = "writeMsMax", default)]
        #[serde(with = "crate::parsed_string")]
        pub write_ms_max: Option<i64>,
        #[doc = "Relative amount of time the average shard spent on writing output."]
        #[serde(rename = "writeRatioAvg", default)]
        pub write_ratio_avg: Option<f64>,
        #[doc = "Relative amount of time the slowest shard spent on writing output."]
        #[serde(rename = "writeRatioMax", default)]
        pub write_ratio_max: Option<f64>,
    }
    impl ::field_selector::FieldSelector for ExplainQueryStage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ExplainQueryStep {
        #[doc = "Machine-readable operation type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Human-readable stage descriptions."]
        #[serde(rename = "substeps", default)]
        pub substeps: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ExplainQueryStep {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ExternalDataConfiguration {
        #[doc = "Try to detect schema and format options automatically. Any option specified explicitly will be honored."]
        #[serde(rename = "autodetect", default)]
        pub autodetect: Option<bool>,
        #[doc = "[Optional] Additional options if sourceFormat is set to BIGTABLE."]
        #[serde(rename = "bigtableOptions", default)]
        pub bigtable_options: Option<crate::schemas::BigtableOptions>,
        #[doc = "[Optional] The compression type of the data source. Possible values include GZIP and NONE. The default value is NONE. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats."]
        #[serde(rename = "compression", default)]
        pub compression: Option<String>,
        #[doc = "Additional properties to set if sourceFormat is set to CSV."]
        #[serde(rename = "csvOptions", default)]
        pub csv_options: Option<crate::schemas::CsvOptions>,
        #[doc = "[Optional] Additional options if sourceFormat is set to GOOGLE_SHEETS."]
        #[serde(rename = "googleSheetsOptions", default)]
        pub google_sheets_options: Option<crate::schemas::GoogleSheetsOptions>,
        #[doc = "[Optional, Trusted Tester] If hive partitioning is enabled, which mode to use. Two modes are supported: - AUTO: automatically infer partition key name(s) and type(s). - STRINGS: automatic infer partition key name(s). All types are strings. Not all storage formats support hive partitioning -- requesting hive partitioning on an unsupported format will lead to an error. Note: this setting is in the process of being deprecated in favor of hivePartitioningOptions."]
        #[serde(rename = "hivePartitioningMode", default)]
        pub hive_partitioning_mode: Option<String>,
        #[doc = "[Optional, Trusted Tester] Options to configure hive partitioning support."]
        #[serde(rename = "hivePartitioningOptions", default)]
        pub hive_partitioning_options: Option<crate::schemas::HivePartitioningOptions>,
        #[doc = "[Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names Google Cloud Bigtable: This setting is ignored. Google Cloud Datastore backups: This setting is ignored. Avro: This setting is ignored."]
        #[serde(rename = "ignoreUnknownValues", default)]
        pub ignore_unknown_values: Option<bool>,
        #[doc = "[Optional] The maximum number of bad records that BigQuery can ignore when reading data. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV, JSON, and Google Sheets. The default value is 0, which requires that all records are valid. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups and Avro formats."]
        #[serde(rename = "maxBadRecords", default)]
        pub max_bad_records: Option<i32>,
        #[doc = "[Optional] The schema for the data. Schema is required for CSV and JSON formats. Schema is disallowed for Google Cloud Bigtable, Cloud Datastore backups, and Avro formats."]
        #[serde(rename = "schema", default)]
        pub schema: Option<crate::schemas::TableSchema>,
        #[doc = "[Required] The data format. For CSV files, specify \"CSV\". For Google sheets, specify \"GOOGLE_SHEETS\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro files, specify \"AVRO\". For Google Cloud Datastore backups, specify \"DATASTORE_BACKUP\". [Beta] For Google Cloud Bigtable, specify \"BIGTABLE\"."]
        #[serde(rename = "sourceFormat", default)]
        pub source_format: Option<String>,
        #[doc = "[Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups, exactly one URI can be specified. Also, the '*' wildcard character is not allowed."]
        #[serde(rename = "sourceUris", default)]
        pub source_uris: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ExternalDataConfiguration {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FeatureValue {
        #[doc = "The categorical feature value."]
        #[serde(rename = "categoricalValue", default)]
        pub categorical_value: Option<crate::schemas::CategoricalValue>,
        #[doc = "The feature column name."]
        #[serde(rename = "featureColumn", default)]
        pub feature_column: Option<String>,
        #[doc = "The numerical feature value. This is the centroid value for this\nfeature."]
        #[serde(rename = "numericalValue", default)]
        pub numerical_value: Option<f64>,
    }
    impl ::field_selector::FieldSelector for FeatureValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GetQueryResultsResponse {
        #[doc = "Whether the query result was fetched from the query cache."]
        #[serde(rename = "cacheHit", default)]
        pub cache_hit: Option<bool>,
        #[doc = "[Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        #[serde(rename = "errors", default)]
        pub errors: Option<Vec<crate::schemas::ErrorProto>>,
        #[doc = "A hash of this response."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available."]
        #[serde(rename = "jobComplete", default)]
        pub job_complete: Option<bool>,
        #[doc = "Reference to the BigQuery Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults)."]
        #[serde(rename = "jobReference", default)]
        pub job_reference: Option<crate::schemas::JobReference>,
        #[doc = "The resource type of the response."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        #[serde(rename = "numDmlAffectedRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_dml_affected_rows: Option<i64>,
        #[doc = "A token used for paging results."]
        #[serde(rename = "pageToken", default)]
        pub page_token: Option<String>,
        #[doc = "An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above. Present only when the query completes successfully."]
        #[serde(rename = "rows", default)]
        pub rows: Option<Vec<crate::schemas::TableRow>>,
        #[doc = "The schema of the results. Present only when the query completes successfully."]
        #[serde(rename = "schema", default)]
        pub schema: Option<crate::schemas::TableSchema>,
        #[doc = "The total number of bytes processed for this query."]
        #[serde(rename = "totalBytesProcessed", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_processed: Option<i64>,
        #[doc = "The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results. Present only when the query completes successfully."]
        #[serde(rename = "totalRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_rows: Option<u64>,
    }
    impl ::field_selector::FieldSelector for GetQueryResultsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct GetServiceAccountResponse {
        #[doc = "The service account email address."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The resource type of the response."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for GetServiceAccountResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct GoogleSheetsOptions {
        #[doc = "[Beta] [Optional] Range of a sheet to query from. Only used when non-empty. Typical format: sheet_name!top_left_cell_id:bottom_right_cell_id For example: sheet1!A1:B20"]
        #[serde(rename = "range", default)]
        pub range: Option<String>,
        #[doc = "[Optional] The number of rows at the top of a sheet that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows that should be skipped. When autodetect is on, behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
        #[serde(rename = "skipLeadingRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub skip_leading_rows: Option<i64>,
    }
    impl ::field_selector::FieldSelector for GoogleSheetsOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct HivePartitioningOptions {
        #[doc = "[Optional, Trusted Tester] When set, what mode of hive partitioning to use when reading data. Two modes are supported. (1) AUTO: automatically infer partition key name(s) and type(s). (2) STRINGS: automatically infer partition key name(s). All types are interpreted as strings. Not all storage formats support hive partitioning. Requesting hive partitioning on an unsupported format will lead to an error. Currently supported types include: AVRO, CSV, JSON, ORC and Parquet."]
        #[serde(rename = "mode", default)]
        pub mode: Option<String>,
        #[doc = "[Optional, Trusted Tester] When hive partition detection is requested, a common prefix for all source uris should be supplied. The prefix must end immediately before the partition key encoding begins. For example, consider files following this data layout. gs://bucket/path_to_table/dt=2019-01-01/country=BR/id=7/file.avro gs://bucket/path_to_table/dt=2018-12-31/country=CA/id=3/file.avro When hive partitioning is requested with either AUTO or STRINGS detection, the common prefix can be either of gs://bucket/path_to_table or gs://bucket/path_to_table/ (trailing slash does not matter)."]
        #[serde(rename = "sourceUriPrefix", default)]
        pub source_uri_prefix: Option<String>,
    }
    impl ::field_selector::FieldSelector for HivePartitioningOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct IterationResult {
        #[doc = "Information about top clusters for clustering models."]
        #[serde(rename = "clusterInfos", default)]
        pub cluster_infos: Option<Vec<crate::schemas::ClusterInfo>>,
        #[doc = "Time taken to run the iteration in milliseconds."]
        #[serde(rename = "durationMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub duration_ms: Option<i64>,
        #[doc = "Loss computed on the eval data at the end of iteration."]
        #[serde(rename = "evalLoss", default)]
        pub eval_loss: Option<f64>,
        #[doc = "Index of the iteration, 0 based."]
        #[serde(rename = "index", default)]
        pub index: Option<i32>,
        #[doc = "Learn rate used for this iteration."]
        #[serde(rename = "learnRate", default)]
        pub learn_rate: Option<f64>,
        #[doc = "Loss computed on the training data at the end of iteration."]
        #[serde(rename = "trainingLoss", default)]
        pub training_loss: Option<f64>,
    }
    impl ::field_selector::FieldSelector for IterationResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Job {
        #[doc = "[Required] Describes the job configuration."]
        #[serde(rename = "configuration", default)]
        pub configuration: Option<crate::schemas::JobConfiguration>,
        #[doc = "[Output-only] A hash of this resource."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "[Output-only] Opaque ID field of the job"]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "[Optional] Reference describing the unique-per-user name of the job."]
        #[serde(rename = "jobReference", default)]
        pub job_reference: Option<crate::schemas::JobReference>,
        #[doc = "[Output-only] The type of the resource."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "[Output-only] A URL that can be used to access this resource again."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
        #[doc = "[Output-only] Information about the job, including starting time and ending time of the job."]
        #[serde(rename = "statistics", default)]
        pub statistics: Option<crate::schemas::JobStatistics>,
        #[doc = "[Output-only] The status of this job. Examine this value when polling an asynchronous job to see if the job is complete."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::JobStatus>,
        #[doc = "[Output-only] Email address of the user who ran the job."]
        #[serde(rename = "user_email", default)]
        pub user_email: Option<String>,
    }
    impl ::field_selector::FieldSelector for Job {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobCancelResponse {
        #[doc = "The final state of the job."]
        #[serde(rename = "job", default)]
        pub job: Option<crate::schemas::Job>,
        #[doc = "The resource type of the response."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobCancelResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobConfiguration {
        #[doc = "[Pick one] Copies a table."]
        #[serde(rename = "copy", default)]
        pub copy: Option<crate::schemas::JobConfigurationTableCopy>,
        #[doc = "[Optional] If set, don't actually run this job. A valid query will return a mostly empty response with some processing statistics, while an invalid query will return the same error it would if it wasn't a dry run. Behavior of non-query jobs is undefined."]
        #[serde(rename = "dryRun", default)]
        pub dry_run: Option<bool>,
        #[doc = "[Pick one] Configures an extract job."]
        #[serde(rename = "extract", default)]
        pub extract: Option<crate::schemas::JobConfigurationExtract>,
        #[doc = "[Optional] Job timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
        #[serde(rename = "jobTimeoutMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub job_timeout_ms: Option<i64>,
        #[doc = "[Output-only] The type of the job. Can be QUERY, LOAD, EXTRACT, COPY or UNKNOWN."]
        #[serde(rename = "jobType", default)]
        pub job_type: Option<String>,
        #[doc = "The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "[Pick one] Configures a load job."]
        #[serde(rename = "load", default)]
        pub load: Option<crate::schemas::JobConfigurationLoad>,
        #[doc = "[Pick one] Configures a query job."]
        #[serde(rename = "query", default)]
        pub query: Option<crate::schemas::JobConfigurationQuery>,
    }
    impl ::field_selector::FieldSelector for JobConfiguration {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobConfigurationExtract {
        #[doc = "[Optional] The compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE. The default value is NONE. DEFLATE and SNAPPY are only supported for Avro."]
        #[serde(rename = "compression", default)]
        pub compression: Option<String>,
        #[doc = "[Optional] The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO. The default value is CSV. Tables with nested or repeated fields cannot be exported as CSV."]
        #[serde(rename = "destinationFormat", default)]
        pub destination_format: Option<String>,
        #[doc = "[Pick one] DEPRECATED: Use destinationUris instead, passing only one URI as necessary. The fully-qualified Google Cloud Storage URI where the extracted table should be written."]
        #[serde(rename = "destinationUri", default)]
        pub destination_uri: Option<String>,
        #[doc = "[Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written."]
        #[serde(rename = "destinationUris", default)]
        pub destination_uris: Option<Vec<String>>,
        #[doc = "[Optional] Delimiter to use between fields in the exported data. Default is ','"]
        #[serde(rename = "fieldDelimiter", default)]
        pub field_delimiter: Option<String>,
        #[doc = "[Optional] Whether to print out a header row in the results. Default is true."]
        #[serde(rename = "printHeader", default)]
        pub print_header: Option<bool>,
        #[doc = "[Required] A reference to the table being exported."]
        #[serde(rename = "sourceTable", default)]
        pub source_table: Option<crate::schemas::TableReference>,
    }
    impl ::field_selector::FieldSelector for JobConfigurationExtract {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobConfigurationLoad {
        #[doc = "[Optional] Accept rows that are missing trailing optional columns. The missing values are treated as nulls. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats."]
        #[serde(rename = "allowJaggedRows", default)]
        pub allow_jagged_rows: Option<bool>,
        #[doc = "Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
        #[serde(rename = "allowQuotedNewlines", default)]
        pub allow_quoted_newlines: Option<bool>,
        #[doc = "[Optional] Indicates if we should automatically infer the options and schema for CSV and JSON sources."]
        #[serde(rename = "autodetect", default)]
        pub autodetect: Option<bool>,
        #[doc = "[Beta] Clustering specification for the destination table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered."]
        #[serde(rename = "clustering", default)]
        pub clustering: Option<crate::schemas::Clustering>,
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(rename = "createDisposition", default)]
        pub create_disposition: Option<String>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        #[serde(rename = "destinationEncryptionConfiguration", default)]
        pub destination_encryption_configuration: Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Required] The destination table to load the data into."]
        #[serde(rename = "destinationTable", default)]
        pub destination_table: Option<crate::schemas::TableReference>,
        #[doc = "[Beta] [Optional] Properties with which to create the destination table if it is new."]
        #[serde(rename = "destinationTableProperties", default)]
        pub destination_table_properties: Option<crate::schemas::DestinationTableProperties>,
        #[doc = "[Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties."]
        #[serde(rename = "encoding", default)]
        pub encoding: Option<String>,
        #[doc = "[Optional] The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character. To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',')."]
        #[serde(rename = "fieldDelimiter", default)]
        pub field_delimiter: Option<String>,
        #[doc = "[Optional, Trusted Tester] If hive partitioning is enabled, which mode to use. Two modes are supported: - AUTO: automatically infer partition key name(s) and type(s). - STRINGS: automatic infer partition key name(s). All types are strings. Not all storage formats support hive partitioning -- requesting hive partitioning on an unsupported format will lead to an error."]
        #[serde(rename = "hivePartitioningMode", default)]
        pub hive_partitioning_mode: Option<String>,
        #[doc = "[Optional, Trusted Tester] Options to configure hive partitioning support."]
        #[serde(rename = "hivePartitioningOptions", default)]
        pub hive_partitioning_options: Option<crate::schemas::HivePartitioningOptions>,
        #[doc = "[Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names"]
        #[serde(rename = "ignoreUnknownValues", default)]
        pub ignore_unknown_values: Option<bool>,
        #[doc = "[Optional] The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV and JSON. The default value is 0, which requires that all records are valid."]
        #[serde(rename = "maxBadRecords", default)]
        pub max_bad_records: Option<i32>,
        #[doc = "[Optional] Specifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value when loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as an empty value."]
        #[serde(rename = "nullMarker", default)]
        pub null_marker: Option<String>,
        #[doc = "If sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup. Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties. If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result."]
        #[serde(rename = "projectionFields", default)]
        pub projection_fields: Option<Vec<String>>,
        #[doc = "[Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
        #[serde(rename = "quote", default)]
        pub quote: Option<String>,
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(rename = "rangePartitioning", default)]
        pub range_partitioning: Option<crate::schemas::RangePartitioning>,
        #[doc = "[Optional] The schema for the destination table. The schema can be omitted if the destination table already exists, or if you're loading data from Google Cloud Datastore."]
        #[serde(rename = "schema", default)]
        pub schema: Option<crate::schemas::TableSchema>,
        #[doc = "[Deprecated] The inline schema. For CSV schemas, specify as \"Field1:Type1[,Field2:Type2]*\". For example, \"foo:STRING, bar:INTEGER, baz:FLOAT\"."]
        #[serde(rename = "schemaInline", default)]
        pub schema_inline: Option<String>,
        #[doc = "[Deprecated] The format of the schemaInline property."]
        #[serde(rename = "schemaInlineFormat", default)]
        pub schema_inline_format: Option<String>,
        #[doc = "Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
        #[serde(rename = "schemaUpdateOptions", default)]
        pub schema_update_options: Option<Vec<String>>,
        #[doc = "[Optional] The number of rows at the top of a CSV file that BigQuery will skip when loading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped."]
        #[serde(rename = "skipLeadingRows", default)]
        pub skip_leading_rows: Option<i32>,
        #[doc = "[Optional] The format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\". For orc, specify \"ORC\". The default value is CSV."]
        #[serde(rename = "sourceFormat", default)]
        pub source_format: Option<String>,
        #[doc = "[Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '*' wildcard character is not allowed."]
        #[serde(rename = "sourceUris", default)]
        pub source_uris: Option<Vec<String>>,
        #[doc = "Time-based partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(rename = "timePartitioning", default)]
        pub time_partitioning: Option<crate::schemas::TimePartitioning>,
        #[doc = "[Optional] If sourceFormat is set to \"AVRO\", indicates whether to enable interpreting logical types into their corresponding types (ie. TIMESTAMP), instead of only using their raw types (ie. INTEGER)."]
        #[serde(rename = "useAvroLogicalTypes", default)]
        pub use_avro_logical_types: Option<bool>,
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(rename = "writeDisposition", default)]
        pub write_disposition: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobConfigurationLoad {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobConfigurationQuery {
        #[doc = "[Optional] If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance. Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed. However, you must still set destinationTable when result size exceeds the allowed maximum response size."]
        #[serde(rename = "allowLargeResults", default)]
        pub allow_large_results: Option<bool>,
        #[doc = "[Beta] Clustering specification for the destination table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered."]
        #[serde(rename = "clustering", default)]
        pub clustering: Option<crate::schemas::Clustering>,
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(rename = "createDisposition", default)]
        pub create_disposition: Option<String>,
        #[doc = "[Optional] Specifies the default dataset to use for unqualified table names in the query. Note that this does not alter behavior of unqualified dataset names."]
        #[serde(rename = "defaultDataset", default)]
        pub default_dataset: Option<crate::schemas::DatasetReference>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        #[serde(rename = "destinationEncryptionConfiguration", default)]
        pub destination_encryption_configuration: Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Optional] Describes the table where the query results should be stored. If not present, a new table will be created to store the results. This property must be set for large results that exceed the maximum response size."]
        #[serde(rename = "destinationTable", default)]
        pub destination_table: Option<crate::schemas::TableReference>,
        #[doc = "[Optional] If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results. allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened."]
        #[serde(rename = "flattenResults", default)]
        pub flatten_results: Option<bool>,
        #[doc = "[Optional] Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge). If unspecified, this will be set to your project default."]
        #[serde(rename = "maximumBillingTier", default)]
        pub maximum_billing_tier: Option<i32>,
        #[doc = "[Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default."]
        #[serde(rename = "maximumBytesBilled", default)]
        #[serde(with = "crate::parsed_string")]
        pub maximum_bytes_billed: Option<i64>,
        #[doc = "Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
        #[serde(rename = "parameterMode", default)]
        pub parameter_mode: Option<String>,
        #[doc = "[Deprecated] This property is deprecated."]
        #[serde(rename = "preserveNulls", default)]
        pub preserve_nulls: Option<bool>,
        #[doc = "[Optional] Specifies a priority for the query. Possible values include INTERACTIVE and BATCH. The default value is INTERACTIVE."]
        #[serde(rename = "priority", default)]
        pub priority: Option<String>,
        #[doc = "[Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL."]
        #[serde(rename = "query", default)]
        pub query: Option<String>,
        #[doc = "Query parameters for standard SQL queries."]
        #[serde(rename = "queryParameters", default)]
        pub query_parameters: Option<Vec<crate::schemas::QueryParameter>>,
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(rename = "rangePartitioning", default)]
        pub range_partitioning: Option<crate::schemas::RangePartitioning>,
        #[doc = "Allows the schema of the destination table to be updated as a side effect of the query job. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
        #[serde(rename = "schemaUpdateOptions", default)]
        pub schema_update_options: Option<Vec<String>>,
        #[doc = "[Optional] If querying an external data source outside of BigQuery, describes the data format, location and other properties of the data source. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
        #[serde(rename = "tableDefinitions", default)]
        pub table_definitions:
            Option<::std::collections::BTreeMap<String, crate::schemas::ExternalDataConfiguration>>,
        #[doc = "Time-based partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(rename = "timePartitioning", default)]
        pub time_partitioning: Option<crate::schemas::TimePartitioning>,
        #[doc = "Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false."]
        #[serde(rename = "useLegacySql", default)]
        pub use_legacy_sql: Option<bool>,
        #[doc = "[Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified. The default value is true."]
        #[serde(rename = "useQueryCache", default)]
        pub use_query_cache: Option<bool>,
        #[doc = "Describes user-defined function resources used in the query."]
        #[serde(rename = "userDefinedFunctionResources", default)]
        pub user_defined_function_resources:
            Option<Vec<crate::schemas::UserDefinedFunctionResource>>,
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(rename = "writeDisposition", default)]
        pub write_disposition: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobConfigurationQuery {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobConfigurationTableCopy {
        #[doc = "[Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(rename = "createDisposition", default)]
        pub create_disposition: Option<String>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        #[serde(rename = "destinationEncryptionConfiguration", default)]
        pub destination_encryption_configuration: Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Required] The destination table"]
        #[serde(rename = "destinationTable", default)]
        pub destination_table: Option<crate::schemas::TableReference>,
        #[doc = "[Pick one] Source table to copy."]
        #[serde(rename = "sourceTable", default)]
        pub source_table: Option<crate::schemas::TableReference>,
        #[doc = "[Pick one] Source tables to copy."]
        #[serde(rename = "sourceTables", default)]
        pub source_tables: Option<Vec<crate::schemas::TableReference>>,
        #[doc = "[Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion."]
        #[serde(rename = "writeDisposition", default)]
        pub write_disposition: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobConfigurationTableCopy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobListJobsItems {
        #[doc = "[Full-projection-only] Specifies the job configuration."]
        #[serde(rename = "configuration", default)]
        pub configuration: Option<crate::schemas::JobConfiguration>,
        #[doc = "A result object that will be present only if the job has failed."]
        #[serde(rename = "errorResult", default)]
        pub error_result: Option<crate::schemas::ErrorProto>,
        #[doc = "Unique opaque ID of the job."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Job reference uniquely identifying the job."]
        #[serde(rename = "jobReference", default)]
        pub job_reference: Option<crate::schemas::JobReference>,
        #[doc = "The resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Running state of the job. When the state is DONE, errorResult can be checked to determine whether the job succeeded or failed."]
        #[serde(rename = "state", default)]
        pub state: Option<String>,
        #[doc = "[Output-only] Information about the job, including starting time and ending time of the job."]
        #[serde(rename = "statistics", default)]
        pub statistics: Option<crate::schemas::JobStatistics>,
        #[doc = "[Full-projection-only] Describes the state of the job."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::JobStatus>,
        #[doc = "[Full-projection-only] Email address of the user who ran the job."]
        #[serde(rename = "user_email", default)]
        pub user_email: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobListJobsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobList {
        #[doc = "A hash of this page of results."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "List of jobs that were requested."]
        #[serde(rename = "jobs", default)]
        pub jobs: Option<Vec<crate::schemas::JobListJobsItems>>,
        #[doc = "The resource type of the response."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "A token to request the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobReference {
        #[doc = "[Required] The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
        #[serde(rename = "jobId", default)]
        pub job_id: Option<String>,
        #[doc = "The geographic location of the job. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "[Required] The ID of the project containing this job."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobReference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobStatisticsReservationUsageItems {
        #[doc = "[Output-only] Reservation name or \"unreserved\" for on-demand resources usage."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "[Output-only] Slot-milliseconds the job spent in the given reservation."]
        #[serde(rename = "slotMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub slot_ms: Option<i64>,
    }
    impl ::field_selector::FieldSelector for JobStatisticsReservationUsageItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobStatistics {
        #[doc = "[TrustedTester] [Output-only] Job progress (0.0 -> 1.0) for LOAD and EXTRACT jobs."]
        #[serde(rename = "completionRatio", default)]
        pub completion_ratio: Option<f64>,
        #[doc = "[Output-only] Creation time of this job, in milliseconds since the epoch. This field will be present on all jobs."]
        #[serde(rename = "creationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: Option<i64>,
        #[doc = "[Output-only] End time of this job, in milliseconds since the epoch. This field will be present whenever a job is in the DONE state."]
        #[serde(rename = "endTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time: Option<i64>,
        #[doc = "[Output-only] Statistics for an extract job."]
        #[serde(rename = "extract", default)]
        pub extract: Option<crate::schemas::JobStatistics4>,
        #[doc = "[Output-only] Statistics for a load job."]
        #[serde(rename = "load", default)]
        pub load: Option<crate::schemas::JobStatistics3>,
        #[doc = "[Output-only] Number of child jobs executed."]
        #[serde(rename = "numChildJobs", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_child_jobs: Option<i64>,
        #[doc = "[Output-only] If this is a child job, the id of the parent."]
        #[serde(rename = "parentJobId", default)]
        pub parent_job_id: Option<String>,
        #[doc = "[Output-only] Statistics for a query job."]
        #[serde(rename = "query", default)]
        pub query: Option<crate::schemas::JobStatistics2>,
        #[doc = "[Output-only] Quotas which delayed this job's start time."]
        #[serde(rename = "quotaDeferments", default)]
        pub quota_deferments: Option<Vec<String>>,
        #[doc = "[Output-only] Name of the primary reservation assigned to this job. Note that this could be different than reservations reported in the reservation usage field if parent reservations were used to execute this job."]
        #[serde(rename = "reservation_id", default)]
        pub reservation_id: Option<String>,
        #[doc = "[Output-only] Job resource usage breakdown by reservation."]
        #[serde(rename = "reservationUsage", default)]
        pub reservation_usage: Option<Vec<crate::schemas::JobStatisticsReservationUsageItems>>,
        #[doc = "[Output-only] Start time of this job, in milliseconds since the epoch. This field will be present when the job transitions from the PENDING state to either RUNNING or DONE."]
        #[serde(rename = "startTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time: Option<i64>,
        #[doc = "[Output-only] [Deprecated] Use the bytes processed in the query statistics instead."]
        #[serde(rename = "totalBytesProcessed", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_processed: Option<i64>,
        #[doc = "[Output-only] Slot-milliseconds for the job."]
        #[serde(rename = "totalSlotMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_slot_ms: Option<i64>,
    }
    impl ::field_selector::FieldSelector for JobStatistics {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobStatistics2ReservationUsageItems {
        #[doc = "[Output-only] Reservation name or \"unreserved\" for on-demand resources usage."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "[Output-only] Slot-milliseconds the job spent in the given reservation."]
        #[serde(rename = "slotMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub slot_ms: Option<i64>,
    }
    impl ::field_selector::FieldSelector for JobStatistics2ReservationUsageItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobStatistics2 {
        #[doc = "[Output-only] Billing tier for the job."]
        #[serde(rename = "billingTier", default)]
        pub billing_tier: Option<i32>,
        #[doc = "[Output-only] Whether the query result was fetched from the query cache."]
        #[serde(rename = "cacheHit", default)]
        pub cache_hit: Option<bool>,
        #[doc = "The DDL operation performed, possibly dependent on the pre-existence of the DDL target. Possible values (new values might be added in the future): \"CREATE\": The query created the DDL target. \"SKIP\": No-op. Example cases: the query is CREATE TABLE IF NOT EXISTS while the table already exists, or the query is DROP TABLE IF EXISTS while the table does not exist. \"REPLACE\": The query replaced the DDL target. Example case: the query is CREATE OR REPLACE TABLE, and the table already exists. \"DROP\": The query deleted the DDL target."]
        #[serde(rename = "ddlOperationPerformed", default)]
        pub ddl_operation_performed: Option<String>,
        #[doc = "The DDL target routine. Present only for CREATE/DROP FUNCTION/PROCEDURE queries."]
        #[serde(rename = "ddlTargetRoutine", default)]
        pub ddl_target_routine: Option<crate::schemas::RoutineReference>,
        #[doc = "The DDL target table. Present only for CREATE/DROP TABLE/VIEW queries."]
        #[serde(rename = "ddlTargetTable", default)]
        pub ddl_target_table: Option<crate::schemas::TableReference>,
        #[doc = "[Output-only] The original estimate of bytes processed for the job."]
        #[serde(rename = "estimatedBytesProcessed", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_bytes_processed: Option<i64>,
        #[doc = "[Output-only, Beta] Information about create model query job progress."]
        #[serde(rename = "modelTraining", default)]
        pub model_training: Option<crate::schemas::BigQueryModelTraining>,
        #[doc = "[Output-only, Beta] Deprecated; do not use."]
        #[serde(rename = "modelTrainingCurrentIteration", default)]
        pub model_training_current_iteration: Option<i32>,
        #[doc = "[Output-only, Beta] Deprecated; do not use."]
        #[serde(rename = "modelTrainingExpectedTotalIteration", default)]
        #[serde(with = "crate::parsed_string")]
        pub model_training_expected_total_iteration: Option<i64>,
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        #[serde(rename = "numDmlAffectedRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_dml_affected_rows: Option<i64>,
        #[doc = "[Output-only] Describes execution plan for the query."]
        #[serde(rename = "queryPlan", default)]
        pub query_plan: Option<Vec<crate::schemas::ExplainQueryStage>>,
        #[doc = "[Output-only] Referenced routines (persistent user-defined functions and stored procedures) for the job."]
        #[serde(rename = "referencedRoutines", default)]
        pub referenced_routines: Option<Vec<crate::schemas::RoutineReference>>,
        #[doc = "[Output-only] Referenced tables for the job. Queries that reference more than 50 tables will not have a complete list."]
        #[serde(rename = "referencedTables", default)]
        pub referenced_tables: Option<Vec<crate::schemas::TableReference>>,
        #[doc = "[Output-only] Job resource usage breakdown by reservation."]
        #[serde(rename = "reservationUsage", default)]
        pub reservation_usage: Option<Vec<crate::schemas::JobStatistics2ReservationUsageItems>>,
        #[doc = "[Output-only] The schema of the results. Present only for successful dry run of non-legacy SQL queries."]
        #[serde(rename = "schema", default)]
        pub schema: Option<crate::schemas::TableSchema>,
        #[doc = "The type of query statement, if valid. Possible values (new values might be added in the future): \"SELECT\": SELECT query. \"INSERT\": INSERT query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"UPDATE\": UPDATE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"DELETE\": DELETE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"MERGE\": MERGE query; see https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language. \"ALTER_TABLE\": ALTER TABLE query. \"ALTER_VIEW\": ALTER VIEW query. \"CREATE_FUNCTION\": CREATE FUNCTION query. \"CREATE_MODEL\": CREATE [OR REPLACE] MODEL ... AS SELECT ... . \"CREATE_PROCEDURE\": CREATE PROCEDURE query. \"CREATE_TABLE\": CREATE [OR REPLACE] TABLE without AS SELECT. \"CREATE_TABLE_AS_SELECT\": CREATE [OR REPLACE] TABLE ... AS SELECT ... . \"CREATE_VIEW\": CREATE [OR REPLACE] VIEW ... AS SELECT ... . \"DROP_FUNCTION\" : DROP FUNCTION query. \"DROP_PROCEDURE\": DROP PROCEDURE query. \"DROP_TABLE\": DROP TABLE query. \"DROP_VIEW\": DROP VIEW query."]
        #[serde(rename = "statementType", default)]
        pub statement_type: Option<String>,
        #[doc = "[Output-only] [Beta] Describes a timeline of job execution."]
        #[serde(rename = "timeline", default)]
        pub timeline: Option<Vec<crate::schemas::QueryTimelineSample>>,
        #[doc = "[Output-only] Total bytes billed for the job."]
        #[serde(rename = "totalBytesBilled", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_billed: Option<i64>,
        #[doc = "[Output-only] Total bytes processed for the job."]
        #[serde(rename = "totalBytesProcessed", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_processed: Option<i64>,
        #[doc = "[Output-only] For dry-run jobs, totalBytesProcessed is an estimate and this field specifies the accuracy of the estimate. Possible values can be: UNKNOWN: accuracy of the estimate is unknown. PRECISE: estimate is precise. LOWER_BOUND: estimate is lower bound of what the query would cost. UPPER_BOUND: estimate is upper bound of what the query would cost."]
        #[serde(rename = "totalBytesProcessedAccuracy", default)]
        pub total_bytes_processed_accuracy: Option<String>,
        #[doc = "[Output-only] Total number of partitions processed from all partitioned tables referenced in the job."]
        #[serde(rename = "totalPartitionsProcessed", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_partitions_processed: Option<i64>,
        #[doc = "[Output-only] Slot-milliseconds for the job."]
        #[serde(rename = "totalSlotMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_slot_ms: Option<i64>,
        #[doc = "Standard SQL only: list of undeclared query parameters detected during a dry run validation."]
        #[serde(rename = "undeclaredQueryParameters", default)]
        pub undeclared_query_parameters: Option<Vec<crate::schemas::QueryParameter>>,
    }
    impl ::field_selector::FieldSelector for JobStatistics2 {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobStatistics3 {
        #[doc = "[Output-only] The number of bad records encountered. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data."]
        #[serde(rename = "badRecords", default)]
        #[serde(with = "crate::parsed_string")]
        pub bad_records: Option<i64>,
        #[doc = "[Output-only] Number of bytes of source data in a load job."]
        #[serde(rename = "inputFileBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub input_file_bytes: Option<i64>,
        #[doc = "[Output-only] Number of source files in a load job."]
        #[serde(rename = "inputFiles", default)]
        #[serde(with = "crate::parsed_string")]
        pub input_files: Option<i64>,
        #[doc = "[Output-only] Size of the loaded data in bytes. Note that while a load job is in the running state, this value may change."]
        #[serde(rename = "outputBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub output_bytes: Option<i64>,
        #[doc = "[Output-only] Number of rows imported in a load job. Note that while an import job is in the running state, this value may change."]
        #[serde(rename = "outputRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub output_rows: Option<i64>,
    }
    impl ::field_selector::FieldSelector for JobStatistics3 {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobStatistics4 {
        #[doc = "[Output-only] Number of files per destination URI or URI pattern specified in the extract configuration. These values will be in the same order as the URIs specified in the 'destinationUris' field."]
        #[serde(rename = "destinationUriFileCounts", default)]
        pub destination_uri_file_counts: Option<Vec<i64>>,
        #[doc = "[Output-only] Number of user bytes extracted into the result. This is the byte count as computed by BigQuery for billing purposes."]
        #[serde(rename = "inputBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub input_bytes: Option<i64>,
    }
    impl ::field_selector::FieldSelector for JobStatistics4 {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct JobStatus {
        #[doc = "[Output-only] Final error result of the job. If present, indicates that the job has completed and was unsuccessful."]
        #[serde(rename = "errorResult", default)]
        pub error_result: Option<crate::schemas::ErrorProto>,
        #[doc = "[Output-only] The first errors encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        #[serde(rename = "errors", default)]
        pub errors: Option<Vec<crate::schemas::ErrorProto>>,
        #[doc = "[Output-only] Running state of the job."]
        #[serde(rename = "state", default)]
        pub state: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobStatus {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListModelsResponse {
        #[doc = "Models in the requested dataset. Only the following fields are populated:\nmodel_reference, model_type, creation_time, last_modified_time and\nlabels."]
        #[serde(rename = "models", default)]
        pub models: Option<Vec<crate::schemas::Model>>,
        #[doc = "A token to request the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListModelsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ListRoutinesResponse {
        #[doc = "A token to request the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "Routines in the requested dataset. Only the following fields are populated:\netag, project_id, dataset_id, routine_id, routine_type, creation_time,\nlast_modified_time, language."]
        #[serde(rename = "routines", default)]
        pub routines: Option<Vec<crate::schemas::Routine>>,
    }
    impl ::field_selector::FieldSelector for ListRoutinesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct LocationMetadata {
        #[doc = "The legacy BigQuery location ID, e.g. \u{201c}EU\u{201d} for the \u{201c}europe\u{201d} location.\nThis is for any API consumers that need the legacy \u{201c}US\u{201d} and \u{201c}EU\u{201d} locations."]
        #[serde(rename = "legacyLocationId", default)]
        pub legacy_location_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for LocationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct MaterializedViewDefinition {
        #[doc = "[Output-only] [TrustedTester] The time when this materialized view was last modified, in milliseconds since the epoch."]
        #[serde(rename = "lastRefreshTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_refresh_time: Option<i64>,
        #[doc = "[Required] A query whose result is persisted."]
        #[serde(rename = "query", default)]
        pub query: Option<String>,
    }
    impl ::field_selector::FieldSelector for MaterializedViewDefinition {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ModelModelType {
        ModelTypeUnspecified,
        #[doc = "Linear regression model."]
        LinearRegression,
        #[doc = "Logistic regression based classification model."]
        LogisticRegression,
        #[doc = "K-means clustering model."]
        Kmeans,
        #[doc = "[Beta] An imported TensorFlow model."]
        Tensorflow,
    }
    impl ModelModelType {
        pub fn as_str(self) -> &'static str {
            match self {
                ModelModelType::ModelTypeUnspecified => "MODEL_TYPE_UNSPECIFIED",
                ModelModelType::LinearRegression => "LINEAR_REGRESSION",
                ModelModelType::LogisticRegression => "LOGISTIC_REGRESSION",
                ModelModelType::Kmeans => "KMEANS",
                ModelModelType::Tensorflow => "TENSORFLOW",
            }
        }
    }
    impl ::std::fmt::Display for ModelModelType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ModelModelType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ModelModelType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MODEL_TYPE_UNSPECIFIED" => ModelModelType::ModelTypeUnspecified,
                "LINEAR_REGRESSION" => ModelModelType::LinearRegression,
                "LOGISTIC_REGRESSION" => ModelModelType::LogisticRegression,
                "KMEANS" => ModelModelType::Kmeans,
                "TENSORFLOW" => ModelModelType::Tensorflow,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Model {
        #[doc = "Output only. The time when this model was created, in millisecs since the\nepoch."]
        #[serde(rename = "creationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: Option<i64>,
        #[doc = "[Optional] A user-friendly description of this model."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Output only. A hash of this resource."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "[Optional] The time when this model expires, in milliseconds since the\nepoch. If not present, the model will persist indefinitely. Expired models\nwill be deleted and their storage reclaimed.  The defaultTableExpirationMs\nproperty of the encapsulating dataset can be used to set a default\nexpirationTime on newly created models."]
        #[serde(rename = "expirationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub expiration_time: Option<i64>,
        #[doc = "Output only. Input feature columns that were used to train this model."]
        #[serde(rename = "featureColumns", default)]
        pub feature_columns: Option<Vec<crate::schemas::StandardSqlField>>,
        #[doc = "[Optional] A descriptive name for this model."]
        #[serde(rename = "friendlyName", default)]
        pub friendly_name: Option<String>,
        #[doc = "Output only. Label columns that were used to train this model.\nThe output of the model will have a \"predicted_\" prefix to these columns."]
        #[serde(rename = "labelColumns", default)]
        pub label_columns: Option<Vec<crate::schemas::StandardSqlField>>,
        #[doc = "[Optional] The labels associated with this model. You can use these to\norganize and group your models. Label keys and values can be no longer\nthan 63 characters, can only contain lowercase letters, numeric\ncharacters, underscores and dashes. International characters are allowed.\nLabel values are optional. Label keys must start with a letter and each\nlabel in the list must have a different key."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. The time when this model was last modified, in millisecs\nsince the epoch."]
        #[serde(rename = "lastModifiedTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_time: Option<i64>,
        #[doc = "Output only. The geographic location where the model resides. This value\nis inherited from the dataset."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "Required. Unique identifier for this model."]
        #[serde(rename = "modelReference", default)]
        pub model_reference: Option<crate::schemas::ModelReference>,
        #[doc = "Output only. Type of the model resource."]
        #[serde(rename = "modelType", default)]
        pub model_type: Option<crate::schemas::ModelModelType>,
        #[doc = "Output only. Information for all training runs in increasing order of\nstart_time."]
        #[serde(rename = "trainingRuns", default)]
        pub training_runs: Option<Vec<crate::schemas::TrainingRun>>,
    }
    impl ::field_selector::FieldSelector for Model {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ModelDefinitionModelOptions {
        #[serde(rename = "labels", default)]
        pub labels: Option<Vec<String>>,
        #[serde(rename = "lossType", default)]
        pub loss_type: Option<String>,
        #[serde(rename = "modelType", default)]
        pub model_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for ModelDefinitionModelOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ModelDefinition {
        #[doc = "[Output-only, Beta] Model options used for the first training run. These options are immutable for subsequent training runs. Default values are used for any options not specified in the input query."]
        #[serde(rename = "modelOptions", default)]
        pub model_options: Option<crate::schemas::ModelDefinitionModelOptions>,
        #[doc = "[Output-only, Beta] Information about ml training runs, each training run comprises of multiple iterations and there may be multiple training runs for the model if warm start is used or if a user decides to continue a previously cancelled query."]
        #[serde(rename = "trainingRuns", default)]
        pub training_runs: Option<Vec<crate::schemas::BqmlTrainingRun>>,
    }
    impl ::field_selector::FieldSelector for ModelDefinition {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ModelReference {
        #[doc = "[Required] The ID of the dataset containing this model."]
        #[serde(rename = "datasetId", default)]
        pub dataset_id: Option<String>,
        #[doc = "[Required] The ID of the model. The ID must contain only\nletters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum\nlength is 1,024 characters."]
        #[serde(rename = "modelId", default)]
        pub model_id: Option<String>,
        #[doc = "[Required] The ID of the project containing this model."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ModelReference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MultiClassClassificationMetrics {
        #[doc = "Aggregate classification metrics."]
        #[serde(rename = "aggregateClassificationMetrics", default)]
        pub aggregate_classification_metrics:
            Option<crate::schemas::AggregateClassificationMetrics>,
        #[doc = "Confusion matrix at different thresholds."]
        #[serde(rename = "confusionMatrixList", default)]
        pub confusion_matrix_list: Option<Vec<crate::schemas::ConfusionMatrix>>,
    }
    impl ::field_selector::FieldSelector for MultiClassClassificationMetrics {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ProjectListProjectsItems {
        #[doc = "A descriptive name for this project."]
        #[serde(rename = "friendlyName", default)]
        pub friendly_name: Option<String>,
        #[doc = "An opaque ID of this project."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The numeric ID of this project."]
        #[serde(rename = "numericId", default)]
        #[serde(with = "crate::parsed_string")]
        pub numeric_id: Option<u64>,
        #[doc = "A unique reference to this project."]
        #[serde(rename = "projectReference", default)]
        pub project_reference: Option<crate::schemas::ProjectReference>,
    }
    impl ::field_selector::FieldSelector for ProjectListProjectsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ProjectList {
        #[doc = "A hash of the page of results"]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The type of list."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "A token to request the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "Projects to which you have at least READ access."]
        #[serde(rename = "projects", default)]
        pub projects: Option<Vec<crate::schemas::ProjectListProjectsItems>>,
        #[doc = "The total number of projects in the list."]
        #[serde(rename = "totalItems", default)]
        pub total_items: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ProjectList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ProjectReference {
        #[doc = "[Required] ID of the project. Can be either the numeric ID or the assigned ID of the project."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ProjectReference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct QueryParameter {
        #[doc = "[Optional] If unset, this is a positional parameter. Otherwise, should be unique within a query."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "[Required] The type of this parameter."]
        #[serde(rename = "parameterType", default)]
        pub parameter_type: Option<crate::schemas::QueryParameterType>,
        #[doc = "[Required] The value of this parameter."]
        #[serde(rename = "parameterValue", default)]
        pub parameter_value: Option<crate::schemas::QueryParameterValue>,
    }
    impl ::field_selector::FieldSelector for QueryParameter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct QueryParameterTypeStructTypesItems {
        #[doc = "[Optional] Human-oriented description of the field."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "[Optional] The name of this field."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "[Required] The type of this field."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::QueryParameterType>,
    }
    impl ::field_selector::FieldSelector for QueryParameterTypeStructTypesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct QueryParameterType {
        #[doc = "[Optional] The type of the array's elements, if this is an array."]
        #[serde(rename = "arrayType", default)]
        pub array_type: Option<crate::schemas::QueryParameterType>,
        #[doc = "[Required] The top level type of this field."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "[Optional] The types of the fields of this struct, in order, if this is a struct."]
        #[serde(rename = "structTypes", default)]
        pub struct_types: Option<Vec<crate::schemas::QueryParameterTypeStructTypesItems>>,
    }
    impl ::field_selector::FieldSelector for QueryParameterType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct QueryParameterValue {
        #[doc = "[Optional] The array values, if this is an array type."]
        #[serde(rename = "arrayValues", default)]
        pub array_values: Option<Vec<crate::schemas::QueryParameterValue>>,
        #[doc = "[Optional] The struct field values, in order of the struct type's declaration."]
        #[serde(rename = "structValues", default)]
        pub struct_values:
            Option<::std::collections::BTreeMap<String, crate::schemas::QueryParameterValue>>,
        #[doc = "[Optional] The value of this value, if a simple scalar type."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for QueryParameterValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct QueryRequest {
        #[doc = "[Optional] Specifies the default datasetId and projectId to assume for any unqualified table names in the query. If not set, all table names in the query string must be qualified in the format 'datasetId.tableId'."]
        #[serde(rename = "defaultDataset", default)]
        pub default_dataset: Option<crate::schemas::DatasetReference>,
        #[doc = "[Optional] If set to true, BigQuery doesn't run the job. Instead, if the query is valid, BigQuery returns statistics about the job such as how many bytes would be processed. If the query is invalid, an error returns. The default value is false."]
        #[serde(rename = "dryRun", default)]
        pub dry_run: Option<bool>,
        #[doc = "The resource type of the request."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The geographic location where the job should run. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "[Optional] The maximum number of rows of data to return per page of results. Setting this flag to a small value such as 1000 and then paging through results might improve reliability when the query result set is large. In addition to this limit, responses are also limited to 10 MB. By default, there is no maximum row count, and only the byte limit applies."]
        #[serde(rename = "maxResults", default)]
        pub max_results: Option<u32>,
        #[doc = "Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
        #[serde(rename = "parameterMode", default)]
        pub parameter_mode: Option<String>,
        #[doc = "[Deprecated] This property is deprecated."]
        #[serde(rename = "preserveNulls", default)]
        pub preserve_nulls: Option<bool>,
        #[doc = "[Required] A query string, following the BigQuery query syntax, of the query to execute. Example: \"SELECT count(f1) FROM [myProjectId:myDatasetId.myTableId]\"."]
        #[serde(rename = "query", default)]
        pub query: Option<String>,
        #[doc = "Query parameters for Standard SQL queries."]
        #[serde(rename = "queryParameters", default)]
        pub query_parameters: Option<Vec<crate::schemas::QueryParameter>>,
        #[doc = "[Optional] How long to wait for the query to complete, in milliseconds, before the request times out and returns. Note that this is only a timeout for the request, not the query. If the query takes longer to run than the timeout value, the call returns without any results and with the 'jobComplete' flag set to false. You can call GetQueryResults() to wait for the query to complete and read the results. The default value is 10000 milliseconds (10 seconds)."]
        #[serde(rename = "timeoutMs", default)]
        pub timeout_ms: Option<u32>,
        #[doc = "Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false."]
        #[serde(rename = "useLegacySql", default)]
        pub use_legacy_sql: Option<bool>,
        #[doc = "[Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. The default value is true."]
        #[serde(rename = "useQueryCache", default)]
        pub use_query_cache: Option<bool>,
    }
    impl ::field_selector::FieldSelector for QueryRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct QueryResponse {
        #[doc = "Whether the query result was fetched from the query cache."]
        #[serde(rename = "cacheHit", default)]
        pub cache_hit: Option<bool>,
        #[doc = "[Output-only] The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful."]
        #[serde(rename = "errors", default)]
        pub errors: Option<Vec<crate::schemas::ErrorProto>>,
        #[doc = "Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available."]
        #[serde(rename = "jobComplete", default)]
        pub job_complete: Option<bool>,
        #[doc = "Reference to the Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults)."]
        #[serde(rename = "jobReference", default)]
        pub job_reference: Option<crate::schemas::JobReference>,
        #[doc = "The resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "[Output-only] The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE."]
        #[serde(rename = "numDmlAffectedRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_dml_affected_rows: Option<i64>,
        #[doc = "A token used for paging results."]
        #[serde(rename = "pageToken", default)]
        pub page_token: Option<String>,
        #[doc = "An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above."]
        #[serde(rename = "rows", default)]
        pub rows: Option<Vec<crate::schemas::TableRow>>,
        #[doc = "The schema of the results. Present only when the query completes successfully."]
        #[serde(rename = "schema", default)]
        pub schema: Option<crate::schemas::TableSchema>,
        #[doc = "The total number of bytes processed for this query. If this query was a dry run, this is the number of bytes that would be processed if the query were run."]
        #[serde(rename = "totalBytesProcessed", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_bytes_processed: Option<i64>,
        #[doc = "The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results."]
        #[serde(rename = "totalRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_rows: Option<u64>,
    }
    impl ::field_selector::FieldSelector for QueryResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct QueryTimelineSample {
        #[doc = "Total number of units currently being processed by workers. This does not correspond directly to slot usage. This is the largest value observed since the last sample."]
        #[serde(rename = "activeUnits", default)]
        #[serde(with = "crate::parsed_string")]
        pub active_units: Option<i64>,
        #[doc = "Total parallel units of work completed by this query."]
        #[serde(rename = "completedUnits", default)]
        #[serde(with = "crate::parsed_string")]
        pub completed_units: Option<i64>,
        #[doc = "Milliseconds elapsed since the start of query execution."]
        #[serde(rename = "elapsedMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub elapsed_ms: Option<i64>,
        #[doc = "Total parallel units of work remaining for the active stages."]
        #[serde(rename = "pendingUnits", default)]
        #[serde(with = "crate::parsed_string")]
        pub pending_units: Option<i64>,
        #[doc = "Cumulative slot-ms consumed by the query."]
        #[serde(rename = "totalSlotMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_slot_ms: Option<i64>,
    }
    impl ::field_selector::FieldSelector for QueryTimelineSample {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct RangePartitioningRange {
        #[doc = "[TrustedTester] [Required] The end of range partitioning, exclusive."]
        #[serde(rename = "end", default)]
        #[serde(with = "crate::parsed_string")]
        pub end: Option<i64>,
        #[doc = "[TrustedTester] [Required] The width of each interval."]
        #[serde(rename = "interval", default)]
        #[serde(with = "crate::parsed_string")]
        pub interval: Option<i64>,
        #[doc = "[TrustedTester] [Required] The start of range partitioning, inclusive."]
        #[serde(rename = "start", default)]
        #[serde(with = "crate::parsed_string")]
        pub start: Option<i64>,
    }
    impl ::field_selector::FieldSelector for RangePartitioningRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct RangePartitioning {
        #[doc = "[TrustedTester] [Required] The table is partitioned by this field. The field must be a top-level NULLABLE/REQUIRED field. The only supported type is INTEGER/INT64."]
        #[serde(rename = "field", default)]
        pub field: Option<String>,
        #[doc = "[TrustedTester] [Required] Defines the ranges for range partitioning."]
        #[serde(rename = "range", default)]
        pub range: Option<crate::schemas::RangePartitioningRange>,
    }
    impl ::field_selector::FieldSelector for RangePartitioning {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RegressionMetrics {
        #[doc = "Mean absolute error."]
        #[serde(rename = "meanAbsoluteError", default)]
        pub mean_absolute_error: Option<f64>,
        #[doc = "Mean squared error."]
        #[serde(rename = "meanSquaredError", default)]
        pub mean_squared_error: Option<f64>,
        #[doc = "Mean squared log error."]
        #[serde(rename = "meanSquaredLogError", default)]
        pub mean_squared_log_error: Option<f64>,
        #[doc = "Median absolute error."]
        #[serde(rename = "medianAbsoluteError", default)]
        pub median_absolute_error: Option<f64>,
        #[doc = "R^2 score."]
        #[serde(rename = "rSquared", default)]
        pub r_squared: Option<f64>,
    }
    impl ::field_selector::FieldSelector for RegressionMetrics {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RoutineLanguage {
        LanguageUnspecified,
        #[doc = "SQL language."]
        Sql,
        #[doc = "JavaScript language."]
        Javascript,
    }
    impl RoutineLanguage {
        pub fn as_str(self) -> &'static str {
            match self {
                RoutineLanguage::LanguageUnspecified => "LANGUAGE_UNSPECIFIED",
                RoutineLanguage::Sql => "SQL",
                RoutineLanguage::Javascript => "JAVASCRIPT",
            }
        }
    }
    impl ::std::fmt::Display for RoutineLanguage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RoutineLanguage {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RoutineLanguage {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LANGUAGE_UNSPECIFIED" => RoutineLanguage::LanguageUnspecified,
                "SQL" => RoutineLanguage::Sql,
                "JAVASCRIPT" => RoutineLanguage::Javascript,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RoutineRoutineType {
        RoutineTypeUnspecified,
        #[doc = "Non-builtin permanent scalar function."]
        ScalarFunction,
        #[doc = "Stored procedure."]
        Procedure,
    }
    impl RoutineRoutineType {
        pub fn as_str(self) -> &'static str {
            match self {
                RoutineRoutineType::RoutineTypeUnspecified => "ROUTINE_TYPE_UNSPECIFIED",
                RoutineRoutineType::ScalarFunction => "SCALAR_FUNCTION",
                RoutineRoutineType::Procedure => "PROCEDURE",
            }
        }
    }
    impl ::std::fmt::Display for RoutineRoutineType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RoutineRoutineType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RoutineRoutineType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ROUTINE_TYPE_UNSPECIFIED" => RoutineRoutineType::RoutineTypeUnspecified,
                "SCALAR_FUNCTION" => RoutineRoutineType::ScalarFunction,
                "PROCEDURE" => RoutineRoutineType::Procedure,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
    pub struct Routine {
        #[doc = "Optional."]
        #[serde(rename = "arguments", default)]
        pub arguments: Option<Vec<crate::schemas::Argument>>,
        #[doc = "Output only. The time when this routine was created, in milliseconds since\nthe epoch."]
        #[serde(rename = "creationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: Option<i64>,
        #[doc = "Required. The body of the routine.\n\nFor functions, this is the expression in the AS clause.\n\nIf language=SQL, it is the substring inside (but excluding) the\nparentheses. For example, for the function created with the following\nstatement:\n\n`CREATE FUNCTION JoinLines(x string, y string) as (concat(x, \"\\n\", y))`\n\nThe definition_body is `concat(x, \"\\n\", y)` (\\n is not replaced with\nlinebreak).\n\nIf language=JAVASCRIPT, it is the evaluated string in the AS clause.\nFor example, for the function created with the following statement:\n\n`CREATE FUNCTION f() RETURNS STRING LANGUAGE js AS 'return \"\\n\";\\n'`\n\nThe definition_body is\n\n`return \"\\n\";\\n`\n\nNote that both \\n are replaced with linebreaks."]
        #[serde(rename = "definitionBody", default)]
        pub definition_body: Option<String>,
        #[doc = "Output only. A hash of this resource."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "Optional. If language = \"JAVASCRIPT\", this field stores the path of the\nimported JAVASCRIPT libraries."]
        #[serde(rename = "importedLibraries", default)]
        pub imported_libraries: Option<Vec<String>>,
        #[doc = "Optional. Defaults to \"SQL\"."]
        #[serde(rename = "language", default)]
        pub language: Option<crate::schemas::RoutineLanguage>,
        #[doc = "Output only. The time when this routine was last modified, in milliseconds\nsince the epoch."]
        #[serde(rename = "lastModifiedTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_time: Option<i64>,
        #[doc = "Optional if language = \"SQL\"; required otherwise.\n\nIf absent, the return type is inferred from definition_body at query time\nin each query that references this routine. If present, then the evaluated\nresult will be cast to the specified returned type at query time.\n\nFor example, for the functions created with the following statements:\n\n* `CREATE FUNCTION Add(x FLOAT64, y FLOAT64) RETURNS FLOAT64 AS (x + y);`\n\n* `CREATE FUNCTION Increment(x FLOAT64) AS (Add(x, 1));`\n\n* `CREATE FUNCTION Decrement(x FLOAT64) RETURNS FLOAT64 AS (Add(x, -1));`\n\nThe return_type is `{type_kind: \"FLOAT64\"}` for `Add` and `Decrement`, and\nis absent for `Increment` (inferred as FLOAT64 at query time).\n\nSuppose the function `Add` is replaced by\n  `CREATE OR REPLACE FUNCTION Add(x INT64, y INT64) AS (x + y);`\n\nThen the inferred return type of `Increment` is automatically changed to\nINT64 at query time, while the return type of `Decrement` remains FLOAT64."]
        #[serde(rename = "returnType", default)]
        pub return_type: Option<crate::schemas::StandardSqlDataType>,
        #[doc = "Required. Reference describing the ID of this routine."]
        #[serde(rename = "routineReference", default)]
        pub routine_reference: Option<crate::schemas::RoutineReference>,
        #[doc = "Required."]
        #[serde(rename = "routineType", default)]
        pub routine_type: Option<crate::schemas::RoutineRoutineType>,
    }
    impl ::field_selector::FieldSelector for Routine {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct RoutineReference {
        #[doc = "[Required] The ID of the dataset containing this routine."]
        #[serde(rename = "datasetId", default)]
        pub dataset_id: Option<String>,
        #[doc = "[Required] The ID of the project containing this routine."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "[Required] The ID of the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
        #[serde(rename = "routineId", default)]
        pub routine_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for RoutineReference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct Row {
        #[doc = "The original label of this row."]
        #[serde(rename = "actualLabel", default)]
        pub actual_label: Option<String>,
        #[doc = "Info describing predicted label distribution."]
        #[serde(rename = "entries", default)]
        pub entries: Option<Vec<crate::schemas::Entry>>,
    }
    impl ::field_selector::FieldSelector for Row {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StandardSqlDataTypeTypeKind {
        #[doc = "Invalid type."]
        TypeKindUnspecified,
        #[doc = "Encoded as a string in decimal format."]
        Int64,
        #[doc = "Encoded as a boolean \"false\" or \"true\"."]
        Bool,
        #[doc = "Encoded as a number, or string \"NaN\", \"Infinity\" or \"-Infinity\"."]
        Float64,
        #[doc = "Encoded as a string value."]
        String,
        #[doc = "Encoded as a base64 string per RFC 4648, section 4."]
        Bytes,
        #[doc = "Encoded as an RFC 3339 timestamp with mandatory \"Z\" time zone string:\n1985-04-12T23:20:50.52Z"]
        Timestamp,
        #[doc = "Encoded as RFC 3339 full-date format string: 1985-04-12"]
        Date,
        #[doc = "Encoded as RFC 3339 partial-time format string: 23:20:50.52"]
        Time,
        #[doc = "Encoded as RFC 3339 full-date \"T\" partial-time: 1985-04-12T23:20:50.52"]
        Datetime,
        #[doc = "Encoded as WKT"]
        Geography,
        #[doc = "Encoded as a decimal string."]
        Numeric,
        #[doc = "Encoded as a list with types matching Type.array_type."]
        Array,
        #[doc = "Encoded as a list with fields of type Type.struct_type[i]. List is used\nbecause a JSON object cannot have duplicate field names."]
        Struct,
    }
    impl StandardSqlDataTypeTypeKind {
        pub fn as_str(self) -> &'static str {
            match self {
                StandardSqlDataTypeTypeKind::TypeKindUnspecified => "TYPE_KIND_UNSPECIFIED",
                StandardSqlDataTypeTypeKind::Int64 => "INT64",
                StandardSqlDataTypeTypeKind::Bool => "BOOL",
                StandardSqlDataTypeTypeKind::Float64 => "FLOAT64",
                StandardSqlDataTypeTypeKind::String => "STRING",
                StandardSqlDataTypeTypeKind::Bytes => "BYTES",
                StandardSqlDataTypeTypeKind::Timestamp => "TIMESTAMP",
                StandardSqlDataTypeTypeKind::Date => "DATE",
                StandardSqlDataTypeTypeKind::Time => "TIME",
                StandardSqlDataTypeTypeKind::Datetime => "DATETIME",
                StandardSqlDataTypeTypeKind::Geography => "GEOGRAPHY",
                StandardSqlDataTypeTypeKind::Numeric => "NUMERIC",
                StandardSqlDataTypeTypeKind::Array => "ARRAY",
                StandardSqlDataTypeTypeKind::Struct => "STRUCT",
            }
        }
    }
    impl ::std::fmt::Display for StandardSqlDataTypeTypeKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StandardSqlDataTypeTypeKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StandardSqlDataTypeTypeKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_KIND_UNSPECIFIED" => StandardSqlDataTypeTypeKind::TypeKindUnspecified,
                "INT64" => StandardSqlDataTypeTypeKind::Int64,
                "BOOL" => StandardSqlDataTypeTypeKind::Bool,
                "FLOAT64" => StandardSqlDataTypeTypeKind::Float64,
                "STRING" => StandardSqlDataTypeTypeKind::String,
                "BYTES" => StandardSqlDataTypeTypeKind::Bytes,
                "TIMESTAMP" => StandardSqlDataTypeTypeKind::Timestamp,
                "DATE" => StandardSqlDataTypeTypeKind::Date,
                "TIME" => StandardSqlDataTypeTypeKind::Time,
                "DATETIME" => StandardSqlDataTypeTypeKind::Datetime,
                "GEOGRAPHY" => StandardSqlDataTypeTypeKind::Geography,
                "NUMERIC" => StandardSqlDataTypeTypeKind::Numeric,
                "ARRAY" => StandardSqlDataTypeTypeKind::Array,
                "STRUCT" => StandardSqlDataTypeTypeKind::Struct,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
    pub struct StandardSqlDataType {
        #[doc = "The type of the array's elements, if type_kind = \"ARRAY\"."]
        #[serde(rename = "arrayElementType", default)]
        pub array_element_type: Option<crate::schemas::StandardSqlDataType>,
        #[doc = "The fields of this struct, in order, if type_kind = \"STRUCT\"."]
        #[serde(rename = "structType", default)]
        pub struct_type: Option<crate::schemas::StandardSqlStructType>,
        #[doc = "Required. The top level type of this field.\nCan be any standard SQL data type (e.g., \"INT64\", \"DATE\", \"ARRAY\")."]
        #[serde(rename = "typeKind", default)]
        pub type_kind: Option<crate::schemas::StandardSqlDataTypeTypeKind>,
    }
    impl ::field_selector::FieldSelector for StandardSqlDataType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct StandardSqlField {
        #[doc = "Optional. The name of this field. Can be absent for struct fields."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Optional. The type of this parameter. Absent if not explicitly\nspecified (e.g., CREATE FUNCTION statement can omit the return type;\nin this case the output parameter does not have this \"type\" field)."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::StandardSqlDataType>,
    }
    impl ::field_selector::FieldSelector for StandardSqlField {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct StandardSqlStructType {
        #[serde(rename = "fields", default)]
        pub fields: Option<Vec<crate::schemas::StandardSqlField>>,
    }
    impl ::field_selector::FieldSelector for StandardSqlStructType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct Streamingbuffer {
        #[doc = "[Output-only] A lower-bound estimate of the number of bytes currently in the streaming buffer."]
        #[serde(rename = "estimatedBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_bytes: Option<u64>,
        #[doc = "[Output-only] A lower-bound estimate of the number of rows currently in the streaming buffer."]
        #[serde(rename = "estimatedRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_rows: Option<u64>,
        #[doc = "[Output-only] Contains the timestamp of the oldest entry in the streaming buffer, in milliseconds since the epoch, if the streaming buffer is available."]
        #[serde(rename = "oldestEntryTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub oldest_entry_time: Option<u64>,
    }
    impl ::field_selector::FieldSelector for Streamingbuffer {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Table {
        #[doc = "[Beta] Clustering specification for the table. Must be specified with partitioning, data in the table will be first partitioned and subsequently clustered."]
        #[serde(rename = "clustering", default)]
        pub clustering: Option<crate::schemas::Clustering>,
        #[doc = "[Output-only] The time when this table was created, in milliseconds since the epoch."]
        #[serde(rename = "creationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: Option<i64>,
        #[doc = "[Optional] A user-friendly description of this table."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Custom encryption configuration (e.g., Cloud KMS keys)."]
        #[serde(rename = "encryptionConfiguration", default)]
        pub encryption_configuration: Option<crate::schemas::EncryptionConfiguration>,
        #[doc = "[Output-only] A hash of the table metadata. Used to ensure there were no concurrent modifications to the resource when attempting an update. Not guaranteed to change when the table contents or the fields numRows, numBytes, numLongTermBytes or lastModifiedTime change."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "[Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created tables."]
        #[serde(rename = "expirationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub expiration_time: Option<i64>,
        #[doc = "[Optional] Describes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
        #[serde(rename = "externalDataConfiguration", default)]
        pub external_data_configuration: Option<crate::schemas::ExternalDataConfiguration>,
        #[doc = "[Optional] A descriptive name for this table."]
        #[serde(rename = "friendlyName", default)]
        pub friendly_name: Option<String>,
        #[doc = "[Output-only] An opaque ID uniquely identifying the table."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "[Output-only] The type of the resource."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "[Output-only] The time when this table was last modified, in milliseconds since the epoch."]
        #[serde(rename = "lastModifiedTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_modified_time: Option<u64>,
        #[doc = "[Output-only] The geographic location where the table resides. This value is inherited from the dataset."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "[Optional] Materialized view definition."]
        #[serde(rename = "materializedView", default)]
        pub materialized_view: Option<crate::schemas::MaterializedViewDefinition>,
        #[doc = "[Output-only, Beta] Present iff this table represents a ML model. Describes the training information for the model, and it is required to run 'PREDICT' queries."]
        #[serde(rename = "model", default)]
        pub model: Option<crate::schemas::ModelDefinition>,
        #[doc = "[Output-only] The size of this table in bytes, excluding any data in the streaming buffer."]
        #[serde(rename = "numBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_bytes: Option<i64>,
        #[doc = "[Output-only] The number of bytes in the table that are considered \"long-term storage\"."]
        #[serde(rename = "numLongTermBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_long_term_bytes: Option<i64>,
        #[doc = "[Output-only] [TrustedTester] The physical size of this table in bytes, excluding any data in the streaming buffer. This includes compression and storage used for time travel."]
        #[serde(rename = "numPhysicalBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_physical_bytes: Option<i64>,
        #[doc = "[Output-only] The number of rows of data in this table, excluding any data in the streaming buffer."]
        #[serde(rename = "numRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_rows: Option<u64>,
        #[doc = "[Output-only] Describes the table type. The following values are supported: TABLE: A normal BigQuery table. VIEW: A virtual table defined by a SQL query. [TrustedTester] MATERIALIZED_VIEW: SQL query whose result is persisted. EXTERNAL: A table that references data stored in an external storage system, such as Google Cloud Storage. The default value is TABLE."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "[TrustedTester] Range partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(rename = "rangePartitioning", default)]
        pub range_partitioning: Option<crate::schemas::RangePartitioning>,
        #[doc = "[Beta] [Optional] If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
        #[serde(rename = "requirePartitionFilter", default)]
        pub require_partition_filter: Option<bool>,
        #[doc = "[Optional] Describes the schema of this table."]
        #[serde(rename = "schema", default)]
        pub schema: Option<crate::schemas::TableSchema>,
        #[doc = "[Output-only] A URL that can be used to access this resource again."]
        #[serde(rename = "selfLink", default)]
        pub self_link: Option<String>,
        #[doc = "[Output-only] Contains information regarding this table's streaming buffer, if one is present. This field will be absent if the table is not being streamed to or if there is no data in the streaming buffer."]
        #[serde(rename = "streamingBuffer", default)]
        pub streaming_buffer: Option<crate::schemas::Streamingbuffer>,
        #[doc = "[Required] Reference describing the ID of this table."]
        #[serde(rename = "tableReference", default)]
        pub table_reference: Option<crate::schemas::TableReference>,
        #[doc = "Time-based partitioning specification for this table. Only one of timePartitioning and rangePartitioning should be specified."]
        #[serde(rename = "timePartitioning", default)]
        pub time_partitioning: Option<crate::schemas::TimePartitioning>,
        #[doc = "[Optional] The view definition."]
        #[serde(rename = "view", default)]
        pub view: Option<crate::schemas::ViewDefinition>,
    }
    impl ::field_selector::FieldSelector for Table {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableCell {
        #[serde(rename = "v", default)]
        pub v: Option<::serde_json::Value>,
    }
    impl ::field_selector::FieldSelector for TableCell {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableDataInsertAllRequestRowsItems {
        #[doc = "[Optional] A unique ID for each row. BigQuery uses this property to detect duplicate insertion requests on a best-effort basis."]
        #[serde(rename = "insertId", default)]
        pub insert_id: Option<String>,
        #[doc = "[Required] A JSON object that contains a row of data. The object's properties and values must match the destination table's schema."]
        #[serde(rename = "json", default)]
        pub json: Option<::std::collections::BTreeMap<String, crate::schemas::JsonValue>>,
    }
    impl ::field_selector::FieldSelector for TableDataInsertAllRequestRowsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableDataInsertAllRequest {
        #[doc = "[Optional] Accept rows that contain values that do not match the schema. The unknown values are ignored. Default is false, which treats unknown values as errors."]
        #[serde(rename = "ignoreUnknownValues", default)]
        pub ignore_unknown_values: Option<bool>,
        #[doc = "The resource type of the response."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The rows to insert."]
        #[serde(rename = "rows", default)]
        pub rows: Option<Vec<crate::schemas::TableDataInsertAllRequestRowsItems>>,
        #[doc = "[Optional] Insert all valid rows of a request, even if invalid rows exist. The default value is false, which causes the entire request to fail if any invalid rows exist."]
        #[serde(rename = "skipInvalidRows", default)]
        pub skip_invalid_rows: Option<bool>,
        #[doc = "If specified, treats the destination table as a base template, and inserts the rows into an instance table named \"{destination}{templateSuffix}\". BigQuery will manage creation of the instance table, using the schema of the base template table. See https://cloud.google.com/bigquery/streaming-data-into-bigquery#template-tables for considerations when working with templates tables."]
        #[serde(rename = "templateSuffix", default)]
        pub template_suffix: Option<String>,
    }
    impl ::field_selector::FieldSelector for TableDataInsertAllRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableDataInsertAllResponseInsertErrorsItems {
        #[doc = "Error information for the row indicated by the index property."]
        #[serde(rename = "errors", default)]
        pub errors: Option<Vec<crate::schemas::ErrorProto>>,
        #[doc = "The index of the row that error applies to."]
        #[serde(rename = "index", default)]
        pub index: Option<u32>,
    }
    impl ::field_selector::FieldSelector for TableDataInsertAllResponseInsertErrorsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableDataInsertAllResponse {
        #[doc = "An array of errors for rows that were not inserted."]
        #[serde(rename = "insertErrors", default)]
        pub insert_errors: Option<Vec<crate::schemas::TableDataInsertAllResponseInsertErrorsItems>>,
        #[doc = "The resource type of the response."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for TableDataInsertAllResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableDataList {
        #[doc = "A hash of this page of results."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The resource type of the response."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "A token used for paging results. Providing this token instead of the startIndex parameter can help you retrieve stable results when an underlying table is changing."]
        #[serde(rename = "pageToken", default)]
        pub page_token: Option<String>,
        #[doc = "Rows of results."]
        #[serde(rename = "rows", default)]
        pub rows: Option<Vec<crate::schemas::TableRow>>,
        #[doc = "The total number of rows in the complete table."]
        #[serde(rename = "totalRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_rows: Option<i64>,
    }
    impl ::field_selector::FieldSelector for TableDataList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableFieldSchemaCategories {
        #[doc = "A list of category resource names. For example, \"projects/1/taxonomies/2/categories/3\". At most 5 categories are allowed."]
        #[serde(rename = "names", default)]
        pub names: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TableFieldSchemaCategories {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableFieldSchema {
        #[doc = "[Optional] The categories attached to this field, used for field-level access control."]
        #[serde(rename = "categories", default)]
        pub categories: Option<crate::schemas::TableFieldSchemaCategories>,
        #[doc = "[Optional] The field description. The maximum length is 1,024 characters."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "[Optional] Describes the nested schema fields if the type property is set to RECORD."]
        #[serde(rename = "fields", default)]
        pub fields: Option<Vec<crate::schemas::TableFieldSchema>>,
        #[doc = "[Optional] The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE."]
        #[serde(rename = "mode", default)]
        pub mode: Option<String>,
        #[doc = "[Required] The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 128 characters."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "[Required] The field data type. Possible values include STRING, BYTES, INTEGER, INT64 (same as INTEGER), FLOAT, FLOAT64 (same as FLOAT), BOOLEAN, BOOL (same as BOOLEAN), TIMESTAMP, DATE, TIME, DATETIME, RECORD (where RECORD indicates that the field contains a nested schema) or STRUCT (same as RECORD)."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
    }
    impl ::field_selector::FieldSelector for TableFieldSchema {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableListTablesItemsView {
        #[doc = "True if view is defined in legacy SQL dialect, false if in standard SQL."]
        #[serde(rename = "useLegacySql", default)]
        pub use_legacy_sql: Option<bool>,
    }
    impl ::field_selector::FieldSelector for TableListTablesItemsView {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableListTablesItems {
        #[doc = "[Beta] Clustering specification for this table, if configured."]
        #[serde(rename = "clustering", default)]
        pub clustering: Option<crate::schemas::Clustering>,
        #[doc = "The time when this table was created, in milliseconds since the epoch."]
        #[serde(rename = "creationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: Option<i64>,
        #[doc = "[Optional] The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
        #[serde(rename = "expirationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub expiration_time: Option<i64>,
        #[doc = "The user-friendly name for this table."]
        #[serde(rename = "friendlyName", default)]
        pub friendly_name: Option<String>,
        #[doc = "An opaque ID of the table"]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The labels associated with this table. You can use these to organize and group your tables."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The type of table. Possible values are: TABLE, VIEW."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "A reference uniquely identifying the table."]
        #[serde(rename = "tableReference", default)]
        pub table_reference: Option<crate::schemas::TableReference>,
        #[doc = "The time-based partitioning specification for this table, if configured."]
        #[serde(rename = "timePartitioning", default)]
        pub time_partitioning: Option<crate::schemas::TimePartitioning>,
        #[doc = "Additional details for a view."]
        #[serde(rename = "view", default)]
        pub view: Option<crate::schemas::TableListTablesItemsView>,
    }
    impl ::field_selector::FieldSelector for TableListTablesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableList {
        #[doc = "A hash of this page of results."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The type of list."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "A token to request the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "Tables in the requested dataset."]
        #[serde(rename = "tables", default)]
        pub tables: Option<Vec<crate::schemas::TableListTablesItems>>,
        #[doc = "The total number of tables in the dataset."]
        #[serde(rename = "totalItems", default)]
        pub total_items: Option<i32>,
    }
    impl ::field_selector::FieldSelector for TableList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableReference {
        #[doc = "[Required] The ID of the dataset containing this table."]
        #[serde(rename = "datasetId", default)]
        pub dataset_id: Option<String>,
        #[doc = "[Required] The ID of the project containing this table."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "[Required] The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
        #[serde(rename = "tableId", default)]
        pub table_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for TableReference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TableRow {
        #[doc = "Represents a single row in the result set, consisting of one or more fields."]
        #[serde(rename = "f", default)]
        pub f: Option<Vec<crate::schemas::TableCell>>,
    }
    impl ::field_selector::FieldSelector for TableRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TableSchema {
        #[doc = "Describes the fields in a table."]
        #[serde(rename = "fields", default)]
        pub fields: Option<Vec<crate::schemas::TableFieldSchema>>,
    }
    impl ::field_selector::FieldSelector for TableSchema {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TimePartitioning {
        #[doc = "[Optional] Number of milliseconds for which to keep the storage for partitions in the table. The storage in a partition will have an expiration time of its partition time plus this value."]
        #[serde(rename = "expirationMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub expiration_ms: Option<i64>,
        #[doc = "[Beta] [Optional] If not set, the table is partitioned by pseudo column, referenced via either '_PARTITIONTIME' as TIMESTAMP type, or '_PARTITIONDATE' as DATE type. If field is specified, the table is instead partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED."]
        #[serde(rename = "field", default)]
        pub field: Option<String>,
        #[doc = "[Required] The only type supported is DAY, which will generate one partition per day."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[serde(rename = "requirePartitionFilter", default)]
        pub require_partition_filter: Option<bool>,
    }
    impl ::field_selector::FieldSelector for TimePartitioning {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsDataSplitMethod {
        DataSplitMethodUnspecified,
        #[doc = "Splits data randomly."]
        Random,
        #[doc = "Splits data with the user provided tags."]
        Custom,
        #[doc = "Splits data sequentially."]
        Sequential,
        #[doc = "Data split will be skipped."]
        NoSplit,
        #[doc = "Splits data automatically: Uses NO_SPLIT if the data size is small.\nOtherwise uses RANDOM."]
        AutoSplit,
    }
    impl TrainingOptionsDataSplitMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsDataSplitMethod::DataSplitMethodUnspecified => {
                    "DATA_SPLIT_METHOD_UNSPECIFIED"
                }
                TrainingOptionsDataSplitMethod::Random => "RANDOM",
                TrainingOptionsDataSplitMethod::Custom => "CUSTOM",
                TrainingOptionsDataSplitMethod::Sequential => "SEQUENTIAL",
                TrainingOptionsDataSplitMethod::NoSplit => "NO_SPLIT",
                TrainingOptionsDataSplitMethod::AutoSplit => "AUTO_SPLIT",
            }
        }
    }
    impl ::std::fmt::Display for TrainingOptionsDataSplitMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsDataSplitMethod {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsDataSplitMethod {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATA_SPLIT_METHOD_UNSPECIFIED" => {
                    TrainingOptionsDataSplitMethod::DataSplitMethodUnspecified
                }
                "RANDOM" => TrainingOptionsDataSplitMethod::Random,
                "CUSTOM" => TrainingOptionsDataSplitMethod::Custom,
                "SEQUENTIAL" => TrainingOptionsDataSplitMethod::Sequential,
                "NO_SPLIT" => TrainingOptionsDataSplitMethod::NoSplit,
                "AUTO_SPLIT" => TrainingOptionsDataSplitMethod::AutoSplit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsDistanceType {
        DistanceTypeUnspecified,
        #[doc = "Eculidean distance."]
        Euclidean,
        #[doc = "Cosine distance."]
        Cosine,
    }
    impl TrainingOptionsDistanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsDistanceType::DistanceTypeUnspecified => "DISTANCE_TYPE_UNSPECIFIED",
                TrainingOptionsDistanceType::Euclidean => "EUCLIDEAN",
                TrainingOptionsDistanceType::Cosine => "COSINE",
            }
        }
    }
    impl ::std::fmt::Display for TrainingOptionsDistanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsDistanceType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsDistanceType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISTANCE_TYPE_UNSPECIFIED" => TrainingOptionsDistanceType::DistanceTypeUnspecified,
                "EUCLIDEAN" => TrainingOptionsDistanceType::Euclidean,
                "COSINE" => TrainingOptionsDistanceType::Cosine,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsKmeansInitializationMethod {
        KmeansInitializationMethodUnspecified,
        #[doc = "Initializes the centroids randomly."]
        Random,
        #[doc = "Initializes the centroids using data specified in\nkmeans_initialization_column."]
        Custom,
    }
    impl TrainingOptionsKmeansInitializationMethod {
        pub fn as_str(self) -> &'static str {
            match self { TrainingOptionsKmeansInitializationMethod :: KmeansInitializationMethodUnspecified => "KMEANS_INITIALIZATION_METHOD_UNSPECIFIED" , TrainingOptionsKmeansInitializationMethod :: Random => "RANDOM" , TrainingOptionsKmeansInitializationMethod :: Custom => "CUSTOM" , }
        }
    }
    impl ::std::fmt::Display for TrainingOptionsKmeansInitializationMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsKmeansInitializationMethod {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsKmeansInitializationMethod {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "KMEANS_INITIALIZATION_METHOD_UNSPECIFIED" => {
                    TrainingOptionsKmeansInitializationMethod::KmeansInitializationMethodUnspecified
                }
                "RANDOM" => TrainingOptionsKmeansInitializationMethod::Random,
                "CUSTOM" => TrainingOptionsKmeansInitializationMethod::Custom,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsLearnRateStrategy {
        LearnRateStrategyUnspecified,
        #[doc = "Use line search to determine learning rate."]
        LineSearch,
        #[doc = "Use a constant learning rate."]
        Constant,
    }
    impl TrainingOptionsLearnRateStrategy {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsLearnRateStrategy::LearnRateStrategyUnspecified => {
                    "LEARN_RATE_STRATEGY_UNSPECIFIED"
                }
                TrainingOptionsLearnRateStrategy::LineSearch => "LINE_SEARCH",
                TrainingOptionsLearnRateStrategy::Constant => "CONSTANT",
            }
        }
    }
    impl ::std::fmt::Display for TrainingOptionsLearnRateStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsLearnRateStrategy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsLearnRateStrategy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LEARN_RATE_STRATEGY_UNSPECIFIED" => {
                    TrainingOptionsLearnRateStrategy::LearnRateStrategyUnspecified
                }
                "LINE_SEARCH" => TrainingOptionsLearnRateStrategy::LineSearch,
                "CONSTANT" => TrainingOptionsLearnRateStrategy::Constant,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsLossType {
        LossTypeUnspecified,
        #[doc = "Mean squared loss, used for linear regression."]
        MeanSquaredLoss,
        #[doc = "Mean log loss, used for logistic regression."]
        MeanLogLoss,
    }
    impl TrainingOptionsLossType {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsLossType::LossTypeUnspecified => "LOSS_TYPE_UNSPECIFIED",
                TrainingOptionsLossType::MeanSquaredLoss => "MEAN_SQUARED_LOSS",
                TrainingOptionsLossType::MeanLogLoss => "MEAN_LOG_LOSS",
            }
        }
    }
    impl ::std::fmt::Display for TrainingOptionsLossType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsLossType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsLossType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOSS_TYPE_UNSPECIFIED" => TrainingOptionsLossType::LossTypeUnspecified,
                "MEAN_SQUARED_LOSS" => TrainingOptionsLossType::MeanSquaredLoss,
                "MEAN_LOG_LOSS" => TrainingOptionsLossType::MeanLogLoss,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TrainingOptionsOptimizationStrategy {
        OptimizationStrategyUnspecified,
        #[doc = "Uses an iterative batch gradient descent algorithm."]
        BatchGradientDescent,
        #[doc = "Uses a normal equation to solve linear regression problem."]
        NormalEquation,
    }
    impl TrainingOptionsOptimizationStrategy {
        pub fn as_str(self) -> &'static str {
            match self {
                TrainingOptionsOptimizationStrategy::OptimizationStrategyUnspecified => {
                    "OPTIMIZATION_STRATEGY_UNSPECIFIED"
                }
                TrainingOptionsOptimizationStrategy::BatchGradientDescent => {
                    "BATCH_GRADIENT_DESCENT"
                }
                TrainingOptionsOptimizationStrategy::NormalEquation => "NORMAL_EQUATION",
            }
        }
    }
    impl ::std::fmt::Display for TrainingOptionsOptimizationStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TrainingOptionsOptimizationStrategy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TrainingOptionsOptimizationStrategy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPTIMIZATION_STRATEGY_UNSPECIFIED" => {
                    TrainingOptionsOptimizationStrategy::OptimizationStrategyUnspecified
                }
                "BATCH_GRADIENT_DESCENT" => {
                    TrainingOptionsOptimizationStrategy::BatchGradientDescent
                }
                "NORMAL_EQUATION" => TrainingOptionsOptimizationStrategy::NormalEquation,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TrainingOptions {
        #[doc = "The column to split data with. This column won't be used as a\nfeature.\n1. When data_split_method is CUSTOM, the corresponding column should\nbe boolean. The rows with true value tag are eval data, and the false\nare training data.\n2. When data_split_method is SEQ, the first DATA_SPLIT_EVAL_FRACTION\nrows (from smallest to largest) in the corresponding column are used\nas training data, and the rest are eval data. It respects the order\nin Orderable data types:\nhttps://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#data-type-properties"]
        #[serde(rename = "dataSplitColumn", default)]
        pub data_split_column: Option<String>,
        #[doc = "The fraction of evaluation data over the whole input data. The rest\nof data will be used as training data. The format should be double.\nAccurate to two decimal places.\nDefault value is 0.2."]
        #[serde(rename = "dataSplitEvalFraction", default)]
        pub data_split_eval_fraction: Option<f64>,
        #[doc = "The data split type for training and evaluation, e.g. RANDOM."]
        #[serde(rename = "dataSplitMethod", default)]
        pub data_split_method: Option<crate::schemas::TrainingOptionsDataSplitMethod>,
        #[doc = "Distance type for clustering models."]
        #[serde(rename = "distanceType", default)]
        pub distance_type: Option<crate::schemas::TrainingOptionsDistanceType>,
        #[doc = "Whether to stop early when the loss doesn't improve significantly\nany more (compared to min_relative_progress). Used only for iterative\ntraining algorithms."]
        #[serde(rename = "earlyStop", default)]
        pub early_stop: Option<bool>,
        #[doc = "Specifies the initial learning rate for the line search learn rate\nstrategy."]
        #[serde(rename = "initialLearnRate", default)]
        pub initial_learn_rate: Option<f64>,
        #[doc = "Name of input label columns in training data."]
        #[serde(rename = "inputLabelColumns", default)]
        pub input_label_columns: Option<Vec<String>>,
        #[doc = "The column used to provide the initial centroids for kmeans algorithm\nwhen kmeans_initialization_method is CUSTOM."]
        #[serde(rename = "kmeansInitializationColumn", default)]
        pub kmeans_initialization_column: Option<String>,
        #[doc = "The method used to initialize the centroids for kmeans algorithm."]
        #[serde(rename = "kmeansInitializationMethod", default)]
        pub kmeans_initialization_method:
            Option<crate::schemas::TrainingOptionsKmeansInitializationMethod>,
        #[doc = "L1 regularization coefficient."]
        #[serde(rename = "l1Regularization", default)]
        pub l_1_regularization: Option<f64>,
        #[doc = "L2 regularization coefficient."]
        #[serde(rename = "l2Regularization", default)]
        pub l_2_regularization: Option<f64>,
        #[doc = "Weights associated with each label class, for rebalancing the\ntraining data. Only applicable for classification models."]
        #[serde(rename = "labelClassWeights", default)]
        pub label_class_weights: Option<::std::collections::BTreeMap<String, f64>>,
        #[doc = "Learning rate in training. Used only for iterative training algorithms."]
        #[serde(rename = "learnRate", default)]
        pub learn_rate: Option<f64>,
        #[doc = "The strategy to determine learn rate for the current iteration."]
        #[serde(rename = "learnRateStrategy", default)]
        pub learn_rate_strategy: Option<crate::schemas::TrainingOptionsLearnRateStrategy>,
        #[doc = "Type of loss function used during training run."]
        #[serde(rename = "lossType", default)]
        pub loss_type: Option<crate::schemas::TrainingOptionsLossType>,
        #[doc = "The maximum number of iterations in training. Used only for iterative\ntraining algorithms."]
        #[serde(rename = "maxIterations", default)]
        #[serde(with = "crate::parsed_string")]
        pub max_iterations: Option<i64>,
        #[doc = "When early_stop is true, stops training when accuracy improvement is\nless than 'min_relative_progress'. Used only for iterative training\nalgorithms."]
        #[serde(rename = "minRelativeProgress", default)]
        pub min_relative_progress: Option<f64>,
        #[doc = "[Beta] Google Cloud Storage URI from which the model was imported. Only\napplicable for imported models."]
        #[serde(rename = "modelUri", default)]
        pub model_uri: Option<String>,
        #[doc = "Number of clusters for clustering models."]
        #[serde(rename = "numClusters", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_clusters: Option<i64>,
        #[doc = "Optimization strategy for training linear regression models."]
        #[serde(rename = "optimizationStrategy", default)]
        pub optimization_strategy: Option<crate::schemas::TrainingOptionsOptimizationStrategy>,
        #[doc = "Whether to train a model from the last checkpoint."]
        #[serde(rename = "warmStart", default)]
        pub warm_start: Option<bool>,
    }
    impl ::field_selector::FieldSelector for TrainingOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TrainingRun {
        #[doc = "The evaluation metrics over training/eval data that were computed at the\nend of training."]
        #[serde(rename = "evaluationMetrics", default)]
        pub evaluation_metrics: Option<crate::schemas::EvaluationMetrics>,
        #[doc = "Output of each iteration run, results.size() <= max_iterations."]
        #[serde(rename = "results", default)]
        pub results: Option<Vec<crate::schemas::IterationResult>>,
        #[doc = "The start time of this training run."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Options that were used for this training run, includes\nuser specified and default options that were used."]
        #[serde(rename = "trainingOptions", default)]
        pub training_options: Option<crate::schemas::TrainingOptions>,
    }
    impl ::field_selector::FieldSelector for TrainingRun {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct UserDefinedFunctionResource {
        #[doc = "[Pick one] An inline resource that contains code for a user-defined function (UDF). Providing a inline code resource is equivalent to providing a URI for a file containing the same code."]
        #[serde(rename = "inlineCode", default)]
        pub inline_code: Option<String>,
        #[doc = "[Pick one] A code resource to load from a Google Cloud Storage URI (gs://bucket/path)."]
        #[serde(rename = "resourceUri", default)]
        pub resource_uri: Option<String>,
    }
    impl ::field_selector::FieldSelector for UserDefinedFunctionResource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct ViewDefinition {
        #[doc = "[Required] A query that BigQuery executes when the view is referenced."]
        #[serde(rename = "query", default)]
        pub query: Option<String>,
        #[doc = "Specifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ Queries and views that reference this view must use the same flag value."]
        #[serde(rename = "useLegacySql", default)]
        pub use_legacy_sql: Option<bool>,
        #[doc = "Describes user-defined function resources used in the query."]
        #[serde(rename = "userDefinedFunctionResources", default)]
        pub user_defined_function_resources:
            Option<Vec<crate::schemas::UserDefinedFunctionResource>>,
    }
    impl ::field_selector::FieldSelector for ViewDefinition {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Upload/Download media content"]
        Media,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the datasets resource"]
    pub fn datasets(&self) -> crate::datasets::DatasetsActions<A> {
        crate::datasets::DatasetsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the jobs resource"]
    pub fn jobs(&self) -> crate::jobs::JobsActions<A> {
        crate::jobs::JobsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the models resource"]
    pub fn models(&self) -> crate::models::ModelsActions<A> {
        crate::models::ModelsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::projects::ProjectsActions<A> {
        crate::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the routines resource"]
    pub fn routines(&self) -> crate::routines::RoutinesActions<A> {
        crate::routines::RoutinesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the tabledata resource"]
    pub fn tabledata(&self) -> crate::tabledata::TabledataActions<A> {
        crate::tabledata::TabledataActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the tables resource"]
    pub fn tables(&self) -> crate::tables::TablesActions<A> {
        crate::tables::TablesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod datasets {
    pub mod params {}
    pub struct DatasetsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> DatasetsActions<'a, A> {
        #[doc = "Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name."]
        pub fn delete(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                delete_contents: None,
            }
        }
        #[doc = "Returns the dataset specified by datasetID."]
        pub fn get(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
            }
        }
        #[doc = "Creates a new empty dataset."]
        pub fn insert(
            &self,
            request: crate::schemas::Dataset,
            project_id: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
            }
        }
        #[doc = "Lists all datasets in the specified project to which you have been granted the READER dataset role."]
        pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                all: None,
                filter: None,
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports patch semantics."]
        pub fn patch(
            &self,
            request: crate::schemas::Dataset,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
            }
        }
        #[doc = "Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource."]
        pub fn update(
            &self,
            request: crate::schemas::Dataset,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        delete_contents: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "If True, delete all the tables in the dataset. If False and the dataset contains tables, the request will fail. Default is False"]
        pub fn delete_contents(&mut self, value: bool) -> &mut Self {
            self.delete_contents = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("deleteContents", &self.delete_contents)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Dataset, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Dataset,
        project_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Dataset, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        all: Option<bool>,
        filter: Option<String>,
        max_results: Option<u32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Whether to list all datasets, including hidden ones"]
        pub fn all(&mut self, value: bool) -> &mut Self {
            self.all = Some(value);
            self
        }
        #[doc = "An expression for filtering the results of the request by label. The syntax is \"labels.<name>[:<value>]\". Multiple filters can be ANDed together by connecting with a space. Example: \"labels.department:receiving labels.active\". See Filtering datasets using labels for details."]
        pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
            self.filter = Some(value.into());
            self
        }
        #[doc = "The maximum number of results to return"]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Page token, returned by a previous call, to request the next page of results"]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_datasets<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "datasets")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::DatasetList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("all", &self.all)]);
            let req = req.query(&[("filter", &self.filter)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Dataset,
        project_id: String,
        dataset_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Dataset, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Dataset,
        project_id: String,
        dataset_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Dataset, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod jobs {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum ListProjection {
            #[doc = "Includes all job data"]
            Full,
            #[doc = "Does not include the job configuration"]
            Minimal,
        }
        impl ListProjection {
            pub fn as_str(self) -> &'static str {
                match self {
                    ListProjection::Full => "full",
                    ListProjection::Minimal => "minimal",
                }
            }
        }
        impl ::std::fmt::Display for ListProjection {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListProjection {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListProjection {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "full" => ListProjection::Full,
                    "minimal" => ListProjection::Minimal,
                    _ => {
                        return Err(::serde::de::Error::custom(format!(
                            "invalid enum for #name: {}",
                            value
                        )))
                    }
                })
            }
        }
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum ListStateFilter {
            #[doc = "Finished jobs"]
            Done,
            #[doc = "Pending jobs"]
            Pending,
            #[doc = "Running jobs"]
            Running,
        }
        impl ListStateFilter {
            pub fn as_str(self) -> &'static str {
                match self {
                    ListStateFilter::Done => "done",
                    ListStateFilter::Pending => "pending",
                    ListStateFilter::Running => "running",
                }
            }
        }
        impl ::std::fmt::Display for ListStateFilter {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListStateFilter {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListStateFilter {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "done" => ListStateFilter::Done,
                    "pending" => ListStateFilter::Pending,
                    "running" => ListStateFilter::Running,
                    _ => {
                        return Err(::serde::de::Error::custom(format!(
                            "invalid enum for #name: {}",
                            value
                        )))
                    }
                })
            }
        }
    }
    pub struct JobsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> JobsActions<'a, A> {
        #[doc = "Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs."]
        pub fn cancel(
            &self,
            project_id: impl Into<String>,
            job_id: impl Into<String>,
        ) -> CancelRequestBuilder<A> {
            CancelRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                job_id: job_id.into(),
                location: None,
            }
        }
        #[doc = "Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role."]
        pub fn get(
            &self,
            project_id: impl Into<String>,
            job_id: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                job_id: job_id.into(),
                location: None,
            }
        }
        #[doc = "Retrieves the results of a query job."]
        pub fn get_query_results(
            &self,
            project_id: impl Into<String>,
            job_id: impl Into<String>,
        ) -> GetQueryResultsRequestBuilder<A> {
            GetQueryResultsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                job_id: job_id.into(),
                location: None,
                max_results: None,
                page_token: None,
                start_index: None,
                timeout_ms: None,
            }
        }
        #[doc = "Starts a new asynchronous job. Requires the Can View project role."]
        pub fn insert(
            &self,
            request: crate::schemas::Job,
            project_id: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
            }
        }
        #[doc = "Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property."]
        pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                all_users: None,
                max_creation_time: None,
                max_results: None,
                min_creation_time: None,
                page_token: None,
                parent_job_id: None,
                projection: None,
                state_filter: None,
            }
        }
        #[doc = "Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout."]
        pub fn query(
            &self,
            request: crate::schemas::QueryRequest,
            project_id: impl Into<String>,
        ) -> QueryRequestBuilder<A> {
            QueryRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CancelRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        job_id: String,
        location: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CancelRequestBuilder<'a, A> {
        #[doc = "The geographic location of the job. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
            self.location = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::JobCancelResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/jobs/");
            output.push_str(&self.job_id);
            output.push_str("/cancel");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("location", &self.location)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        job_id: String,
        location: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "The geographic location of the job. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
            self.location = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/jobs/");
            output.push_str(&self.job_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("location", &self.location)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetQueryResultsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        job_id: String,
        location: Option<String>,
        max_results: Option<u32>,
        page_token: Option<String>,
        start_index: Option<u64>,
        timeout_ms: Option<u32>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetQueryResultsRequestBuilder<'a, A> {
        #[doc = "The geographic location where the job should run. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location."]
        pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
            self.location = Some(value.into());
            self
        }
        #[doc = "Maximum number of results to read"]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Page token, returned by a previous call, to request the next page of results"]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Zero-based index of the starting row"]
        pub fn start_index(&mut self, value: u64) -> &mut Self {
            self.start_index = Some(value);
            self
        }
        #[doc = "How long to wait for the query to complete, in milliseconds, before returning. Default is 10 seconds. If the timeout passes before the job completes, the 'jobComplete' field in the response will be false"]
        pub fn timeout_ms(&mut self, value: u32) -> &mut Self {
            self.timeout_ms = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::GetQueryResultsResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/queries/");
            output.push_str(&self.job_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("location", &self.location)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("startIndex", &self.start_index)]);
            let req = req.query(&[("timeoutMs", &self.timeout_ms)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Job,
        project_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        fn _simple_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("upload/bigquery/v2/projects/");
            output.push_str(&self.project_id);
            output.push_str("/jobs");
            output
        }
        pub fn upload<T, R>(
            mut self,
            content: R,
            mime_type: ::mime::Mime,
        ) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            R: ::std::io::Read + ::std::io::Seek + Send + 'static,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._simple_upload_path());
            let req = req.query(&[("uploadType", "multipart")]);
            use crate::multipart::{Part, RelatedMultiPart};
            let mut multipart = RelatedMultiPart::new();
            let request_json = ::serde_json::to_vec(&self.request)?;
            multipart.new_part(Part::new(
                ::mime::APPLICATION_JSON,
                Box::new(::std::io::Cursor::new(request_json)),
            ));
            multipart.new_part(Part::new(mime_type, Box::new(content)));
            let req = req.header(
                ::reqwest::header::CONTENT_TYPE,
                format!("multipart/related; boundary={}", multipart.boundary()),
            );
            let req = req.body(reqwest::Body::new(multipart.into_reader()));
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _resumable_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("resumable/upload/bigquery/v2/projects/");
            output.push_str(&self.project_id);
            output.push_str("/jobs");
            output
        }
        pub fn start_resumable_upload(
            self,
            mime_type: ::mime::Mime,
        ) -> Result<crate::ResumableUpload, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._resumable_upload_path());
            let req = req.query(&[("uploadType", "resumable")]);
            let req = req.header(
                ::reqwest::header::HeaderName::from_static("x-upload-content-type"),
                mime_type.to_string(),
            );
            let req = req.json(&self.request);
            let resp = req.send()?.error_for_status()?;
            let location_header =
                resp.headers()
                    .get(::reqwest::header::LOCATION)
                    .ok_or_else(|| {
                        format!("No LOCATION header returned when initiating resumable upload")
                    })?;
            let upload_url = ::std::str::from_utf8(location_header.as_bytes())?.to_owned();
            Ok(crate::ResumableUpload::new(
                self.reqwest.clone(),
                upload_url,
            ))
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/jobs");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        all_users: Option<bool>,
        max_creation_time: Option<u64>,
        max_results: Option<u32>,
        min_creation_time: Option<u64>,
        page_token: Option<String>,
        parent_job_id: Option<String>,
        projection: Option<crate::jobs::params::ListProjection>,
        state_filter: Option<crate::jobs::params::ListStateFilter>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Whether to display jobs owned by all users in the project. Default false"]
        pub fn all_users(&mut self, value: bool) -> &mut Self {
            self.all_users = Some(value);
            self
        }
        #[doc = "Max value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created before or at this timestamp are returned"]
        pub fn max_creation_time(&mut self, value: u64) -> &mut Self {
            self.max_creation_time = Some(value);
            self
        }
        #[doc = "Maximum number of results to return"]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Min value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created after or at this timestamp are returned"]
        pub fn min_creation_time(&mut self, value: u64) -> &mut Self {
            self.min_creation_time = Some(value);
            self
        }
        #[doc = "Page token, returned by a previous call, to request the next page of results"]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "If set, retrieves only jobs whose parent is this job. Otherwise, retrieves only jobs which have no parent"]
        pub fn parent_job_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.parent_job_id = Some(value.into());
            self
        }
        #[doc = "Restrict information returned to a set of selected fields"]
        pub fn projection(&mut self, value: crate::jobs::params::ListProjection) -> &mut Self {
            self.projection = Some(value);
            self
        }
        #[doc = "Filter for job state"]
        pub fn state_filter(&mut self, value: crate::jobs::params::ListStateFilter) -> &mut Self {
            self.state_filter = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_jobs<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "jobs")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::JobList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/jobs");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("allUsers", &self.all_users)]);
            let req = req.query(&[("maxCreationTime", &self.max_creation_time)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("minCreationTime", &self.min_creation_time)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("parentJobId", &self.parent_job_id)]);
            let req = req.query(&[("projection", &self.projection)]);
            let req = req.query(&[("stateFilter", &self.state_filter)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct QueryRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::QueryRequest,
        project_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> QueryRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::QueryResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/queries");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod models {
    pub mod params {}
    pub struct ModelsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ModelsActions<'a, A> {
        #[doc = "Deletes the model specified by modelId from the dataset."]
        pub fn delete(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            model_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                model_id: model_id.into(),
            }
        }
        #[doc = "Gets the specified model resource by model ID."]
        pub fn get(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            model_id: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                model_id: model_id.into(),
            }
        }
        #[doc = "Lists all models in the specified dataset. Requires the READER dataset\nrole."]
        pub fn list(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Patch specific fields in the specified model."]
        pub fn patch(
            &self,
            request: crate::schemas::Model,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            model_id: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                model_id: model_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        model_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/models/");
            output.push_str(&self.model_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        model_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Model, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/models/");
            output.push_str(&self.model_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The maximum number of results to return in a single response page.\nLeverage the page tokens to iterate through the entire collection."]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Page token, returned by a previous call to request the next page of\nresults"]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_models<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "models")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ListModelsResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/models");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Model,
        project_id: String,
        dataset_id: String,
        model_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Model, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/models/");
            output.push_str(&self.model_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod projects {
    pub mod params {}
    pub struct ProjectsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
        #[doc = "Returns the email address of the service account for your project used for interactions with Google Cloud KMS."]
        pub fn get_service_account(
            &self,
            project_id: impl Into<String>,
        ) -> GetServiceAccountRequestBuilder<A> {
            GetServiceAccountRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
            }
        }
        #[doc = "Lists all projects to which you have been granted any project role."]
        pub fn list(&self) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                max_results: None,
                page_token: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetServiceAccountRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetServiceAccountRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::GetServiceAccountResponse, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/serviceAccount");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        max_results: Option<u32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Maximum number of results to return"]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Page token, returned by a previous call, to request the next page of results"]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_projects<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "projects")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ProjectList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
}
pub mod routines {
    pub mod params {}
    pub struct RoutinesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> RoutinesActions<'a, A> {
        #[doc = "Deletes the routine specified by routineId from the dataset."]
        pub fn delete(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            routine_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                routine_id: routine_id.into(),
            }
        }
        #[doc = "Gets the specified routine resource by routine ID."]
        pub fn get(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            routine_id: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                routine_id: routine_id.into(),
                field_mask: None,
            }
        }
        #[doc = "Creates a new routine in the dataset."]
        pub fn insert(
            &self,
            request: crate::schemas::Routine,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
            }
        }
        #[doc = "Lists all routines in the specified dataset. Requires the READER dataset\nrole."]
        pub fn list(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Updates information in an existing routine. The update method replaces the\nentire Routine resource."]
        pub fn update(
            &self,
            request: crate::schemas::Routine,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            routine_id: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                routine_id: routine_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        routine_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/routines/");
            output.push_str(&self.routine_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        routine_id: String,
        field_mask: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "If set, only the Routine fields in the field mask are returned in the\nresponse. If unset, all Routine fields are returned."]
        pub fn field_mask(&mut self, value: impl Into<String>) -> &mut Self {
            self.field_mask = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Routine, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/routines/");
            output.push_str(&self.routine_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("fieldMask", &self.field_mask)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Routine,
        project_id: String,
        dataset_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Routine, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/routines");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The maximum number of results to return in a single response page.\nLeverage the page tokens to iterate through the entire collection."]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Page token, returned by a previous call, to request the next page of\nresults"]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_routines<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "routines")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ListRoutinesResponse, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/routines");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Routine,
        project_id: String,
        dataset_id: String,
        routine_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Routine, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/routines/");
            output.push_str(&self.routine_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod tabledata {
    pub mod params {}
    pub struct TabledataActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> TabledataActions<'a, A> {
        #[doc = "Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset role."]
        pub fn insert_all(
            &self,
            request: crate::schemas::TableDataInsertAllRequest,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            table_id: impl Into<String>,
        ) -> InsertAllRequestBuilder<A> {
            InsertAllRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                table_id: table_id.into(),
            }
        }
        #[doc = "Retrieves table data from a specified set of rows. Requires the READER dataset role."]
        pub fn list(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            table_id: impl Into<String>,
        ) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                table_id: table_id.into(),
                max_results: None,
                page_token: None,
                selected_fields: None,
                start_index: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertAllRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::TableDataInsertAllRequest,
        project_id: String,
        dataset_id: String,
        table_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertAllRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::TableDataInsertAllResponse, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/tables/");
            output.push_str(&self.table_id);
            output.push_str("/insertAll");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        table_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        selected_fields: Option<String>,
        start_index: Option<u64>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Maximum number of results to return"]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Page token, returned by a previous call, identifying the result set"]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "List of fields to return (comma-separated). If unspecified, all fields are returned"]
        pub fn selected_fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.selected_fields = Some(value.into());
            self
        }
        #[doc = "Zero-based index of the starting row to read"]
        pub fn start_index(&mut self, value: u64) -> &mut Self {
            self.start_index = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::TableDataList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/tables/");
            output.push_str(&self.table_id);
            output.push_str("/data");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("selectedFields", &self.selected_fields)]);
            let req = req.query(&[("startIndex", &self.start_index)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod tables {
    pub mod params {}
    pub struct TablesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> TablesActions<'a, A> {
        #[doc = "Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted."]
        pub fn delete(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            table_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                table_id: table_id.into(),
            }
        }
        #[doc = "Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table."]
        pub fn get(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            table_id: impl Into<String>,
        ) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                table_id: table_id.into(),
                selected_fields: None,
            }
        }
        #[doc = "Creates a new, empty table in the dataset."]
        pub fn insert(
            &self,
            request: crate::schemas::Table,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
            }
        }
        #[doc = "Lists all tables in the specified dataset. Requires the READER dataset role."]
        pub fn list(
            &self,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
        ) -> ListRequestBuilder<A> {
            ListRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics."]
        pub fn patch(
            &self,
            request: crate::schemas::Table,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            table_id: impl Into<String>,
        ) -> PatchRequestBuilder<A> {
            PatchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                table_id: table_id.into(),
            }
        }
        #[doc = "Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource."]
        pub fn update(
            &self,
            request: crate::schemas::Table,
            project_id: impl Into<String>,
            dataset_id: impl Into<String>,
            table_id: impl Into<String>,
        ) -> UpdateRequestBuilder<A> {
            UpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                request,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                project_id: project_id.into(),
                dataset_id: dataset_id.into(),
                table_id: table_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        table_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/tables/");
            output.push_str(&self.table_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        table_id: String,
        selected_fields: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "List of fields to return (comma-separated). If unspecified, all fields are returned"]
        pub fn selected_fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.selected_fields = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Table, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/tables/");
            output.push_str(&self.table_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("selectedFields", &self.selected_fields)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Table,
        project_id: String,
        dataset_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Table, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/tables");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        dataset_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Maximum number of results to return"]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Page token, returned by a previous call, to request the next page of results"]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_tables<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "tables")]
                        items: Option<Vec<T>>,
                        #[serde(rename = "nextPageToken")]
                        next_page_token: Option<String>,
                    }
                    loop {
                        if let Some(iter) = self.items_iter.as_mut() {
                            match iter.next() {
                                Some(v) => return Some(Ok(v)),
                                None => {}
                            }
                        }
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::TableList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/tables");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
        fn set_page_token(&mut self, value: String) {
            self.page_token = value.into();
        }
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Table,
        project_id: String,
        dataset_id: String,
        table_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Table, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/tables/");
            output.push_str(&self.table_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Table,
        project_id: String,
        dataset_id: String,
        table_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Table, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/bigquery/v2/".to_owned();
            output.push_str("projects/");
            output.push_str(&self.project_id);
            output.push_str("/datasets/");
            output.push_str(&self.dataset_id);
            output.push_str("/tables/");
            output.push_str(&self.table_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/bigquery"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
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
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
        ResumableUpload {
            reqwest,
            url,
            progress: None,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn upload<R>(&mut self, mut reader: R) -> Result<(), Box<dyn ::std::error::Error>>
    where
        R: ::std::io::Read + ::std::io::Seek + Send + 'static,
    {
        let reader_len = {
            let start = reader.seek(::std::io::SeekFrom::Current(0))?;
            let end = reader.seek(::std::io::SeekFrom::End(0))?;
            reader.seek(::std::io::SeekFrom::Start(start))?;
            end
        };
        let progress = match self.progress {
            Some(progress) => progress,
            None => {
                let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
                let req = req.header(::reqwest::header::CONTENT_LENGTH, 0);
                let req = req.header(
                    ::reqwest::header::CONTENT_RANGE,
                    format!("bytes */{}", reader_len),
                );
                let resp = req.send()?.error_for_status()?;
                match resp.headers().get(::reqwest::header::RANGE) {
                    Some(range_header) => {
                        let (_, progress) = parse_range_header(range_header)
                            .map_err(|e| format!("invalid RANGE header: {}", e))?;
                        progress + 1
                    }
                    None => 0,
                }
            }
        };

        reader.seek(::std::io::SeekFrom::Start(progress as u64))?;
        let content_length = reader_len - progress as u64;
        let content_range = format!("bytes {}-{}/{}", progress, reader_len - 1, reader_len);
        let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
        let req = req.header(::reqwest::header::CONTENT_RANGE, content_range);
        let req = req.body(::reqwest::Body::sized(reader, content_length));
        req.send()?.error_for_status()?;
        Ok(())
    }
}

fn parse_range_header(
    range: &::reqwest::header::HeaderValue,
) -> Result<(i64, i64), Box<dyn ::std::error::Error>> {
    let range = range.to_str()?;
    if !range.starts_with("bytes ") {
        return Err(r#"does not begin with "bytes""#.to_owned().into());
    }
    let range = &range[6..];
    let slash_idx = range
        .find('/')
        .ok_or_else(|| r#"does not contain"#.to_owned())?;
    let (begin, end) = range.split_at(slash_idx);
    let end = &end[1..]; // remove '/'
    let begin: i64 = begin.parse()?;
    let end: i64 = end.parse()?;
    Ok((begin, end))
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
mod parsed_string {
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
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

trait IterableMethod {
    fn set_page_token(&mut self, value: String);
    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector;
}

struct PageIter<'a, M, T> {
    method: &'a mut M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<'a, M, T> Iterator for PageIter<'a, M, T>
where
    M: IterableMethod,
    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
{
    type Item = Result<T, Box<dyn ::std::error::Error>>;

    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
        use ::field_selector::FieldSelector;
        #[derive(::serde::Deserialize, FieldSelector)]
        struct PaginatedResult<T>
        where
            T: FieldSelector,
        {
            #[serde(rename = "nextPageToken")]
            next_page_token: Option<String>,

            #[serde(flatten)]
            page_contents: T,
        }

        if self.finished {
            return None;
        }

        let paginated_result: PaginatedResult<T> = match self.method.execute() {
            Ok(r) => r,
            Err(err) => return Some(Err(err)),
        };

        if let Some(next_page_token) = paginated_result.next_page_token {
            self.method.set_page_token(next_page_token);
        } else {
            self.finished = true;
        }

        Some(Ok(paginated_result.page_contents))
    }
}