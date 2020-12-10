using System.Threading;
using AOT;
using NUnit.Framework;

namespace CheetahRelay.Tests
{
    [TestFixture]
    public class StructureTest : AbstractTest
    {
        private CheetahBuffer changedData;
        private CheetahObjectId changedObjectId;
        private ushort changedField;

        [Test]
        public void Test()
        {
            CheetahClient.SetCurrentClient(clientB);
            CheetahStructure.SetListener(Listener);

            CheetahClient.SetCurrentClient(clientA);
            var bytes = new CheetahBuffer().Add(1).Add(2).Add(3);
            CheetahStructure.Set(ref objectId, 1, ref bytes);
            Thread.Sleep(100);

            CheetahClient.SetCurrentClient(clientB);
            CheetahClient.Receive();
            Assert.AreEqual(changedData, bytes);
            Assert.AreEqual(changedField, 1);
            Assert.AreEqual(changedObjectId, objectId);
        }

        [MonoPInvokeCallback(typeof(CheetahStructure.Listener))]
        private void Listener(ref CheetahCommandMeta meta, ref CheetahObjectId objectId, ushort fieldId, ref CheetahBuffer data)
        {
            changedData = data;
            changedObjectId = objectId;
            changedField = fieldId;
        }
    }
}