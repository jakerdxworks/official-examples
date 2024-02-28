# 20. Radiswap

The Radiswap dApp is the last example in the step-by-step learning journey. It
takes the concepts learned in the previous examples and combines them into a
single, more complex example. The Radiswap dApp is a decentralized exchange
(DEX) that allows users to swap tokens. The version that exists in this
repository is a simplified version of a DEX, but it still demonstrates the core
concepts of a DEX.

### Contents

- [The Radiswap Scrypto Package](#the-radiswap-scrypto-package)
- [The Radiswap Front End](#the-radiswap-front-end)
- [Using the Radiswap Scrypto Package in resim](#using-the-radiswap-scrypto-package-in-resim)
- [Using the Radiswap Front End on Stokenet](#using-the-radiswap-front-end-on-stokenet)

## The Radiswap Scrypto Package

The Radiswap package is a customised wrapper around the standard TwoResourcePool
native blueprint with the addition of a swap function. There are a range of
[native pool blueprints](https://docs.radixdlt.com/docs/pool-component)
available in the Radix Engine, and the TwoResourcePool is one of the most
commonly used. It holds two resources and allows users to deposit and withdraw
from the pool in exchange for Pool Unit resources (often called LP tokens).
Radiswap package extends this functionality to allow users to swap between the
two resources in the pool.

To instantiate a Radiswap component we need to provide 3 arguments:

- `owner_role` - what rule defines the component owner
- `resource_address1` - the first resource in the pool
- `resource_address2` - the second resource in the pool

The first of these is new to us. It's simply the full owner role declaration
that we've been declaring in blueprints before, usually either as
`OwnerRole::None` or using the `rule!` macro and some resource address, e.g.

```rust
OwnerRole::Fixed(rule!(require(
                owner_badge.resource_address()
            )))
```

Now that we've made it an argument we'll need to provide the full role in a
transaction manifest when we instantiate the component. To do that we'll use
some new
[Manifest Value Syntax](https://docs.radixdlt.com/v1/docs/manifest-value-syntax),
instead of the `rule!` shorthand, that works for Scrypto but doesn't in
manifests. This will give us a function call that looks something like this:

```
CALL_FUNCTION
    Address("<PACKAGE_ADDRESS>")
    "Radiswap"
    "new"
    Enum<OwnerRole::Fixed>(
        Enum<AccessRule::Protected>(
            Enum<AccessRuleNode::ProofRule>(
                Enum<ProofRule::Require>(
                    Enum<ResourceOrNonFungible::Resource>(
                        Address("<OWNER_BADGE_ADDRESS>")
                    )
                )
            )
        )
    )
    Address("<RESOURCE_ADDRESS_1>")
    Address("<RESOURCE_ADDRESS_2>")
;
```

Though for no owner we could just put:

```
CALL_FUNCTION
    Address("<PACKAGE_ADDRESS>")
    "Radiswap"
    "new"
Enum<OwnerRole::None>()
    Address("<RESOURCE_ADDRESS_1>")
    Address("<RESOURCE_ADDRESS_2>")
```

## The Radiswap Front End

The Radiswap front end is a web application that allows users to interact with
Radiswap.

## Using the Radiswap Scrypto Package in resim

## Using the Radiswap Front End on Stokenet
