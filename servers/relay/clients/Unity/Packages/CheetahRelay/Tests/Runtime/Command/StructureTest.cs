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
            CheetahClient.SetCurrentClient(ClientB);
            CheetahStructure.SetListener(Listener);

            CheetahClient.SetCurrentClient(ClientA);
            var bytes = new CheetahBuffer().Add(1).Add(2).Add(3);
            CheetahStructure.Set(ref ObjectId, 1, ref bytes);
            Thread.Sleep(100);

            CheetahClient.SetCurrentClient(ClientB);
            CheetahClient.Receive();
            Assert.AreEqual(changedData, bytes);
            Assert.AreEqual(changedField, 1);
            Assert.AreEqual(changedObjectId, ObjectId);
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