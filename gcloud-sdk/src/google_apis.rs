pub mod ccc {
    pub mod hosted {
        pub mod marketplace {
            pub mod v2 {
                #[cfg(any(feature = "ccc-hosted-marketplace-v2"))]
                include_proto!("ccc.hosted.marketplace.v2");
            }
        }
    }
}
pub mod google {
    pub mod actions {
        pub mod r#type {
            #[cfg(any(feature = "google-actions-type"))]
            include_proto!("google.actions.r#type");
        }
        pub mod sdk {
            pub mod v2 {
                #[cfg(any(feature = "google-actions-sdk-v2"))]
                include_proto!("google.actions.sdk.v2");
                pub mod conversation {
                    #[cfg(
                        any(
                            feature = "google-actions-sdk-v2",
                            feature = "google-actions-sdk-v2-conversation",
                        )
                    )]
                    include_proto!("google.actions.sdk.v2.conversation");
                }
                pub mod interactionmodel {
                    #[cfg(
                        any(
                            feature = "google-actions-sdk-v2",
                            feature = "google-actions-sdk-v2-interactionmodel",
                        )
                    )]
                    include_proto!("google.actions.sdk.v2.interactionmodel");
                    pub mod prompt {
                        #[cfg(
                            any(
                                feature = "google-actions-sdk-v2",
                                feature = "google-actions-sdk-v2-interactionmodel",
                                feature = "google-actions-sdk-v2-interactionmodel-prompt",
                            )
                        )]
                        include_proto!("google.actions.sdk.v2.interactionmodel.prompt");
                    }
                    pub mod r#type {
                        #[cfg(
                            any(
                                feature = "google-actions-sdk-v2",
                                feature = "google-actions-sdk-v2-interactionmodel",
                                feature = "google-actions-sdk-v2-interactionmodel-type",
                            )
                        )]
                        include_proto!("google.actions.sdk.v2.interactionmodel.r#type");
                    }
                }
            }
        }
    }
    pub mod ads {
        pub mod admob {
            pub mod v1 {
                #[cfg(any(feature = "google-ads-admob-v1"))]
                include_proto!("google.ads.admob.v1");
            }
        }
        pub mod googleads {
            pub mod v10 {
                pub mod common {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v10-common",
                            feature = "google-ads-googleads-v10-errors",
                            feature = "google-ads-googleads-v10-resources",
                            feature = "google-ads-googleads-v10-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v10.common");
                }
                pub mod enums {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v10-common",
                            feature = "google-ads-googleads-v10-enums",
                            feature = "google-ads-googleads-v10-errors",
                            feature = "google-ads-googleads-v10-resources",
                            feature = "google-ads-googleads-v10-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v10.enums");
                }
                pub mod errors {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v10-errors",
                            feature = "google-ads-googleads-v10-resources",
                            feature = "google-ads-googleads-v10-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v10.errors");
                }
                pub mod resources {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v10-resources",
                            feature = "google-ads-googleads-v10-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v10.resources");
                }
                pub mod services {
                    #[cfg(any(feature = "google-ads-googleads-v10-services"))]
                    include_proto!("google.ads.googleads.v10.services");
                }
            }
            pub mod v11 {
                pub mod common {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v11-common",
                            feature = "google-ads-googleads-v11-errors",
                            feature = "google-ads-googleads-v11-resources",
                            feature = "google-ads-googleads-v11-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v11.common");
                }
                pub mod enums {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v11-common",
                            feature = "google-ads-googleads-v11-enums",
                            feature = "google-ads-googleads-v11-errors",
                            feature = "google-ads-googleads-v11-resources",
                            feature = "google-ads-googleads-v11-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v11.enums");
                }
                pub mod errors {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v11-errors",
                            feature = "google-ads-googleads-v11-resources",
                            feature = "google-ads-googleads-v11-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v11.errors");
                }
                pub mod resources {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v11-resources",
                            feature = "google-ads-googleads-v11-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v11.resources");
                }
                pub mod services {
                    #[cfg(any(feature = "google-ads-googleads-v11-services"))]
                    include_proto!("google.ads.googleads.v11.services");
                }
            }
            pub mod v9 {
                pub mod common {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v9-common",
                            feature = "google-ads-googleads-v9-errors",
                            feature = "google-ads-googleads-v9-resources",
                            feature = "google-ads-googleads-v9-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v9.common");
                }
                pub mod enums {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v9-common",
                            feature = "google-ads-googleads-v9-enums",
                            feature = "google-ads-googleads-v9-errors",
                            feature = "google-ads-googleads-v9-resources",
                            feature = "google-ads-googleads-v9-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v9.enums");
                }
                pub mod errors {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v9-errors",
                            feature = "google-ads-googleads-v9-resources",
                            feature = "google-ads-googleads-v9-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v9.errors");
                }
                pub mod resources {
                    #[cfg(
                        any(
                            feature = "google-ads-googleads-v9-resources",
                            feature = "google-ads-googleads-v9-services",
                        )
                    )]
                    include_proto!("google.ads.googleads.v9.resources");
                }
                pub mod services {
                    #[cfg(any(feature = "google-ads-googleads-v9-services"))]
                    include_proto!("google.ads.googleads.v9.services");
                }
            }
        }
    }
    pub mod analytics {
        pub mod admin {
            pub mod v1alpha {
                #[cfg(any(feature = "google-analytics-admin-v1alpha"))]
                include_proto!("google.analytics.admin.v1alpha");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-analytics-admin-v1beta"))]
                include_proto!("google.analytics.admin.v1beta");
            }
        }
        pub mod data {
            pub mod v1alpha {
                #[cfg(any(feature = "google-analytics-data-v1alpha"))]
                include_proto!("google.analytics.data.v1alpha");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-analytics-data-v1beta"))]
                include_proto!("google.analytics.data.v1beta");
            }
        }
    }
    pub mod api {
        #[cfg(
            any(
                feature = "ccc-hosted-marketplace-v2",
                feature = "google-actions-sdk-v2",
                feature = "google-actions-sdk-v2-interactionmodel",
                feature = "google-actions-sdk-v2-interactionmodel-prompt",
                feature = "google-actions-sdk-v2-interactionmodel-type",
                feature = "google-ads-admob-v1",
                feature = "google-ads-googleads-v10-common",
                feature = "google-ads-googleads-v10-resources",
                feature = "google-ads-googleads-v10-services",
                feature = "google-ads-googleads-v11-common",
                feature = "google-ads-googleads-v11-resources",
                feature = "google-ads-googleads-v11-services",
                feature = "google-ads-googleads-v9-common",
                feature = "google-ads-googleads-v9-enums",
                feature = "google-ads-googleads-v9-errors",
                feature = "google-ads-googleads-v9-resources",
                feature = "google-ads-googleads-v9-services",
                feature = "google-analytics-admin-v1alpha",
                feature = "google-analytics-admin-v1beta",
                feature = "google-analytics-data-v1alpha",
                feature = "google-analytics-data-v1beta",
                feature = "google-api",
                feature = "google-api-expr-conformance-v1alpha1",
                feature = "google-api-servicecontrol-v1",
                feature = "google-api-servicecontrol-v2",
                feature = "google-api-servicemanagement-v1",
                feature = "google-api-serviceusage-v1",
                feature = "google-api-serviceusage-v1beta1",
                feature = "google-appengine-v1",
                feature = "google-appengine-v1beta",
                feature = "google-apps-alertcenter-v1beta1",
                feature = "google-apps-drive-activity-v2",
                feature = "google-apps-script-type-calendar",
                feature = "google-apps-script-type-docs",
                feature = "google-apps-script-type-sheets",
                feature = "google-apps-script-type-slides",
                feature = "google-area120-tables-v1alpha1",
                feature = "google-assistant-embedded-v1alpha1",
                feature = "google-assistant-embedded-v1alpha2",
                feature = "google-bigtable-admin-v2",
                feature = "google-bigtable-v2",
                feature = "google-bytestream",
                feature = "google-chromeos-moblab-v1beta1",
                feature = "google-chromeos-uidetection-v1",
                feature = "google-cloud",
                feature = "google-cloud-accessapproval-v1",
                feature = "google-cloud-aiplatform-v1",
                feature = "google-cloud-aiplatform-v1-schema-predict-instance",
                feature = "google-cloud-aiplatform-v1-schema-predict-params",
                feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
                feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
                feature = "google-cloud-aiplatform-v1beta1",
                feature = "google-cloud-aiplatform-v1beta1-schema",
                feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
                feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
                feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
                feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
                feature = "google-cloud-apigateway-v1",
                feature = "google-cloud-apigeeconnect-v1",
                feature = "google-cloud-apigeeregistry-v1",
                feature = "google-cloud-asset-v1",
                feature = "google-cloud-asset-v1p1beta1",
                feature = "google-cloud-asset-v1p2beta1",
                feature = "google-cloud-asset-v1p4beta1",
                feature = "google-cloud-asset-v1p5beta1",
                feature = "google-cloud-asset-v1p7beta1",
                feature = "google-cloud-assuredworkloads-v1",
                feature = "google-cloud-assuredworkloads-v1beta1",
                feature = "google-cloud-automl-v1",
                feature = "google-cloud-automl-v1beta1",
                feature = "google-cloud-baremetalsolution-v2",
                feature = "google-cloud-batch-v1",
                feature = "google-cloud-batch-v1alpha",
                feature = "google-cloud-beyondcorp-appconnections-v1",
                feature = "google-cloud-beyondcorp-appconnectors-v1",
                feature = "google-cloud-beyondcorp-appgateways-v1",
                feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
                feature = "google-cloud-beyondcorp-clientgateways-v1",
                feature = "google-cloud-bigquery-connection-v1",
                feature = "google-cloud-bigquery-connection-v1beta1",
                feature = "google-cloud-bigquery-dataexchange-v1beta1",
                feature = "google-cloud-bigquery-datatransfer-v1",
                feature = "google-cloud-bigquery-logging-v1",
                feature = "google-cloud-bigquery-migration-v2",
                feature = "google-cloud-bigquery-migration-v2alpha",
                feature = "google-cloud-bigquery-reservation-v1",
                feature = "google-cloud-bigquery-reservation-v1beta1",
                feature = "google-cloud-bigquery-storage-v1",
                feature = "google-cloud-bigquery-storage-v1beta1",
                feature = "google-cloud-bigquery-storage-v1beta2",
                feature = "google-cloud-bigquery-v2",
                feature = "google-cloud-billing-budgets-v1",
                feature = "google-cloud-billing-budgets-v1beta1",
                feature = "google-cloud-billing-v1",
                feature = "google-cloud-binaryauthorization-v1",
                feature = "google-cloud-binaryauthorization-v1beta1",
                feature = "google-cloud-certificatemanager-v1",
                feature = "google-cloud-channel-v1",
                feature = "google-cloud-clouddms-logging-v1",
                feature = "google-cloud-clouddms-v1",
                feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
                feature = "google-cloud-common",
                feature = "google-cloud-compute-v1",
                feature = "google-cloud-compute-v1small",
                feature = "google-cloud-contactcenterinsights-v1",
                feature = "google-cloud-datacatalog-v1",
                feature = "google-cloud-datacatalog-v1beta1",
                feature = "google-cloud-dataform-v1alpha2",
                feature = "google-cloud-datafusion-v1",
                feature = "google-cloud-datafusion-v1beta1",
                feature = "google-cloud-datalabeling-v1beta1",
                feature = "google-cloud-dataplex-v1",
                feature = "google-cloud-dataproc-v1",
                feature = "google-cloud-dataqna-v1alpha",
                feature = "google-cloud-datastream-v1",
                feature = "google-cloud-datastream-v1alpha1",
                feature = "google-cloud-deploy-v1",
                feature = "google-cloud-dialogflow-cx-v3",
                feature = "google-cloud-dialogflow-cx-v3beta1",
                feature = "google-cloud-dialogflow-v2",
                feature = "google-cloud-dialogflow-v2beta1",
                feature = "google-cloud-documentai-v1",
                feature = "google-cloud-documentai-v1beta1",
                feature = "google-cloud-documentai-v1beta2",
                feature = "google-cloud-documentai-v1beta3",
                feature = "google-cloud-domains-v1",
                feature = "google-cloud-domains-v1alpha2",
                feature = "google-cloud-domains-v1beta1",
                feature = "google-cloud-essentialcontacts-v1",
                feature = "google-cloud-eventarc-publishing-v1",
                feature = "google-cloud-eventarc-v1",
                feature = "google-cloud-filestore-v1",
                feature = "google-cloud-filestore-v1beta1",
                feature = "google-cloud-functions-v1",
                feature = "google-cloud-functions-v2",
                feature = "google-cloud-functions-v2alpha",
                feature = "google-cloud-functions-v2beta",
                feature = "google-cloud-gaming-v1",
                feature = "google-cloud-gaming-v1beta",
                feature = "google-cloud-gkebackup-v1",
                feature = "google-cloud-gkeconnect-gateway-v1",
                feature = "google-cloud-gkeconnect-gateway-v1alpha1",
                feature = "google-cloud-gkeconnect-gateway-v1beta1",
                feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
                feature = "google-cloud-gkehub-configmanagement-v1alpha",
                feature = "google-cloud-gkehub-configmanagement-v1beta",
                feature = "google-cloud-gkehub-metering-v1alpha",
                feature = "google-cloud-gkehub-metering-v1beta",
                feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
                feature = "google-cloud-gkehub-multiclusteringress-v1beta",
                feature = "google-cloud-gkehub-servicemesh-v1alpha",
                feature = "google-cloud-gkehub-v1",
                feature = "google-cloud-gkehub-v1alpha",
                feature = "google-cloud-gkehub-v1alpha2",
                feature = "google-cloud-gkehub-v1beta",
                feature = "google-cloud-gkehub-v1beta1",
                feature = "google-cloud-gkemulticloud-v1",
                feature = "google-cloud-gsuiteaddons-v1",
                feature = "google-cloud-iap-v1",
                feature = "google-cloud-iap-v1beta1",
                feature = "google-cloud-identitytoolkit-v2",
                feature = "google-cloud-ids-v1",
                feature = "google-cloud-integrations-v1alpha",
                feature = "google-cloud-iot-v1",
                feature = "google-cloud-kms-v1",
                feature = "google-cloud-language-v1",
                feature = "google-cloud-language-v1beta1",
                feature = "google-cloud-language-v1beta2",
                feature = "google-cloud-lifesciences-v2beta",
                feature = "google-cloud-location",
                feature = "google-cloud-managedidentities-v1",
                feature = "google-cloud-managedidentities-v1beta1",
                feature = "google-cloud-mediatranslation-v1alpha1",
                feature = "google-cloud-mediatranslation-v1beta1",
                feature = "google-cloud-memcache-v1",
                feature = "google-cloud-memcache-v1beta2",
                feature = "google-cloud-metastore-v1",
                feature = "google-cloud-metastore-v1alpha",
                feature = "google-cloud-metastore-v1beta",
                feature = "google-cloud-networkconnectivity-v1",
                feature = "google-cloud-networkconnectivity-v1alpha1",
                feature = "google-cloud-networkmanagement-v1",
                feature = "google-cloud-networkmanagement-v1beta1",
                feature = "google-cloud-networksecurity-v1",
                feature = "google-cloud-networksecurity-v1beta1",
                feature = "google-cloud-networkservices-v1",
                feature = "google-cloud-networkservices-v1beta1",
                feature = "google-cloud-notebooks-logging-v1",
                feature = "google-cloud-notebooks-v1",
                feature = "google-cloud-notebooks-v1beta1",
                feature = "google-cloud-optimization-v1",
                feature = "google-cloud-orchestration-airflow-service-v1",
                feature = "google-cloud-orchestration-airflow-service-v1beta1",
                feature = "google-cloud-orgpolicy-v1",
                feature = "google-cloud-orgpolicy-v2",
                feature = "google-cloud-osconfig-agentendpoint-v1",
                feature = "google-cloud-osconfig-agentendpoint-v1beta",
                feature = "google-cloud-osconfig-v1",
                feature = "google-cloud-osconfig-v1alpha",
                feature = "google-cloud-osconfig-v1beta",
                feature = "google-cloud-oslogin-common",
                feature = "google-cloud-oslogin-v1",
                feature = "google-cloud-oslogin-v1alpha",
                feature = "google-cloud-oslogin-v1beta",
                feature = "google-cloud-paymentgateway-issuerswitch-v1",
                feature = "google-cloud-phishingprotection-v1beta1",
                feature = "google-cloud-policytroubleshooter-v1",
                feature = "google-cloud-privatecatalog-v1beta1",
                feature = "google-cloud-pubsublite-v1",
                feature = "google-cloud-recaptchaenterprise-v1",
                feature = "google-cloud-recaptchaenterprise-v1beta1",
                feature = "google-cloud-recommendationengine-v1beta1",
                feature = "google-cloud-recommender-logging-v1",
                feature = "google-cloud-recommender-logging-v1beta1",
                feature = "google-cloud-recommender-v1",
                feature = "google-cloud-recommender-v1beta1",
                feature = "google-cloud-redis-v1",
                feature = "google-cloud-redis-v1beta1",
                feature = "google-cloud-resourcemanager-v2",
                feature = "google-cloud-resourcemanager-v3",
                feature = "google-cloud-resourcesettings-v1",
                feature = "google-cloud-retail-v2",
                feature = "google-cloud-retail-v2alpha",
                feature = "google-cloud-retail-v2beta",
                feature = "google-cloud-run-v2",
                feature = "google-cloud-runtimeconfig-v1beta1",
                feature = "google-cloud-scheduler-v1",
                feature = "google-cloud-scheduler-v1beta1",
                feature = "google-cloud-secretmanager-v1",
                feature = "google-cloud-secrets-v1beta1",
                feature = "google-cloud-security-privateca-v1",
                feature = "google-cloud-security-privateca-v1beta1",
                feature = "google-cloud-security-publicca-v1beta1",
                feature = "google-cloud-securitycenter-settings-v1beta1",
                feature = "google-cloud-securitycenter-v1",
                feature = "google-cloud-securitycenter-v1beta1",
                feature = "google-cloud-securitycenter-v1p1beta1",
                feature = "google-cloud-servicedirectory-v1",
                feature = "google-cloud-servicedirectory-v1beta1",
                feature = "google-cloud-shell-v1",
                feature = "google-cloud-speech-v1",
                feature = "google-cloud-speech-v1p1beta1",
                feature = "google-cloud-sql-v1",
                feature = "google-cloud-sql-v1beta4",
                feature = "google-cloud-support-common",
                feature = "google-cloud-support-v1alpha1",
                feature = "google-cloud-talent-v4",
                feature = "google-cloud-talent-v4beta1",
                feature = "google-cloud-tasks-v2",
                feature = "google-cloud-tasks-v2beta2",
                feature = "google-cloud-tasks-v2beta3",
                feature = "google-cloud-texttospeech-v1",
                feature = "google-cloud-texttospeech-v1beta1",
                feature = "google-cloud-tpu-v1",
                feature = "google-cloud-tpu-v2alpha1",
                feature = "google-cloud-translation-v3",
                feature = "google-cloud-translation-v3beta1",
                feature = "google-cloud-video-livestream-logging-v1",
                feature = "google-cloud-video-livestream-v1",
                feature = "google-cloud-video-stitcher-v1",
                feature = "google-cloud-video-transcoder-v1",
                feature = "google-cloud-videointelligence-v1",
                feature = "google-cloud-videointelligence-v1beta2",
                feature = "google-cloud-videointelligence-v1p1beta1",
                feature = "google-cloud-videointelligence-v1p2beta1",
                feature = "google-cloud-videointelligence-v1p3beta1",
                feature = "google-cloud-vision-v1",
                feature = "google-cloud-vision-v1p1beta1",
                feature = "google-cloud-vision-v1p2beta1",
                feature = "google-cloud-vision-v1p3beta1",
                feature = "google-cloud-vision-v1p4beta1",
                feature = "google-cloud-vmmigration-v1",
                feature = "google-cloud-vpcaccess-v1",
                feature = "google-cloud-webrisk-v1",
                feature = "google-cloud-webrisk-v1beta1",
                feature = "google-cloud-websecurityscanner-v1",
                feature = "google-cloud-websecurityscanner-v1alpha",
                feature = "google-cloud-websecurityscanner-v1beta",
                feature = "google-cloud-workflows-executions-v1",
                feature = "google-cloud-workflows-executions-v1beta",
                feature = "google-cloud-workflows-v1",
                feature = "google-cloud-workflows-v1beta",
                feature = "google-container-v1",
                feature = "google-container-v1alpha1",
                feature = "google-container-v1beta1",
                feature = "google-dataflow-v1beta3",
                feature = "google-datastore-admin-v1",
                feature = "google-datastore-admin-v1beta1",
                feature = "google-datastore-v1",
                feature = "google-datastore-v1beta3",
                feature = "google-devtools-artifactregistry-v1",
                feature = "google-devtools-artifactregistry-v1beta2",
                feature = "google-devtools-build-v1",
                feature = "google-devtools-cloudbuild-v1",
                feature = "google-devtools-clouddebugger-v2",
                feature = "google-devtools-clouderrorreporting-v1beta1",
                feature = "google-devtools-cloudprofiler-v2",
                feature = "google-devtools-cloudtrace-v1",
                feature = "google-devtools-cloudtrace-v2",
                feature = "google-devtools-containeranalysis-v1",
                feature = "google-devtools-containeranalysis-v1beta1",
                feature = "google-devtools-remoteworkers-v1test2",
                feature = "google-devtools-resultstore-v2",
                feature = "google-devtools-sourcerepo-v1",
                feature = "google-devtools-testing-v1",
                feature = "google-example-endpointsapis-v1",
                feature = "google-example-library-v1",
                feature = "google-firebase-fcm-connection-v1alpha1",
                feature = "google-firestore-admin-v1",
                feature = "google-firestore-admin-v1beta1",
                feature = "google-firestore-admin-v1beta2",
                feature = "google-firestore-bundle",
                feature = "google-firestore-v1",
                feature = "google-firestore-v1beta1",
                feature = "google-genomics-v1",
                feature = "google-genomics-v1alpha2",
                feature = "google-home-enterprise-sdm-v1",
                feature = "google-home-graph-v1",
                feature = "google-iam-admin-v1",
                feature = "google-iam-credentials-v1",
                feature = "google-iam-v1",
                feature = "google-iam-v1-logging",
                feature = "google-iam-v1beta",
                feature = "google-iam-v2beta",
                feature = "google-identity-accesscontextmanager-v1",
                feature = "google-logging-v2",
                feature = "google-longrunning",
                feature = "google-maps-playablelocations-v3",
                feature = "google-maps-playablelocations-v3-sample",
                feature = "google-maps-regionlookup-v1alpha",
                feature = "google-maps-roads-v1op",
                feature = "google-maps-routes-v1",
                feature = "google-maps-routes-v1alpha",
                feature = "google-maps-routing-v2",
                feature = "google-monitoring-dashboard-v1",
                feature = "google-monitoring-metricsscope-v1",
                feature = "google-monitoring-v3",
                feature = "google-partner-aistreams-v1alpha1",
                feature = "google-privacy-dlp-v2",
                feature = "google-pubsub-v1",
                feature = "google-spanner-admin-database-v1",
                feature = "google-spanner-admin-instance-v1",
                feature = "google-spanner-v1",
                feature = "google-storage-v1",
                feature = "google-storage-v2",
                feature = "google-storagetransfer-v1",
                feature = "google-streetview-publish-v1",
                feature = "google-watcher-v1",
                feature = "grafeas-v1",
                feature = "grafeas-v1beta1",
                feature = "maps-fleetengine-delivery-v1",
                feature = "maps-fleetengine-v1",
            )
        )]
        include_proto!("google.api");
        pub mod expr {
            pub mod conformance {
                pub mod v1alpha1 {
                    #[cfg(any(feature = "google-api-expr-conformance-v1alpha1"))]
                    include_proto!("google.api.expr.conformance.v1alpha1");
                }
            }
            pub mod v1alpha1 {
                #[cfg(
                    any(
                        feature = "google-api-expr-conformance-v1alpha1",
                        feature = "google-api-expr-v1alpha1",
                    )
                )]
                include_proto!("google.api.expr.v1alpha1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-api-expr-v1beta1"))]
                include_proto!("google.api.expr.v1beta1");
            }
        }
        pub mod servicecontrol {
            pub mod v1 {
                #[cfg(any(feature = "google-api-servicecontrol-v1"))]
                include_proto!("google.api.servicecontrol.v1");
            }
            pub mod v2 {
                #[cfg(any(feature = "google-api-servicecontrol-v2"))]
                include_proto!("google.api.servicecontrol.v2");
            }
        }
        pub mod servicemanagement {
            pub mod v1 {
                #[cfg(any(feature = "google-api-servicemanagement-v1"))]
                include_proto!("google.api.servicemanagement.v1");
            }
        }
        pub mod serviceusage {
            pub mod v1 {
                #[cfg(any(feature = "google-api-serviceusage-v1"))]
                include_proto!("google.api.serviceusage.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-api-serviceusage-v1beta1"))]
                include_proto!("google.api.serviceusage.v1beta1");
            }
        }
    }
    pub mod appengine {
        pub mod legacy {
            #[cfg(any(feature = "google-appengine-legacy"))]
            include_proto!("google.appengine.legacy");
        }
        pub mod logging {
            pub mod v1 {
                #[cfg(any(feature = "google-appengine-logging-v1"))]
                include_proto!("google.appengine.logging.v1");
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "google-appengine-v1"))]
            include_proto!("google.appengine.v1");
        }
        pub mod v1beta {
            #[cfg(any(feature = "google-appengine-v1beta"))]
            include_proto!("google.appengine.v1beta");
        }
    }
    pub mod apps {
        pub mod alertcenter {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-apps-alertcenter-v1beta1"))]
                include_proto!("google.apps.alertcenter.v1beta1");
            }
        }
        pub mod drive {
            pub mod activity {
                pub mod v2 {
                    #[cfg(any(feature = "google-apps-drive-activity-v2"))]
                    include_proto!("google.apps.drive.activity.v2");
                }
            }
        }
        pub mod script {
            pub mod r#type {
                #[cfg(
                    any(
                        feature = "google-apps-script-type",
                        feature = "google-apps-script-type-calendar",
                        feature = "google-apps-script-type-docs",
                        feature = "google-apps-script-type-drive",
                        feature = "google-apps-script-type-gmail",
                        feature = "google-apps-script-type-sheets",
                        feature = "google-apps-script-type-slides",
                        feature = "google-cloud-gsuiteaddons-v1",
                    )
                )]
                include_proto!("google.apps.script.r#type");
                pub mod calendar {
                    #[cfg(
                        any(
                            feature = "google-apps-script-type-calendar",
                            feature = "google-cloud-gsuiteaddons-v1",
                        )
                    )]
                    include_proto!("google.apps.script.r#type.calendar");
                }
                pub mod docs {
                    #[cfg(
                        any(
                            feature = "google-apps-script-type-docs",
                            feature = "google-cloud-gsuiteaddons-v1",
                        )
                    )]
                    include_proto!("google.apps.script.r#type.docs");
                }
                pub mod drive {
                    #[cfg(
                        any(
                            feature = "google-apps-script-type-drive",
                            feature = "google-cloud-gsuiteaddons-v1",
                        )
                    )]
                    include_proto!("google.apps.script.r#type.drive");
                }
                pub mod gmail {
                    #[cfg(
                        any(
                            feature = "google-apps-script-type-gmail",
                            feature = "google-cloud-gsuiteaddons-v1",
                        )
                    )]
                    include_proto!("google.apps.script.r#type.gmail");
                }
                pub mod sheets {
                    #[cfg(
                        any(
                            feature = "google-apps-script-type-sheets",
                            feature = "google-cloud-gsuiteaddons-v1",
                        )
                    )]
                    include_proto!("google.apps.script.r#type.sheets");
                }
                pub mod slides {
                    #[cfg(
                        any(
                            feature = "google-apps-script-type-slides",
                            feature = "google-cloud-gsuiteaddons-v1",
                        )
                    )]
                    include_proto!("google.apps.script.r#type.slides");
                }
            }
        }
    }
    pub mod area120 {
        pub mod tables {
            pub mod v1alpha1 {
                #[cfg(any(feature = "google-area120-tables-v1alpha1"))]
                include_proto!("google.area120.tables.v1alpha1");
            }
        }
    }
    pub mod assistant {
        pub mod embedded {
            pub mod v1alpha1 {
                #[cfg(any(feature = "google-assistant-embedded-v1alpha1"))]
                include_proto!("google.assistant.embedded.v1alpha1");
            }
            pub mod v1alpha2 {
                #[cfg(any(feature = "google-assistant-embedded-v1alpha2"))]
                include_proto!("google.assistant.embedded.v1alpha2");
            }
        }
    }
    pub mod bigtable {
        pub mod admin {
            pub mod v2 {
                #[cfg(any(feature = "google-bigtable-admin-v2"))]
                include_proto!("google.bigtable.admin.v2");
            }
        }
        pub mod v2 {
            #[cfg(any(feature = "google-bigtable-v2"))]
            include_proto!("google.bigtable.v2");
        }
    }
    pub mod bytestream {
        #[cfg(any(feature = "google-bytestream"))]
        include_proto!("google.bytestream");
    }
    pub mod chat {
        pub mod logging {
            pub mod v1 {
                #[cfg(any(feature = "google-chat-logging-v1"))]
                include_proto!("google.chat.logging.v1");
            }
        }
    }
    pub mod chromeos {
        pub mod moblab {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-chromeos-moblab-v1beta1"))]
                include_proto!("google.chromeos.moblab.v1beta1");
            }
        }
        pub mod uidetection {
            pub mod v1 {
                #[cfg(any(feature = "google-chromeos-uidetection-v1"))]
                include_proto!("google.chromeos.uidetection.v1");
            }
        }
    }
    pub mod cloud {
        #[cfg(
            any(
                feature = "google-cloud",
                feature = "google-cloud-compute-v1",
                feature = "google-cloud-compute-v1small",
            )
        )]
        include_proto!("google.cloud");
        pub mod accessapproval {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-accessapproval-v1"))]
                include_proto!("google.cloud.accessapproval.v1");
            }
        }
        pub mod aiplatform {
            pub mod logging {
                #[cfg(any(feature = "google-cloud-aiplatform-logging"))]
                include_proto!("google.cloud.aiplatform.logging");
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-aiplatform-v1"))]
                include_proto!("google.cloud.aiplatform.v1");
                pub mod schema {
                    pub mod predict {
                        pub mod instance {
                            #[cfg(
                                any(
                                    feature = "google-cloud-aiplatform-v1-schema-predict-instance",
                                )
                            )]
                            include_proto!(
                                "google.cloud.aiplatform.v1.schema.predict.instance"
                            );
                        }
                        pub mod params {
                            #[cfg(
                                any(
                                    feature = "google-cloud-aiplatform-v1-schema-predict-params",
                                )
                            )]
                            include_proto!(
                                "google.cloud.aiplatform.v1.schema.predict.params"
                            );
                        }
                        pub mod prediction {
                            #[cfg(
                                any(
                                    feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
                                )
                            )]
                            include_proto!(
                                "google.cloud.aiplatform.v1.schema.predict.prediction"
                            );
                        }
                    }
                    pub mod trainingjob {
                        pub mod definition {
                            #[cfg(
                                any(
                                    feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
                                )
                            )]
                            include_proto!(
                                "google.cloud.aiplatform.v1.schema.trainingjob.definition"
                            );
                        }
                    }
                }
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-aiplatform-v1beta1"))]
                include_proto!("google.cloud.aiplatform.v1beta1");
                pub mod schema {
                    #[cfg(any(feature = "google-cloud-aiplatform-v1beta1-schema"))]
                    include_proto!("google.cloud.aiplatform.v1beta1.schema");
                    pub mod predict {
                        pub mod instance {
                            #[cfg(
                                any(
                                    feature = "google-cloud-aiplatform-v1beta1-schema",
                                    feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
                                )
                            )]
                            include_proto!(
                                "google.cloud.aiplatform.v1beta1.schema.predict.instance"
                            );
                        }
                        pub mod params {
                            #[cfg(
                                any(
                                    feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
                                )
                            )]
                            include_proto!(
                                "google.cloud.aiplatform.v1beta1.schema.predict.params"
                            );
                        }
                        pub mod prediction {
                            #[cfg(
                                any(
                                    feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
                                )
                            )]
                            include_proto!(
                                "google.cloud.aiplatform.v1beta1.schema.predict.prediction"
                            );
                        }
                    }
                    pub mod trainingjob {
                        pub mod definition {
                            #[cfg(
                                any(
                                    feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
                                )
                            )]
                            include_proto!(
                                "google.cloud.aiplatform.v1beta1.schema.trainingjob.definition"
                            );
                        }
                    }
                }
            }
        }
        pub mod apigateway {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-apigateway-v1"))]
                include_proto!("google.cloud.apigateway.v1");
            }
        }
        pub mod apigeeconnect {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-apigeeconnect-v1"))]
                include_proto!("google.cloud.apigeeconnect.v1");
            }
        }
        pub mod apigeeregistry {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-apigeeregistry-v1"))]
                include_proto!("google.cloud.apigeeregistry.v1");
            }
        }
        pub mod asset {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-asset-v1"))]
                include_proto!("google.cloud.asset.v1");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google-cloud-asset-v1p1beta1"))]
                include_proto!("google.cloud.asset.v1p1beta1");
            }
            pub mod v1p2beta1 {
                #[cfg(any(feature = "google-cloud-asset-v1p2beta1"))]
                include_proto!("google.cloud.asset.v1p2beta1");
            }
            pub mod v1p4beta1 {
                #[cfg(any(feature = "google-cloud-asset-v1p4beta1"))]
                include_proto!("google.cloud.asset.v1p4beta1");
            }
            pub mod v1p5beta1 {
                #[cfg(any(feature = "google-cloud-asset-v1p5beta1"))]
                include_proto!("google.cloud.asset.v1p5beta1");
            }
            pub mod v1p7beta1 {
                #[cfg(any(feature = "google-cloud-asset-v1p7beta1"))]
                include_proto!("google.cloud.asset.v1p7beta1");
            }
        }
        pub mod assuredworkloads {
            pub mod regulatoryintercept {
                pub mod logging {
                    pub mod v1 {
                        #[cfg(
                            any(
                                feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
                            )
                        )]
                        include_proto!(
                            "google.cloud.assuredworkloads.regulatoryintercept.logging.v1"
                        );
                    }
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-assuredworkloads-v1"))]
                include_proto!("google.cloud.assuredworkloads.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-assuredworkloads-v1beta1"))]
                include_proto!("google.cloud.assuredworkloads.v1beta1");
            }
        }
        pub mod audit {
            #[cfg(any(feature = "google-cloud-audit"))]
            include_proto!("google.cloud.audit");
        }
        pub mod automl {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-automl-v1"))]
                include_proto!("google.cloud.automl.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-automl-v1beta1"))]
                include_proto!("google.cloud.automl.v1beta1");
            }
        }
        pub mod baremetalsolution {
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-baremetalsolution-v2"))]
                include_proto!("google.cloud.baremetalsolution.v2");
            }
        }
        pub mod batch {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-batch-v1"))]
                include_proto!("google.cloud.batch.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google-cloud-batch-v1alpha"))]
                include_proto!("google.cloud.batch.v1alpha");
            }
        }
        pub mod beyondcorp {
            pub mod appconnections {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-beyondcorp-appconnections-v1"))]
                    include_proto!("google.cloud.beyondcorp.appconnections.v1");
                }
            }
            pub mod appconnectors {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-beyondcorp-appconnectors-v1"))]
                    include_proto!("google.cloud.beyondcorp.appconnectors.v1");
                }
            }
            pub mod appgateways {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-beyondcorp-appgateways-v1"))]
                    include_proto!("google.cloud.beyondcorp.appgateways.v1");
                }
            }
            pub mod clientconnectorservices {
                pub mod v1 {
                    #[cfg(
                        any(
                            feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
                        )
                    )]
                    include_proto!("google.cloud.beyondcorp.clientconnectorservices.v1");
                }
            }
            pub mod clientgateways {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-beyondcorp-clientgateways-v1"))]
                    include_proto!("google.cloud.beyondcorp.clientgateways.v1");
                }
            }
        }
        pub mod bigquery {
            pub mod connection {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-bigquery-connection-v1"))]
                    include_proto!("google.cloud.bigquery.connection.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-bigquery-connection-v1beta1"))]
                    include_proto!("google.cloud.bigquery.connection.v1beta1");
                }
            }
            pub mod dataexchange {
                pub mod common {
                    #[cfg(
                        any(
                            feature = "google-cloud-bigquery-dataexchange-common",
                            feature = "google-cloud-bigquery-dataexchange-v1beta1",
                        )
                    )]
                    include_proto!("google.cloud.bigquery.dataexchange.common");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-bigquery-dataexchange-v1beta1"))]
                    include_proto!("google.cloud.bigquery.dataexchange.v1beta1");
                }
            }
            pub mod datatransfer {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-bigquery-datatransfer-v1"))]
                    include_proto!("google.cloud.bigquery.datatransfer.v1");
                }
            }
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-bigquery-logging-v1"))]
                    include_proto!("google.cloud.bigquery.logging.v1");
                }
            }
            pub mod migration {
                pub mod v2 {
                    #[cfg(any(feature = "google-cloud-bigquery-migration-v2"))]
                    include_proto!("google.cloud.bigquery.migration.v2");
                }
                pub mod v2alpha {
                    #[cfg(any(feature = "google-cloud-bigquery-migration-v2alpha"))]
                    include_proto!("google.cloud.bigquery.migration.v2alpha");
                }
            }
            pub mod reservation {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-bigquery-reservation-v1"))]
                    include_proto!("google.cloud.bigquery.reservation.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-bigquery-reservation-v1beta1"))]
                    include_proto!("google.cloud.bigquery.reservation.v1beta1");
                }
            }
            pub mod storage {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-bigquery-storage-v1"))]
                    include_proto!("google.cloud.bigquery.storage.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-bigquery-storage-v1beta1"))]
                    include_proto!("google.cloud.bigquery.storage.v1beta1");
                }
                pub mod v1beta2 {
                    #[cfg(any(feature = "google-cloud-bigquery-storage-v1beta2"))]
                    include_proto!("google.cloud.bigquery.storage.v1beta2");
                }
            }
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-bigquery-v2"))]
                include_proto!("google.cloud.bigquery.v2");
            }
        }
        pub mod billing {
            pub mod budgets {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-billing-budgets-v1"))]
                    include_proto!("google.cloud.billing.budgets.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-billing-budgets-v1beta1"))]
                    include_proto!("google.cloud.billing.budgets.v1beta1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-billing-v1"))]
                include_proto!("google.cloud.billing.v1");
            }
        }
        pub mod binaryauthorization {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-binaryauthorization-v1"))]
                include_proto!("google.cloud.binaryauthorization.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-binaryauthorization-v1beta1"))]
                include_proto!("google.cloud.binaryauthorization.v1beta1");
            }
        }
        pub mod certificatemanager {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-certificatemanager-v1"))]
                include_proto!("google.cloud.certificatemanager.v1");
            }
        }
        pub mod channel {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-channel-v1"))]
                include_proto!("google.cloud.channel.v1");
            }
        }
        pub mod clouddms {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-clouddms-logging-v1"))]
                    include_proto!("google.cloud.clouddms.logging.v1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-clouddms-v1"))]
                include_proto!("google.cloud.clouddms.v1");
            }
        }
        pub mod commerce {
            pub mod consumer {
                pub mod procurement {
                    pub mod v1alpha1 {
                        #[cfg(
                            any(
                                feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
                            )
                        )]
                        include_proto!(
                            "google.cloud.commerce.consumer.procurement.v1alpha1"
                        );
                    }
                }
            }
        }
        pub mod common {
            #[cfg(
                any(
                    feature = "google-cloud-common",
                    feature = "google-cloud-filestore-v1",
                )
            )]
            include_proto!("google.cloud.common");
        }
        pub mod compute {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-compute-v1"))]
                include_proto!("google.cloud.compute.v1");
            }
            pub mod v1small {
                #[cfg(any(feature = "google-cloud-compute-v1small"))]
                include_proto!("google.cloud.compute.v1small");
            }
        }
        pub mod contactcenterinsights {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-contactcenterinsights-v1"))]
                include_proto!("google.cloud.contactcenterinsights.v1");
            }
        }
        pub mod datacatalog {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-datacatalog-v1"))]
                include_proto!("google.cloud.datacatalog.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-datacatalog-v1beta1"))]
                include_proto!("google.cloud.datacatalog.v1beta1");
            }
        }
        pub mod dataform {
            pub mod v1alpha2 {
                #[cfg(any(feature = "google-cloud-dataform-v1alpha2"))]
                include_proto!("google.cloud.dataform.v1alpha2");
            }
        }
        pub mod datafusion {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-datafusion-v1"))]
                include_proto!("google.cloud.datafusion.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-datafusion-v1beta1"))]
                include_proto!("google.cloud.datafusion.v1beta1");
            }
        }
        pub mod datalabeling {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-datalabeling-v1beta1"))]
                include_proto!("google.cloud.datalabeling.v1beta1");
            }
        }
        pub mod dataplex {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-dataplex-v1"))]
                include_proto!("google.cloud.dataplex.v1");
            }
        }
        pub mod dataproc {
            pub mod logging {
                #[cfg(any(feature = "google-cloud-dataproc-logging"))]
                include_proto!("google.cloud.dataproc.logging");
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-dataproc-v1"))]
                include_proto!("google.cloud.dataproc.v1");
            }
        }
        pub mod dataqna {
            pub mod v1alpha {
                #[cfg(any(feature = "google-cloud-dataqna-v1alpha"))]
                include_proto!("google.cloud.dataqna.v1alpha");
            }
        }
        pub mod datastream {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-datastream-v1"))]
                include_proto!("google.cloud.datastream.v1");
            }
            pub mod v1alpha1 {
                #[cfg(any(feature = "google-cloud-datastream-v1alpha1"))]
                include_proto!("google.cloud.datastream.v1alpha1");
            }
        }
        pub mod deploy {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-deploy-v1"))]
                include_proto!("google.cloud.deploy.v1");
            }
        }
        pub mod dialogflow {
            pub mod cx {
                pub mod v3 {
                    #[cfg(any(feature = "google-cloud-dialogflow-cx-v3"))]
                    include_proto!("google.cloud.dialogflow.cx.v3");
                }
                pub mod v3beta1 {
                    #[cfg(any(feature = "google-cloud-dialogflow-cx-v3beta1"))]
                    include_proto!("google.cloud.dialogflow.cx.v3beta1");
                }
            }
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-dialogflow-v2"))]
                include_proto!("google.cloud.dialogflow.v2");
            }
            pub mod v2beta1 {
                #[cfg(any(feature = "google-cloud-dialogflow-v2beta1"))]
                include_proto!("google.cloud.dialogflow.v2beta1");
            }
        }
        pub mod documentai {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-documentai-v1"))]
                include_proto!("google.cloud.documentai.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-documentai-v1beta1"))]
                include_proto!("google.cloud.documentai.v1beta1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google-cloud-documentai-v1beta2"))]
                include_proto!("google.cloud.documentai.v1beta2");
            }
            pub mod v1beta3 {
                #[cfg(any(feature = "google-cloud-documentai-v1beta3"))]
                include_proto!("google.cloud.documentai.v1beta3");
            }
        }
        pub mod domains {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-domains-v1"))]
                include_proto!("google.cloud.domains.v1");
            }
            pub mod v1alpha2 {
                #[cfg(any(feature = "google-cloud-domains-v1alpha2"))]
                include_proto!("google.cloud.domains.v1alpha2");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-domains-v1beta1"))]
                include_proto!("google.cloud.domains.v1beta1");
            }
        }
        pub mod essentialcontacts {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-essentialcontacts-v1"))]
                include_proto!("google.cloud.essentialcontacts.v1");
            }
        }
        pub mod eventarc {
            pub mod publishing {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-eventarc-publishing-v1"))]
                    include_proto!("google.cloud.eventarc.publishing.v1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-eventarc-v1"))]
                include_proto!("google.cloud.eventarc.v1");
            }
        }
        pub mod filestore {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-filestore-v1"))]
                include_proto!("google.cloud.filestore.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-filestore-v1beta1"))]
                include_proto!("google.cloud.filestore.v1beta1");
            }
        }
        pub mod functions {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-functions-v1"))]
                include_proto!("google.cloud.functions.v1");
            }
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-functions-v2"))]
                include_proto!("google.cloud.functions.v2");
            }
            pub mod v2alpha {
                #[cfg(any(feature = "google-cloud-functions-v2alpha"))]
                include_proto!("google.cloud.functions.v2alpha");
            }
            pub mod v2beta {
                #[cfg(any(feature = "google-cloud-functions-v2beta"))]
                include_proto!("google.cloud.functions.v2beta");
            }
        }
        pub mod gaming {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-gaming-v1"))]
                include_proto!("google.cloud.gaming.v1");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-cloud-gaming-v1beta"))]
                include_proto!("google.cloud.gaming.v1beta");
            }
        }
        pub mod gkebackup {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-gkebackup-logging-v1"))]
                    include_proto!("google.cloud.gkebackup.logging.v1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-gkebackup-v1"))]
                include_proto!("google.cloud.gkebackup.v1");
            }
        }
        pub mod gkeconnect {
            pub mod gateway {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-gkeconnect-gateway-v1"))]
                    include_proto!("google.cloud.gkeconnect.gateway.v1");
                }
                pub mod v1alpha1 {
                    #[cfg(any(feature = "google-cloud-gkeconnect-gateway-v1alpha1"))]
                    include_proto!("google.cloud.gkeconnect.gateway.v1alpha1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-gkeconnect-gateway-v1beta1"))]
                    include_proto!("google.cloud.gkeconnect.gateway.v1beta1");
                }
            }
        }
        pub mod gkehub {
            pub mod cloudauditlogging {
                pub mod v1alpha {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
                            feature = "google-cloud-gkehub-v1alpha",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.cloudauditlogging.v1alpha");
                }
            }
            pub mod configmanagement {
                pub mod v1 {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-configmanagement-v1",
                            feature = "google-cloud-gkehub-v1",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.configmanagement.v1");
                }
                pub mod v1alpha {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-configmanagement-v1alpha",
                            feature = "google-cloud-gkehub-v1alpha",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.configmanagement.v1alpha");
                }
                pub mod v1beta {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-configmanagement-v1beta",
                            feature = "google-cloud-gkehub-v1beta",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.configmanagement.v1beta");
                }
            }
            pub mod metering {
                pub mod v1alpha {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-metering-v1alpha",
                            feature = "google-cloud-gkehub-v1alpha",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.metering.v1alpha");
                }
                pub mod v1beta {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-metering-v1beta",
                            feature = "google-cloud-gkehub-v1beta",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.metering.v1beta");
                }
            }
            pub mod multiclusteringress {
                pub mod v1 {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-multiclusteringress-v1",
                            feature = "google-cloud-gkehub-v1",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.multiclusteringress.v1");
                }
                pub mod v1alpha {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
                            feature = "google-cloud-gkehub-v1alpha",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.multiclusteringress.v1alpha");
                }
                pub mod v1beta {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-multiclusteringress-v1beta",
                            feature = "google-cloud-gkehub-v1beta",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.multiclusteringress.v1beta");
                }
            }
            pub mod servicemesh {
                pub mod v1alpha {
                    #[cfg(
                        any(
                            feature = "google-cloud-gkehub-servicemesh-v1alpha",
                            feature = "google-cloud-gkehub-v1alpha",
                        )
                    )]
                    include_proto!("google.cloud.gkehub.servicemesh.v1alpha");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-gkehub-v1"))]
                include_proto!("google.cloud.gkehub.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google-cloud-gkehub-v1alpha"))]
                include_proto!("google.cloud.gkehub.v1alpha");
            }
            pub mod v1alpha2 {
                #[cfg(any(feature = "google-cloud-gkehub-v1alpha2"))]
                include_proto!("google.cloud.gkehub.v1alpha2");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-cloud-gkehub-v1beta"))]
                include_proto!("google.cloud.gkehub.v1beta");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-gkehub-v1beta1"))]
                include_proto!("google.cloud.gkehub.v1beta1");
            }
        }
        pub mod gkemulticloud {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-gkemulticloud-v1"))]
                include_proto!("google.cloud.gkemulticloud.v1");
            }
        }
        pub mod gsuiteaddons {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-gsuiteaddons-logging-v1"))]
                    include_proto!("google.cloud.gsuiteaddons.logging.v1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-gsuiteaddons-v1"))]
                include_proto!("google.cloud.gsuiteaddons.v1");
            }
        }
        pub mod iap {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-iap-v1"))]
                include_proto!("google.cloud.iap.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-iap-v1beta1"))]
                include_proto!("google.cloud.iap.v1beta1");
            }
        }
        pub mod identitytoolkit {
            pub mod logging {
                #[cfg(any(feature = "google-cloud-identitytoolkit-logging"))]
                include_proto!("google.cloud.identitytoolkit.logging");
            }
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-identitytoolkit-v2"))]
                include_proto!("google.cloud.identitytoolkit.v2");
            }
        }
        pub mod ids {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-ids-v1"))]
                include_proto!("google.cloud.ids.v1");
            }
        }
        pub mod integrations {
            pub mod v1alpha {
                #[cfg(any(feature = "google-cloud-integrations-v1alpha"))]
                include_proto!("google.cloud.integrations.v1alpha");
            }
        }
        pub mod iot {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-iot-v1"))]
                include_proto!("google.cloud.iot.v1");
            }
        }
        pub mod kms {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-kms-v1"))]
                include_proto!("google.cloud.kms.v1");
            }
        }
        pub mod language {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-language-v1"))]
                include_proto!("google.cloud.language.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-language-v1beta1"))]
                include_proto!("google.cloud.language.v1beta1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google-cloud-language-v1beta2"))]
                include_proto!("google.cloud.language.v1beta2");
            }
        }
        pub mod lifesciences {
            pub mod v2beta {
                #[cfg(any(feature = "google-cloud-lifesciences-v2beta"))]
                include_proto!("google.cloud.lifesciences.v2beta");
            }
        }
        pub mod location {
            #[cfg(any(feature = "google-cloud-location"))]
            include_proto!("google.cloud.location");
        }
        pub mod managedidentities {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-managedidentities-v1"))]
                include_proto!("google.cloud.managedidentities.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-managedidentities-v1beta1"))]
                include_proto!("google.cloud.managedidentities.v1beta1");
            }
        }
        pub mod mediatranslation {
            pub mod v1alpha1 {
                #[cfg(any(feature = "google-cloud-mediatranslation-v1alpha1"))]
                include_proto!("google.cloud.mediatranslation.v1alpha1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-mediatranslation-v1beta1"))]
                include_proto!("google.cloud.mediatranslation.v1beta1");
            }
        }
        pub mod memcache {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-memcache-v1"))]
                include_proto!("google.cloud.memcache.v1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google-cloud-memcache-v1beta2"))]
                include_proto!("google.cloud.memcache.v1beta2");
            }
        }
        pub mod metastore {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-metastore-logging-v1"))]
                    include_proto!("google.cloud.metastore.logging.v1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-metastore-v1"))]
                include_proto!("google.cloud.metastore.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google-cloud-metastore-v1alpha"))]
                include_proto!("google.cloud.metastore.v1alpha");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-cloud-metastore-v1beta"))]
                include_proto!("google.cloud.metastore.v1beta");
            }
        }
        pub mod networkanalyzer {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-networkanalyzer-logging-v1"))]
                    include_proto!("google.cloud.networkanalyzer.logging.v1");
                }
            }
        }
        pub mod networkconnectivity {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-networkconnectivity-v1"))]
                include_proto!("google.cloud.networkconnectivity.v1");
            }
            pub mod v1alpha1 {
                #[cfg(any(feature = "google-cloud-networkconnectivity-v1alpha1"))]
                include_proto!("google.cloud.networkconnectivity.v1alpha1");
            }
        }
        pub mod networkmanagement {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-networkmanagement-v1"))]
                include_proto!("google.cloud.networkmanagement.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-networkmanagement-v1beta1"))]
                include_proto!("google.cloud.networkmanagement.v1beta1");
            }
        }
        pub mod networksecurity {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-networksecurity-v1"))]
                include_proto!("google.cloud.networksecurity.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-networksecurity-v1beta1"))]
                include_proto!("google.cloud.networksecurity.v1beta1");
            }
        }
        pub mod networkservices {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-networkservices-v1"))]
                include_proto!("google.cloud.networkservices.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-networkservices-v1beta1"))]
                include_proto!("google.cloud.networkservices.v1beta1");
            }
        }
        pub mod notebooks {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-notebooks-logging-v1"))]
                    include_proto!("google.cloud.notebooks.logging.v1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-notebooks-v1"))]
                include_proto!("google.cloud.notebooks.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-notebooks-v1beta1"))]
                include_proto!("google.cloud.notebooks.v1beta1");
            }
        }
        pub mod optimization {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-optimization-v1"))]
                include_proto!("google.cloud.optimization.v1");
            }
        }
        pub mod orchestration {
            pub mod airflow {
                pub mod service {
                    pub mod v1 {
                        #[cfg(
                            any(
                                feature = "google-cloud-orchestration-airflow-service-v1",
                            )
                        )]
                        include_proto!("google.cloud.orchestration.airflow.service.v1");
                    }
                    pub mod v1beta1 {
                        #[cfg(
                            any(
                                feature = "google-cloud-orchestration-airflow-service-v1beta1",
                            )
                        )]
                        include_proto!(
                            "google.cloud.orchestration.airflow.service.v1beta1"
                        );
                    }
                }
            }
        }
        pub mod orgpolicy {
            pub mod v1 {
                #[cfg(
                    any(
                        feature = "google-cloud-asset-v1",
                        feature = "google-cloud-asset-v1p5beta1",
                        feature = "google-cloud-asset-v1p7beta1",
                        feature = "google-cloud-orgpolicy-v1",
                    )
                )]
                include_proto!("google.cloud.orgpolicy.v1");
            }
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-orgpolicy-v2"))]
                include_proto!("google.cloud.orgpolicy.v2");
            }
        }
        pub mod osconfig {
            pub mod agentendpoint {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-osconfig-agentendpoint-v1"))]
                    include_proto!("google.cloud.osconfig.agentendpoint.v1");
                }
                pub mod v1beta {
                    #[cfg(any(feature = "google-cloud-osconfig-agentendpoint-v1beta"))]
                    include_proto!("google.cloud.osconfig.agentendpoint.v1beta");
                }
            }
            pub mod v1 {
                #[cfg(
                    any(
                        feature = "google-cloud-asset-v1",
                        feature = "google-cloud-asset-v1p7beta1",
                        feature = "google-cloud-osconfig-v1",
                    )
                )]
                include_proto!("google.cloud.osconfig.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google-cloud-osconfig-v1alpha"))]
                include_proto!("google.cloud.osconfig.v1alpha");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-cloud-osconfig-v1beta"))]
                include_proto!("google.cloud.osconfig.v1beta");
            }
        }
        pub mod oslogin {
            pub mod common {
                #[cfg(
                    any(
                        feature = "google-cloud-oslogin-common",
                        feature = "google-cloud-oslogin-v1",
                        feature = "google-cloud-oslogin-v1alpha",
                        feature = "google-cloud-oslogin-v1beta",
                    )
                )]
                include_proto!("google.cloud.oslogin.common");
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-oslogin-v1"))]
                include_proto!("google.cloud.oslogin.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google-cloud-oslogin-v1alpha"))]
                include_proto!("google.cloud.oslogin.v1alpha");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-cloud-oslogin-v1beta"))]
                include_proto!("google.cloud.oslogin.v1beta");
            }
        }
        pub mod paymentgateway {
            pub mod issuerswitch {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-paymentgateway-issuerswitch-v1"))]
                    include_proto!("google.cloud.paymentgateway.issuerswitch.v1");
                }
            }
        }
        pub mod phishingprotection {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-phishingprotection-v1beta1"))]
                include_proto!("google.cloud.phishingprotection.v1beta1");
            }
        }
        pub mod policytroubleshooter {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-policytroubleshooter-v1"))]
                include_proto!("google.cloud.policytroubleshooter.v1");
            }
        }
        pub mod privatecatalog {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-privatecatalog-v1beta1"))]
                include_proto!("google.cloud.privatecatalog.v1beta1");
            }
        }
        pub mod pubsublite {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-pubsublite-v1"))]
                include_proto!("google.cloud.pubsublite.v1");
            }
        }
        pub mod recaptchaenterprise {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-recaptchaenterprise-v1"))]
                include_proto!("google.cloud.recaptchaenterprise.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-recaptchaenterprise-v1beta1"))]
                include_proto!("google.cloud.recaptchaenterprise.v1beta1");
            }
        }
        pub mod recommendationengine {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-recommendationengine-v1beta1"))]
                include_proto!("google.cloud.recommendationengine.v1beta1");
            }
        }
        pub mod recommender {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-recommender-logging-v1"))]
                    include_proto!("google.cloud.recommender.logging.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-recommender-logging-v1beta1"))]
                    include_proto!("google.cloud.recommender.logging.v1beta1");
                }
            }
            pub mod v1 {
                #[cfg(
                    any(
                        feature = "google-cloud-recommender-logging-v1",
                        feature = "google-cloud-recommender-v1",
                    )
                )]
                include_proto!("google.cloud.recommender.v1");
            }
            pub mod v1beta1 {
                #[cfg(
                    any(
                        feature = "google-cloud-recommender-logging-v1beta1",
                        feature = "google-cloud-recommender-v1beta1",
                    )
                )]
                include_proto!("google.cloud.recommender.v1beta1");
            }
        }
        pub mod redis {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-redis-v1"))]
                include_proto!("google.cloud.redis.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-redis-v1beta1"))]
                include_proto!("google.cloud.redis.v1beta1");
            }
        }
        pub mod resourcemanager {
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-resourcemanager-v2"))]
                include_proto!("google.cloud.resourcemanager.v2");
            }
            pub mod v3 {
                #[cfg(any(feature = "google-cloud-resourcemanager-v3"))]
                include_proto!("google.cloud.resourcemanager.v3");
            }
        }
        pub mod resourcesettings {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-resourcesettings-v1"))]
                include_proto!("google.cloud.resourcesettings.v1");
            }
        }
        pub mod retail {
            pub mod logging {
                #[cfg(any(feature = "google-cloud-retail-logging"))]
                include_proto!("google.cloud.retail.logging");
            }
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-retail-v2"))]
                include_proto!("google.cloud.retail.v2");
            }
            pub mod v2alpha {
                #[cfg(any(feature = "google-cloud-retail-v2alpha"))]
                include_proto!("google.cloud.retail.v2alpha");
            }
            pub mod v2beta {
                #[cfg(any(feature = "google-cloud-retail-v2beta"))]
                include_proto!("google.cloud.retail.v2beta");
            }
        }
        pub mod run {
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-run-v2"))]
                include_proto!("google.cloud.run.v2");
            }
        }
        pub mod runtimeconfig {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-runtimeconfig-v1beta1"))]
                include_proto!("google.cloud.runtimeconfig.v1beta1");
            }
        }
        pub mod saasaccelerator {
            pub mod management {
                pub mod logs {
                    pub mod v1 {
                        #[cfg(
                            any(
                                feature = "google-cloud-saasaccelerator-management-logs-v1",
                            )
                        )]
                        include_proto!(
                            "google.cloud.saasaccelerator.management.logs.v1"
                        );
                    }
                }
            }
        }
        pub mod scheduler {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-scheduler-v1"))]
                include_proto!("google.cloud.scheduler.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-scheduler-v1beta1"))]
                include_proto!("google.cloud.scheduler.v1beta1");
            }
        }
        pub mod secretmanager {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-secretmanager-logging-v1"))]
                    include_proto!("google.cloud.secretmanager.logging.v1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-secretmanager-v1"))]
                include_proto!("google.cloud.secretmanager.v1");
            }
        }
        pub mod secrets {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-secrets-v1beta1"))]
                include_proto!("google.cloud.secrets.v1beta1");
            }
        }
        pub mod security {
            pub mod privateca {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-security-privateca-v1"))]
                    include_proto!("google.cloud.security.privateca.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-security-privateca-v1beta1"))]
                    include_proto!("google.cloud.security.privateca.v1beta1");
                }
            }
            pub mod publicca {
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-security-publicca-v1beta1"))]
                    include_proto!("google.cloud.security.publicca.v1beta1");
                }
            }
        }
        pub mod securitycenter {
            pub mod settings {
                pub mod v1beta1 {
                    #[cfg(any(feature = "google-cloud-securitycenter-settings-v1beta1"))]
                    include_proto!("google.cloud.securitycenter.settings.v1beta1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-securitycenter-v1"))]
                include_proto!("google.cloud.securitycenter.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-securitycenter-v1beta1"))]
                include_proto!("google.cloud.securitycenter.v1beta1");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google-cloud-securitycenter-v1p1beta1"))]
                include_proto!("google.cloud.securitycenter.v1p1beta1");
            }
        }
        pub mod servicedirectory {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-servicedirectory-v1"))]
                include_proto!("google.cloud.servicedirectory.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-servicedirectory-v1beta1"))]
                include_proto!("google.cloud.servicedirectory.v1beta1");
            }
        }
        pub mod shell {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-shell-v1"))]
                include_proto!("google.cloud.shell.v1");
            }
        }
        pub mod speech {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-speech-v1"))]
                include_proto!("google.cloud.speech.v1");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google-cloud-speech-v1p1beta1"))]
                include_proto!("google.cloud.speech.v1p1beta1");
            }
        }
        pub mod sql {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-sql-v1"))]
                include_proto!("google.cloud.sql.v1");
            }
            pub mod v1beta4 {
                #[cfg(any(feature = "google-cloud-sql-v1beta4"))]
                include_proto!("google.cloud.sql.v1beta4");
            }
        }
        pub mod stream {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-stream-logging-v1"))]
                    include_proto!("google.cloud.stream.logging.v1");
                }
            }
        }
        pub mod support {
            pub mod common {
                #[cfg(
                    any(
                        feature = "google-cloud-support-common",
                        feature = "google-cloud-support-v1alpha1",
                    )
                )]
                include_proto!("google.cloud.support.common");
            }
            pub mod v1alpha1 {
                #[cfg(any(feature = "google-cloud-support-v1alpha1"))]
                include_proto!("google.cloud.support.v1alpha1");
            }
        }
        pub mod talent {
            pub mod v4 {
                #[cfg(any(feature = "google-cloud-talent-v4"))]
                include_proto!("google.cloud.talent.v4");
            }
            pub mod v4beta1 {
                #[cfg(any(feature = "google-cloud-talent-v4beta1"))]
                include_proto!("google.cloud.talent.v4beta1");
            }
        }
        pub mod tasks {
            pub mod v2 {
                #[cfg(any(feature = "google-cloud-tasks-v2"))]
                include_proto!("google.cloud.tasks.v2");
            }
            pub mod v2beta2 {
                #[cfg(any(feature = "google-cloud-tasks-v2beta2"))]
                include_proto!("google.cloud.tasks.v2beta2");
            }
            pub mod v2beta3 {
                #[cfg(any(feature = "google-cloud-tasks-v2beta3"))]
                include_proto!("google.cloud.tasks.v2beta3");
            }
        }
        pub mod texttospeech {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-texttospeech-v1"))]
                include_proto!("google.cloud.texttospeech.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-texttospeech-v1beta1"))]
                include_proto!("google.cloud.texttospeech.v1beta1");
            }
        }
        pub mod tpu {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-tpu-v1"))]
                include_proto!("google.cloud.tpu.v1");
            }
            pub mod v2alpha1 {
                #[cfg(any(feature = "google-cloud-tpu-v2alpha1"))]
                include_proto!("google.cloud.tpu.v2alpha1");
            }
        }
        pub mod translation {
            pub mod v3 {
                #[cfg(any(feature = "google-cloud-translation-v3"))]
                include_proto!("google.cloud.translation.v3");
            }
            pub mod v3beta1 {
                #[cfg(any(feature = "google-cloud-translation-v3beta1"))]
                include_proto!("google.cloud.translation.v3beta1");
            }
        }
        pub mod video {
            pub mod livestream {
                pub mod logging {
                    pub mod v1 {
                        #[cfg(any(feature = "google-cloud-video-livestream-logging-v1"))]
                        include_proto!("google.cloud.video.livestream.logging.v1");
                    }
                }
                pub mod v1 {
                    #[cfg(
                        any(
                            feature = "google-cloud-video-livestream-logging-v1",
                            feature = "google-cloud-video-livestream-v1",
                        )
                    )]
                    include_proto!("google.cloud.video.livestream.v1");
                }
            }
            pub mod stitcher {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-video-stitcher-v1"))]
                    include_proto!("google.cloud.video.stitcher.v1");
                }
            }
            pub mod transcoder {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-video-transcoder-v1"))]
                    include_proto!("google.cloud.video.transcoder.v1");
                }
            }
        }
        pub mod videointelligence {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-videointelligence-v1"))]
                include_proto!("google.cloud.videointelligence.v1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google-cloud-videointelligence-v1beta2"))]
                include_proto!("google.cloud.videointelligence.v1beta2");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google-cloud-videointelligence-v1p1beta1"))]
                include_proto!("google.cloud.videointelligence.v1p1beta1");
            }
            pub mod v1p2beta1 {
                #[cfg(any(feature = "google-cloud-videointelligence-v1p2beta1"))]
                include_proto!("google.cloud.videointelligence.v1p2beta1");
            }
            pub mod v1p3beta1 {
                #[cfg(any(feature = "google-cloud-videointelligence-v1p3beta1"))]
                include_proto!("google.cloud.videointelligence.v1p3beta1");
            }
        }
        pub mod vision {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-vision-v1"))]
                include_proto!("google.cloud.vision.v1");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google-cloud-vision-v1p1beta1"))]
                include_proto!("google.cloud.vision.v1p1beta1");
            }
            pub mod v1p2beta1 {
                #[cfg(any(feature = "google-cloud-vision-v1p2beta1"))]
                include_proto!("google.cloud.vision.v1p2beta1");
            }
            pub mod v1p3beta1 {
                #[cfg(any(feature = "google-cloud-vision-v1p3beta1"))]
                include_proto!("google.cloud.vision.v1p3beta1");
            }
            pub mod v1p4beta1 {
                #[cfg(any(feature = "google-cloud-vision-v1p4beta1"))]
                include_proto!("google.cloud.vision.v1p4beta1");
            }
        }
        pub mod vmmigration {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-vmmigration-v1"))]
                include_proto!("google.cloud.vmmigration.v1");
            }
        }
        pub mod vpcaccess {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-vpcaccess-v1"))]
                include_proto!("google.cloud.vpcaccess.v1");
            }
        }
        pub mod webrisk {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-webrisk-v1"))]
                include_proto!("google.cloud.webrisk.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-cloud-webrisk-v1beta1"))]
                include_proto!("google.cloud.webrisk.v1beta1");
            }
        }
        pub mod websecurityscanner {
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-websecurityscanner-v1"))]
                include_proto!("google.cloud.websecurityscanner.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google-cloud-websecurityscanner-v1alpha"))]
                include_proto!("google.cloud.websecurityscanner.v1alpha");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-cloud-websecurityscanner-v1beta"))]
                include_proto!("google.cloud.websecurityscanner.v1beta");
            }
        }
        pub mod workflows {
            pub mod executions {
                pub mod v1 {
                    #[cfg(any(feature = "google-cloud-workflows-executions-v1"))]
                    include_proto!("google.cloud.workflows.executions.v1");
                }
                pub mod v1beta {
                    #[cfg(any(feature = "google-cloud-workflows-executions-v1beta"))]
                    include_proto!("google.cloud.workflows.executions.v1beta");
                }
            }
            pub mod r#type {
                #[cfg(any(feature = "google-cloud-workflows-type"))]
                include_proto!("google.cloud.workflows.r#type");
            }
            pub mod v1 {
                #[cfg(any(feature = "google-cloud-workflows-v1"))]
                include_proto!("google.cloud.workflows.v1");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google-cloud-workflows-v1beta"))]
                include_proto!("google.cloud.workflows.v1beta");
            }
        }
    }
    pub mod container {
        pub mod v1 {
            #[cfg(any(feature = "google-container-v1"))]
            include_proto!("google.container.v1");
        }
        pub mod v1alpha1 {
            #[cfg(any(feature = "google-container-v1alpha1"))]
            include_proto!("google.container.v1alpha1");
        }
        pub mod v1beta1 {
            #[cfg(any(feature = "google-container-v1beta1"))]
            include_proto!("google.container.v1beta1");
        }
    }
    pub mod dataflow {
        pub mod v1beta3 {
            #[cfg(any(feature = "google-dataflow-v1beta3"))]
            include_proto!("google.dataflow.v1beta3");
        }
    }
    pub mod datastore {
        pub mod admin {
            pub mod v1 {
                #[cfg(any(feature = "google-datastore-admin-v1"))]
                include_proto!("google.datastore.admin.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-datastore-admin-v1beta1"))]
                include_proto!("google.datastore.admin.v1beta1");
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "google-datastore-v1"))]
            include_proto!("google.datastore.v1");
        }
        pub mod v1beta3 {
            #[cfg(any(feature = "google-datastore-v1beta3"))]
            include_proto!("google.datastore.v1beta3");
        }
    }
    pub mod devtools {
        pub mod artifactregistry {
            pub mod v1 {
                #[cfg(any(feature = "google-devtools-artifactregistry-v1"))]
                include_proto!("google.devtools.artifactregistry.v1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google-devtools-artifactregistry-v1beta2"))]
                include_proto!("google.devtools.artifactregistry.v1beta2");
            }
        }
        pub mod build {
            pub mod v1 {
                #[cfg(any(feature = "google-devtools-build-v1"))]
                include_proto!("google.devtools.build.v1");
            }
        }
        pub mod cloudbuild {
            pub mod v1 {
                #[cfg(any(feature = "google-devtools-cloudbuild-v1"))]
                include_proto!("google.devtools.cloudbuild.v1");
            }
        }
        pub mod clouddebugger {
            pub mod v2 {
                #[cfg(any(feature = "google-devtools-clouddebugger-v2"))]
                include_proto!("google.devtools.clouddebugger.v2");
            }
        }
        pub mod clouderrorreporting {
            pub mod v1beta1 {
                #[cfg(any(feature = "google-devtools-clouderrorreporting-v1beta1"))]
                include_proto!("google.devtools.clouderrorreporting.v1beta1");
            }
        }
        pub mod cloudprofiler {
            pub mod v2 {
                #[cfg(any(feature = "google-devtools-cloudprofiler-v2"))]
                include_proto!("google.devtools.cloudprofiler.v2");
            }
        }
        pub mod cloudtrace {
            pub mod v1 {
                #[cfg(any(feature = "google-devtools-cloudtrace-v1"))]
                include_proto!("google.devtools.cloudtrace.v1");
            }
            pub mod v2 {
                #[cfg(any(feature = "google-devtools-cloudtrace-v2"))]
                include_proto!("google.devtools.cloudtrace.v2");
            }
        }
        pub mod containeranalysis {
            pub mod v1 {
                #[cfg(any(feature = "google-devtools-containeranalysis-v1"))]
                include_proto!("google.devtools.containeranalysis.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-devtools-containeranalysis-v1beta1"))]
                include_proto!("google.devtools.containeranalysis.v1beta1");
            }
        }
        pub mod remoteworkers {
            pub mod v1test2 {
                #[cfg(any(feature = "google-devtools-remoteworkers-v1test2"))]
                include_proto!("google.devtools.remoteworkers.v1test2");
            }
        }
        pub mod resultstore {
            pub mod v2 {
                #[cfg(any(feature = "google-devtools-resultstore-v2"))]
                include_proto!("google.devtools.resultstore.v2");
            }
        }
        pub mod source {
            pub mod v1 {
                #[cfg(
                    any(
                        feature = "google-devtools-clouddebugger-v2",
                        feature = "google-devtools-source-v1",
                    )
                )]
                include_proto!("google.devtools.source.v1");
            }
        }
        pub mod sourcerepo {
            pub mod v1 {
                #[cfg(any(feature = "google-devtools-sourcerepo-v1"))]
                include_proto!("google.devtools.sourcerepo.v1");
            }
        }
        pub mod testing {
            pub mod v1 {
                #[cfg(any(feature = "google-devtools-testing-v1"))]
                include_proto!("google.devtools.testing.v1");
            }
        }
    }
    pub mod example {
        pub mod endpointsapis {
            pub mod v1 {
                #[cfg(any(feature = "google-example-endpointsapis-v1"))]
                include_proto!("google.example.endpointsapis.v1");
            }
        }
        pub mod library {
            pub mod v1 {
                #[cfg(any(feature = "google-example-library-v1"))]
                include_proto!("google.example.library.v1");
            }
        }
    }
    pub mod firebase {
        pub mod fcm {
            pub mod connection {
                pub mod v1alpha1 {
                    #[cfg(any(feature = "google-firebase-fcm-connection-v1alpha1"))]
                    include_proto!("google.firebase.fcm.connection.v1alpha1");
                }
            }
        }
    }
    pub mod firestore {
        pub mod admin {
            pub mod v1 {
                #[cfg(any(feature = "google-firestore-admin-v1"))]
                include_proto!("google.firestore.admin.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google-firestore-admin-v1beta1"))]
                include_proto!("google.firestore.admin.v1beta1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google-firestore-admin-v1beta2"))]
                include_proto!("google.firestore.admin.v1beta2");
            }
        }
        pub mod bundle {
            #[cfg(any(feature = "google-firestore-bundle"))]
            include_proto!("google.firestore.bundle");
        }
        pub mod v1 {
            #[cfg(
                any(feature = "google-firestore-bundle", feature = "google-firestore-v1")
            )]
            include_proto!("google.firestore.v1");
        }
        pub mod v1beta1 {
            #[cfg(any(feature = "google-firestore-v1beta1"))]
            include_proto!("google.firestore.v1beta1");
        }
    }
    pub mod gapic {
        pub mod metadata {
            #[cfg(any(feature = "google-gapic-metadata"))]
            include_proto!("google.gapic.metadata");
        }
    }
    pub mod genomics {
        pub mod v1 {
            #[cfg(any(feature = "google-genomics-v1"))]
            include_proto!("google.genomics.v1");
        }
        pub mod v1alpha2 {
            #[cfg(any(feature = "google-genomics-v1alpha2"))]
            include_proto!("google.genomics.v1alpha2");
        }
    }
    pub mod geo {
        pub mod r#type {
            #[cfg(
                any(
                    feature = "google-geo-type",
                    feature = "google-maps-routes-v1",
                    feature = "google-maps-routes-v1alpha",
                    feature = "google-maps-routing-v2",
                    feature = "maps-fleetengine-delivery-v1",
                )
            )]
            include_proto!("google.geo.r#type");
        }
    }
    pub mod home {
        pub mod enterprise {
            pub mod sdm {
                pub mod v1 {
                    #[cfg(any(feature = "google-home-enterprise-sdm-v1"))]
                    include_proto!("google.home.enterprise.sdm.v1");
                }
            }
        }
        pub mod graph {
            pub mod v1 {
                #[cfg(any(feature = "google-home-graph-v1"))]
                include_proto!("google.home.graph.v1");
            }
        }
    }
    pub mod iam {
        pub mod admin {
            pub mod v1 {
                #[cfg(any(feature = "google-iam-admin-v1"))]
                include_proto!("google.iam.admin.v1");
            }
        }
        pub mod credentials {
            pub mod v1 {
                #[cfg(any(feature = "google-iam-credentials-v1"))]
                include_proto!("google.iam.credentials.v1");
            }
        }
        pub mod v1 {
            #[cfg(
                any(
                    feature = "google-bigtable-admin-v2",
                    feature = "google-cloud-asset-v1",
                    feature = "google-cloud-asset-v1p1beta1",
                    feature = "google-cloud-asset-v1p2beta1",
                    feature = "google-cloud-asset-v1p4beta1",
                    feature = "google-cloud-asset-v1p5beta1",
                    feature = "google-cloud-asset-v1p7beta1",
                    feature = "google-cloud-audit",
                    feature = "google-cloud-bigquery-connection-v1",
                    feature = "google-cloud-bigquery-connection-v1beta1",
                    feature = "google-cloud-bigquery-dataexchange-v1beta1",
                    feature = "google-cloud-bigquery-logging-v1",
                    feature = "google-cloud-billing-v1",
                    feature = "google-cloud-datacatalog-v1",
                    feature = "google-cloud-datacatalog-v1beta1",
                    feature = "google-cloud-datafusion-v1beta1",
                    feature = "google-cloud-dataplex-v1",
                    feature = "google-cloud-functions-v1",
                    feature = "google-cloud-iap-v1",
                    feature = "google-cloud-iap-v1beta1",
                    feature = "google-cloud-iot-v1",
                    feature = "google-cloud-policytroubleshooter-v1",
                    feature = "google-cloud-resourcemanager-v2",
                    feature = "google-cloud-resourcemanager-v3",
                    feature = "google-cloud-run-v2",
                    feature = "google-cloud-secretmanager-v1",
                    feature = "google-cloud-secrets-v1beta1",
                    feature = "google-cloud-securitycenter-v1",
                    feature = "google-cloud-securitycenter-v1beta1",
                    feature = "google-cloud-securitycenter-v1p1beta1",
                    feature = "google-cloud-servicedirectory-v1",
                    feature = "google-cloud-servicedirectory-v1beta1",
                    feature = "google-cloud-tasks-v2",
                    feature = "google-cloud-tasks-v2beta2",
                    feature = "google-cloud-tasks-v2beta3",
                    feature = "google-devtools-artifactregistry-v1",
                    feature = "google-devtools-artifactregistry-v1beta2",
                    feature = "google-devtools-containeranalysis-v1",
                    feature = "google-devtools-containeranalysis-v1beta1",
                    feature = "google-devtools-sourcerepo-v1",
                    feature = "google-genomics-v1",
                    feature = "google-iam-admin-v1",
                    feature = "google-iam-v1",
                    feature = "google-iam-v1-logging",
                    feature = "google-spanner-admin-database-v1",
                    feature = "google-spanner-admin-instance-v1",
                    feature = "google-storage-v1",
                    feature = "google-storage-v2",
                )
            )]
            include_proto!("google.iam.v1");
            pub mod logging {
                #[cfg(any(feature = "google-iam-v1-logging"))]
                include_proto!("google.iam.v1.logging");
            }
        }
        pub mod v1beta {
            #[cfg(any(feature = "google-iam-v1beta"))]
            include_proto!("google.iam.v1beta");
        }
        pub mod v2beta {
            #[cfg(any(feature = "google-iam-v2beta"))]
            include_proto!("google.iam.v2beta");
        }
    }
    pub mod identity {
        pub mod accesscontextmanager {
            pub mod r#type {
                #[cfg(
                    any(
                        feature = "google-cloud-asset-v1",
                        feature = "google-cloud-asset-v1p5beta1",
                        feature = "google-cloud-asset-v1p7beta1",
                        feature = "google-identity-accesscontextmanager-type",
                        feature = "google-identity-accesscontextmanager-v1",
                    )
                )]
                include_proto!("google.identity.accesscontextmanager.r#type");
            }
            pub mod v1 {
                #[cfg(
                    any(
                        feature = "google-cloud-asset-v1",
                        feature = "google-cloud-asset-v1p5beta1",
                        feature = "google-cloud-asset-v1p7beta1",
                        feature = "google-identity-accesscontextmanager-v1",
                    )
                )]
                include_proto!("google.identity.accesscontextmanager.v1");
            }
        }
    }
    pub mod logging {
        pub mod r#type {
            #[cfg(
                any(
                    feature = "google-api-servicecontrol-v1",
                    feature = "google-appengine-logging-v1",
                    feature = "google-logging-type",
                    feature = "google-logging-v2",
                )
            )]
            include_proto!("google.logging.r#type");
        }
        pub mod v2 {
            #[cfg(any(feature = "google-logging-v2"))]
            include_proto!("google.logging.v2");
        }
    }
    pub mod longrunning {
        #[cfg(
            any(
                feature = "google-ads-googleads-v10-services",
                feature = "google-ads-googleads-v11-services",
                feature = "google-ads-googleads-v9-services",
                feature = "google-api-servicemanagement-v1",
                feature = "google-api-serviceusage-v1",
                feature = "google-api-serviceusage-v1beta1",
                feature = "google-appengine-v1",
                feature = "google-appengine-v1beta",
                feature = "google-bigtable-admin-v2",
                feature = "google-chromeos-moblab-v1beta1",
                feature = "google-cloud-aiplatform-v1",
                feature = "google-cloud-aiplatform-v1beta1",
                feature = "google-cloud-apigateway-v1",
                feature = "google-cloud-apigeeregistry-v1",
                feature = "google-cloud-asset-v1",
                feature = "google-cloud-asset-v1p2beta1",
                feature = "google-cloud-asset-v1p4beta1",
                feature = "google-cloud-asset-v1p7beta1",
                feature = "google-cloud-assuredworkloads-v1",
                feature = "google-cloud-assuredworkloads-v1beta1",
                feature = "google-cloud-automl-v1",
                feature = "google-cloud-automl-v1beta1",
                feature = "google-cloud-baremetalsolution-v2",
                feature = "google-cloud-batch-v1",
                feature = "google-cloud-batch-v1alpha",
                feature = "google-cloud-beyondcorp-appconnections-v1",
                feature = "google-cloud-beyondcorp-appconnectors-v1",
                feature = "google-cloud-beyondcorp-appgateways-v1",
                feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
                feature = "google-cloud-beyondcorp-clientgateways-v1",
                feature = "google-cloud-certificatemanager-v1",
                feature = "google-cloud-channel-v1",
                feature = "google-cloud-clouddms-v1",
                feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
                feature = "google-cloud-contactcenterinsights-v1",
                feature = "google-cloud-datafusion-v1",
                feature = "google-cloud-datafusion-v1beta1",
                feature = "google-cloud-datalabeling-v1beta1",
                feature = "google-cloud-dataplex-v1",
                feature = "google-cloud-dataproc-v1",
                feature = "google-cloud-datastream-v1",
                feature = "google-cloud-datastream-v1alpha1",
                feature = "google-cloud-deploy-v1",
                feature = "google-cloud-dialogflow-cx-v3",
                feature = "google-cloud-dialogflow-cx-v3beta1",
                feature = "google-cloud-dialogflow-v2",
                feature = "google-cloud-dialogflow-v2beta1",
                feature = "google-cloud-documentai-v1",
                feature = "google-cloud-documentai-v1beta1",
                feature = "google-cloud-documentai-v1beta2",
                feature = "google-cloud-documentai-v1beta3",
                feature = "google-cloud-domains-v1",
                feature = "google-cloud-domains-v1alpha2",
                feature = "google-cloud-domains-v1beta1",
                feature = "google-cloud-eventarc-v1",
                feature = "google-cloud-filestore-v1",
                feature = "google-cloud-filestore-v1beta1",
                feature = "google-cloud-functions-v1",
                feature = "google-cloud-functions-v2",
                feature = "google-cloud-functions-v2alpha",
                feature = "google-cloud-functions-v2beta",
                feature = "google-cloud-gaming-v1",
                feature = "google-cloud-gaming-v1beta",
                feature = "google-cloud-gkebackup-v1",
                feature = "google-cloud-gkehub-v1",
                feature = "google-cloud-gkehub-v1alpha",
                feature = "google-cloud-gkehub-v1alpha2",
                feature = "google-cloud-gkehub-v1beta",
                feature = "google-cloud-gkehub-v1beta1",
                feature = "google-cloud-gkemulticloud-v1",
                feature = "google-cloud-ids-v1",
                feature = "google-cloud-lifesciences-v2beta",
                feature = "google-cloud-managedidentities-v1",
                feature = "google-cloud-managedidentities-v1beta1",
                feature = "google-cloud-memcache-v1",
                feature = "google-cloud-memcache-v1beta2",
                feature = "google-cloud-metastore-v1",
                feature = "google-cloud-metastore-v1alpha",
                feature = "google-cloud-metastore-v1beta",
                feature = "google-cloud-networkconnectivity-v1",
                feature = "google-cloud-networkconnectivity-v1alpha1",
                feature = "google-cloud-networkmanagement-v1",
                feature = "google-cloud-networkmanagement-v1beta1",
                feature = "google-cloud-networksecurity-v1",
                feature = "google-cloud-networksecurity-v1beta1",
                feature = "google-cloud-networkservices-v1",
                feature = "google-cloud-networkservices-v1beta1",
                feature = "google-cloud-notebooks-v1",
                feature = "google-cloud-notebooks-v1beta1",
                feature = "google-cloud-optimization-v1",
                feature = "google-cloud-orchestration-airflow-service-v1",
                feature = "google-cloud-orchestration-airflow-service-v1beta1",
                feature = "google-cloud-osconfig-v1",
                feature = "google-cloud-osconfig-v1alpha",
                feature = "google-cloud-paymentgateway-issuerswitch-v1",
                feature = "google-cloud-privatecatalog-v1beta1",
                feature = "google-cloud-pubsublite-v1",
                feature = "google-cloud-recommendationengine-v1beta1",
                feature = "google-cloud-redis-v1",
                feature = "google-cloud-redis-v1beta1",
                feature = "google-cloud-resourcemanager-v2",
                feature = "google-cloud-resourcemanager-v3",
                feature = "google-cloud-retail-v2",
                feature = "google-cloud-retail-v2alpha",
                feature = "google-cloud-retail-v2beta",
                feature = "google-cloud-run-v2",
                feature = "google-cloud-runtimeconfig-v1beta1",
                feature = "google-cloud-security-privateca-v1",
                feature = "google-cloud-security-privateca-v1beta1",
                feature = "google-cloud-securitycenter-v1",
                feature = "google-cloud-securitycenter-v1beta1",
                feature = "google-cloud-securitycenter-v1p1beta1",
                feature = "google-cloud-shell-v1",
                feature = "google-cloud-speech-v1",
                feature = "google-cloud-speech-v1p1beta1",
                feature = "google-cloud-talent-v4",
                feature = "google-cloud-talent-v4beta1",
                feature = "google-cloud-tpu-v1",
                feature = "google-cloud-tpu-v2alpha1",
                feature = "google-cloud-translation-v3",
                feature = "google-cloud-translation-v3beta1",
                feature = "google-cloud-video-livestream-v1",
                feature = "google-cloud-videointelligence-v1",
                feature = "google-cloud-videointelligence-v1beta2",
                feature = "google-cloud-videointelligence-v1p1beta1",
                feature = "google-cloud-videointelligence-v1p2beta1",
                feature = "google-cloud-videointelligence-v1p3beta1",
                feature = "google-cloud-vision-v1",
                feature = "google-cloud-vision-v1p2beta1",
                feature = "google-cloud-vision-v1p3beta1",
                feature = "google-cloud-vision-v1p4beta1",
                feature = "google-cloud-vmmigration-v1",
                feature = "google-cloud-vpcaccess-v1",
                feature = "google-cloud-workflows-v1",
                feature = "google-cloud-workflows-v1beta",
                feature = "google-datastore-admin-v1",
                feature = "google-datastore-admin-v1beta1",
                feature = "google-devtools-artifactregistry-v1",
                feature = "google-devtools-artifactregistry-v1beta2",
                feature = "google-devtools-cloudbuild-v1",
                feature = "google-firestore-admin-v1",
                feature = "google-firestore-admin-v1beta1",
                feature = "google-firestore-admin-v1beta2",
                feature = "google-genomics-v1",
                feature = "google-genomics-v1alpha2",
                feature = "google-iam-v1beta",
                feature = "google-iam-v2beta",
                feature = "google-identity-accesscontextmanager-v1",
                feature = "google-logging-v2",
                feature = "google-longrunning",
                feature = "google-monitoring-metricsscope-v1",
                feature = "google-partner-aistreams-v1alpha1",
                feature = "google-spanner-admin-database-v1",
                feature = "google-spanner-admin-instance-v1",
                feature = "google-storagetransfer-v1",
                feature = "google-streetview-publish-v1",
            )
        )]
        include_proto!("google.longrunning");
    }
    pub mod maps {
        pub mod playablelocations {
            pub mod v3 {
                #[cfg(any(feature = "google-maps-playablelocations-v3"))]
                include_proto!("google.maps.playablelocations.v3");
                pub mod sample {
                    #[cfg(
                        any(
                            feature = "google-maps-playablelocations-v3",
                            feature = "google-maps-playablelocations-v3-sample",
                        )
                    )]
                    include_proto!("google.maps.playablelocations.v3.sample");
                }
            }
        }
        pub mod regionlookup {
            pub mod v1alpha {
                #[cfg(any(feature = "google-maps-regionlookup-v1alpha"))]
                include_proto!("google.maps.regionlookup.v1alpha");
            }
        }
        pub mod roads {
            pub mod v1op {
                #[cfg(any(feature = "google-maps-roads-v1op"))]
                include_proto!("google.maps.roads.v1op");
            }
        }
        pub mod routes {
            pub mod v1 {
                #[cfg(
                    any(
                        feature = "google-maps-routes-v1",
                        feature = "google-maps-routes-v1alpha",
                    )
                )]
                include_proto!("google.maps.routes.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google-maps-routes-v1alpha"))]
                include_proto!("google.maps.routes.v1alpha");
            }
        }
        pub mod routing {
            pub mod v2 {
                #[cfg(any(feature = "google-maps-routing-v2"))]
                include_proto!("google.maps.routing.v2");
            }
        }
        pub mod unity {
            #[cfg(
                any(
                    feature = "google-maps-playablelocations-v3",
                    feature = "google-maps-unity",
                )
            )]
            include_proto!("google.maps.unity");
        }
    }
    pub mod monitoring {
        pub mod dashboard {
            pub mod v1 {
                #[cfg(any(feature = "google-monitoring-dashboard-v1"))]
                include_proto!("google.monitoring.dashboard.v1");
            }
        }
        pub mod metricsscope {
            pub mod v1 {
                #[cfg(any(feature = "google-monitoring-metricsscope-v1"))]
                include_proto!("google.monitoring.metricsscope.v1");
            }
        }
        pub mod v3 {
            #[cfg(any(feature = "google-monitoring-v3"))]
            include_proto!("google.monitoring.v3");
        }
    }
    pub mod networking {
        pub mod trafficdirector {
            pub mod r#type {
                #[cfg(any(feature = "google-networking-trafficdirector-type"))]
                include_proto!("google.networking.trafficdirector.r#type");
            }
        }
    }
    pub mod partner {
        pub mod aistreams {
            pub mod v1alpha1 {
                #[cfg(any(feature = "google-partner-aistreams-v1alpha1"))]
                include_proto!("google.partner.aistreams.v1alpha1");
            }
        }
    }
    pub mod privacy {
        pub mod dlp {
            pub mod v2 {
                #[cfg(any(feature = "google-privacy-dlp-v2"))]
                include_proto!("google.privacy.dlp.v2");
            }
        }
    }
    pub mod pubsub {
        pub mod v1 {
            #[cfg(any(feature = "google-pubsub-v1"))]
            include_proto!("google.pubsub.v1");
        }
        pub mod v1beta2 {
            #[cfg(any(feature = "google-pubsub-v1beta2"))]
            include_proto!("google.pubsub.v1beta2");
        }
    }
    pub mod r#type {
        #[cfg(
            any(
                feature = "google-actions-sdk-v2",
                feature = "google-actions-type",
                feature = "google-ads-admob-v1",
                feature = "google-assistant-embedded-v1alpha2",
                feature = "google-bigtable-admin-v2",
                feature = "google-cloud-aiplatform-v1",
                feature = "google-cloud-aiplatform-v1beta1",
                feature = "google-cloud-aiplatform-v1beta1-schema",
                feature = "google-cloud-asset-v1",
                feature = "google-cloud-asset-v1p1beta1",
                feature = "google-cloud-asset-v1p2beta1",
                feature = "google-cloud-asset-v1p4beta1",
                feature = "google-cloud-asset-v1p5beta1",
                feature = "google-cloud-asset-v1p7beta1",
                feature = "google-cloud-audit",
                feature = "google-cloud-bigquery-connection-v1",
                feature = "google-cloud-bigquery-connection-v1beta1",
                feature = "google-cloud-bigquery-dataexchange-v1beta1",
                feature = "google-cloud-bigquery-logging-v1",
                feature = "google-cloud-billing-budgets-v1",
                feature = "google-cloud-billing-budgets-v1beta1",
                feature = "google-cloud-billing-v1",
                feature = "google-cloud-channel-v1",
                feature = "google-cloud-datacatalog-v1",
                feature = "google-cloud-datacatalog-v1beta1",
                feature = "google-cloud-dataform-v1alpha2",
                feature = "google-cloud-datafusion-v1beta1",
                feature = "google-cloud-dataplex-v1",
                feature = "google-cloud-deploy-v1",
                feature = "google-cloud-dialogflow-cx-v3",
                feature = "google-cloud-dialogflow-cx-v3beta1",
                feature = "google-cloud-dialogflow-v2",
                feature = "google-cloud-dialogflow-v2beta1",
                feature = "google-cloud-documentai-v1",
                feature = "google-cloud-documentai-v1beta1",
                feature = "google-cloud-documentai-v1beta2",
                feature = "google-cloud-documentai-v1beta3",
                feature = "google-cloud-domains-v1",
                feature = "google-cloud-domains-v1alpha2",
                feature = "google-cloud-domains-v1beta1",
                feature = "google-cloud-functions-v1",
                feature = "google-cloud-iap-v1",
                feature = "google-cloud-iap-v1beta1",
                feature = "google-cloud-iot-v1",
                feature = "google-cloud-metastore-v1",
                feature = "google-cloud-metastore-v1alpha",
                feature = "google-cloud-metastore-v1beta",
                feature = "google-cloud-optimization-v1",
                feature = "google-cloud-orchestration-airflow-service-v1",
                feature = "google-cloud-orchestration-airflow-service-v1beta1",
                feature = "google-cloud-orgpolicy-v2",
                feature = "google-cloud-osconfig-agentendpoint-v1",
                feature = "google-cloud-osconfig-v1",
                feature = "google-cloud-osconfig-v1alpha",
                feature = "google-cloud-osconfig-v1beta",
                feature = "google-cloud-paymentgateway-issuerswitch-v1",
                feature = "google-cloud-policytroubleshooter-v1",
                feature = "google-cloud-recommendationengine-v1beta1",
                feature = "google-cloud-recommender-logging-v1",
                feature = "google-cloud-recommender-logging-v1beta1",
                feature = "google-cloud-recommender-v1",
                feature = "google-cloud-recommender-v1beta1",
                feature = "google-cloud-redis-v1",
                feature = "google-cloud-redis-v1beta1",
                feature = "google-cloud-resourcemanager-v2",
                feature = "google-cloud-resourcemanager-v3",
                feature = "google-cloud-retail-v2",
                feature = "google-cloud-retail-v2alpha",
                feature = "google-cloud-retail-v2beta",
                feature = "google-cloud-run-v2",
                feature = "google-cloud-secretmanager-v1",
                feature = "google-cloud-secrets-v1beta1",
                feature = "google-cloud-security-privateca-v1",
                feature = "google-cloud-securitycenter-v1",
                feature = "google-cloud-securitycenter-v1beta1",
                feature = "google-cloud-securitycenter-v1p1beta1",
                feature = "google-cloud-servicedirectory-v1",
                feature = "google-cloud-servicedirectory-v1beta1",
                feature = "google-cloud-talent-v4",
                feature = "google-cloud-talent-v4beta1",
                feature = "google-cloud-tasks-v2",
                feature = "google-cloud-tasks-v2beta2",
                feature = "google-cloud-tasks-v2beta3",
                feature = "google-cloud-vision-v1",
                feature = "google-cloud-vision-v1p1beta1",
                feature = "google-cloud-vision-v1p2beta1",
                feature = "google-cloud-vision-v1p3beta1",
                feature = "google-cloud-vision-v1p4beta1",
                feature = "google-container-v1beta1",
                feature = "google-datastore-v1",
                feature = "google-datastore-v1beta3",
                feature = "google-devtools-artifactregistry-v1",
                feature = "google-devtools-artifactregistry-v1beta2",
                feature = "google-devtools-containeranalysis-v1",
                feature = "google-devtools-containeranalysis-v1beta1",
                feature = "google-devtools-sourcerepo-v1",
                feature = "google-devtools-testing-v1",
                feature = "google-firestore-admin-v1beta1",
                feature = "google-firestore-bundle",
                feature = "google-firestore-v1",
                feature = "google-firestore-v1beta1",
                feature = "google-genomics-v1",
                feature = "google-geo-type",
                feature = "google-iam-admin-v1",
                feature = "google-iam-v1",
                feature = "google-iam-v1-logging",
                feature = "google-iam-v2beta",
                feature = "google-identity-accesscontextmanager-v1",
                feature = "google-maps-playablelocations-v3",
                feature = "google-maps-playablelocations-v3-sample",
                feature = "google-maps-regionlookup-v1alpha",
                feature = "google-maps-roads-v1op",
                feature = "google-maps-routes-v1",
                feature = "google-maps-routes-v1alpha",
                feature = "google-maps-routing-v2",
                feature = "google-monitoring-v3",
                feature = "google-privacy-dlp-v2",
                feature = "google-spanner-admin-database-v1",
                feature = "google-spanner-admin-instance-v1",
                feature = "google-storage-v1",
                feature = "google-storage-v2",
                feature = "google-storagetransfer-v1",
                feature = "google-streetview-publish-v1",
                feature = "google-type",
                feature = "maps-fleetengine-delivery-v1",
                feature = "maps-fleetengine-v1",
            )
        )]
        include_proto!("google.r#type");
    }
    pub mod rpc {
        #[cfg(
            any(
                feature = "google-actions-sdk-v2",
                feature = "google-ads-googleads-v10-services",
                feature = "google-ads-googleads-v11-services",
                feature = "google-ads-googleads-v9-services",
                feature = "google-api-expr-conformance-v1alpha1",
                feature = "google-api-expr-v1alpha1",
                feature = "google-api-expr-v1beta1",
                feature = "google-api-servicecontrol-v1",
                feature = "google-api-servicecontrol-v2",
                feature = "google-api-servicemanagement-v1",
                feature = "google-api-serviceusage-v1",
                feature = "google-api-serviceusage-v1beta1",
                feature = "google-appengine-v1",
                feature = "google-appengine-v1beta",
                feature = "google-apps-alertcenter-v1beta1",
                feature = "google-assistant-embedded-v1alpha1",
                feature = "google-bigtable-admin-v2",
                feature = "google-bigtable-v2",
                feature = "google-chat-logging-v1",
                feature = "google-chromeos-moblab-v1beta1",
                feature = "google-cloud-aiplatform-logging",
                feature = "google-cloud-aiplatform-v1",
                feature = "google-cloud-aiplatform-v1beta1",
                feature = "google-cloud-aiplatform-v1beta1-schema",
                feature = "google-cloud-apigateway-v1",
                feature = "google-cloud-apigeeconnect-v1",
                feature = "google-cloud-apigeeregistry-v1",
                feature = "google-cloud-asset-v1",
                feature = "google-cloud-asset-v1p2beta1",
                feature = "google-cloud-asset-v1p4beta1",
                feature = "google-cloud-asset-v1p7beta1",
                feature = "google-cloud-assuredworkloads-v1",
                feature = "google-cloud-assuredworkloads-v1beta1",
                feature = "google-cloud-audit",
                feature = "google-cloud-automl-v1",
                feature = "google-cloud-automl-v1beta1",
                feature = "google-cloud-baremetalsolution-v2",
                feature = "google-cloud-batch-v1",
                feature = "google-cloud-batch-v1alpha",
                feature = "google-cloud-beyondcorp-appconnections-v1",
                feature = "google-cloud-beyondcorp-appconnectors-v1",
                feature = "google-cloud-beyondcorp-appgateways-v1",
                feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
                feature = "google-cloud-beyondcorp-clientgateways-v1",
                feature = "google-cloud-bigquery-datatransfer-v1",
                feature = "google-cloud-bigquery-logging-v1",
                feature = "google-cloud-bigquery-migration-v2",
                feature = "google-cloud-bigquery-migration-v2alpha",
                feature = "google-cloud-bigquery-reservation-v1",
                feature = "google-cloud-bigquery-reservation-v1beta1",
                feature = "google-cloud-bigquery-storage-v1",
                feature = "google-cloud-bigquery-storage-v1beta2",
                feature = "google-cloud-certificatemanager-v1",
                feature = "google-cloud-channel-v1",
                feature = "google-cloud-clouddms-logging-v1",
                feature = "google-cloud-clouddms-v1",
                feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
                feature = "google-cloud-contactcenterinsights-v1",
                feature = "google-cloud-datafusion-v1",
                feature = "google-cloud-datafusion-v1beta1",
                feature = "google-cloud-datalabeling-v1beta1",
                feature = "google-cloud-dataplex-v1",
                feature = "google-cloud-dataproc-v1",
                feature = "google-cloud-dataqna-v1alpha",
                feature = "google-cloud-datastream-v1",
                feature = "google-cloud-datastream-v1alpha1",
                feature = "google-cloud-deploy-v1",
                feature = "google-cloud-dialogflow-cx-v3",
                feature = "google-cloud-dialogflow-cx-v3beta1",
                feature = "google-cloud-dialogflow-v2",
                feature = "google-cloud-dialogflow-v2beta1",
                feature = "google-cloud-documentai-v1",
                feature = "google-cloud-documentai-v1beta1",
                feature = "google-cloud-documentai-v1beta2",
                feature = "google-cloud-documentai-v1beta3",
                feature = "google-cloud-domains-v1",
                feature = "google-cloud-domains-v1alpha2",
                feature = "google-cloud-domains-v1beta1",
                feature = "google-cloud-eventarc-v1",
                feature = "google-cloud-filestore-v1",
                feature = "google-cloud-filestore-v1beta1",
                feature = "google-cloud-functions-v1",
                feature = "google-cloud-functions-v2",
                feature = "google-cloud-functions-v2alpha",
                feature = "google-cloud-functions-v2beta",
                feature = "google-cloud-gaming-v1",
                feature = "google-cloud-gaming-v1beta",
                feature = "google-cloud-gkebackup-logging-v1",
                feature = "google-cloud-gkebackup-v1",
                feature = "google-cloud-gkehub-v1",
                feature = "google-cloud-gkehub-v1alpha",
                feature = "google-cloud-gkehub-v1alpha2",
                feature = "google-cloud-gkehub-v1beta",
                feature = "google-cloud-gkehub-v1beta1",
                feature = "google-cloud-gkemulticloud-v1",
                feature = "google-cloud-gsuiteaddons-logging-v1",
                feature = "google-cloud-identitytoolkit-logging",
                feature = "google-cloud-ids-v1",
                feature = "google-cloud-iot-v1",
                feature = "google-cloud-lifesciences-v2beta",
                feature = "google-cloud-managedidentities-v1",
                feature = "google-cloud-managedidentities-v1beta1",
                feature = "google-cloud-mediatranslation-v1alpha1",
                feature = "google-cloud-mediatranslation-v1beta1",
                feature = "google-cloud-memcache-v1",
                feature = "google-cloud-memcache-v1beta2",
                feature = "google-cloud-metastore-v1",
                feature = "google-cloud-metastore-v1alpha",
                feature = "google-cloud-metastore-v1beta",
                feature = "google-cloud-networkconnectivity-v1",
                feature = "google-cloud-networkconnectivity-v1alpha1",
                feature = "google-cloud-networkmanagement-v1",
                feature = "google-cloud-networkmanagement-v1beta1",
                feature = "google-cloud-networksecurity-v1",
                feature = "google-cloud-networksecurity-v1beta1",
                feature = "google-cloud-networkservices-v1",
                feature = "google-cloud-networkservices-v1beta1",
                feature = "google-cloud-notebooks-v1",
                feature = "google-cloud-notebooks-v1beta1",
                feature = "google-cloud-optimization-v1",
                feature = "google-cloud-orchestration-airflow-service-v1",
                feature = "google-cloud-orchestration-airflow-service-v1beta1",
                feature = "google-cloud-osconfig-v1",
                feature = "google-cloud-osconfig-v1alpha",
                feature = "google-cloud-paymentgateway-issuerswitch-v1",
                feature = "google-cloud-privatecatalog-v1beta1",
                feature = "google-cloud-pubsublite-v1",
                feature = "google-cloud-recommendationengine-v1beta1",
                feature = "google-cloud-redis-v1",
                feature = "google-cloud-redis-v1beta1",
                feature = "google-cloud-resourcemanager-v2",
                feature = "google-cloud-resourcemanager-v3",
                feature = "google-cloud-retail-logging",
                feature = "google-cloud-retail-v2",
                feature = "google-cloud-retail-v2alpha",
                feature = "google-cloud-retail-v2beta",
                feature = "google-cloud-run-v2",
                feature = "google-cloud-runtimeconfig-v1beta1",
                feature = "google-cloud-scheduler-v1",
                feature = "google-cloud-scheduler-v1beta1",
                feature = "google-cloud-security-privateca-v1",
                feature = "google-cloud-security-privateca-v1beta1",
                feature = "google-cloud-securitycenter-v1",
                feature = "google-cloud-securitycenter-v1beta1",
                feature = "google-cloud-securitycenter-v1p1beta1",
                feature = "google-cloud-shell-v1",
                feature = "google-cloud-speech-v1",
                feature = "google-cloud-speech-v1p1beta1",
                feature = "google-cloud-talent-v4",
                feature = "google-cloud-talent-v4beta1",
                feature = "google-cloud-tasks-v2",
                feature = "google-cloud-tasks-v2beta2",
                feature = "google-cloud-tasks-v2beta3",
                feature = "google-cloud-tpu-v1",
                feature = "google-cloud-tpu-v2alpha1",
                feature = "google-cloud-translation-v3",
                feature = "google-cloud-translation-v3beta1",
                feature = "google-cloud-video-livestream-logging-v1",
                feature = "google-cloud-video-livestream-v1",
                feature = "google-cloud-video-transcoder-v1",
                feature = "google-cloud-videointelligence-v1",
                feature = "google-cloud-videointelligence-v1beta2",
                feature = "google-cloud-videointelligence-v1p1beta1",
                feature = "google-cloud-videointelligence-v1p2beta1",
                feature = "google-cloud-videointelligence-v1p3beta1",
                feature = "google-cloud-vision-v1",
                feature = "google-cloud-vision-v1p1beta1",
                feature = "google-cloud-vision-v1p2beta1",
                feature = "google-cloud-vision-v1p3beta1",
                feature = "google-cloud-vision-v1p4beta1",
                feature = "google-cloud-vmmigration-v1",
                feature = "google-cloud-vpcaccess-v1",
                feature = "google-cloud-workflows-v1",
                feature = "google-cloud-workflows-v1beta",
                feature = "google-container-v1",
                feature = "google-container-v1beta1",
                feature = "google-dataflow-v1beta3",
                feature = "google-datastore-admin-v1",
                feature = "google-datastore-admin-v1beta1",
                feature = "google-devtools-artifactregistry-v1",
                feature = "google-devtools-artifactregistry-v1beta2",
                feature = "google-devtools-cloudbuild-v1",
                feature = "google-devtools-cloudtrace-v2",
                feature = "google-devtools-remoteworkers-v1test2",
                feature = "google-firestore-admin-v1",
                feature = "google-firestore-admin-v1beta1",
                feature = "google-firestore-admin-v1beta2",
                feature = "google-firestore-v1",
                feature = "google-firestore-v1beta1",
                feature = "google-genomics-v1",
                feature = "google-genomics-v1alpha2",
                feature = "google-iam-v1beta",
                feature = "google-iam-v2beta",
                feature = "google-identity-accesscontextmanager-v1",
                feature = "google-logging-v2",
                feature = "google-longrunning",
                feature = "google-maps-routes-v1",
                feature = "google-maps-routes-v1alpha",
                feature = "google-maps-routing-v2",
                feature = "google-monitoring-metricsscope-v1",
                feature = "google-monitoring-v3",
                feature = "google-partner-aistreams-v1alpha1",
                feature = "google-privacy-dlp-v2",
                feature = "google-rpc",
                feature = "google-spanner-admin-database-v1",
                feature = "google-spanner-admin-instance-v1",
                feature = "google-spanner-v1",
                feature = "google-storagetransfer-v1",
                feature = "google-streetview-publish-v1",
                feature = "grafeas-v1",
                feature = "grafeas-v1beta1",
                feature = "grafeas-v1beta1-discovery",
            )
        )]
        include_proto!("google.rpc");
        pub mod context {
            #[cfg(
                any(
                    feature = "google-api-servicecontrol-v2",
                    feature = "google-cloud-audit",
                    feature = "google-rpc-context",
                )
            )]
            include_proto!("google.rpc.context");
        }
    }
    pub mod search {
        pub mod partnerdataingestion {
            pub mod logging {
                pub mod v1 {
                    #[cfg(
                        any(feature = "google-search-partnerdataingestion-logging-v1")
                    )]
                    include_proto!("google.search.partnerdataingestion.logging.v1");
                }
            }
        }
    }
    pub mod spanner {
        pub mod admin {
            pub mod database {
                pub mod v1 {
                    #[cfg(any(feature = "google-spanner-admin-database-v1"))]
                    include_proto!("google.spanner.admin.database.v1");
                }
            }
            pub mod instance {
                pub mod v1 {
                    #[cfg(any(feature = "google-spanner-admin-instance-v1"))]
                    include_proto!("google.spanner.admin.instance.v1");
                }
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "google-spanner-v1"))]
            include_proto!("google.spanner.v1");
        }
    }
    pub mod storage {
        pub mod v1 {
            #[cfg(any(feature = "google-storage-v1"))]
            include_proto!("google.storage.v1");
        }
        pub mod v2 {
            #[cfg(any(feature = "google-storage-v2"))]
            include_proto!("google.storage.v2");
        }
    }
    pub mod storagetransfer {
        pub mod v1 {
            #[cfg(any(feature = "google-storagetransfer-v1"))]
            include_proto!("google.storagetransfer.v1");
        }
    }
    pub mod streetview {
        pub mod publish {
            pub mod v1 {
                #[cfg(any(feature = "google-streetview-publish-v1"))]
                include_proto!("google.streetview.publish.v1");
            }
        }
    }
    pub mod watcher {
        pub mod v1 {
            #[cfg(any(feature = "google-watcher-v1"))]
            include_proto!("google.watcher.v1");
        }
    }
}
pub mod grafeas {
    pub mod v1 {
        #[cfg(
            any(
                feature = "google-cloud-binaryauthorization-v1",
                feature = "google-devtools-containeranalysis-v1",
                feature = "grafeas-v1",
            )
        )]
        include_proto!("grafeas.v1");
    }
    pub mod v1beta1 {
        #[cfg(
            any(
                feature = "grafeas-v1beta1",
                feature = "grafeas-v1beta1-attestation",
                feature = "grafeas-v1beta1-discovery",
                feature = "grafeas-v1beta1-vulnerability",
            )
        )]
        include_proto!("grafeas.v1beta1");
        pub mod attestation {
            #[cfg(
                any(feature = "grafeas-v1beta1", feature = "grafeas-v1beta1-attestation")
            )]
            include_proto!("grafeas.v1beta1.attestation");
        }
        pub mod build {
            #[cfg(any(feature = "grafeas-v1beta1", feature = "grafeas-v1beta1-build"))]
            include_proto!("grafeas.v1beta1.build");
        }
        pub mod deployment {
            #[cfg(
                any(feature = "grafeas-v1beta1", feature = "grafeas-v1beta1-deployment")
            )]
            include_proto!("grafeas.v1beta1.deployment");
        }
        pub mod discovery {
            #[cfg(
                any(feature = "grafeas-v1beta1", feature = "grafeas-v1beta1-discovery")
            )]
            include_proto!("grafeas.v1beta1.discovery");
        }
        pub mod image {
            #[cfg(any(feature = "grafeas-v1beta1", feature = "grafeas-v1beta1-image"))]
            include_proto!("grafeas.v1beta1.image");
        }
        pub mod package {
            #[cfg(
                any(
                    feature = "grafeas-v1beta1",
                    feature = "grafeas-v1beta1-package",
                    feature = "grafeas-v1beta1-vulnerability",
                )
            )]
            include_proto!("grafeas.v1beta1.package");
        }
        pub mod provenance {
            #[cfg(
                any(
                    feature = "grafeas-v1beta1",
                    feature = "grafeas-v1beta1-build",
                    feature = "grafeas-v1beta1-provenance",
                )
            )]
            include_proto!("grafeas.v1beta1.provenance");
        }
        pub mod source {
            #[cfg(
                any(
                    feature = "grafeas-v1beta1",
                    feature = "grafeas-v1beta1-build",
                    feature = "grafeas-v1beta1-provenance",
                    feature = "grafeas-v1beta1-source",
                )
            )]
            include_proto!("grafeas.v1beta1.source");
        }
        pub mod vulnerability {
            #[cfg(
                any(
                    feature = "grafeas-v1beta1",
                    feature = "grafeas-v1beta1-vulnerability",
                )
            )]
            include_proto!("grafeas.v1beta1.vulnerability");
        }
    }
}
pub mod maps {
    pub mod fleetengine {
        pub mod delivery {
            pub mod v1 {
                #[cfg(any(feature = "maps-fleetengine-delivery-v1"))]
                include_proto!("maps.fleetengine.delivery.v1");
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "maps-fleetengine-v1"))]
            include_proto!("maps.fleetengine.v1");
        }
    }
}
