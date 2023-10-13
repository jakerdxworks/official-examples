use scrypto::prelude::*;

#[blueprint]
mod the_works {
    struct MintableToken {
        // Store a reference to the mintable token ResourceManager
        the_works_token: ResourceManager,
        init_supply_tokens: Vault,
        init_supply_token_resource_manager: ResourceManager,
    }

    impl MintableToken {
        // It is useful to consider that you can have multiple instantiate functions, each returning a different type of component
        pub fn instantiate_mintable_token() -> (Global<MintableToken>, Bucket) {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(MintableToken::blueprint_id());

            let minter_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(init{"name"=>"minter badge", locked;}))
                .mint_roles(mint_roles! {
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => OWNER;
                })
                .burn_roles(burn_roles! {
                        burner => rule!(deny_all);
                        burner_updater => OWNER;
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(allow_all);
                    withdrawer_updater => rule!(deny_all);
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(deny_all);
                })
                .recall_roles(recall_roles! {
                    recaller => rule!(allow_all);
                    recaller_updater => rule!(deny_all);
                })
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1)
                .into();
            // Create a new mintable token called "lazy_mint_token," with a no initial supply, and store a reference to its ResourceManager
            let the_works_token = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata! {
                    init {
                        "name" => "TheWorksToken", locked;
                        "symbol" => "TWT", locked;
                    }
                })
                .mint_roles(mint_roles! {
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => OWNER;
                })
                .burn_roles(burn_roles! {
                        burner => rule!(deny_all);
                        burner_updater => OWNER;
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(allow_all);
                    withdrawer_updater => rule!(deny_all);
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(deny_all);
                })
                .recall_roles(recall_roles! {
                    recaller => rule!(allow_all);
                    recaller_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            // Create a new token called "mint_init_supply_token," with an initial supply of 1000, and put that supply into a bucket
            let the_works_init_supply_token: Bucket =
                ResourceBuilder::new_fungible(OwnerRole::None)
                    .divisibility(DIVISIBILITY_NONE)
                    .metadata(metadata! {
                        init {
                            "name" => "TheWorksInitSupplyToken", locked;
                            "symbol" => "TWIST", locked;
                        }
                    })
                    .mint_roles(mint_roles! {
                            minter => rule!(require(global_caller(component_address)));
                            minter_updater => OWNER;
                    })
                    .burn_roles(burn_roles! {
                            burner => rule!(deny_all);
                            burner_updater => OWNER;
                    })
                    .withdraw_roles(withdraw_roles! {
                        withdrawer => rule!(allow_all);
                        withdrawer_updater => rule!(deny_all);
                    })
                    .deposit_roles(deposit_roles! {
                        depositor => rule!(allow_all);
                        depositor_updater => rule!(deny_all);
                    })
                    .recall_roles(recall_roles! {
                        recaller => rule!(allow_all);
                        recaller_updater => rule!(deny_all);
                    })
                    .mint_initial_supply(1000)
                    .into();

            // Store a reference to the lazy_mint_token ResourceManager
            // Store a reference to the mint_init_supply_token ResourceManager
            // Store our initial supply of 1000 mint_init_supply_token in a Vault
            let component = Self {
                the_works_token: the_works_token,
                // Note the order below is important. The ResourceManager reference must be stored before moving the mint_init_supply_token into the Vault
                init_supply_token_resource_manager: the_works_init_supply_token.resource_manager(),
                init_supply_tokens: Vault::with_bucket(the_works_init_supply_token),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize();
            return (component, minter_badge);
        }

        // This is an example of a function to mint a mintable token and return a bucket containing the minted tokens
        // minting these tokens requires the minter badge
        pub fn mint_lazy_tokens(&mut self) -> Bucket {
            let lazy_minted_tokens = self.the_works_token.mint(10);
            lazy_minted_tokens
        }

        // Note the difference between accessing the ResourceManager on the token with no supply vs with initial supply
        // in the Self {} block above yet at this level the ResourceManager is accessed the same way as the method above
        // minting these tokens uses the actor virtual badge so you would need to protect this method with an accessrule
        pub fn mint_init_supply_tokens(&mut self) -> Bucket {
            let minted_tokens = self.init_supply_token_resource_manager.mint(100);
            minted_tokens
        }
    }
}