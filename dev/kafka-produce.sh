if [ $# -lt 1 ]; then
    echo "Usage: $0 <topic_name>"
    exit 1
fi

docker compose -f ./docker/docker-compose-kafka.yaml exec kafka bash -c "kafka-console-producer \
--bootstrap-server kafka:9092 --topic $1"
