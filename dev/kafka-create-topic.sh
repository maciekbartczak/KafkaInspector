if [ $# -lt 1 ]; then
    echo "Usage: $0 <topic_name>"
    exit 1
fi

for topic_name in "$@"
do
    docker compose -f ./docker/docker-compose-kafka.yaml exec kafka bash -c "kafka-topics --create \
    --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1 --topic $topic_name"
done
