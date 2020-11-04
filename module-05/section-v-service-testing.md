# Section V - service testing

Since the genesis service is not a RESTful service, we will have to verify that it is successfully running using Kafka consumer and a S3 Bucket lookup.

### Kafka

The genesis service dynamically creates topics based on the metadata of the DaaSDocument that is consumes.

Let's see what topics it has created.

> NOTE: You will want to run this in a new terminal.

```text
./kafka_2.13-2.6.0/bin/kafka-topics.sh --list --zookeeper localhost:2181
```

You should see the following topics in the results.

```text
__consumer_offsets
genesis
iStore
order
order.clothing
order.clothing.iStore
```

