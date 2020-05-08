pub mod google {
    pub mod actions {
        pub mod r#type {
            #[cfg(any(feature = "google.actions.r#type",))]
            include_proto!("google.actions.r#type");
        }
    }
    pub mod ads {
        pub mod admob {
            pub mod v1 {
                #[cfg(any(feature = "google.ads.admob.v1",))]
                include_proto!("google.ads.admob.v1");
            }
        }
        pub mod googleads {
            pub mod v1 {
                pub mod common {
                    #[cfg(any(
                        feature = "google.ads.googleads.v1.common",
                        feature = "google.ads.googleads.v1.errors",
                        feature = "google.ads.googleads.v1.resources",
                        feature = "google.ads.googleads.v1.services",
                    ))]
                    include_proto!("google.ads.googleads.v1.common");
                }
                pub mod enums {
                    #[cfg(any(
                        feature = "google.ads.googleads.v1.common",
                        feature = "google.ads.googleads.v1.enums",
                        feature = "google.ads.googleads.v1.resources",
                        feature = "google.ads.googleads.v1.services",
                    ))]
                    include_proto!("google.ads.googleads.v1.enums");
                }
                pub mod errors {
                    #[cfg(any(
                        feature = "google.ads.googleads.v1.errors",
                        feature = "google.ads.googleads.v1.resources",
                        feature = "google.ads.googleads.v1.services",
                    ))]
                    include_proto!("google.ads.googleads.v1.errors");
                }
                pub mod resources {
                    #[cfg(any(
                        feature = "google.ads.googleads.v1.resources",
                        feature = "google.ads.googleads.v1.services",
                    ))]
                    include_proto!("google.ads.googleads.v1.resources");
                }
                pub mod services {
                    #[cfg(any(feature = "google.ads.googleads.v1.services",))]
                    include_proto!("google.ads.googleads.v1.services");
                }
            }
            pub mod v2 {
                pub mod common {
                    #[cfg(any(
                        feature = "google.ads.googleads.v2.common",
                        feature = "google.ads.googleads.v2.errors",
                        feature = "google.ads.googleads.v2.resources",
                        feature = "google.ads.googleads.v2.services",
                    ))]
                    include_proto!("google.ads.googleads.v2.common");
                }
                pub mod enums {
                    #[cfg(any(
                        feature = "google.ads.googleads.v2.common",
                        feature = "google.ads.googleads.v2.enums",
                        feature = "google.ads.googleads.v2.resources",
                        feature = "google.ads.googleads.v2.services",
                    ))]
                    include_proto!("google.ads.googleads.v2.enums");
                }
                pub mod errors {
                    #[cfg(any(
                        feature = "google.ads.googleads.v2.errors",
                        feature = "google.ads.googleads.v2.resources",
                        feature = "google.ads.googleads.v2.services",
                    ))]
                    include_proto!("google.ads.googleads.v2.errors");
                }
                pub mod resources {
                    #[cfg(any(
                        feature = "google.ads.googleads.v2.resources",
                        feature = "google.ads.googleads.v2.services",
                    ))]
                    include_proto!("google.ads.googleads.v2.resources");
                }
                pub mod services {
                    #[cfg(any(feature = "google.ads.googleads.v2.services",))]
                    include_proto!("google.ads.googleads.v2.services");
                }
            }
            pub mod v3 {
                pub mod common {
                    #[cfg(any(
                        feature = "google.ads.googleads.v3.common",
                        feature = "google.ads.googleads.v3.errors",
                        feature = "google.ads.googleads.v3.resources",
                        feature = "google.ads.googleads.v3.services",
                    ))]
                    include_proto!("google.ads.googleads.v3.common");
                }
                pub mod enums {
                    #[cfg(any(
                        feature = "google.ads.googleads.v3.common",
                        feature = "google.ads.googleads.v3.enums",
                        feature = "google.ads.googleads.v3.resources",
                        feature = "google.ads.googleads.v3.services",
                    ))]
                    include_proto!("google.ads.googleads.v3.enums");
                }
                pub mod errors {
                    #[cfg(any(
                        feature = "google.ads.googleads.v3.errors",
                        feature = "google.ads.googleads.v3.resources",
                        feature = "google.ads.googleads.v3.services",
                    ))]
                    include_proto!("google.ads.googleads.v3.errors");
                }
                pub mod resources {
                    #[cfg(any(
                        feature = "google.ads.googleads.v3.resources",
                        feature = "google.ads.googleads.v3.services",
                    ))]
                    include_proto!("google.ads.googleads.v3.resources");
                }
                pub mod services {
                    #[cfg(any(feature = "google.ads.googleads.v3.services",))]
                    include_proto!("google.ads.googleads.v3.services");
                }
            }
        }
    }
    pub mod analytics {
        pub mod management {
            pub mod v1alpha {
                #[cfg(any(feature = "google.analytics.management.v1alpha",))]
                include_proto!("google.analytics.management.v1alpha");
            }
        }
    }
    pub mod api {
        #[cfg(any(
            feature = "google.ads.admob.v1",
            feature = "google.ads.googleads.v1.common",
            feature = "google.ads.googleads.v1.enums",
            feature = "google.ads.googleads.v1.errors",
            feature = "google.ads.googleads.v1.resources",
            feature = "google.ads.googleads.v1.services",
            feature = "google.ads.googleads.v2.common",
            feature = "google.ads.googleads.v2.enums",
            feature = "google.ads.googleads.v2.errors",
            feature = "google.ads.googleads.v2.resources",
            feature = "google.ads.googleads.v2.services",
            feature = "google.ads.googleads.v3.common",
            feature = "google.ads.googleads.v3.enums",
            feature = "google.ads.googleads.v3.errors",
            feature = "google.ads.googleads.v3.resources",
            feature = "google.ads.googleads.v3.services",
            feature = "google.analytics.management.v1alpha",
            feature = "google.api",
            feature = "google.api.servicecontrol.v1",
            feature = "google.api.servicemanagement.v1",
            feature = "google.appengine.logging.v1",
            feature = "google.appengine.v1",
            feature = "google.assistant.embedded.v1alpha1",
            feature = "google.assistant.embedded.v1alpha2",
            feature = "google.bigtable.admin.cluster.v1",
            feature = "google.bigtable.admin.table.v1",
            feature = "google.bigtable.admin.v2",
            feature = "google.bigtable.v1",
            feature = "google.bigtable.v2",
            feature = "google.bytestream",
            feature = "google.chromeos.moblab.v1beta1",
            feature = "google.cloud",
            feature = "google.cloud.accessapproval.v1",
            feature = "google.cloud.asset.v1",
            feature = "google.cloud.asset.v1beta1",
            feature = "google.cloud.asset.v1p1beta1",
            feature = "google.cloud.asset.v1p2beta1",
            feature = "google.cloud.asset.v1p4beta1",
            feature = "google.cloud.audit",
            feature = "google.cloud.automl.v1",
            feature = "google.cloud.automl.v1beta1",
            feature = "google.cloud.bigquery.connection.v1",
            feature = "google.cloud.bigquery.connection.v1beta1",
            feature = "google.cloud.bigquery.datatransfer.v1",
            feature = "google.cloud.bigquery.logging.v1",
            feature = "google.cloud.bigquery.reservation.v1",
            feature = "google.cloud.bigquery.reservation.v1beta1",
            feature = "google.cloud.bigquery.storage.v1",
            feature = "google.cloud.bigquery.storage.v1alpha2",
            feature = "google.cloud.bigquery.storage.v1beta1",
            feature = "google.cloud.bigquery.storage.v1beta2",
            feature = "google.cloud.bigquery.v2",
            feature = "google.cloud.billing.budgets.v1alpha1",
            feature = "google.cloud.billing.budgets.v1beta1",
            feature = "google.cloud.billing.v1",
            feature = "google.cloud.binaryauthorization.v1beta1",
            feature = "google.cloud.datacatalog.v1",
            feature = "google.cloud.datacatalog.v1beta1",
            feature = "google.cloud.datalabeling.v1beta1",
            feature = "google.cloud.dataproc.v1",
            feature = "google.cloud.dataproc.v1beta2",
            feature = "google.cloud.dialogflow.v2",
            feature = "google.cloud.dialogflow.v2beta1",
            feature = "google.cloud.documentai.v1beta1",
            feature = "google.cloud.documentai.v1beta2",
            feature = "google.cloud.functions.v1beta2",
            feature = "google.cloud.gaming.v1beta",
            feature = "google.cloud.iot.v1",
            feature = "google.cloud.irm.v1alpha2",
            feature = "google.cloud.kms.v1",
            feature = "google.cloud.language.v1",
            feature = "google.cloud.language.v1beta1",
            feature = "google.cloud.language.v1beta2",
            feature = "google.cloud.location",
            feature = "google.cloud.managedidentities.v1",
            feature = "google.cloud.managedidentities.v1beta1",
            feature = "google.cloud.mediatranslation.v1beta1",
            feature = "google.cloud.memcache.v1beta2",
            feature = "google.cloud.ml.v1",
            feature = "google.cloud.orgpolicy.v1",
            feature = "google.cloud.osconfig.agentendpoint.v1",
            feature = "google.cloud.osconfig.agentendpoint.v1beta",
            feature = "google.cloud.osconfig.v1",
            feature = "google.cloud.osconfig.v1beta",
            feature = "google.cloud.oslogin.common",
            feature = "google.cloud.oslogin.v1",
            feature = "google.cloud.oslogin.v1alpha",
            feature = "google.cloud.oslogin.v1beta",
            feature = "google.cloud.phishingprotection.v1beta1",
            feature = "google.cloud.policytroubleshooter.v1",
            feature = "google.cloud.recaptchaenterprise.v1",
            feature = "google.cloud.recaptchaenterprise.v1beta1",
            feature = "google.cloud.recommendationengine.v1beta1",
            feature = "google.cloud.recommender.logging.v1beta1",
            feature = "google.cloud.recommender.v1",
            feature = "google.cloud.recommender.v1beta1",
            feature = "google.cloud.redis.v1",
            feature = "google.cloud.redis.v1beta1",
            feature = "google.cloud.resourcemanager.v2",
            feature = "google.cloud.runtimeconfig.v1beta1",
            feature = "google.cloud.scheduler.v1",
            feature = "google.cloud.scheduler.v1beta1",
            feature = "google.cloud.secretmanager.v1",
            feature = "google.cloud.secrets.v1beta1",
            feature = "google.cloud.securitycenter.settings.v1beta1",
            feature = "google.cloud.securitycenter.v1",
            feature = "google.cloud.securitycenter.v1beta1",
            feature = "google.cloud.securitycenter.v1p1beta1",
            feature = "google.cloud.servicedirectory.v1beta1",
            feature = "google.cloud.speech.v1",
            feature = "google.cloud.speech.v1p1beta1",
            feature = "google.cloud.support.common",
            feature = "google.cloud.support.v1alpha1",
            feature = "google.cloud.talent.v4beta1",
            feature = "google.cloud.tasks.v2",
            feature = "google.cloud.tasks.v2beta2",
            feature = "google.cloud.tasks.v2beta3",
            feature = "google.cloud.texttospeech.v1",
            feature = "google.cloud.texttospeech.v1beta1",
            feature = "google.cloud.translation.v3",
            feature = "google.cloud.translation.v3beta1",
            feature = "google.cloud.videointelligence.v1",
            feature = "google.cloud.videointelligence.v1beta2",
            feature = "google.cloud.videointelligence.v1p1beta1",
            feature = "google.cloud.videointelligence.v1p2beta1",
            feature = "google.cloud.videointelligence.v1p3beta1",
            feature = "google.cloud.vision.v1",
            feature = "google.cloud.vision.v1p1beta1",
            feature = "google.cloud.vision.v1p2beta1",
            feature = "google.cloud.vision.v1p3beta1",
            feature = "google.cloud.vision.v1p4beta1",
            feature = "google.cloud.webrisk.v1",
            feature = "google.cloud.webrisk.v1beta1",
            feature = "google.cloud.websecurityscanner.v1",
            feature = "google.cloud.websecurityscanner.v1alpha",
            feature = "google.cloud.websecurityscanner.v1beta",
            feature = "google.container.v1",
            feature = "google.container.v1alpha1",
            feature = "google.container.v1beta1",
            feature = "google.datastore.admin.v1",
            feature = "google.datastore.admin.v1beta1",
            feature = "google.datastore.v1",
            feature = "google.datastore.v1beta3",
            feature = "google.devtools.build.v1",
            feature = "google.devtools.cloudbuild.v1",
            feature = "google.devtools.clouddebugger.v2",
            feature = "google.devtools.clouderrorreporting.v1beta1",
            feature = "google.devtools.cloudprofiler.v2",
            feature = "google.devtools.cloudtrace.v1",
            feature = "google.devtools.cloudtrace.v2",
            feature = "google.devtools.containeranalysis.v1",
            feature = "google.devtools.containeranalysis.v1beta1",
            feature = "google.devtools.remoteworkers.v1test2",
            feature = "google.devtools.resultstore.v2",
            feature = "google.devtools.source.v1",
            feature = "google.devtools.sourcerepo.v1",
            feature = "google.example.library.v1",
            feature = "google.firebase.fcm.connection.v1alpha1",
            feature = "google.firestore.admin.v1",
            feature = "google.firestore.admin.v1beta1",
            feature = "google.firestore.admin.v1beta2",
            feature = "google.firestore.v1",
            feature = "google.firestore.v1beta1",
            feature = "google.genomics.v1",
            feature = "google.genomics.v1alpha2",
            feature = "google.home.graph.v1",
            feature = "google.iam.admin.v1",
            feature = "google.iam.credentials.v1",
            feature = "google.iam.v1",
            feature = "google.iam.v1.logging",
            feature = "google.identity.accesscontextmanager.r#type",
            feature = "google.identity.accesscontextmanager.v1",
            feature = "google.logging.r#type",
            feature = "google.logging.v2",
            feature = "google.longrunning",
            feature = "google.maps.playablelocations.v3",
            feature = "google.maps.playablelocations.v3.sample",
            feature = "google.maps.roads.v1op",
            feature = "google.maps.routes.v1",
            feature = "google.maps.routes.v1alpha",
            feature = "google.monitoring.dashboard.v1",
            feature = "google.monitoring.v3",
            feature = "google.privacy.dlp.v2",
            feature = "google.pubsub.v1",
            feature = "google.spanner.admin.database.v1",
            feature = "google.spanner.admin.instance.v1",
            feature = "google.spanner.v1",
            feature = "google.storage.v1",
            feature = "google.storagetransfer.v1",
            feature = "google.streetview.publish.v1",
            feature = "google.watcher.v1",
            feature = "grafeas.v1",
            feature = "grafeas.v1beta1",
        ))]
        include_proto!("google.api");
        pub mod expr {
            pub mod v1alpha1 {
                #[cfg(any(feature = "google.api.expr.v1alpha1",))]
                include_proto!("google.api.expr.v1alpha1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.api.expr.v1beta1",))]
                include_proto!("google.api.expr.v1beta1");
            }
        }
        pub mod servicecontrol {
            pub mod v1 {
                #[cfg(any(feature = "google.api.servicecontrol.v1",))]
                include_proto!("google.api.servicecontrol.v1");
            }
        }
        pub mod servicemanagement {
            pub mod v1 {
                #[cfg(any(feature = "google.api.servicemanagement.v1",))]
                include_proto!("google.api.servicemanagement.v1");
            }
        }
    }
    pub mod appengine {
        pub mod legacy {
            #[cfg(any(feature = "google.appengine.legacy",))]
            include_proto!("google.appengine.legacy");
        }
        pub mod logging {
            pub mod v1 {
                #[cfg(any(feature = "google.appengine.logging.v1",))]
                include_proto!("google.appengine.logging.v1");
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "google.appengine.v1",))]
            include_proto!("google.appengine.v1");
        }
    }
    pub mod assistant {
        pub mod embedded {
            pub mod v1alpha1 {
                #[cfg(any(feature = "google.assistant.embedded.v1alpha1",))]
                include_proto!("google.assistant.embedded.v1alpha1");
            }
            pub mod v1alpha2 {
                #[cfg(any(feature = "google.assistant.embedded.v1alpha2",))]
                include_proto!("google.assistant.embedded.v1alpha2");
            }
        }
    }
    pub mod bigtable {
        pub mod admin {
            pub mod cluster {
                pub mod v1 {
                    #[cfg(any(feature = "google.bigtable.admin.cluster.v1",))]
                    include_proto!("google.bigtable.admin.cluster.v1");
                }
            }
            pub mod table {
                pub mod v1 {
                    #[cfg(any(feature = "google.bigtable.admin.table.v1",))]
                    include_proto!("google.bigtable.admin.table.v1");
                }
            }
            pub mod v2 {
                #[cfg(any(feature = "google.bigtable.admin.v2",))]
                include_proto!("google.bigtable.admin.v2");
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "google.bigtable.v1",))]
            include_proto!("google.bigtable.v1");
        }
        pub mod v2 {
            #[cfg(any(feature = "google.bigtable.v2",))]
            include_proto!("google.bigtable.v2");
        }
    }
    pub mod bytestream {
        #[cfg(any(feature = "google.bytestream",))]
        include_proto!("google.bytestream");
    }
    pub mod chromeos {
        pub mod moblab {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.chromeos.moblab.v1beta1",))]
                include_proto!("google.chromeos.moblab.v1beta1");
            }
        }
    }
    pub mod cloud {
        #[cfg(any(feature = "google.cloud",))]
        include_proto!("google.cloud");
        pub mod accessapproval {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.accessapproval.v1",))]
                include_proto!("google.cloud.accessapproval.v1");
            }
        }
        pub mod asset {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.asset.v1",))]
                include_proto!("google.cloud.asset.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.asset.v1beta1",))]
                include_proto!("google.cloud.asset.v1beta1");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google.cloud.asset.v1p1beta1",))]
                include_proto!("google.cloud.asset.v1p1beta1");
            }
            pub mod v1p2beta1 {
                #[cfg(any(feature = "google.cloud.asset.v1p2beta1",))]
                include_proto!("google.cloud.asset.v1p2beta1");
            }
            pub mod v1p4beta1 {
                #[cfg(any(feature = "google.cloud.asset.v1p4beta1",))]
                include_proto!("google.cloud.asset.v1p4beta1");
            }
        }
        pub mod audit {
            #[cfg(any(feature = "google.cloud.audit",))]
            include_proto!("google.cloud.audit");
        }
        pub mod automl {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.automl.v1",))]
                include_proto!("google.cloud.automl.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.automl.v1beta1",))]
                include_proto!("google.cloud.automl.v1beta1");
            }
        }
        pub mod bigquery {
            pub mod connection {
                pub mod v1 {
                    #[cfg(any(feature = "google.cloud.bigquery.connection.v1",))]
                    include_proto!("google.cloud.bigquery.connection.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google.cloud.bigquery.connection.v1beta1",))]
                    include_proto!("google.cloud.bigquery.connection.v1beta1");
                }
            }
            pub mod datatransfer {
                pub mod v1 {
                    #[cfg(any(feature = "google.cloud.bigquery.datatransfer.v1",))]
                    include_proto!("google.cloud.bigquery.datatransfer.v1");
                }
            }
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google.cloud.bigquery.logging.v1",))]
                    include_proto!("google.cloud.bigquery.logging.v1");
                }
            }
            pub mod reservation {
                pub mod v1 {
                    #[cfg(any(feature = "google.cloud.bigquery.reservation.v1",))]
                    include_proto!("google.cloud.bigquery.reservation.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google.cloud.bigquery.reservation.v1beta1",))]
                    include_proto!("google.cloud.bigquery.reservation.v1beta1");
                }
            }
            pub mod storage {
                pub mod v1 {
                    #[cfg(any(feature = "google.cloud.bigquery.storage.v1",))]
                    include_proto!("google.cloud.bigquery.storage.v1");
                }
                pub mod v1alpha2 {
                    #[cfg(any(feature = "google.cloud.bigquery.storage.v1alpha2",))]
                    include_proto!("google.cloud.bigquery.storage.v1alpha2");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google.cloud.bigquery.storage.v1beta1",))]
                    include_proto!("google.cloud.bigquery.storage.v1beta1");
                }
                pub mod v1beta2 {
                    #[cfg(any(feature = "google.cloud.bigquery.storage.v1beta2",))]
                    include_proto!("google.cloud.bigquery.storage.v1beta2");
                }
            }
            pub mod v2 {
                #[cfg(any(feature = "google.cloud.bigquery.v2",))]
                include_proto!("google.cloud.bigquery.v2");
            }
        }
        pub mod billing {
            pub mod budgets {
                pub mod v1alpha1 {
                    #[cfg(any(feature = "google.cloud.billing.budgets.v1alpha1",))]
                    include_proto!("google.cloud.billing.budgets.v1alpha1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google.cloud.billing.budgets.v1beta1",))]
                    include_proto!("google.cloud.billing.budgets.v1beta1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.billing.v1",))]
                include_proto!("google.cloud.billing.v1");
            }
        }
        pub mod binaryauthorization {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.binaryauthorization.v1beta1",))]
                include_proto!("google.cloud.binaryauthorization.v1beta1");
            }
        }
        pub mod datacatalog {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.datacatalog.v1",))]
                include_proto!("google.cloud.datacatalog.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.datacatalog.v1beta1",))]
                include_proto!("google.cloud.datacatalog.v1beta1");
            }
        }
        pub mod datalabeling {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.datalabeling.v1beta1",))]
                include_proto!("google.cloud.datalabeling.v1beta1");
            }
        }
        pub mod dataproc {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.dataproc.v1",))]
                include_proto!("google.cloud.dataproc.v1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google.cloud.dataproc.v1beta2",))]
                include_proto!("google.cloud.dataproc.v1beta2");
            }
        }
        pub mod dialogflow {
            pub mod v2 {
                #[cfg(any(feature = "google.cloud.dialogflow.v2",))]
                include_proto!("google.cloud.dialogflow.v2");
            }
            pub mod v2beta1 {
                #[cfg(any(feature = "google.cloud.dialogflow.v2beta1",))]
                include_proto!("google.cloud.dialogflow.v2beta1");
            }
        }
        pub mod documentai {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.documentai.v1beta1",))]
                include_proto!("google.cloud.documentai.v1beta1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google.cloud.documentai.v1beta2",))]
                include_proto!("google.cloud.documentai.v1beta2");
            }
        }
        pub mod functions {
            pub mod v1beta2 {
                #[cfg(any(feature = "google.cloud.functions.v1beta2",))]
                include_proto!("google.cloud.functions.v1beta2");
            }
        }
        pub mod gaming {
            pub mod v1beta {
                #[cfg(any(feature = "google.cloud.gaming.v1beta",))]
                include_proto!("google.cloud.gaming.v1beta");
            }
        }
        pub mod iot {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.iot.v1",))]
                include_proto!("google.cloud.iot.v1");
            }
        }
        pub mod irm {
            pub mod v1alpha2 {
                #[cfg(any(feature = "google.cloud.irm.v1alpha2",))]
                include_proto!("google.cloud.irm.v1alpha2");
            }
        }
        pub mod kms {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.kms.v1",))]
                include_proto!("google.cloud.kms.v1");
            }
        }
        pub mod language {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.language.v1",))]
                include_proto!("google.cloud.language.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.language.v1beta1",))]
                include_proto!("google.cloud.language.v1beta1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google.cloud.language.v1beta2",))]
                include_proto!("google.cloud.language.v1beta2");
            }
        }
        pub mod location {
            #[cfg(any(feature = "google.cloud.location",))]
            include_proto!("google.cloud.location");
        }
        pub mod managedidentities {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.managedidentities.v1",))]
                include_proto!("google.cloud.managedidentities.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.managedidentities.v1beta1",))]
                include_proto!("google.cloud.managedidentities.v1beta1");
            }
        }
        pub mod mediatranslation {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.mediatranslation.v1beta1",))]
                include_proto!("google.cloud.mediatranslation.v1beta1");
            }
        }
        pub mod memcache {
            pub mod v1beta2 {
                #[cfg(any(feature = "google.cloud.memcache.v1beta2",))]
                include_proto!("google.cloud.memcache.v1beta2");
            }
        }
        pub mod ml {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.ml.v1",))]
                include_proto!("google.cloud.ml.v1");
            }
        }
        pub mod orgpolicy {
            pub mod v1 {
                #[cfg(any(
                    feature = "google.cloud.asset.v1",
                    feature = "google.cloud.orgpolicy.v1",
                ))]
                include_proto!("google.cloud.orgpolicy.v1");
            }
        }
        pub mod osconfig {
            pub mod agentendpoint {
                pub mod v1 {
                    #[cfg(any(feature = "google.cloud.osconfig.agentendpoint.v1",))]
                    include_proto!("google.cloud.osconfig.agentendpoint.v1");
                }
                pub mod v1beta {
                    #[cfg(any(feature = "google.cloud.osconfig.agentendpoint.v1beta",))]
                    include_proto!("google.cloud.osconfig.agentendpoint.v1beta");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.osconfig.v1",))]
                include_proto!("google.cloud.osconfig.v1");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google.cloud.osconfig.v1beta",))]
                include_proto!("google.cloud.osconfig.v1beta");
            }
        }
        pub mod oslogin {
            pub mod common {
                #[cfg(any(
                    feature = "google.cloud.oslogin.common",
                    feature = "google.cloud.oslogin.v1",
                    feature = "google.cloud.oslogin.v1alpha",
                    feature = "google.cloud.oslogin.v1beta",
                ))]
                include_proto!("google.cloud.oslogin.common");
            }
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.oslogin.v1",))]
                include_proto!("google.cloud.oslogin.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google.cloud.oslogin.v1alpha",))]
                include_proto!("google.cloud.oslogin.v1alpha");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google.cloud.oslogin.v1beta",))]
                include_proto!("google.cloud.oslogin.v1beta");
            }
        }
        pub mod phishingprotection {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.phishingprotection.v1beta1",))]
                include_proto!("google.cloud.phishingprotection.v1beta1");
            }
        }
        pub mod policytroubleshooter {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.policytroubleshooter.v1",))]
                include_proto!("google.cloud.policytroubleshooter.v1");
            }
        }
        pub mod recaptchaenterprise {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.recaptchaenterprise.v1",))]
                include_proto!("google.cloud.recaptchaenterprise.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.recaptchaenterprise.v1beta1",))]
                include_proto!("google.cloud.recaptchaenterprise.v1beta1");
            }
        }
        pub mod recommendationengine {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.recommendationengine.v1beta1",))]
                include_proto!("google.cloud.recommendationengine.v1beta1");
            }
        }
        pub mod recommender {
            pub mod logging {
                pub mod v1 {
                    #[cfg(any(feature = "google.cloud.recommender.logging.v1",))]
                    include_proto!("google.cloud.recommender.logging.v1");
                }
                pub mod v1beta1 {
                    #[cfg(any(feature = "google.cloud.recommender.logging.v1beta1",))]
                    include_proto!("google.cloud.recommender.logging.v1beta1");
                }
            }
            pub mod v1 {
                #[cfg(any(
                    feature = "google.cloud.recommender.logging.v1",
                    feature = "google.cloud.recommender.v1",
                ))]
                include_proto!("google.cloud.recommender.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google.cloud.recommender.logging.v1beta1",
                    feature = "google.cloud.recommender.v1beta1",
                ))]
                include_proto!("google.cloud.recommender.v1beta1");
            }
        }
        pub mod redis {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.redis.v1",))]
                include_proto!("google.cloud.redis.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.redis.v1beta1",))]
                include_proto!("google.cloud.redis.v1beta1");
            }
        }
        pub mod resourcemanager {
            pub mod v2 {
                #[cfg(any(feature = "google.cloud.resourcemanager.v2",))]
                include_proto!("google.cloud.resourcemanager.v2");
            }
        }
        pub mod runtimeconfig {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.runtimeconfig.v1beta1",))]
                include_proto!("google.cloud.runtimeconfig.v1beta1");
            }
        }
        pub mod scheduler {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.scheduler.v1",))]
                include_proto!("google.cloud.scheduler.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.scheduler.v1beta1",))]
                include_proto!("google.cloud.scheduler.v1beta1");
            }
        }
        pub mod secretmanager {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.secretmanager.v1",))]
                include_proto!("google.cloud.secretmanager.v1");
            }
        }
        pub mod secrets {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.secrets.v1beta1",))]
                include_proto!("google.cloud.secrets.v1beta1");
            }
        }
        pub mod securitycenter {
            pub mod settings {
                pub mod v1beta1 {
                    #[cfg(any(feature = "google.cloud.securitycenter.settings.v1beta1",))]
                    include_proto!("google.cloud.securitycenter.settings.v1beta1");
                }
            }
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.securitycenter.v1",))]
                include_proto!("google.cloud.securitycenter.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.securitycenter.v1beta1",))]
                include_proto!("google.cloud.securitycenter.v1beta1");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google.cloud.securitycenter.v1p1beta1",))]
                include_proto!("google.cloud.securitycenter.v1p1beta1");
            }
        }
        pub mod servicedirectory {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.servicedirectory.v1beta1",))]
                include_proto!("google.cloud.servicedirectory.v1beta1");
            }
        }
        pub mod speech {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.speech.v1",))]
                include_proto!("google.cloud.speech.v1");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google.cloud.speech.v1p1beta1",))]
                include_proto!("google.cloud.speech.v1p1beta1");
            }
        }
        pub mod support {
            pub mod common {
                #[cfg(any(
                    feature = "google.cloud.support.common",
                    feature = "google.cloud.support.v1alpha1",
                ))]
                include_proto!("google.cloud.support.common");
            }
            pub mod v1alpha1 {
                #[cfg(any(feature = "google.cloud.support.v1alpha1",))]
                include_proto!("google.cloud.support.v1alpha1");
            }
        }
        pub mod talent {
            pub mod v4beta1 {
                #[cfg(any(feature = "google.cloud.talent.v4beta1",))]
                include_proto!("google.cloud.talent.v4beta1");
            }
        }
        pub mod tasks {
            pub mod v2 {
                #[cfg(any(feature = "google.cloud.tasks.v2",))]
                include_proto!("google.cloud.tasks.v2");
            }
            pub mod v2beta2 {
                #[cfg(any(feature = "google.cloud.tasks.v2beta2",))]
                include_proto!("google.cloud.tasks.v2beta2");
            }
            pub mod v2beta3 {
                #[cfg(any(feature = "google.cloud.tasks.v2beta3",))]
                include_proto!("google.cloud.tasks.v2beta3");
            }
        }
        pub mod texttospeech {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.texttospeech.v1",))]
                include_proto!("google.cloud.texttospeech.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.texttospeech.v1beta1",))]
                include_proto!("google.cloud.texttospeech.v1beta1");
            }
        }
        pub mod translation {
            pub mod v3 {
                #[cfg(any(feature = "google.cloud.translation.v3",))]
                include_proto!("google.cloud.translation.v3");
            }
            pub mod v3beta1 {
                #[cfg(any(feature = "google.cloud.translation.v3beta1",))]
                include_proto!("google.cloud.translation.v3beta1");
            }
        }
        pub mod videointelligence {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.videointelligence.v1",))]
                include_proto!("google.cloud.videointelligence.v1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google.cloud.videointelligence.v1beta2",))]
                include_proto!("google.cloud.videointelligence.v1beta2");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google.cloud.videointelligence.v1p1beta1",))]
                include_proto!("google.cloud.videointelligence.v1p1beta1");
            }
            pub mod v1p2beta1 {
                #[cfg(any(feature = "google.cloud.videointelligence.v1p2beta1",))]
                include_proto!("google.cloud.videointelligence.v1p2beta1");
            }
            pub mod v1p3beta1 {
                #[cfg(any(feature = "google.cloud.videointelligence.v1p3beta1",))]
                include_proto!("google.cloud.videointelligence.v1p3beta1");
            }
        }
        pub mod vision {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.vision.v1",))]
                include_proto!("google.cloud.vision.v1");
            }
            pub mod v1p1beta1 {
                #[cfg(any(feature = "google.cloud.vision.v1p1beta1",))]
                include_proto!("google.cloud.vision.v1p1beta1");
            }
            pub mod v1p2beta1 {
                #[cfg(any(feature = "google.cloud.vision.v1p2beta1",))]
                include_proto!("google.cloud.vision.v1p2beta1");
            }
            pub mod v1p3beta1 {
                #[cfg(any(feature = "google.cloud.vision.v1p3beta1",))]
                include_proto!("google.cloud.vision.v1p3beta1");
            }
            pub mod v1p4beta1 {
                #[cfg(any(feature = "google.cloud.vision.v1p4beta1",))]
                include_proto!("google.cloud.vision.v1p4beta1");
            }
        }
        pub mod webrisk {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.webrisk.v1",))]
                include_proto!("google.cloud.webrisk.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.cloud.webrisk.v1beta1",))]
                include_proto!("google.cloud.webrisk.v1beta1");
            }
        }
        pub mod websecurityscanner {
            pub mod v1 {
                #[cfg(any(feature = "google.cloud.websecurityscanner.v1",))]
                include_proto!("google.cloud.websecurityscanner.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google.cloud.websecurityscanner.v1alpha",))]
                include_proto!("google.cloud.websecurityscanner.v1alpha");
            }
            pub mod v1beta {
                #[cfg(any(feature = "google.cloud.websecurityscanner.v1beta",))]
                include_proto!("google.cloud.websecurityscanner.v1beta");
            }
        }
    }
    pub mod container {
        pub mod v1 {
            #[cfg(any(feature = "google.container.v1",))]
            include_proto!("google.container.v1");
        }
        pub mod v1alpha1 {
            #[cfg(any(feature = "google.container.v1alpha1",))]
            include_proto!("google.container.v1alpha1");
        }
        pub mod v1beta1 {
            #[cfg(any(feature = "google.container.v1beta1",))]
            include_proto!("google.container.v1beta1");
        }
    }
    pub mod datastore {
        pub mod admin {
            pub mod v1 {
                #[cfg(any(feature = "google.datastore.admin.v1",))]
                include_proto!("google.datastore.admin.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.datastore.admin.v1beta1",))]
                include_proto!("google.datastore.admin.v1beta1");
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "google.datastore.v1",))]
            include_proto!("google.datastore.v1");
        }
        pub mod v1beta3 {
            #[cfg(any(feature = "google.datastore.v1beta3",))]
            include_proto!("google.datastore.v1beta3");
        }
    }
    pub mod devtools {
        pub mod build {
            pub mod v1 {
                #[cfg(any(feature = "google.devtools.build.v1",))]
                include_proto!("google.devtools.build.v1");
            }
        }
        pub mod cloudbuild {
            pub mod v1 {
                #[cfg(any(feature = "google.devtools.cloudbuild.v1",))]
                include_proto!("google.devtools.cloudbuild.v1");
            }
        }
        pub mod clouddebugger {
            pub mod v2 {
                #[cfg(any(feature = "google.devtools.clouddebugger.v2",))]
                include_proto!("google.devtools.clouddebugger.v2");
            }
        }
        pub mod clouderrorreporting {
            pub mod v1beta1 {
                #[cfg(any(feature = "google.devtools.clouderrorreporting.v1beta1",))]
                include_proto!("google.devtools.clouderrorreporting.v1beta1");
            }
        }
        pub mod cloudprofiler {
            pub mod v2 {
                #[cfg(any(feature = "google.devtools.cloudprofiler.v2",))]
                include_proto!("google.devtools.cloudprofiler.v2");
            }
        }
        pub mod cloudtrace {
            pub mod v1 {
                #[cfg(any(feature = "google.devtools.cloudtrace.v1",))]
                include_proto!("google.devtools.cloudtrace.v1");
            }
            pub mod v2 {
                #[cfg(any(feature = "google.devtools.cloudtrace.v2",))]
                include_proto!("google.devtools.cloudtrace.v2");
            }
        }
        pub mod containeranalysis {
            pub mod v1 {
                #[cfg(any(feature = "google.devtools.containeranalysis.v1",))]
                include_proto!("google.devtools.containeranalysis.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.devtools.containeranalysis.v1beta1",))]
                include_proto!("google.devtools.containeranalysis.v1beta1");
            }
        }
        pub mod remoteworkers {
            pub mod v1test2 {
                #[cfg(any(feature = "google.devtools.remoteworkers.v1test2",))]
                include_proto!("google.devtools.remoteworkers.v1test2");
            }
        }
        pub mod resultstore {
            pub mod v2 {
                #[cfg(any(feature = "google.devtools.resultstore.v2",))]
                include_proto!("google.devtools.resultstore.v2");
            }
        }
        pub mod source {
            pub mod v1 {
                #[cfg(any(
                    feature = "google.devtools.clouddebugger.v2",
                    feature = "google.devtools.source.v1",
                ))]
                include_proto!("google.devtools.source.v1");
            }
        }
        pub mod sourcerepo {
            pub mod v1 {
                #[cfg(any(feature = "google.devtools.sourcerepo.v1",))]
                include_proto!("google.devtools.sourcerepo.v1");
            }
        }
    }
    pub mod example {
        pub mod library {
            pub mod v1 {
                #[cfg(any(feature = "google.example.library.v1",))]
                include_proto!("google.example.library.v1");
            }
        }
    }
    pub mod firebase {
        pub mod fcm {
            pub mod connection {
                pub mod v1alpha1 {
                    #[cfg(any(feature = "google.firebase.fcm.connection.v1alpha1",))]
                    include_proto!("google.firebase.fcm.connection.v1alpha1");
                }
            }
        }
    }
    pub mod firestore {
        pub mod admin {
            pub mod v1 {
                #[cfg(any(feature = "google.firestore.admin.v1",))]
                include_proto!("google.firestore.admin.v1");
            }
            pub mod v1beta1 {
                #[cfg(any(feature = "google.firestore.admin.v1beta1",))]
                include_proto!("google.firestore.admin.v1beta1");
            }
            pub mod v1beta2 {
                #[cfg(any(feature = "google.firestore.admin.v1beta2",))]
                include_proto!("google.firestore.admin.v1beta2");
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "google.firestore.v1",))]
            include_proto!("google.firestore.v1");
        }
        pub mod v1beta1 {
            #[cfg(any(feature = "google.firestore.v1beta1",))]
            include_proto!("google.firestore.v1beta1");
        }
    }
    pub mod genomics {
        pub mod v1 {
            #[cfg(any(feature = "google.genomics.v1",))]
            include_proto!("google.genomics.v1");
        }
        pub mod v1alpha2 {
            #[cfg(any(feature = "google.genomics.v1alpha2",))]
            include_proto!("google.genomics.v1alpha2");
        }
    }
    pub mod geo {
        pub mod r#type {
            #[cfg(any(
                feature = "google.geo.r#type",
                feature = "google.maps.routes.v1",
                feature = "google.maps.routes.v1alpha",
            ))]
            include_proto!("google.geo.r#type");
        }
    }
    pub mod home {
        pub mod graph {
            pub mod v1 {
                #[cfg(any(feature = "google.home.graph.v1",))]
                include_proto!("google.home.graph.v1");
            }
        }
    }
    pub mod iam {
        pub mod admin {
            pub mod v1 {
                #[cfg(any(feature = "google.iam.admin.v1",))]
                include_proto!("google.iam.admin.v1");
            }
        }
        pub mod credentials {
            pub mod v1 {
                #[cfg(any(feature = "google.iam.credentials.v1",))]
                include_proto!("google.iam.credentials.v1");
            }
        }
        pub mod v1 {
            #[cfg(any(
                feature = "google.appengine.v1",
                feature = "google.bigtable.admin.v2",
                feature = "google.cloud.asset.v1",
                feature = "google.cloud.asset.v1beta1",
                feature = "google.cloud.asset.v1p1beta1",
                feature = "google.cloud.asset.v1p2beta1",
                feature = "google.cloud.asset.v1p4beta1",
                feature = "google.cloud.bigquery.connection.v1",
                feature = "google.cloud.bigquery.connection.v1beta1",
                feature = "google.cloud.billing.v1",
                feature = "google.cloud.datacatalog.v1",
                feature = "google.cloud.datacatalog.v1beta1",
                feature = "google.cloud.iot.v1",
                feature = "google.cloud.policytroubleshooter.v1",
                feature = "google.cloud.resourcemanager.v2",
                feature = "google.cloud.secretmanager.v1",
                feature = "google.cloud.secrets.v1beta1",
                feature = "google.cloud.securitycenter.v1",
                feature = "google.cloud.securitycenter.v1beta1",
                feature = "google.cloud.securitycenter.v1p1beta1",
                feature = "google.cloud.servicedirectory.v1beta1",
                feature = "google.cloud.tasks.v2",
                feature = "google.cloud.tasks.v2beta2",
                feature = "google.cloud.tasks.v2beta3",
                feature = "google.devtools.containeranalysis.v1",
                feature = "google.devtools.containeranalysis.v1beta1",
                feature = "google.devtools.sourcerepo.v1",
                feature = "google.genomics.v1",
                feature = "google.iam.admin.v1",
                feature = "google.iam.v1",
                feature = "google.iam.v1.logging",
                feature = "google.spanner.admin.database.v1",
                feature = "google.spanner.admin.instance.v1",
                feature = "google.storage.v1",
            ))]
            include_proto!("google.iam.v1");
            pub mod logging {
                #[cfg(any(feature = "google.iam.v1.logging",))]
                include_proto!("google.iam.v1.logging");
            }
        }
    }
    pub mod identity {
        pub mod accesscontextmanager {
            pub mod r#type {
                #[cfg(any(
                    feature = "google.cloud.asset.v1",
                    feature = "google.identity.accesscontextmanager.r#type",
                    feature = "google.identity.accesscontextmanager.v1",
                ))]
                include_proto!("google.identity.accesscontextmanager.r#type");
            }
            pub mod v1 {
                #[cfg(any(
                    feature = "google.cloud.asset.v1",
                    feature = "google.identity.accesscontextmanager.v1",
                ))]
                include_proto!("google.identity.accesscontextmanager.v1");
            }
        }
    }
    pub mod logging {
        pub mod r#type {
            #[cfg(any(
                feature = "google.api.servicecontrol.v1",
                feature = "google.appengine.logging.v1",
                feature = "google.logging.r#type",
                feature = "google.logging.v2",
            ))]
            include_proto!("google.logging.r#type");
        }
        pub mod v2 {
            #[cfg(any(feature = "google.logging.v2",))]
            include_proto!("google.logging.v2");
        }
    }
    pub mod longrunning {
        #[cfg(any(
            feature = "google.ads.googleads.v1.services",
            feature = "google.ads.googleads.v2.services",
            feature = "google.ads.googleads.v3.services",
            feature = "google.api.servicemanagement.v1",
            feature = "google.appengine.v1",
            feature = "google.bigtable.admin.cluster.v1",
            feature = "google.bigtable.admin.table.v1",
            feature = "google.bigtable.admin.v2",
            feature = "google.chromeos.moblab.v1beta1",
            feature = "google.cloud.asset.v1",
            feature = "google.cloud.asset.v1beta1",
            feature = "google.cloud.asset.v1p2beta1",
            feature = "google.cloud.asset.v1p4beta1",
            feature = "google.cloud.automl.v1",
            feature = "google.cloud.automl.v1beta1",
            feature = "google.cloud.datalabeling.v1beta1",
            feature = "google.cloud.dataproc.v1",
            feature = "google.cloud.dataproc.v1beta2",
            feature = "google.cloud.dialogflow.v2",
            feature = "google.cloud.dialogflow.v2beta1",
            feature = "google.cloud.documentai.v1beta1",
            feature = "google.cloud.documentai.v1beta2",
            feature = "google.cloud.functions.v1beta2",
            feature = "google.cloud.gaming.v1beta",
            feature = "google.cloud.managedidentities.v1",
            feature = "google.cloud.managedidentities.v1beta1",
            feature = "google.cloud.memcache.v1beta2",
            feature = "google.cloud.ml.v1",
            feature = "google.cloud.recommendationengine.v1beta1",
            feature = "google.cloud.redis.v1",
            feature = "google.cloud.redis.v1beta1",
            feature = "google.cloud.resourcemanager.v2",
            feature = "google.cloud.runtimeconfig.v1beta1",
            feature = "google.cloud.securitycenter.v1",
            feature = "google.cloud.securitycenter.v1beta1",
            feature = "google.cloud.securitycenter.v1p1beta1",
            feature = "google.cloud.speech.v1",
            feature = "google.cloud.speech.v1p1beta1",
            feature = "google.cloud.talent.v4beta1",
            feature = "google.cloud.translation.v3",
            feature = "google.cloud.translation.v3beta1",
            feature = "google.cloud.videointelligence.v1",
            feature = "google.cloud.videointelligence.v1beta2",
            feature = "google.cloud.videointelligence.v1p1beta1",
            feature = "google.cloud.videointelligence.v1p2beta1",
            feature = "google.cloud.videointelligence.v1p3beta1",
            feature = "google.cloud.vision.v1",
            feature = "google.cloud.vision.v1p2beta1",
            feature = "google.cloud.vision.v1p3beta1",
            feature = "google.cloud.vision.v1p4beta1",
            feature = "google.datastore.admin.v1",
            feature = "google.datastore.admin.v1beta1",
            feature = "google.devtools.cloudbuild.v1",
            feature = "google.firestore.admin.v1",
            feature = "google.firestore.admin.v1beta1",
            feature = "google.firestore.admin.v1beta2",
            feature = "google.genomics.v1",
            feature = "google.genomics.v1alpha2",
            feature = "google.longrunning",
            feature = "google.spanner.admin.database.v1",
            feature = "google.spanner.admin.instance.v1",
        ))]
        include_proto!("google.longrunning");
    }
    pub mod maps {
        pub mod playablelocations {
            pub mod v3 {
                #[cfg(any(feature = "google.maps.playablelocations.v3",))]
                include_proto!("google.maps.playablelocations.v3");
                pub mod sample {
                    #[cfg(any(
                        feature = "google.maps.playablelocations.v3",
                        feature = "google.maps.playablelocations.v3.sample",
                    ))]
                    include_proto!("google.maps.playablelocations.v3.sample");
                }
            }
        }
        pub mod roads {
            pub mod v1op {
                #[cfg(any(feature = "google.maps.roads.v1op",))]
                include_proto!("google.maps.roads.v1op");
            }
        }
        pub mod routes {
            pub mod v1 {
                #[cfg(any(
                    feature = "google.maps.routes.v1",
                    feature = "google.maps.routes.v1alpha",
                ))]
                include_proto!("google.maps.routes.v1");
            }
            pub mod v1alpha {
                #[cfg(any(feature = "google.maps.routes.v1alpha",))]
                include_proto!("google.maps.routes.v1alpha");
            }
        }
        pub mod unity {
            #[cfg(any(
                feature = "google.maps.playablelocations.v3",
                feature = "google.maps.unity",
            ))]
            include_proto!("google.maps.unity");
        }
    }
    pub mod monitoring {
        pub mod dashboard {
            pub mod v1 {
                #[cfg(any(feature = "google.monitoring.dashboard.v1",))]
                include_proto!("google.monitoring.dashboard.v1");
            }
        }
        pub mod v3 {
            #[cfg(any(feature = "google.monitoring.v3",))]
            include_proto!("google.monitoring.v3");
        }
    }
    pub mod privacy {
        pub mod dlp {
            pub mod v2 {
                #[cfg(any(feature = "google.privacy.dlp.v2",))]
                include_proto!("google.privacy.dlp.v2");
            }
        }
    }
    pub mod pubsub {
        pub mod v1 {
            #[cfg(any(feature = "google.pubsub.v1",))]
            include_proto!("google.pubsub.v1");
        }
        pub mod v1beta2 {
            #[cfg(any(feature = "google.pubsub.v1beta2",))]
            include_proto!("google.pubsub.v1beta2");
        }
    }
    pub mod r#type {
        #[cfg(any(
            feature = "google.actions.r#type",
            feature = "google.ads.admob.v1",
            feature = "google.api.servicecontrol.v1",
            feature = "google.appengine.v1",
            feature = "google.assistant.embedded.v1alpha2",
            feature = "google.bigtable.admin.v2",
            feature = "google.cloud.asset.v1",
            feature = "google.cloud.asset.v1beta1",
            feature = "google.cloud.asset.v1p1beta1",
            feature = "google.cloud.asset.v1p2beta1",
            feature = "google.cloud.asset.v1p4beta1",
            feature = "google.cloud.bigquery.connection.v1",
            feature = "google.cloud.bigquery.connection.v1beta1",
            feature = "google.cloud.billing.budgets.v1alpha1",
            feature = "google.cloud.billing.budgets.v1beta1",
            feature = "google.cloud.billing.v1",
            feature = "google.cloud.datacatalog.v1",
            feature = "google.cloud.datacatalog.v1beta1",
            feature = "google.cloud.dialogflow.v2",
            feature = "google.cloud.dialogflow.v2beta1",
            feature = "google.cloud.documentai.v1beta1",
            feature = "google.cloud.documentai.v1beta2",
            feature = "google.cloud.iot.v1",
            feature = "google.cloud.osconfig.v1",
            feature = "google.cloud.osconfig.v1beta",
            feature = "google.cloud.policytroubleshooter.v1",
            feature = "google.cloud.recommendationengine.v1beta1",
            feature = "google.cloud.recommender.v1",
            feature = "google.cloud.recommender.v1beta1",
            feature = "google.cloud.resourcemanager.v2",
            feature = "google.cloud.secretmanager.v1",
            feature = "google.cloud.secrets.v1beta1",
            feature = "google.cloud.securitycenter.v1",
            feature = "google.cloud.securitycenter.v1beta1",
            feature = "google.cloud.securitycenter.v1p1beta1",
            feature = "google.cloud.servicedirectory.v1beta1",
            feature = "google.cloud.talent.v4beta1",
            feature = "google.cloud.tasks.v2",
            feature = "google.cloud.tasks.v2beta2",
            feature = "google.cloud.tasks.v2beta3",
            feature = "google.cloud.vision.v1",
            feature = "google.cloud.vision.v1p1beta1",
            feature = "google.cloud.vision.v1p2beta1",
            feature = "google.cloud.vision.v1p3beta1",
            feature = "google.cloud.vision.v1p4beta1",
            feature = "google.datastore.v1",
            feature = "google.datastore.v1beta3",
            feature = "google.devtools.containeranalysis.v1",
            feature = "google.devtools.containeranalysis.v1beta1",
            feature = "google.devtools.sourcerepo.v1",
            feature = "google.firestore.admin.v1",
            feature = "google.firestore.admin.v1beta1",
            feature = "google.firestore.v1",
            feature = "google.firestore.v1beta1",
            feature = "google.genomics.v1",
            feature = "google.geo.r#type",
            feature = "google.iam.admin.v1",
            feature = "google.iam.v1",
            feature = "google.identity.accesscontextmanager.v1",
            feature = "google.maps.playablelocations.v3",
            feature = "google.maps.playablelocations.v3.sample",
            feature = "google.maps.roads.v1op",
            feature = "google.maps.routes.v1",
            feature = "google.maps.routes.v1alpha",
            feature = "google.monitoring.v3",
            feature = "google.privacy.dlp.v2",
            feature = "google.r#type",
            feature = "google.spanner.admin.database.v1",
            feature = "google.spanner.admin.instance.v1",
            feature = "google.storage.v1",
            feature = "google.storagetransfer.v1",
            feature = "google.streetview.publish.v1",
        ))]
        include_proto!("google.r#type");
    }
    pub mod rpc {
        #[cfg(any(
            feature = "google.ads.googleads.v1.services",
            feature = "google.ads.googleads.v2.services",
            feature = "google.ads.googleads.v3.services",
            feature = "google.api.expr.v1alpha1",
            feature = "google.api.expr.v1beta1",
            feature = "google.api.servicecontrol.v1",
            feature = "google.api.servicemanagement.v1",
            feature = "google.appengine.v1",
            feature = "google.assistant.embedded.v1alpha1",
            feature = "google.bigtable.admin.cluster.v1",
            feature = "google.bigtable.admin.table.v1",
            feature = "google.bigtable.v1",
            feature = "google.bigtable.v2",
            feature = "google.cloud.asset.v1p4beta1",
            feature = "google.cloud.audit",
            feature = "google.cloud.automl.v1",
            feature = "google.cloud.automl.v1beta1",
            feature = "google.cloud.bigquery.datatransfer.v1",
            feature = "google.cloud.bigquery.logging.v1",
            feature = "google.cloud.bigquery.reservation.v1",
            feature = "google.cloud.bigquery.reservation.v1beta1",
            feature = "google.cloud.bigquery.storage.v1alpha2",
            feature = "google.cloud.datalabeling.v1beta1",
            feature = "google.cloud.dataproc.v1",
            feature = "google.cloud.dataproc.v1beta2",
            feature = "google.cloud.dialogflow.v2",
            feature = "google.cloud.dialogflow.v2beta1",
            feature = "google.cloud.documentai.v1beta1",
            feature = "google.cloud.documentai.v1beta2",
            feature = "google.cloud.iot.v1",
            feature = "google.cloud.mediatranslation.v1beta1",
            feature = "google.cloud.ml.v1",
            feature = "google.cloud.recommendationengine.v1beta1",
            feature = "google.cloud.runtimeconfig.v1beta1",
            feature = "google.cloud.scheduler.v1",
            feature = "google.cloud.scheduler.v1beta1",
            feature = "google.cloud.speech.v1",
            feature = "google.cloud.speech.v1p1beta1",
            feature = "google.cloud.talent.v4beta1",
            feature = "google.cloud.tasks.v2",
            feature = "google.cloud.tasks.v2beta2",
            feature = "google.cloud.tasks.v2beta3",
            feature = "google.cloud.videointelligence.v1",
            feature = "google.cloud.videointelligence.v1beta2",
            feature = "google.cloud.videointelligence.v1p1beta1",
            feature = "google.cloud.videointelligence.v1p2beta1",
            feature = "google.cloud.videointelligence.v1p3beta1",
            feature = "google.cloud.vision.v1",
            feature = "google.cloud.vision.v1p1beta1",
            feature = "google.cloud.vision.v1p2beta1",
            feature = "google.cloud.vision.v1p3beta1",
            feature = "google.cloud.vision.v1p4beta1",
            feature = "google.devtools.cloudtrace.v2",
            feature = "google.devtools.remoteworkers.v1test2",
            feature = "google.firestore.v1",
            feature = "google.firestore.v1beta1",
            feature = "google.genomics.v1",
            feature = "google.genomics.v1alpha2",
            feature = "google.logging.v2",
            feature = "google.longrunning",
            feature = "google.maps.routes.v1",
            feature = "google.monitoring.v3",
            feature = "google.privacy.dlp.v2",
            feature = "google.rpc",
            feature = "google.spanner.admin.database.v1",
            feature = "google.spanner.v1",
            feature = "google.storagetransfer.v1",
            feature = "google.streetview.publish.v1",
            feature = "grafeas.v1",
            feature = "grafeas.v1beta1.discovery",
        ))]
        include_proto!("google.rpc");
        pub mod context {
            #[cfg(any(feature = "google.rpc.context",))]
            include_proto!("google.rpc.context");
        }
    }
    pub mod spanner {
        pub mod admin {
            pub mod database {
                pub mod v1 {
                    #[cfg(any(feature = "google.spanner.admin.database.v1",))]
                    include_proto!("google.spanner.admin.database.v1");
                }
            }
            pub mod instance {
                pub mod v1 {
                    #[cfg(any(feature = "google.spanner.admin.instance.v1",))]
                    include_proto!("google.spanner.admin.instance.v1");
                }
            }
        }
        pub mod v1 {
            #[cfg(any(feature = "google.spanner.v1",))]
            include_proto!("google.spanner.v1");
        }
    }
    pub mod storage {
        pub mod v1 {
            #[cfg(any(feature = "google.storage.v1",))]
            include_proto!("google.storage.v1");
        }
    }
    pub mod storagetransfer {
        pub mod v1 {
            #[cfg(any(feature = "google.storagetransfer.v1",))]
            include_proto!("google.storagetransfer.v1");
        }
    }
    pub mod streetview {
        pub mod publish {
            pub mod v1 {
                #[cfg(any(feature = "google.streetview.publish.v1",))]
                include_proto!("google.streetview.publish.v1");
            }
        }
    }
    pub mod watcher {
        pub mod v1 {
            #[cfg(any(feature = "google.watcher.v1",))]
            include_proto!("google.watcher.v1");
        }
    }
}
pub mod grafeas {
    pub mod v1 {
        #[cfg(any(feature = "grafeas.v1",))]
        include_proto!("grafeas.v1");
    }
    pub mod v1beta1 {
        #[cfg(any(
            feature = "grafeas.v1beta1",
            feature = "grafeas.v1beta1.attestation",
            feature = "grafeas.v1beta1.discovery",
            feature = "grafeas.v1beta1.vulnerability",
        ))]
        include_proto!("grafeas.v1beta1");
        pub mod attestation {
            #[cfg(any(feature = "grafeas.v1beta1", feature = "grafeas.v1beta1.attestation",))]
            include_proto!("grafeas.v1beta1.attestation");
        }
        pub mod build {
            #[cfg(any(feature = "grafeas.v1beta1", feature = "grafeas.v1beta1.build",))]
            include_proto!("grafeas.v1beta1.build");
        }
        pub mod deployment {
            #[cfg(any(feature = "grafeas.v1beta1", feature = "grafeas.v1beta1.deployment",))]
            include_proto!("grafeas.v1beta1.deployment");
        }
        pub mod discovery {
            #[cfg(any(feature = "grafeas.v1beta1", feature = "grafeas.v1beta1.discovery",))]
            include_proto!("grafeas.v1beta1.discovery");
        }
        pub mod image {
            #[cfg(any(feature = "grafeas.v1beta1", feature = "grafeas.v1beta1.image",))]
            include_proto!("grafeas.v1beta1.image");
        }
        pub mod package {
            #[cfg(any(
                feature = "grafeas.v1beta1",
                feature = "grafeas.v1beta1.package",
                feature = "grafeas.v1beta1.vulnerability",
            ))]
            include_proto!("grafeas.v1beta1.package");
        }
        pub mod provenance {
            #[cfg(any(
                feature = "grafeas.v1beta1",
                feature = "grafeas.v1beta1.build",
                feature = "grafeas.v1beta1.provenance",
            ))]
            include_proto!("grafeas.v1beta1.provenance");
        }
        pub mod source {
            #[cfg(any(
                feature = "grafeas.v1beta1",
                feature = "grafeas.v1beta1.provenance",
                feature = "grafeas.v1beta1.source",
            ))]
            include_proto!("grafeas.v1beta1.source");
        }
        pub mod vulnerability {
            #[cfg(any(feature = "grafeas.v1beta1", feature = "grafeas.v1beta1.vulnerability",))]
            include_proto!("grafeas.v1beta1.vulnerability");
        }
    }
}
