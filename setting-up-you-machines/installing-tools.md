# Installing Tools

To help us get setup with the tools and services for this workshop, \(e.g.: Rust, Kafka\), we have built a setup script that will perform all the automated installations and configurations.

### Step 1

In the main terminal at the bottom where it states `ArchConfWorkshopUser:~/environment $` run the following commands - one at a time.

```text
aws s3 cp s3://iapp-archconf-workshop/workshop.sh workshop.sh
```

```text
sudo chmod +x workshop.sh
```

```text
./workshop.sh
```

```text
source $HOME/.cargo/env
```

