# Further Exploration

### Creating a New Data Flow

Change the url path parameter when calling the sourcing service in the `curl-sourcing.sh` file.

```text
curl --location --request POST 'http://localhost:8000/order/clothing/myStore/7101' \
--header 'Data-Usage-Agreement: [{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]' \
--header 'Content-Type: application/json' \
--header 'Data-Tracker-Chain: W3siaWRlbnRpZmllciI6eyJkYXRhX2lkIjoib3JkZXJ+Y2xvdGhpbmd+aVN0b3JlfjUwMDAiLCJpbmRleCI6MCwidGltZXN0YW1wIjowLCJhY3Rvcl9pZCI6IiIsInByZXZpb3VzX2hhc2giOiIwIn0sImhhc2giOiI3MjI1OTUwMzMyNzI3NjAyMDk1MjEwMjM2ODY3MjE0ODM1ODQ4NSIsIm5vbmNlIjo1fV0=' \
--header 'Authorization: Basic aXN0b3JlX2FwcDpzZWNyZXQ=' \
--data-raw '{
	"product":"leather jacket",
	"quantity": 1,
	"status":"new"
}'
```

### Pushing Data Downstream

### Mutiple Provisioning of a Single Topic

## Helpful Tools

* [Postman Helper](https://github.com/dsietz/daas-sdk/blob/master/examples/postman-helper.rs) - Command line utility to assist in the building of Postman collections for testing your DaaS services

