# Section I - Overview

Before we begin constructing our first RESTful service for the DaaS Pattern, we need to first review the architecture overview of the DaaS Pattern. We see in the pattern that our DaaS services interact with a **database**, \(we will be using CouchDB\) and a **broker** \(we will be using Kafka\). There is also a **data pattern**, \(a metadata data model\) that causes the DaaS pPattern to become a reactive architecture.

\[diagram here\]

Based on these system interactions, and applying Object Oriented Design, we quickly realize that there needs to be three modules:

1. daas \(managing the DaaS data object\)
2. couchdb \(integration with the database\)
3. kafka\(integration with the event broker\)

Each of these modules will act as abstraction layers for our services to interact with the infrastructural components.

Fortunately, we won't need to build these modules. Because there are SDKs for the [DaaS](https://crates.io/crates/daas) pattern as well as [Privacy by Design](https://crates.io/crates/pbd), we will not need to build out the underlying _object wrappers,_ _data model_ support, or logic to implement common _privacy strategies_.

