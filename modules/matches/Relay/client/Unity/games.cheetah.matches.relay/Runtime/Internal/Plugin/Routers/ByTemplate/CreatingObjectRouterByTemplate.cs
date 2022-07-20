using Cheetah.Matches.Relay.Internal.Plugin.Routers.ByTemplate.Abstract;
using Cheetah.Matches.Relay.Internal.Plugin.Routers.FFI;
using Cheetah.Matches.Relay.Types;

namespace Cheetah.Matches.Relay.Internal.Plugin.Routers.ByTemplate
{
    /// <summary>
    /// Маршрутизация событий создания объекта с фильтрацией по шаблону
    /// </summary>
    public class CreatingObjectRouterByTemplate : AbstractObjectEventRouterByTemplate
    {
        private ObjectCommandRouter objectCommandRouter;

        public override void Init(CheetahClient client)
        {
            base.Init(client);
            objectCommandRouter = client.GetPlugin<ObjectCommandRouter>();
            objectCommandRouter.ObjectCreatingListener += OnObjectCreating;
        }

        private void OnObjectCreating(ref CheetahObjectId objectId, ushort template)
        {
            Notify(ref objectId);
        }
    }
}