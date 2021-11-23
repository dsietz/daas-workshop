# Section VI - testing the service

{% hint style="warning" %}
If you are not seeing the correct results when calling the services, double check that you are running all the services from the same relative path, (e.g.: `cargo run` vs `./target/debug/...`)&#x20;

The WORKSPACE\_LOCAL\_STORAGE variable is relative (`./workshop_storage`) and therefore depends on where you are running the executable.
{% endhint %}

### Step 1 - Checking the services

Let's first make sure all our services are running and restart our order clothing service.

In a new terminal, run the following command:

```
./rust-daas/target/debug/myapp_order_clothing
```

In another terminal, let's run the sourcing script.

```
./scripts/curl-sourcing.sh 
```

You should see all the services printing to the console about the data they have touched.

#### Sourcing RESTful service

```
[2020-11-09T13:24:06Z INFO  actix_web::middleware::logger] 127.0.0.1:33482 curl/7.61.1
[2020-11-09T13:24:06Z INFO  actix_web::middleware::logger] 127.0.0.1:33482 "POST /order/clothing/iStore/5000 HTTP/1.1" 200 15 "-" "curl/7.61.1" 0.002745
```

#### Genesis service

```
[2020-11-09T13:24:06Z INFO  daas::service::processor] Putting document order~clothing~iStore~5000 in S3
[2020-11-09T13:24:06Z INFO  daas::service::processor] Brokering document order~clothing~iStore~5000 ... 
```

#### Order Clothing service

```
ArchConfWorkshopUser:~/environment $ ./rust-daas/target/debug/myapp_order_clothing 
Clothing Orders processor is running ...
Press [Enter] to stop the Clothing Orders processor.
Order Number 5000 from the iStore has a status of "new"...
Retreiving leather_jacket file
```

### Step 2 - Checking the data storage

There should now be a new directory int he `environment` directory named `workshop_storage`. Inside the `workshop_storage` directory should be a `clothing-leather_jacket.json` file. This is where the aggregated records is stored.

```
ls $HOME/environment/workshop_storage -l
```

```
ArchConfWorkshopUser:~/environment $ ls $HOME/environment/workshop_storage -l
total 4
-rw-rw-r-- 1 ec2-user ec2-user 12 Nov  9 13:24 clothing-leather_jacket.json
```

If we look at the `clothing-leater_jacket.json` the JSON document should look something like this:

```javascript
{"orders":5}
```
