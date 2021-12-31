[redcar]: https://github.com/redcar-io/redcar
## Press Release - Redcar

**Redcar: A real-time event-oriented data-hub**

On December 11, 2021, Redcar-io is pleased to announce that we have released 
a real-time event-oriented data-hub product. The goal is to provide a high-performance 
event hosting solution to help users solve the asynchronous problem between 
services (or functions). Hope to improve the energy efficiency of modern clouds 
through the event-driven architecture(EDA) model.

Redcar uses gRPC to provide services, which is user-friendly and has low access 
costs (currently encapsulates `Rust` and `Go` clients). With high-performance **event 
exchange** capabilities, it provides point-to-point event awareness based on the 
traditional event-driven model. Provides transaction processing capabilities and 
data persistence capabilities for streaming events. It adopts a design architecture 
that is more in line with modern clouds.

Improving system performance is an eternal topic of continuous iteration and 
evolution in the technical field. Asynchronization has always been an important 
label of high-performance systems. Decoupling between services through message 
queues is the primary form of application asynchronization. In today's modern 
cloud scenario, the use of static messages to further improve the ability of 
application asynchronization reveals its shortcomings and deficiencies, which 
has become increasingly unable to meet the needs of users. "My system relies on 
a group of external inputs. I want to delegate this message to an intermediate 
system and notify me when there is an action trigger on the message, so that my 
attention can focus on the current business logic. Redcar has helped us do this 
well, and its performance is good enough to give us a lot in service asynchronous 
transformation and technology evolution surprise" said Miguel, head of infrastructure 
at Tick co., ltd.

Let’s use [Redcar][redcar] now. Our products are still evolving. Please tell us if you have 
any suggestions.

##
### FAQ
###
> Who are the users of Redcar?

* Redcar For R&D engineers, it provides users with the asynchronous capability between 
services and continuously improves the overall performance of the application. If you 
have such a demand, you can use this product as a solution.
###

###
> Which scenarios are suitable for Redcar?

* Redcar can be used in cross-cloud event routing, IoT device monitoring,  
and other related fields.
###

###
> What are the similar products in the industry?

* There are Amazon EventBridge/EventBus, Azure Event Hubs, Google Cloud Eventarc and Apache EventMesh.
###

###
> What is the difference between Redcar and Message Queue?

* Message queues deal with static messages, while Redcar is geared towards dynamic events. 
This is the essential difference between them. So their application scenarios are different. 
###

###
> How to ensure the reliability of events?

* Redcar possesses transaction processing and persistence capabilities.
###

###
> Which access methods are currently supported?

* Redcar use gPRC to open interfaces to users and support multiple development languages to access.
###
