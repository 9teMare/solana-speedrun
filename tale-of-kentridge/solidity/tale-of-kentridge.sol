@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract tale_of_kentridge {
    event Initilized();
    event PlayerOnboard(address playerAddress, string nickname);
    event PlayerEnqueued(address playerAddress);
    event PlayerDequeued(address playerAddress);
    event JoinRoom(string roomId, address player1, address player2);

    address[] private queue;

    mapping(address => Player) private players;
    mapping(address => Card[]) private playerCards;
    mapping(string => address[]) private rooms;

    struct Player {
        address playerAddress;
        string nickname;
        bool isPlayerExist;
        bool isMatching;
        bool isInGame;
    }

    struct Card {
        bytes32 uniqueId;
        uint8 cardId;
        string cardName;
        string cardDescription;
        string race;
        uint8 attack;
        uint8 health;
        uint8 manaCost;
        uint8 rarity;
    }

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
        public
        view
        returns (string)
    {
        bytes32 roomId = keccak256(abi.encodePacked(player1, player2, block.timestamp));
        return bytes32ToString(roomId);
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
        returns (Card[] memory)
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

    function joinRoom() public {
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
