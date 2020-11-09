# Section I - Overview

Before we begin constructing our first RESTful service for the DaaS Pattern, we need to first review the architecture overview of the DaaS Pattern. We see in the pattern that our DaaS services interact with:

* **microservices** using persistant data storage, \(we will be using local storage and a S3 bucket\) 
* a **broker** \(we will be using Kafka\)
* a **data pattern**, \(a metadata data model\) 

Together, these components allow the DaaS Pattern to become a reactive architecture.

![](../.gitbook/assets/overview-02.jpg)

Based on these system interactions, and applying Object Oriented Design, we quickly realize that there needs to be multiple `modules` in our library to stand up a DaaS platform.

Fortunately, we won't need to build these modules. Because there are SDKs for the [DaaS](https://crates.io/crates/daas) pattern as well as [Privacy by Design](https://crates.io/crates/pbd), we will not need to build out the underlying _object wrappers,_ _data model_ support, or logic to implement common _privacy strategies_.

