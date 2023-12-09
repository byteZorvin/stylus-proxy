/**
 * This file was automatically generated by Stylus and represents a Rust program.
 * For more information, please see [The Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs).
 */

interface IProxy {
    function init(address owner) external;

    function getImplementation() external view returns (address);

    function setImplementation(address implementation) external;

    function relayToImplementation(uint8[] memory data) external returns (uint8[] memory);
}
