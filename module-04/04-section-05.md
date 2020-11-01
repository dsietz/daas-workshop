# Section V - service testing

> daas-workshop.postman\_collection.json

A very powerful feature of the `actix-web` crate is the built in service testing that comes out-of-the-box. We utilized this feature in `web-service-tests.rs` as part of our integrated testing. However, we don't need to test the code for the service modules since they have already been tested in the SDKs. Instead, we will smoke test the the RESTful service is working correctly by using an external utility - [Postman](https://github.com/dsietz/daas-workshop/tree/4242659a82c3d0bb5f75f091e77cac8ea4a369c2/docs/reference-postman.md).

You can [import](https://learning.getpostman.com/docs/postman/collections/data_formats/#importing-postman-data) the `daas-workshop.postman_collection.json` file and perform the service tests.

> NOTE: The service should already be running since we had started it in the prior section.

Now you can open the `stage-data` POST request in Postman and monitor the logging on the command line of the running service.

The response payload should be the following:

```text
{
    "status": "OK"
}
```

> **Note** If you get an error message stating "Could not broker document", then make sure the Kafka service is running.

> _TIP_ If you start up a consumer \(e.g.: [Quick Start - Step 5](https://kafka.apache.org/quickstart#quickstart_consume)\), you should see the brokered DaaS object.
>
> ```text
> [kafka@brokerserver kafka_2.12-2.3.0]# bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic order-clothing-iStore --from-beginning
> {"_id":"order|clothing|iStore|8003","_rev":"2-c295ee927aff853b50814447981173e0","source_name":"iStore","source_uid":8003,"category":"order","subcategory":"clothing","author":"istore_app","process_ind":false,"last_updated":1572021078,"data_obj":{"name":"high heals","status":"new"}}
> ```

