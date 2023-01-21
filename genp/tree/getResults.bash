
#docker exec $(docker container ls -aq) grep Client refbox_latest.log
docker exec $(docker container ls -aq) grep PRODUCTION refbox_latest.log | grep "Giving"

