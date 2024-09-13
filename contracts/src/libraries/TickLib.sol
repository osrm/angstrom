// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {LibBit} from "solady/src/utils/LibBit.sol";
import {TICK_SPACING} from "../Constants.sol";

/// @author philogy <https://github.com/philogy>
library TickLib {
    function isInitialized(uint256 word, uint8 bitPos) internal pure returns (bool) {
        return word & (uint256(1) << bitPos) != 0;
    }

    function nextBitPosLte(uint256 word, uint8 bitPos) internal pure returns (bool initialized, uint8 nextBitPos) {
        unchecked {
            uint8 offset = 0xff - bitPos;
            uint256 relativePos = LibBit.fls(word << offset);
            initialized = relativePos != 256;
            nextBitPos = initialized ? uint8(relativePos - offset) : 0;
        }
    }

    function nextBitPosGte(uint256 word, uint8 bitPos) internal pure returns (bool initialized, uint8 nextBitPos) {
        unchecked {
            uint256 relativePos = LibBit.ffs(word >> bitPos);
            initialized = relativePos != 256;
            nextBitPos = initialized ? uint8(relativePos + bitPos) : type(uint8).max;
        }
    }

    function compress(int24 tick) internal pure returns (int24 compressed) {
        assembly {
            compressed := sub(sdiv(tick, TICK_SPACING), slt(smod(tick, TICK_SPACING), 0))
        }
    }

    function position(int24 compressed) internal pure returns (int16 wordPos, uint8 bitPos) {
        unchecked {
            wordPos = int16(compressed >> 8);
            bitPos = uint8(int8(compressed));
        }
    }

    function toTick(int16 wordPos, uint8 bitPos) internal pure returns (int24) {
        return (int24(wordPos) * 256 + int24(uint24(bitPos))) * TICK_SPACING;
    }
}
