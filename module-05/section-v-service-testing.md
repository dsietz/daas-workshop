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

The genesis service parsed the parameters in the resource path of the RESTful call and created topics accordingly.

`http://localhost:8000/order/clothing/iStore/5000`

This feature allows us to create downstream provisioning services based on the data we wish to process. \(We'll see this in the next module.\)

Let's look at the documents that have been sent to one of these topics.

```text
./kafka_2.13-2.6.0/bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic order.clothing --from-beginning
```

