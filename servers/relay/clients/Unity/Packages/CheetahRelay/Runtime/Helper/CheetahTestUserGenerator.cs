using System;

namespace CheetahRelay
{
    public static class CheetahTestUserGenerator
    {
        private static uint nextUserPublicKey = (uint) DateTime.Now.Ticks;

        public static UserKeys Generate()
        {
            var keys = new UserKeys();
            keys.publicKey = nextUserPublicKey;
            keys.privateKey = new CheetahBuffer();
            for (var i = 0; i < 32; i++)
            {
                keys.privateKey.Add(5);
            }

            nextUserPublicKey++;
            return keys;
        }

        public struct UserKeys
        {
            public uint publicKey;
            public CheetahBuffer privateKey;
        }
    }
}