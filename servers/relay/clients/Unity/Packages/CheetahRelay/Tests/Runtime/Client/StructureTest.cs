using System.Threading;
using AOT;
using NUnit.Framework;

namespace CheetahRelay.Tests
{
    [TestFixture]
    public class StructureTest : AbstractCommandTest
    {
        private RelayBuffer changedData;
        private RelayObjectId changedObjectId;
        private ushort changedField;

        [Test]
        public void Test()
        {
            ClientCommands.SetCurrentClient(clientB);
            StructureCommands.SetListener(Listener);

            ClientCommands.SetCurrentClient(clientA);
            var bytes = new RelayBuffer().Add(1).Add(2).Add(3);
            StructureCommands.Set(ref objectId, 1, ref bytes);
            Thread.Sleep(100);

            ClientCommands.SetCurrentClient(clientB);
            ClientCommands.Receive();
            Assert.AreEqual(changedData, bytes);
            Assert.AreEqual(changedField, 1);
            Assert.AreEqual(changedObjectId, objectId);
        }

        [MonoPInvokeCallback(typeof(StructureCommands.Listener))]
        private void Listener(ref CommandMeta meta, ref RelayObjectId objectId, ushort fieldId, ref RelayBuffer data)
        {
            changedData = data;
            changedObjectId = objectId;
            changedField = fieldId;
        }
    }
}