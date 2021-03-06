# cleanish-supers

Super Powered People Microservice written in [RUST](https://www.rust-lang.org/) with cleanish abstractions and layer separation. The layers are a demonstration of how to architect a service with change in mind. In general each layer implementation should be easily replaceable with another implementation of that same layer with idealy no changes to other layers. Each layer should fulfill [Liskov Substitution Principle](https://en.wikipedia.org/wiki/Liskov_substitution_principle).

## Layers

### Presentation

This is the interface to the outside world. This layer provides serialization / rendering / access to data

### Use-cases

Application specific business rules

### Domain Entities

Domain entities represent the core of your application and model the [Bounded Context](https://martinfowler.com/bliki/BoundedContext.html) of your use cases.

### Repositories

Thin abstraction over data stores that encapsulates domain entity storage and retrieval.

### Drivers

Encapsulates communication with external services such as block storage.

## Crates used

* axum
* serde
