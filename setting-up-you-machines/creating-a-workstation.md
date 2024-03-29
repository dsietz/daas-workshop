# Creating a Workstation

{% hint style="info" %}
virtual workstations kindly hosted by [IAPP.org](https://iapp.org)
{% endhint %}

First login to the AWS Console. Open your browser and navigate to the AWS Cloud9 Product page:

[https://us-east-2.console.aws.amazon.com/cloud9/home/product](https://us-east-2.console.aws.amazon.com/cloud9/home/product)

> NOTE: Your instructor will provide you with the login information.

{% hint style="danger" %}
> Make sure you are in the `US East N. Virginia (us-east-1)` region
{% endhint %}

Click on the `Create Environment` button

### Step 1

Enter your name as the `Name` of the new environment.&#x20;

> NOTE: Do not use the Name `The Instructor`

![](../.gitbook/assets/cloud9-01.jpg)

### Step 2

Select the following:

* Environment type: `Create a new EC2 instance for environment (direct access)`
* Instance type: `m5.large`
* Platform: `Amazon Linux2`
* Cost-saving setting:` After a day`
* Network Settings > VPC: `vpc-aws-sandbox`
* Network Settings > Subnet: `subnet-aws-sandbox-DMZ-subnet-A`

![](../.gitbook/assets/cloud9-02.jpg)

![](<../.gitbook/assets/image (2) (1).png>)



{% hint style="warning" %}
Please add the following New Tag before clicking `Next step`
{% endhint %}

![](../.gitbook/assets/image.png)

Click `Next step`

### Step 3

Your review page should look like this (with your own name as the `Name`)

![](<../.gitbook/assets/image (2).png>)

Click `Create environment`

After your environment is created, you should in your virtual workspace.

![](../.gitbook/assets/cloud9-05.jpg)
