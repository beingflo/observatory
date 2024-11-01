# Steps (deploy)
## Check that there are no uncommited changes
## Print current version
## Prompt for next version (default current version)
## Write version to Cargo.toml and package.json (make sure its in the lock files as well after docker build)
## Build docker image
## Commit and tag version
## Push docker image
## Upgrade image running on omni
docker buildx build --platform=linux/amd64 -t ghcr.io/beingflo/observatory:0.1.1 .
docker push ghcr.io/beingflo/observatory:0.1.1
docker --context omni compose --file docker-compose.prod.yml up