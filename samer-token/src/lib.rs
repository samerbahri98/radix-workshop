use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        supply: Vault,
        price_per_token: Decimal,
        collected_xrd: Vault,
    }

    impl TokenSale {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(_price_per_token:Decimal) -> (ComponentAddress, Bucket) {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let seller_badge: Bucket = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .metadata("name", "Seller Badge")
            .initial_supply(1);

            let access_rules = AccessRules::new()
            .method("chance_price",rule!(require(seller_badge.resource_address())))
            .method("withdraw_funds",rule!(require(seller_badge.resource_address())))
            .default(rule!(allow_all));

            let token_sale: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "SamerToken")
                .metadata("team-member-1-ticket-number", "41303857096634377329001")
                .initial_supply(100000)
                ;

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
        (Self {

                supply: Vault::with_bucket(token_sale),
                price_per_token: _price_per_token,
                collected_xrd: Vault::new(RADIX_TOKEN)

            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize(),
            seller_badge
        )
        }
        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let xrd_amount : Decimal = funds.amount(); 
            let token_amount: Decimal = xrd_amount / self.price_per_token;
            self.collected_xrd.put(funds);
            self.supply.take(token_amount)
        }
        pub fn change_price(&mut self, price:Decimal)  {
            self.price_per_token = price
        }
    }
}
