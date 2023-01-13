
docker exec $(docker container ls -aq) grep PRODUCTION refbox_latest.log | grep "| CYAN"

