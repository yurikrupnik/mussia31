// Path1
// use futures::{StreamExt, TryStreamExt};
// use kube::{Client, api::{Api, ResourceExt, ListParams, PostParams}};
// use k8s_openapi::api::core::v1::Pod;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//   // Infer the runtime environment and try to create a Kubernetes Client
//   let client = Client::try_default().await?;
//
//   // Read pods in the configured namespace into the typed interface from k8s-openapi
//   let pods: Api<Pod> = Api::default_namespaced(client);
//   for p in pods.list(&ListParams::default()).await? {
//     println!("found pod {}", p.name_any());
//   }
//   Ok(())
// }

// use k8s_openapi::api::core::v1::Pod;
// use kube::{Api, Client, Error};
// use serde_json::json;
// use kube::Error::Api;

// Path2
// mod crd;
//
// // use crd::Yuri;
// // use crd::YuriSpec;
//
// #[tokio::main]
// async fn main() {
//   // let kubernetes_client = Client::try_default()
//   //     .await
//   //     .expect("Expected a valid Kubeconfig env var");
//
//   let namespace = std::env::var("NAMESPACE").unwrap_or_else(|_| "default".into());
//   // let pods: Api<Pod> = Api::namespaced(kubernetes_client, &namespace);
//   // let pods: Api<Yuri> = Api::namespaced(kubernetes_client, &namespace);
//   // let crd_api: Api<crd::Yuri> = Api::all(kubernetes_client.clone());
//   // let d: Pod = serde_json::json!({
//   //     "apiVersion": "v1",
//   //   "kind": "Pod",
//   //   "metadata": { "name": "blog"},
//   //   "spec": {
//   //     "containers": [
//   //       {
//   //         "name": "blod",
//   //         "image": "clux/blod:0.1.0"
//   //       }
//   //     ]
//   //   }
//   // })
//   // let p: Pod = serde_json::from_value(json!({
//   //   "apiVersion": "v1",
//   //   "kind": "Pod",
//   //   "metadata": { "name": "blog"},
//   //   "spec": {
//   //     "containers": [
//   //       {
//   //         "name": "blod",
//   //         "image": "clux/blod:0.1.0"
//   //       }
//   //     ]
//   //   }
//   // }))
//   // pods.get()
//   println!("hello {}", namespace);
//   println!("Hello, world!");
// }
// Path3
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
  api::{Api, DeleteParams, ListParams, PatchParams, Patch, ResourceExt},
  core::CustomResourceExt,
  Client, CustomResource,
  runtime::{watcher, WatchStreamExt, wait::{conditions, await_condition}},
};

// Our custom resource
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, Validate, JsonSchema)]
#[kube(group = "clux.dev", version = "v1", kind = "Foo", namespaced)]
pub struct FooSpec {
  info: String,
  #[validate(length(min = 3))]
  name: String,
  replicas: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = Client::try_default().await?;
  let crds: Api<CustomResourceDefinition> = Api::all(client.clone());

  // Apply the CRD so users can create Foo instances in Kubernetes
  crds.patch("foos.clux.dev",
             &PatchParams::apply("my_manager"),
             &Patch::Apply(Foo::crd())
  ).await?;

  // Wait for the CRD to be ready
  tokio::time::timeout(
    std::time::Duration::from_secs(10),
    await_condition(crds, "foos.clux.dev", conditions::is_crd_established())
  ).await?;

  // Watch for changes to foos in the configured namespace
  let foos: Api<Foo> = Api::default_namespaced(client.clone());
  let lp = ListParams::default();
  let mut apply_stream = watcher(foos, lp).applied_objects().boxed();
  while let Some(f) = apply_stream.try_next().await? {
    println!("saw apply to {}", f.name_any());
  }
  Ok(())
}
