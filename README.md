# rust-example-redis-federated-identities

## Showcase to demonstrate the combination of Redis and Federated Identities

see: https://learn.microsoft.com/en-us/azure/azure-cache-for-redis/cache-rust-get-started
and here: https://learn.microsoft.com/de-de/azure/azure-cache-for-redis/cache-azure-active-directory-for-authentication#configure-your-redis-client-to-use-azure-active-directory

Tested with gitlab federated identities and Azure Redis Cache 
Tested with azure cli and Azure Redis Cache 

https://docs.gitlab.com/ee/ci/cloud_services/azure/

```yaml
test:
  stage: .post
  image: rust:latest
  variables:
    REDIS_HOSTNAME: "<YOUR_CLUSTER>.cache.windows.net:6380"
    AZURE_SP_OBJECT_ID: <THE SERVICE PRINCIPLE OBJECT ID>
  id_tokens:
    AZURE_FEDERATED_TOKEN:
      aud: https://gitlab.com
  script:
    - cd test
    - cargo run --release
```

TODOs: 
 - test with github + and azure ad
 - test with k8s workload identity