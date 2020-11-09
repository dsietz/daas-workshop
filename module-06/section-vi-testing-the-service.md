# Section VI - testing the service

### Step 1 - Checking the services

Let's first make sure all our services are running and restart our reporting service.

In a new terminal, run the following command:

```text
./rust-daas/target/debug/myapp_reporting
```

In another terminal, let's run the sourcing script.

```text
./scripts/curl-sourcing.sh 
```

You should see all the services printing to the console about the data they have touched.

#### Sourcing RESTful service

```text
[2020-11-09T13:24:06Z INFO  actix_web::middleware::logger] 127.0.0.1:33482 curl/7.61.1
[2020-11-09T13:24:06Z INFO  actix_web::middleware::logger] 127.0.0.1:33482 "POST /order/clothing/iStore/5000 HTTP/1.1" 200 15 "-" "curl/7.61.1" 0.002745
```

#### Genesis service

```text
[2020-11-09T13:24:06Z INFO  daas::service::processor] Putting document order~clothing~iStore~5000 in S3
[2020-11-09T13:24:06Z INFO  daas::service::processor] Brokering document order~clothing~iStore~5000 ... 
```

#### Order Clothing service

```text
ArchConfWorkshopUser:~/environment $ ./rust-daas/target/debug/myapp_order_clothing 
Clothing Orders processor is running ...
Press [Enter] to stop the Clothing Orders processor.
Order Number 5000 from the iStore has a status of "new"...
Retreiving leather_jacket file
```

#### Reporting service

```text
ArchConfWorkshopUser:~/environment $ ./rust-daas/target/debug/myapp_reporting
```

### Step 2 - Calling the Reporting RESTful service

Let's first make sure the returned payload is correct based on the resource path.

In an available terminal, run the following script using a specific product

```text
./scripts/curl-reporting.sh -p "leather jacket"
```

> NOTE: The JSON is an object for the specific product.

```javascript
{"orders":6}
```

Now let's make sure the payload is correct when a product is not specified.

```javascript
./scripts/curl-reporting.sh
```

> NOTE: The JSON is an array verses an object

```javascript
[{"orders":6}]
```

