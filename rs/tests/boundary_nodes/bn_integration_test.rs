#[rustfmt::skip]

use anyhow::Result;

use ic_boundary_nodes_integration_test_common::{
    asset_canister_test, canister_allowlist_test, canister_routing_test, canister_test,
    denylist_test, direct_to_replica_options_test, direct_to_replica_rosetta_test,
    direct_to_replica_test, headers_test, http_canister_test, http_endpoint_test, ic_gateway_test,
    prefix_canister_id_test, proxy_http_canister_test, read_state_via_subnet_path_test,
    reboot_test, redirect_http_to_https_test, redirect_to_dashboard_test, seo_test,
};
use ic_boundary_nodes_system_test_utils::{
    constants::BOUNDARY_NODE_NAME, helpers::BoundaryNodeHttpsConfig, setup::setup_ic_with_bn,
};
use ic_system_test_driver::{
    driver::group::{SystemTestGroup, SystemTestSubGroup},
    systest,
};

fn main() -> Result<()> {
    let setup = |env| {
        setup_ic_with_bn(
            BOUNDARY_NODE_NAME,
            BoundaryNodeHttpsConfig::AcceptInvalidCertsAndResolveClientSide,
            env,
        )
    };
    SystemTestGroup::new()
        .with_setup(setup)
        .add_parallel(
            SystemTestSubGroup::new()
                .add_test(systest!(canister_test))
                .add_test(systest!(asset_canister_test))
                .add_test(systest!(http_canister_test))
                .add_test(systest!(prefix_canister_id_test))
                .add_test(systest!(proxy_http_canister_test))
                .add_test(systest!(redirect_http_to_https_test))
                .add_test(systest!(redirect_to_dashboard_test))
                .add_test(systest!(http_endpoint_test))
                .add_test(systest!(ic_gateway_test))
                .add_test(systest!(direct_to_replica_test))
                .add_test(systest!(direct_to_replica_rosetta_test))
                .add_test(systest!(direct_to_replica_options_test))
                .add_test(systest!(seo_test))
                .add_test(systest!(canister_routing_test))
                .add_test(systest!(read_state_via_subnet_path_test))
                .add_test(systest!(headers_test)),
        )
        .add_test(systest!(denylist_test))
        .add_test(systest!(canister_allowlist_test))
        .add_test(systest!(reboot_test))
        .execute_from_args()?;

    Ok(())
}
