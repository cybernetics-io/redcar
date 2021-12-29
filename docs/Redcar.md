# Redcar
![arch](https://github.com/redcar-io/redcar/blob/master/docs/figure/redcar.png)
> Why should we optimize the performance at the software level? Even if we do nothing, 
the performance of our software will double every 18 months.

In the past ten years, relying on the technical dividend of hardware performance
brought by Moore's law, we can focus on the construction of software products and
user experience, which has made great progress in software applications. With
the advent of the post Moore's law era, the performance improvement range of hardware
has gradually shrunk, and there is no longer the advantage of the past. Therefore, in
recent years, the infrastructure and programming tool chain represented by high-performance
software have attracted more and more attention, and this technological revolution will
bring us a new technical architecture upgrade and extreme performance experience, and our
product **Redcar is a connector from desktop to cloud and edge, The asynchronous collaboration
between functions on modern cloud is solved by event driven core model**.

## Backgroud

With the gradual maturity of a number of new technologies such as wasm / Wasi, the performance 
of the web side is getting better and better, and it is possible for the desktop to connect to 
the cloud / edge. On the surface, it reduces the cost of the cloud, but actually puts forward 
more requirements for the cloud or edge, both at the performance and architecture levels. The 
evolution of server-side technology driven by the upgrading of desktop software architecture 
is in progress.

## Overview
Redcar allows you to build event-driven architectures without having to implement, customize, or maintain the underlying infrastructure. Redcar offers a standardized solution to manage the flow of state changes, called events, between decoupled microservices. When triggered, Redcar routes these events through Pub/Sub subscriptions to Cloud Run or Cloud Run for Anthos while managing delivery, security, authorization, observability, and error-handling for you.
An event-driven architecture consists of event producers that generate a stream of events, and 
event consumers that listen for the events.

figure

* Events from Google Cloud sources are either sent directly (Cloud Storage) or through Cloud Audit Logs entries, and use Events Core and Pub/Sub as the transport layer.

* Events from custom sources only use Pub/Sub as the transport layer.

* Events for Cloud Run for Anthos destinations use Event Forwarder to pull new events from Pub/Sub and forward it to the target. This component acts as a mediator between the Pub/Sub transport layer and the target service. It works on existing services and also supports signaling services (including those not exposed outside of the fully-managed cluster) while simplifying setup and maintenance.


You can manage Redcar from the Google Cloud Console (using Cloud Run or Cloud Run for Anthos), from the command line using the Cloud SDK, or by using the Redcar API.

## Key use cases
Redcar supports many use cases for applications running on Cloud Run or Cloud Run for Anthos. Some examples are:
Configure and monitor
* System configuration—Install a configuration management tool on a new VM as soon as it is started.
* Automated remediation—Detect if a service is not responding properly and automatically restart it.
* Alerts and notifications—Monitor the balance of a cryptocurrency wallet address and trigger notifications.

Harmonize
* Directory registrations—Activate an employee badge as soon as a new employee joins a company.
* Data synchronization—Trigger an accounting workflow when a prospect is converted in a CRM system.
* Resource labeling—Label and identify the creator of a VM when it is created.

Analyze
* Sentiment analysis—Use the Cloud Natural Language API to train and deploy an ML model that attaches a satisfaction score to a customer service ticket as soon as it is completed.
* Image retouching and analysis—Remove the background and automatically categorize an image when a retailer adds it to an object store.

## Events
An event is a data record expressing an occurrence and its context. An event is a discrete unit of communication, independent of other events. For example, an event might be a change to data in a database, a file added to a storage system, or a scheduled job.

For a list of the events supported by Redcar, see Events supported by Redcar.

## Event sources

## Event targets

## Event format and libraries

## Redcar triggers

## Trigger location

## Reliability and delivery

## Event retry policy
