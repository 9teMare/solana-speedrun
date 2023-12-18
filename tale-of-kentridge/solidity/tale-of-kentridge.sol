contract tale_of_kentridge {
    event Initilized();
    event PlayerOnboard(address playerAddress, string nickname);
    event PlayerEnqueued(address playerAddress);
    event PlayerDequeued(address playerAddress);
    event JoinRoom(string roomId, address player1, address player2);

    address[] private queue;

    struct Player {
        address playerAddress;
        string nickname;
        bool isPlayerExist;
        bool isMatching;
        bool isInGame;
    }

    enum Race { ROYAL, HUMANOID, UNDEAD }

    struct Card {
        uint8 id;
        string name;
        Race race;
        uint8 attack;
        uint8 hp;
        uint8 curr_hp;
        uint8 mana;
        uint8 rarity;
    }

    mapping(address => Player) private players;
    mapping(string => address[]) private rooms;
    mapping(address => Card[30]) private playerCards;

    @payer(payer)
    @seed("seed")
    @space(8000)
    constructor(
        @seed bytes payer, 
        @bump bytes1 bump) {
        emit Initilized();
    }

    function toByte(uint8 _uint8) public pure returns (byte) {
        if(_uint8 < 10) {
            return byte(_uint8 + 48);
        } else {
            return byte(_uint8 + 87);
        }
    }

    function bytes32ToString(bytes32 _bytes32) public pure returns (string memory) {
        uint8 i = 0;
        bytes memory bytesArray = new bytes(64);
        for (i = 0; i < bytesArray.length; i++) {

            uint8 _f = uint8(_bytes32[i/2] & 0x0f);
            uint8 _l = uint8(_bytes32[i/2] >> 4);

            bytesArray[i] = toByte(_f);
            i = i + 1;
            bytesArray[i] = toByte(_l);
        }
        return string(bytesArray);
    }

    function generateRoomId(address player1, address player2)
        private
        view
        returns (string)
    {
        bytes32 roomId = keccak256(abi.encodePacked(player1, player2, block.timestamp));
        return bytes32ToString(roomId);
    }

    function initCards(address playerAddress, string race) public {
        require(players[playerAddress].isPlayerExist, "PLAYER_NOT_EXIST");
        Race _race = race == "ROYAL" ? Race.ROYAL : race == "HUMANOID" ? Race.HUMANOID : Race.UNDEAD;
        Card[30] cards;

        if (_race == Race.ROYAL) {
            require(_race == Race.ROYAL, "WRONG_RACE");
            cards = [
                Card(0, "Royal Knight", Race.ROYAL, 1, 3, 3, 3, 0),
                Card(1, "Royal Knight", Race.ROYAL, 1, 3, 3, 3, 0),
                Card(2, "Royal Knight", Race.ROYAL, 1, 3, 3, 3, 0),

                Card(3, "Royal Champion", Race.ROYAL, 3, 2, 2, 4, 0),
                Card(4, "Royal Champion", Race.ROYAL, 3, 2, 2, 4, 0),
                Card(5, "Royal Champion", Race.ROYAL, 3, 2, 2, 4, 0),

                Card(6, "Royal Priest", Race.ROYAL, 1, 2, 2, 2, 0),
                Card(7, "Royal Priest", Race.ROYAL, 1, 2, 2, 2, 0),
                Card(8, "Royal Priest", Race.ROYAL, 1, 2, 2, 2, 0),

                Card(9, "Royal Soldier", Race.ROYAL, 1, 2, 2, 2, 0),
                Card(10, "Royal Soldier", Race.ROYAL, 1, 2, 2, 2, 0),
                Card(11,  "Royal Soldier", Race.ROYAL, 1, 2, 2, 2, 0),

                Card(12, "Royal Shield", Race.ROYAL, 0, 5, 5, 3, 0),
                Card(13, "Royal Shield", Race.ROYAL, 0, 5, 5, 3, 0),
                Card(14, "Royal Shield", Race.ROYAL, 0, 5, 5, 3, 0),

                Card(15, "Royal Paladin", Race.ROYAL, 3, 4, 4, 5, 2),
                Card(16, "Royal Paladin", Race.ROYAL, 3, 4, 4, 5, 2),
                Card(17, "Royal Paladin", Race.ROYAL, 3, 4, 4, 5, 2),

                Card(18, "Royal Cleric", Race.ROYAL, 1, 3, 3, 3, 0),
                Card(19, "Royal Cleric", Race.ROYAL, 1, 3, 3, 3, 0),
                Card(20, "Royal Cleric", Race.ROYAL, 1, 3, 3, 3, 0),

                Card(21, "Royal Swordman", Race.ROYAL, 2, 1, 1, 2, 0),
                Card(22, "Royal Swordman", Race.ROYAL, 2, 1, 1, 2, 0),
                Card(23, "Royal Swordman", Race.ROYAL, 2, 1, 1, 2, 0),

                Card(24, "Royal Angel", Race.ROYAL, 3, 3, 3, 4, 2),
                Card(25, "Royal Angel", Race.ROYAL, 3, 3, 3, 4, 2),
                Card(26, "Royal Angel", Race.ROYAL, 3, 3, 3, 4, 2),

                Card(27, "Royal Crusade", Race.ROYAL, 6, 1, 1, 6, 0),
                Card(28, "Royal Crusade", Race.ROYAL, 6, 1, 1, 6, 0),
                Card(29, "Royal Crusade", Race.ROYAL, 6, 1, 1, 6, 0)
            ];
        } else if (_race == Race.HUMANOID) {
            require(_race == Race.HUMANOID, "WRONG_RACE");
            cards = [
                Card(30, "Dwarf King", Race.HUMANOID, 1, 3, 3, 3, 0),
                Card(31, "Dwarf King", Race.HUMANOID, 1, 3, 3, 3, 0),
                Card(32, "Dwarf King", Race.HUMANOID, 1, 3, 3, 3, 0),

                Card(33, "Gnoll Grunt", Race.HUMANOID, 3, 2, 2, 4, 0),
                Card(34, "Gnoll Grunt", Race.HUMANOID, 3, 2, 2, 4, 0),
                Card(35, "Gnoll Grunt", Race.HUMANOID, 3, 2, 2, 4, 0),

                Card(36, "Gnome Tinkerer", Race.HUMANOID, 1, 2, 2, 2, 0),
                Card(37, "Gnome Tinkerer", Race.HUMANOID, 1, 2, 2, 2, 0),
                Card(38, "Gnome Tinkerer", Race.HUMANOID, 1, 2, 2, 2, 0),
                
                Card(39, "Halfling Assassin", Race.HUMANOID, 1, 2, 2, 2, 0),
                Card(40, "Halfling Assassin", Race.HUMANOID, 1, 2, 2, 2, 0),
                Card(41, "Halfling Assassin", Race.HUMANOID, 1, 2, 2, 2, 0),

                Card(42, "Lizardfolk Archer", Race.HUMANOID, 0, 5, 5, 3, 0),
                Card(43, "Lizardfolk Archer", Race.HUMANOID, 0, 5, 5, 3, 0),
                Card(44, "Lizardfolk Archer", Race.HUMANOID, 0, 5, 5, 3, 0),

                Card(45, "Orc Juggernaut", Race.HUMANOID, 3, 4, 4, 5, 2),
                Card(46, "Orc Juggernaut", Race.HUMANOID, 3, 4, 4, 5, 2),
                Card(47, "Orc Juggernaut", Race.HUMANOID, 3, 4, 4, 5, 2),

                Card(48, "Goblin Fanatic", Race.HUMANOID, 1, 3, 3, 3, 0),
                Card(49, "Goblin Fanatic", Race.HUMANOID, 1, 3, 3, 3, 0),
                Card(50, "Goblin Fanatic", Race.HUMANOID, 1, 3, 3, 3, 0),

                Card(51, "Goblin Occultist", Race.HUMANOID, 2, 1, 1, 2, 0),
                Card(52, "Goblin Occultist", Race.HUMANOID, 2, 1, 1, 2, 0),
                Card(53, "Goblin Occultist", Race.HUMANOID, 2, 1, 1, 2, 0),

                Card(54, "Halfling Ranger", Race.HUMANOID, 3, 3, 3, 4, 2),
                Card(55, "Halfling Ranger", Race.HUMANOID, 3, 3, 3, 4, 2),
                Card(56, "Halfling Ranger", Race.HUMANOID, 3, 3, 3, 4, 2),

                Card(57, "Gnoll Pikeman", Race.HUMANOID, 6, 1, 1, 6, 0),
                Card(58, "Gnoll Pikeman", Race.HUMANOID, 6, 1, 1, 6, 0),
                Card(59, "Gnoll Pikeman", Race.HUMANOID, 6, 1, 1, 6, 0)
            ];
        } else {
            require(_race == Race.UNDEAD, "WRONG_RACE");
            cards = [
                Card(60, "Bound Cadaver", Race.UNDEAD, 1, 3, 3, 3, 0),
                Card(61, "Bound Cadaver", Race.UNDEAD, 1, 3, 3, 3, 0),
                Card(62, "Bound Cadaver", Race.UNDEAD, 1, 3, 3, 3, 0),

                Card(63, "Brittle Archer", Race.UNDEAD, 3, 2, 2, 4, 0),
                Card(64, "Brittle Archer", Race.UNDEAD, 3, 2, 2, 4, 0),
                Card(65, "Brittle Archer", Race.UNDEAD, 3, 2, 2, 4, 0),

                Card(66, "Carcass Feeder", Race.UNDEAD, 1, 2, 2, 2, 0),
                Card(67, "Carcass Feeder", Race.UNDEAD, 1, 2, 2, 2, 0),
                Card(68, "Carcass Feeder", Race.UNDEAD, 1, 2, 2, 2, 0),
                
                Card(69, "Decrepit Bones", Race.UNDEAD, 1, 2, 2, 2, 0),
                Card(70, "Decrepit Bones", Race.UNDEAD, 1, 2, 2, 2, 0),
                Card(71, "Decrepit Bones", Race.UNDEAD, 1, 2, 2, 2, 0),

                Card(72, "Dismembered Crawler", Race.UNDEAD, 0, 5, 5, 3, 0),
                Card(73, "Dismembered Crawler", Race.UNDEAD, 0, 5, 5, 3, 0),
                Card(74, "Dismembered Crawler", Race.UNDEAD, 0, 5, 5, 3, 0),

                Card(75, "Ghastly Eye", Race.UNDEAD, 3, 4, 4, 5, 2),
                Card(76, "Ghastly Eye", Race.UNDEAD, 3, 4, 4, 5, 2),
                Card(77, "Ghastly Eye", Race.UNDEAD, 3, 4, 4, 5, 2),

                Card(78, "Grave Revenant", Race.UNDEAD, 1, 3, 3, 3, 0),
                Card(79, "Grave Revenant", Race.UNDEAD, 1, 3, 3, 3, 0),
                Card(80, "Grave Revenant", Race.UNDEAD, 1, 3, 3, 3, 0),

                Card(81, "Mutilated Stumbler", Race.UNDEAD, 2, 1, 1, 2, 0),
                Card(82, "Mutilated Stumbler", Race.UNDEAD, 2, 1, 1, 2, 0),
                Card(83, "Mutilated Stumbler", Race.UNDEAD, 2, 1, 1, 2, 0),

                Card(84, "Royal Scarab", Race.UNDEAD, 3, 3, 3, 4, 2),
                Card(85, "Royal Scarab", Race.UNDEAD, 3, 3, 3, 4, 2),
                Card(86, "Royal Scarab", Race.UNDEAD, 3, 3, 3, 4, 2),

                Card(87, "Sand Ghoul", Race.UNDEAD, 6, 1, 1, 6, 0),
                Card(88, "Sand Ghoul", Race.UNDEAD, 6, 1, 1, 6, 0),
                Card(89, "Sand Ghoul", Race.UNDEAD, 6, 1, 1, 6, 0)
            ];
        }
        playerCards[playerAddress] = cards;
    }

    function onboardPlayer(address playerAddress, string nickname) public {
        require(!players[playerAddress].isPlayerExist, "PLAYER_ALREADY_EXIST");
        players[playerAddress] = Player(playerAddress, nickname, true, false, false);

        emit PlayerOnboard(playerAddress, nickname);
    }

    function getPlayer(address playerAddress) public view returns (Player) {
        return players[playerAddress];
    }

    function getPlayerCards(address playerAddress)
        public
        view
        returns (Card[30] memory)
    {
        return playerCards[playerAddress];
    }

    function dequeuePlayer(address playerAddress) public {
        require(players[playerAddress].isPlayerExist, "PLAYER_NOT_EXIST");
        require(!players[playerAddress].isInGame, "PLAYER_IN_GAME");
        players[playerAddress].isInGame = false;
        emit PlayerDequeued(playerAddress);
    }

    function enqueuePlayer(address playerAddress) public {
        require(players[playerAddress].isPlayerExist, "PLAYER_NOT_EXIST");
        require(!players[playerAddress].isInGame, "PLAYER_ALREADY_IN_GAME");
        players[playerAddress].isMatching = true;
        queue.push(playerAddress);
        if (queue.length == 2) {
            joinRoom();
        }
        emit PlayerEnqueued(playerAddress);
    }

    function joinRoom() private {
        address player1Address = queue[0];
        address player2Address = queue[1];

        string roomId =
            generateRoomId(player1Address, player2Address);

        rooms[roomId].push(player1Address);
        rooms[roomId].push(player2Address);

        queue.pop();
        queue.pop();

        players[player1Address].isInGame = true;
        players[player2Address].isInGame = true;

        players[player1Address].isMatching = false;
        players[player2Address].isMatching = false;

        emit JoinRoom(
            roomId, player1Address, player2Address
        );
    }

    function getPlayersInRoom(string roomId)
        public
        view
        returns (address[] memory)
    {
        return rooms[roomId];
    }

    function getQueue() public view returns (address[] memory) {
        return queue;
    }
}
