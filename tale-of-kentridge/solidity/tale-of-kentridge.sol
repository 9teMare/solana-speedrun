@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract tale_of_kentridge {
    event Initilized();
    event PlayerOnboarded(address playerAddress, string nickname);
    event PlayerEnqueued(address playerAddress);
    event PlayerDequeued(address playerAddress);
    event MatchFound(bytes32 roomId, address player1, address player2);

    address[] private queue;

    mapping(address => Player) private players;
    mapping(address => Card[]) private playerCards;
    mapping(bytes32 => address[]) private rooms;

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

    function generateRoomId(address player1, address player2)
        public
        pure
        returns (bytes32)
    {
        bytes32 roomId = keccak256(abi.encodePacked(player1, player2));
        return roomId;
    }

    function onboardPlayer(address playerAddress, string nickname) public {
        require(!players[playerAddress].isPlayerExist, "PLAYER_ALREADY_EXIST");
        players[playerAddress] = Player(playerAddress, nickname, true, false, false);
        emit PlayerOnboarded(playerAddress, nickname);
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
        emit PlayerEnqueued(playerAddress);

        if (queue.length == 2) {
            address player1Address = queue[0];
            address player2Address = queue[1];

            bytes32 roomId =
                generateRoomId(player1Address, player2Address);

            rooms[roomId].push(player1Address);
            rooms[roomId].push(player2Address);

            queue.pop();
            queue.pop();

            players[player1Address].isInGame = true;
            players[player2Address].isInGame = true;

            players[player1Address].isMatching = false;
            players[player2Address].isMatching = false;

            emit MatchFound(
                roomId, playerAddress, playerAddress
            );
        }
    }

    function getPlayersInRoom(bytes32 roomId)
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
