 // SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleStorage {
    uint256 private data;

    // Setter function to store a value
    function setValue(uint256 _data) public returns(bool){
        data = _data;
        return true;
    }

    // Getter function to retrieve the stored value
    function getValue() public view returns (uint256) {
        return data;
    }
}