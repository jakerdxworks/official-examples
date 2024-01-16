use scrypto::prelude::*;

#[blueprint]
mod gumball_machine {
    enable_method_auth! {
        // decide which methods are public and which are restricted to the component's owner
        methods {
            buy_gumball => PUBLIC;
            get_price => PUBLIC;
            set_price => restrict_to: [OWNER];
            withdraw_earnings => restrict_to: [OWNER];
        }
    }
    struct GumballMachine {
        gumballs: Vault,
        collected_xrd: Vault,
        price: Decimal,
    }

    impl GumballMachine {
        // given a price in XRD, creates a ready-to-use gumball machine
        pub fn instantiate_gumball_machine(price: Decimal) -> (Global<GumballMachine>, Bucket) {
            // create a new Owner Badge resource, with a fixed quantity of 1
            let owner_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(init{
                    "name" => "GumballMachine Owner Badge", locked;
                    "symbol" => "OWN", locked;
                }))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1)
                .into();

            // create a new Gumball resource, with an initial supply of 100
            let bucket_of_gumballs: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "Gumball", locked;
                        "symbol" => "GUM", locked;
                        "description" => "A delicious gumball", locked;
                    }
                ))
                .mint_initial_supply(100)
                .into();

            // populate a GumballMachine struct and instantiate a new component
            let component = Self {
                gumballs: Vault::with_bucket(bucket_of_gumballs),
                collected_xrd: Vault::new(XRD),
                price: price,
            }
            .instantiate()
            // Assign the component owner role to the possessor of the owner_badge resource
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(
                owner_badge.resource_address()
            ))))
            .globalize();

            (component, owner_badge)
        }

        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            // take our price in XRD out of the payment
            // if the caller has sent too few, or sent something other than XRD, they'll get a runtime error
            let our_share = payment.take(self.price);
            self.collected_xrd.put(our_share);

            // we could have simplified the above into a single line, like so:
            // self.collected_xrd.put(payment.take(self.price));

            // return a tuple containing a gumball, plus whatever change is left on the input payment (if any)
            // if we're out of gumballs to give, we'll see a runtime error when we try to grab one
            (self.gumballs.take(1), payment)
        }

        pub fn get_price(&self) -> Decimal {
            self.price
        }

        pub fn set_price(&mut self, price: Decimal) {
            // update the price of a gumball. requires the owner badge
            self.price = price
        }

        pub fn withdraw_earnings(&mut self) -> Bucket {
            // retrieve all the XRD collected by the gumball machine component.
            // requires the owner badge
            self.collected_xrd.take_all()
        }
    }
}
