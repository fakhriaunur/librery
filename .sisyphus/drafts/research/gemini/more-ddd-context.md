## **1\. The Federation Model (Bounded Contexts)**

Librery avoids the immense complexity of multi-tenancy. Each deployment is a single-tenant instance that considers itself the Local authority. All external branches, campuses, or separate institutions are treated as Federated entities.

### **Domain Concept: The Origin**

Instead of a tenant\_id on every row, entities are scoped by their origin. The core domain only enforces strict local invariants on Local entities.

pub enum Origin {

    Local,

    Federated(FederationId),

}

pub struct Book {

    pub id: Uuid,

    pub origin: Origin,

    pub status: PhysicalStatus,

}

## **2\. The Deterministic Core (Handling Time & UUIDs)**

A pure Functional Core must yield the exact same output given the same input, meaning it cannot generate its own random numbers or timestamps.

### **The Injection Pattern**

The Imperative Shell (Axum backend or Offline TUI) generates the non-deterministic data and injects it into the Command.

// ? BAD: Impure Core (Causes replay failures)

fn checkout(state: \&mut State, book\_id: Uuid) {

    let event\_id \= Uuid::new\_v4(); // IMPURE\!

    let time \= Utc::now();         // IMPURE\!

}

// ? GOOD: Pure Core (Deterministic)

pub struct CheckoutCommand {

    pub book\_id: Uuid,

    pub patron\_id: Uuid,

    pub timestamp: DateTime\<Utc\>, // Injected by the Shell

    pub correlation\_id: Uuid,     // Injected by the Shell

}

// The core simply validates and returns the Event to be saved.

pub fn process\_checkout(state: \&LibraryState, cmd: CheckoutCommand) \-\> Result\<Vec\<Event\>, DomainError\> {

    // Pure logic only...

}

## **3\. Offline-First Trust (Biscuit Tokens & Capabilities)**

When a mobile library van is operating entirely offline, it cannot query the Axum database to verify a librarian's password.

Librery utilizes **Biscuit Tokens** (decentralized, public-key authorization tokens).

1. **The Shell:** The offline TUI verifies the cryptographic signature of the Biscuit token using a locally cached public key.  
2. **The Core:** The Shell extracts the "Capabilities" from the token and passes them to the Core. The Core knows nothing about cryptography; it only evaluates the deterministic capabilities.

// The core only cares about what the token \*allows\*, not the crypto behind it.

pub struct AuthContext {

    pub actor\_id: Uuid,

    pub capabilities: Vec\<Capability\>, 

}

pub enum Capability {

    IssueLoans,

    OverrideFines { max\_amount: u32 },

    ManageCatalog,

}

## **4\. The Actor Taxonomy (Humans vs. Machines)**

To handle SIP2 hardware integrations (RFID gates, self-checkout kiosks) securely, the domain model strictly differentiates between human users and machine systems.

### **The Initiator Enum**

By modeling the actor this way, Rust's pattern matching forces the developer to handle both scenarios whenever a domain rule is evaluated.

pub enum Initiator {

    Human(Uuid),        // A registered Patron or Librarian

    Machine(TerminalId) // A SIP2 Kiosk or RFID Gate

}

pub struct CommandContext {

    pub initiator: Initiator,

    pub auth: AuthContext,

}

// Example Domain Rule Enforcement

fn evaluate\_fine\_override(context: \&CommandContext, amount: u32) \-\> Result\<(), DomainError\> {

    match context.initiator {

        Initiator::Human(\_) \=\> {

            // Check if human has Override capability

            if context.auth.capabilities.contains(\&Capability::OverrideFines { max\_amount: amount }) {

                Ok(())

            } else {

                Err(DomainError::Unauthorized)

            }

        },

        Initiator::Machine(\_) \=\> {

            // A self-checkout machine is NEVER allowed to override a fine, 

            // regardless of capabilities. Pure, unbreakable logic.

            Err(DomainError::MachineCannotOverrideFines)

        }

    }

}
