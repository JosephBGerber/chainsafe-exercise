// SPDX-License-Identifier: MIT
pragma solidity >=0.4.25 <0.7.0;

// Contract allowing ethereum users to declare an icon that describes themself.
contract Icon {

    mapping(address => uint256) public icons;

    function setIcon(uint256 icon) public {
        icons[msg.sender] = icon;
    }

    function getIcon() public view returns (uint256) {
        return icons[msg.sender];
    }

    function getIcon(address target) public view returns (uint256) {
        return icons[target];
    }
}