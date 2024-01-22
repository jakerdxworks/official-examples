use crate::gumball_machine::gumball_machine::*;
use scrypto::prelude::*;

#[blueprint]
mod candy_store {
    enable_method_auth! {
        // decide which methods are public and which are restricted to certain roles
        methods {
            buy_gumball=> PUBLIC;
            get_prices => PUBLIC;
            set_gumball_price => restrict_to: [OWNER];
            restock_store => restrict_to: [OWNER];
            withdraw_earnings => restrict_to: [OWNER];
        }
    }
    struct CandyStore {
        gumball_machine: Global<GumballMachine>,
        gumball_machine_owner_badges: Vault,
    }

    impl CandyStore {
        // create a new CandyStore component with a price for gumballs
        pub fn instantiate_candy_store(gumball_price: Decimal) -> (Global<CandyStore>, Bucket) {
            let owner_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "Owner Badge", locked;
                        "symbol" => "OWNR", locked;
                    }
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1)
                .into();

            // instantiate a new gumball machine producing both a component and owner badge
            let (gumball_machine, gumball_machine_owner_badge) =
                GumballMachine::instantiate_gumball_machine(gumball_price);

            // populate a CandyStore struct and instantiate a new component
            let component = Self {
                // use shorthand syntax to assign the gumball_machine component to the gumball_machine field
                gumball_machine,
                gumball_machine_owner_badges: Vault::with_bucket(gumball_machine_owner_badge),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(
                owner_badge.resource_address()
            ))))
            .globalize();

            // return the component, plus the owner badge
            (component, owner_badge)
        }

        pub fn get_prices(&self) -> Decimal {
            // get the current price of gumballs by calling the gumball machine's price getter
            let price = self.gumball_machine.get_price();
            info!("Gumball price is {} XRD", price);
            // return the current price
            price
        }

        pub fn buy_gumball(&mut self, payment: Bucket) -> (Bucket, Bucket) {
            // buy a gumball
            self.gumball_machine.buy_gumball(payment)
        }

        pub fn set_gumball_price(&mut self, new_price: Decimal) {
            // use gumball machine owner badge to authorize the method and then set the gumball machine's
            // price. requires owner badge
            self.gumball_machine_owner_badges
                .as_fungible()
                .authorize_with_amount(1, || self.gumball_machine.set_price(new_price));
        }

        pub fn restock_store(&mut self) {
            // use gumball machine owner badge to authorize the method and then refill the gumball machine.
            // requires owner badge
            self.gumball_machine_owner_badges
                .as_fungible()
                .authorize_with_amount(1, || self.gumball_machine.refill_gumball_machine());
        }

        pub fn withdraw_earnings(&mut self) -> Bucket {
            // use gumball machine owner badge to authorize the method and then withdraw all the XRD
            // collected from the gumball machine. requires owner badge
            self.gumball_machine_owner_badges
                .as_fungible()
                .authorize_with_amount(1, || self.gumball_machine.withdraw_earnings())
        }
    }
}
