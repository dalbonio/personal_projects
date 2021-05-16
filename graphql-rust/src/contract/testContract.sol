// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.4 <0.9.0;

contract MyToken {
    
    struct VaultItem {
        bool used;
        string password;
        string login;
    }

    address public minter;
    mapping (address => uint) public balances;
    mapping (address => mapping (string => VaultItem)) public vaults;
    
    constructor(){
        minter = msg.sender;
    }
    
    function mint(address receiver, uint amount) public {
        require(msg.sender == minter);
        balances[receiver] += amount;
    }

    function send(address receiver, uint amount) public {
        require(amount <= balances[msg.sender], "Insufficient amount");
        balances[msg.sender] -= amount;
        balances[receiver] += amount;
    }
    
    function addItemToVault(string memory itemName, string memory login, string memory password) public {
        require(!vaults[msg.sender][itemName].used, "item ja foi criado");
        
        vaults[msg.sender][itemName].used = true;
        vaults[msg.sender][itemName].login = login;
        vaults[msg.sender][itemName].password = password;
    }
}