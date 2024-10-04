#!/usr/bin/env bash
set -x
set -eo pipefail
heroku container:login
docker pull ohaddahan/brainwar:latest
docker tag ohaddahan/brainwar:latest registry.heroku.com/brainwars/web
docker push registry.heroku.com/brainwars/web
#heroku container:push web -a brainwars
heroku container:release web -a brainwars
heroku restart -a brainwars