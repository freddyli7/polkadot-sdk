(function() {var implementors = {
"polkadot_test_runtime":[["impl <a class=\"trait\" href=\"staging_xcm_executor/traits/transact_asset/trait.TransactAsset.html\" title=\"trait staging_xcm_executor::traits::transact_asset::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"polkadot_test_runtime/xcm_config/struct.DummyAssetTransactor.html\" title=\"struct polkadot_test_runtime::xcm_config::DummyAssetTransactor\">DummyAssetTransactor</a>"]],
"staging_xcm_builder":[["impl&lt;Assets: <a class=\"trait\" href=\"frame_support/traits/tokens/nonfungibles/trait.Mutate.html\" title=\"trait frame_support::traits::tokens::nonfungibles::Mutate\">Mutate</a>&lt;AccountId&gt; + <a class=\"trait\" href=\"frame_support/traits/tokens/nonfungibles/trait.Transfer.html\" title=\"trait frame_support::traits::tokens::nonfungibles::Transfer\">Transfer</a>&lt;AccountId&gt;, Matcher: MatchesNonFungibles&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/nonfungibles/trait.Inspect.html#associatedtype.CollectionId\" title=\"type frame_support::traits::tokens::nonfungibles::Inspect::CollectionId\">CollectionId</a>, Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/nonfungibles/trait.Inspect.html#associatedtype.ItemId\" title=\"type frame_support::traits::tokens::nonfungibles::Inspect::ItemId\">ItemId</a>&gt;, AccountIdConverter: ConvertLocation&lt;AccountId&gt;, AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>, CheckAsset: <a class=\"trait\" href=\"staging_xcm_builder/trait.AssetChecking.html\" title=\"trait staging_xcm_builder::AssetChecking\">AssetChecking</a>&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/nonfungibles/trait.Inspect.html#associatedtype.CollectionId\" title=\"type frame_support::traits::tokens::nonfungibles::Inspect::CollectionId\">CollectionId</a>&gt;, CheckingAccount: Get&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.74.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;AccountId&gt;&gt;&gt; <a class=\"trait\" href=\"staging_xcm_builder/test_utils/trait.TransactAsset.html\" title=\"trait staging_xcm_builder::test_utils::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"staging_xcm_builder/struct.NonFungiblesAdapter.html\" title=\"struct staging_xcm_builder::NonFungiblesAdapter\">NonFungiblesAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount&gt;"],["impl&lt;Assets: <a class=\"trait\" href=\"frame_support/traits/tokens/nonfungibles/trait.Mutate.html\" title=\"trait frame_support::traits::tokens::nonfungibles::Mutate\">Mutate</a>&lt;AccountId&gt;, Matcher: MatchesNonFungibles&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/nonfungibles/trait.Inspect.html#associatedtype.CollectionId\" title=\"type frame_support::traits::tokens::nonfungibles::Inspect::CollectionId\">CollectionId</a>, Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/nonfungibles/trait.Inspect.html#associatedtype.ItemId\" title=\"type frame_support::traits::tokens::nonfungibles::Inspect::ItemId\">ItemId</a>&gt;, AccountIdConverter: ConvertLocation&lt;AccountId&gt;, AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>, CheckAsset: <a class=\"trait\" href=\"staging_xcm_builder/trait.AssetChecking.html\" title=\"trait staging_xcm_builder::AssetChecking\">AssetChecking</a>&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/nonfungibles/trait.Inspect.html#associatedtype.CollectionId\" title=\"type frame_support::traits::tokens::nonfungibles::Inspect::CollectionId\">CollectionId</a>&gt;, CheckingAccount: Get&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.74.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;AccountId&gt;&gt;&gt; <a class=\"trait\" href=\"staging_xcm_builder/test_utils/trait.TransactAsset.html\" title=\"trait staging_xcm_builder::test_utils::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"staging_xcm_builder/struct.NonFungiblesMutateAdapter.html\" title=\"struct staging_xcm_builder::NonFungiblesMutateAdapter\">NonFungiblesMutateAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount&gt;"],["impl&lt;Assets: <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Mutate.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Mutate\">Mutate</a>&lt;AccountId&gt;, Matcher: MatchesFungibles&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html#associatedtype.AssetId\" title=\"type frame_support::traits::tokens::fungibles::regular::Inspect::AssetId\">AssetId</a>, Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html#associatedtype.Balance\" title=\"type frame_support::traits::tokens::fungibles::regular::Inspect::Balance\">Balance</a>&gt;, AccountIdConverter: ConvertLocation&lt;AccountId&gt;, AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"staging_xcm_builder/test_utils/trait.TransactAsset.html\" title=\"trait staging_xcm_builder::test_utils::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"staging_xcm_builder/struct.FungiblesTransferAdapter.html\" title=\"struct staging_xcm_builder::FungiblesTransferAdapter\">FungiblesTransferAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId&gt;"],["impl&lt;Currency: <a class=\"trait\" href=\"frame_support/traits/tokens/currency/trait.Currency.html\" title=\"trait frame_support::traits::tokens::currency::Currency\">Currency</a>&lt;AccountId&gt;, Matcher: MatchesFungible&lt;Currency::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/currency/trait.Currency.html#associatedtype.Balance\" title=\"type frame_support::traits::tokens::currency::Currency::Balance\">Balance</a>&gt;, AccountIdConverter: ConvertLocation&lt;AccountId&gt;, AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, CheckedAccount: Get&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.74.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;(AccountId, <a class=\"enum\" href=\"staging_xcm_builder/enum.MintLocation.html\" title=\"enum staging_xcm_builder::MintLocation\">MintLocation</a>)&gt;&gt;&gt; <a class=\"trait\" href=\"staging_xcm_builder/test_utils/trait.TransactAsset.html\" title=\"trait staging_xcm_builder::test_utils::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"staging_xcm_builder/struct.CurrencyAdapter.html\" title=\"struct staging_xcm_builder::CurrencyAdapter\">CurrencyAdapter</a>&lt;Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount&gt;"],["impl&lt;Assets: <a class=\"trait\" href=\"frame_support/traits/tokens/nonfungibles/trait.Transfer.html\" title=\"trait frame_support::traits::tokens::nonfungibles::Transfer\">Transfer</a>&lt;AccountId&gt;, Matcher: MatchesNonFungibles&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/nonfungibles/trait.Inspect.html#associatedtype.CollectionId\" title=\"type frame_support::traits::tokens::nonfungibles::Inspect::CollectionId\">CollectionId</a>, Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/nonfungibles/trait.Inspect.html#associatedtype.ItemId\" title=\"type frame_support::traits::tokens::nonfungibles::Inspect::ItemId\">ItemId</a>&gt;, AccountIdConverter: ConvertLocation&lt;AccountId&gt;, AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"staging_xcm_builder/test_utils/trait.TransactAsset.html\" title=\"trait staging_xcm_builder::test_utils::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"staging_xcm_builder/struct.NonFungiblesTransferAdapter.html\" title=\"struct staging_xcm_builder::NonFungiblesTransferAdapter\">NonFungiblesTransferAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId&gt;"],["impl&lt;Assets: <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Mutate.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Mutate\">Mutate</a>&lt;AccountId&gt;, Matcher: MatchesFungibles&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html#associatedtype.AssetId\" title=\"type frame_support::traits::tokens::fungibles::regular::Inspect::AssetId\">AssetId</a>, Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html#associatedtype.Balance\" title=\"type frame_support::traits::tokens::fungibles::regular::Inspect::Balance\">Balance</a>&gt;, AccountIdConverter: ConvertLocation&lt;AccountId&gt;, AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, CheckAsset: <a class=\"trait\" href=\"staging_xcm_builder/trait.AssetChecking.html\" title=\"trait staging_xcm_builder::AssetChecking\">AssetChecking</a>&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html#associatedtype.AssetId\" title=\"type frame_support::traits::tokens::fungibles::regular::Inspect::AssetId\">AssetId</a>&gt;, CheckingAccount: Get&lt;AccountId&gt;&gt; <a class=\"trait\" href=\"staging_xcm_builder/test_utils/trait.TransactAsset.html\" title=\"trait staging_xcm_builder::test_utils::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"staging_xcm_builder/struct.FungiblesMutateAdapter.html\" title=\"struct staging_xcm_builder::FungiblesMutateAdapter\">FungiblesMutateAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount&gt;"],["impl&lt;Assets: <a class=\"trait\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Mutate.html\" title=\"trait frame_support::traits::tokens::fungibles::regular::Mutate\">Mutate</a>&lt;AccountId&gt;, Matcher: MatchesFungibles&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html#associatedtype.AssetId\" title=\"type frame_support::traits::tokens::fungibles::regular::Inspect::AssetId\">AssetId</a>, Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html#associatedtype.Balance\" title=\"type frame_support::traits::tokens::fungibles::regular::Inspect::Balance\">Balance</a>&gt;, AccountIdConverter: ConvertLocation&lt;AccountId&gt;, AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, CheckAsset: <a class=\"trait\" href=\"staging_xcm_builder/trait.AssetChecking.html\" title=\"trait staging_xcm_builder::AssetChecking\">AssetChecking</a>&lt;Assets::<a class=\"associatedtype\" href=\"frame_support/traits/tokens/fungibles/regular/trait.Inspect.html#associatedtype.AssetId\" title=\"type frame_support::traits::tokens::fungibles::regular::Inspect::AssetId\">AssetId</a>&gt;, CheckingAccount: Get&lt;AccountId&gt;&gt; <a class=\"trait\" href=\"staging_xcm_builder/test_utils/trait.TransactAsset.html\" title=\"trait staging_xcm_builder::test_utils::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"staging_xcm_builder/struct.FungiblesAdapter.html\" title=\"struct staging_xcm_builder::FungiblesAdapter\">FungiblesAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount&gt;"]],
"staging_xcm_executor":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()