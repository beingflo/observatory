# version increment
# subcommands
#  deploy
#  rollback
#  stop
#  start
docker buildx build --platform=linux/amd64 -t ghcr.io/beingflo/observatory:0.1.1 .
docker push ghcr.io/beingflo/observatory:0.1.1
docker --context omni compose --file docker-compose.prod.yml up