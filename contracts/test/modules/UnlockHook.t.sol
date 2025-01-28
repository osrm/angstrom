// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Angstrom, UnlockHook} from "src/Angstrom.sol";

import {PoolManager} from "v4-core/src/PoolManager.sol";
import {TickMath} from "v4-core/src/libraries/TickMath.sol";
import {Hooks, IHooks} from "v4-core/src/libraries/Hooks.sol";
import {CustomRevert} from "v4-core/src/libraries/CustomRevert.sol";

import {BaseTest} from "test/_helpers/BaseTest.sol";
import {RouterActor, PoolKey} from "test/_mocks/RouterActor.sol";
import {MockERC20} from "super-sol/mocks/MockERC20.sol";

import {console} from "forge-std/console.sol";

/// @author philogy <https://github.com/philogy>
contract UnlookHookTest is BaseTest {
    Angstrom angstrom;
    PoolManager uni;

    address asset0;
    address asset1;

    address controller = makeAddr("controller");
    address node = makeAddr("the_one_node");

    RouterActor actor;

    function setUp() public {
        uni = new PoolManager(address(0));
        angstrom = Angstrom(deployAngstrom(type(Angstrom).creationCode, uni, controller));
        (asset0, asset1) = deployTokensSorted();
        vm.prank(controller);
        angstrom.toggleNodes(addressArray(abi.encode(node)));

        actor = new RouterActor(uni);

        MockERC20(asset0).mint(address(actor), 100_000_000e18);
        MockERC20(asset1).mint(address(actor), 100_000_000e18);
    }

    function test_fuzzing_preventsSwappingBeforeUnlock(uint32 bn) public {
        vm.roll(boundBlock(bn));

        // Create pool.
        vm.prank(controller);
        angstrom.configurePool(asset0, asset1, 60, 0, 0);
        angstrom.initializePool(asset0, asset1, 0, TickMath.getSqrtPriceAtTick(0));
        PoolKey memory pk = poolKey(angstrom, asset0, asset1, 60);
        actor.modifyLiquidity(pk, -60, 60, 100_000e21, bytes32(0));

        vm.expectRevert(
            abi.encodeWithSelector(
                CustomRevert.WrappedError.selector,
                address(angstrom),
                IHooks.beforeSwap.selector,
                bytes.concat(UnlockHook.CannotSwapWhileLocked.selector),
                bytes.concat(Hooks.HookCallFailed.selector)
            )
        );
        actor.swap(pk, true, 1e18, 4295128740);
    }

    function test_fuzzing_swapAfterUnlock(uint32 bn, uint24 unlockedFee, uint256 swapAmount)
        public
    {
        vm.roll(boundBlock(bn));

        unlockedFee = uint24(bound(unlockedFee, 0.01e6, 1.0e6));

        // Create pool.
        vm.prank(controller);
        angstrom.configurePool(asset0, asset1, 60, 0, unlockedFee);
        angstrom.initializePool(asset0, asset1, 0, TickMath.getSqrtPriceAtTick(0));
        PoolKey memory pk = poolKey(angstrom, asset0, asset1, 60);
        actor.modifyLiquidity(pk, -60, 60, 100_000e21, bytes32(0));

        vm.prank(node);
        angstrom.execute("");

        swapAmount = bound(swapAmount, 1e8, 10e18);
        actor.swap(pk, true, -int256(swapAmount), 4295128740);
    }

    function boundBlock(uint32 bn) internal pure returns (uint32) {
        return uint32(bound(bn, 1, type(uint32).max));
    }
}
