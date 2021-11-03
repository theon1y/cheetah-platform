using Cheetah.Auth.Cookie;
using Cheetah.Matches.Matchmaking;
using Cheetah.Matches.Relay.Command;
using Cheetah.Platform;
using UnityEngine;

namespace Relay
{
    /// <summary>
    /// Запуск "игры" на relay сервер
    /// </summary>
    public class RelayTestComponent : MonoBehaviour
    {
        private ushort clientId;
        private CheetahObjectId objectA;
        private CheetahObjectId objectB;
        private GRPCConnector grpcConnector;

        private async void OnEnable()
        {
            // устанавливаем связь с кластером
            grpcConnector = new GRPCConnector("127.0.0.1", 7777, false);

            // создаем нового пользователя
            var cookieAuthenticator = new CookieAuthenticator(grpcConnector, "user1");
            cookieAuthenticator.RemoveLocalCookie();
            var loginOrRegister = await cookieAuthenticator.LoginOrRegister();

            // сообщаем mm о желании попасть в битву
            var player = loginOrRegister.Player;
            var ticket = await MatchmakingScheduler.ScheduleUserToMatch(player, "gubaha", 256);

            var privateKey = new CheetahBuffer();
            foreach (var b in ticket.PrivateKey)
            {
                privateKey.Add(b);
            }

            var client = CheetahClient.CreateClient(ticket.RelayGameHost + ":" + ticket.RelayGamePort, (ushort)ticket.UserId, ticket.RoomId,
                ref privateKey, 0, out clientId);
            CheetahClient.SetCurrentClient(clientId);
            CheetahObject.Create(1, 256, ref objectA);
            CheetahObject.Created(ref objectA);
            CheetahObject.Create(100, 256, ref objectB);
            CheetahObject.Created(ref objectB);
        }

        private long counter;

        private void Update()
        {
            if (clientId == 0)
            {
                return;
            }

            CheetahLong.Increment(ref objectA, 2, counter);
            CheetahDouble.Increment(ref objectB, 20, counter);
            CheetahDouble.Increment(ref objectB, 30, 10);
            CheetahClient.Receive();
            counter++;
        }

        private async void OnDestroy()
        {
            await grpcConnector.Destroy();
        }
    }
}